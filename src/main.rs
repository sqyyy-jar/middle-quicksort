#![feature(is_sorted)]

pub fn quick_sort(array: &mut [i64]) {
    if array.len() <= 1 {
        return;
    }
    let partition_index = partition(array);
    quick_sort(&mut array[0..partition_index]);
    quick_sort(&mut array[partition_index + 1..]);
}

fn partition(array: &mut [i64]) -> usize {
    let mut pivot_index = (array.len() - 1) / 2;
    let pivot = array[pivot_index];
    let mut left = 0;
    let mut right = array.len() - 1;
    while left < right {
        if array[left] > pivot {
            // avoid unnecessary swaps
            if array[right] > pivot {
                right -= 1;
                continue;
            }
            if right == pivot_index {
                pivot_index = left;
            }
            array.swap(left, right);
            right -= 1;
        } else {
            left += 1;
        }
    }
    // pivot is not in the "middle"
    if left != pivot_index {
        // element in the "middle"
        let element = array[left];
        // pivot is on the right
        if left < pivot_index {
            if element > pivot {
                // swap pivot with left
                array.swap(pivot_index, left);
                pivot_index = left;
            } else {
                // swap pivot with element after left
                array.swap(pivot_index, left + 1);
                pivot_index = left + 1;
            }
        } else
        // pivot is on the left
        if element < pivot {
            // swap pivot with left
            array.swap(pivot_index, left);
            pivot_index = left;
        } else {
            // swap pivot with element before left
            array.swap(pivot_index, left - 1);
            pivot_index = left - 1;
        }
    }
    pivot_index
}

fn main() {
    let mut array = [10, 1, 9, 2, 8, 3, 7, 4, 6, 5];
    quick_sort(&mut array);
    println!("{array:?}");
}

#[cfg(test)]
mod test {
    use rand::{seq::SliceRandom, thread_rng};

    use crate::quick_sort;

    #[test]
    fn test_sorted_array() {
        let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        quick_sort(&mut array);
        assert!(array.is_sorted(), "{array:?}");
    }

    #[test]
    fn test_inverse_array() {
        let mut array = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        quick_sort(&mut array);
        assert!(array.is_sorted(), "{array:?}");
    }

    #[test]
    fn test_varying_array() {
        let mut array = [10, 1, 9, 2, 8, 3, 7, 4, 6, 5];
        quick_sort(&mut array);
        assert!(array.is_sorted(), "{array:?}");
    }

    #[test]
    fn test_rand_arrays() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let mut array: Vec<i64> = (1..=100).collect();
            array.shuffle(&mut rng);
            quick_sort(&mut array);
            assert!(array.is_sorted(), "{array:?}");
        }
    }
}
