
use super::*;

fn from_indices(len: usize, idxs: &[usize]) -> PatternSet {
    let mut s = PatternSet::new_empty(len);
    for &i in idxs {
        s.set(PatternId::from_index(i), true);
    }
    s
}

#[test]
fn empty_and_full() {
    let e = PatternSet::new_empty(10);
    assert!(e.is_all_zero());
    assert_eq!(e.count_ones(), 0);
    let f = PatternSet::new_full(10);
    assert_eq!(f.count_ones(), 10);
    for i in 0..10 {
        assert!(f.get(PatternId::from_index(i)));
    }
}

#[test]
fn full_respects_boundary_bits() {
    // 70 patterns spans two words; the second word must not have stray set bits above 70.
    let f = PatternSet::new_full(70);
    assert_eq!(f.count_ones(), 70);
}

#[test]
fn set_get_roundtrip() {
    let mut s = PatternSet::new_empty(5);
    s.set(PatternId::from_index(2), true);
    assert!(s.get(PatternId::from_index(2)));
    assert!(!s.get(PatternId::from_index(3)));
    s.set(PatternId::from_index(2), false);
    assert!(!s.get(PatternId::from_index(2)));
}

#[test]
fn and_or_and_not() {
    let a = from_indices(8, &[0, 1, 2, 3]);
    let b = from_indices(8, &[2, 3, 4, 5]);

    let mut and_result = a.clone();
    and_result.and_with(&b);
    assert_eq!(and_result.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![2, 3]);

    let mut or_result = a.clone();
    or_result.or_with(&b);
    assert_eq!(or_result.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 5]);

    let mut sub = a;
    sub.and_not_with(&b);
    assert_eq!(sub.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![0, 1]);
}

#[test]
fn first_set_and_iter_ones_skip_zero_words() {
    let s = from_indices(200, &[130, 199]);
    assert_eq!(s.first_set().unwrap().index(), 130);
    assert_eq!(s.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![130, 199]);
}

#[test]
fn subset_and_intersects() {
    let a = from_indices(8, &[0, 1]);
    let b = from_indices(8, &[0, 1, 2]);
    assert!(a.is_subset_of(&b));
    assert!(!b.is_subset_of(&a));
    assert!(a.intersects(&b));
    let c = from_indices(8, &[6, 7]);
    assert!(!a.intersects(&c));
}

#[test]
fn restrict_returning_removed_matches_naive() {
    let mut s = from_indices(10, &[0, 1, 2, 3, 4]);
    let allowed = from_indices(10, &[2, 3, 5]);
    let mut removed = PatternSet::new_empty(10);
    let count = s.restrict_returning_removed(&allowed, &mut removed);
    assert_eq!(count, 3); // 0, 1, 4 removed
    assert_eq!(s.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![2, 3]);
    assert_eq!(removed.iter_ones().map(|p| p.index()).collect::<Vec<_>>(), vec![0, 1, 4]);
}

#[test]
fn restrict_no_change_returns_zero() {
    let mut s = from_indices(6, &[1, 2]);
    let allowed = PatternSet::new_full(6);
    let mut removed = PatternSet::new_empty(6);
    let count = s.restrict_returning_removed(&allowed, &mut removed);
    assert_eq!(count, 0);
    assert!(removed.is_all_zero());
}

#[test]
fn freshly_built_sets_are_well_formed() {
    assert!(PatternSet::new_empty(0).is_well_formed());
    assert!(PatternSet::new_empty(70).is_well_formed());
    assert!(PatternSet::new_full(70).is_well_formed());
    assert!(from_indices(200, &[130, 199]).is_well_formed());
}

#[test]
fn wrong_word_count_is_not_well_formed() {
    let mut s = from_indices(70, &[10]);
    s.words.push(0); // one extra word beyond what 70 patterns needs
    assert!(!s.is_well_formed());
}

#[test]
fn stray_bits_past_len_in_final_word_are_not_well_formed() {
    let mut s = from_indices(10, &[2]);
    s.words[0] |= 1 << 20; // bit 20 is past `len = 10`, still within the single backing word
    assert!(!s.is_well_formed());
}

#[test]
fn serde_round_trip_preserves_bits_and_len() {
    let s = from_indices(70, &[3, 64, 69]);
    let json = protocol::json::to_json_string(&s);
    let back: PatternSet = protocol::json::from_json_str(&json).unwrap();
    assert_eq!(back, s);
    assert!(back.is_well_formed());
}

mod quick {
    use super::*;

    #[test]
    fn random_and_or_matches_vec_bool_model() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(12345);
        for _ in 0..200 {
            let len = 1 + (rng.next_range(0, 200) as usize);
            let mut model_a = vec![false; len];
            let mut model_b = vec![false; len];
            let mut a = PatternSet::new_empty(len);
            let mut b = PatternSet::new_empty(len);
            for i in 0..len {
                if rng.next_bool(0.5) {
                    model_a[i] = true;
                    a.set(PatternId::from_index(i), true);
                }
                if rng.next_bool(0.5) {
                    model_b[i] = true;
                    b.set(PatternId::from_index(i), true);
                }
            }
            let mut and_r = a.clone();
            and_r.and_with(&b);
            for i in 0..len {
                assert_eq!(and_r.get(PatternId::from_index(i)), model_a[i] && model_b[i]);
            }
            let mut or_r = a.clone();
            or_r.or_with(&b);
            for i in 0..len {
                assert_eq!(or_r.get(PatternId::from_index(i)), model_a[i] || model_b[i]);
            }
            let expected_count = model_a.iter().filter(|&&x| x).count() as u32;
            assert_eq!(a.count_ones(), expected_count);
        }
    }
}
