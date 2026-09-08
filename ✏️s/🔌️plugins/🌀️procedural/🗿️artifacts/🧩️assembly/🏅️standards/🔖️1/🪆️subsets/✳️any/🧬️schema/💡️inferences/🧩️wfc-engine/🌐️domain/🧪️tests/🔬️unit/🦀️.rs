
use super::*;

fn table() -> WeightTable {
    WeightTable::new(&[1.0, 2.0, 4.0, 8.0]).unwrap()
}

#[test]
fn new_full_has_all_patterns_and_correct_sums() {
    let w = table();
    let d = Domain::new_full(&w);
    assert_eq!(d.cardinality(), 4);
    assert_eq!(d.sum_w(), 15.0);
    assert_eq!(d.sum_w_int(), Some(15));
    d.debug_assert_consistent(&w);
}

#[test]
fn restrict_reduces_and_updates_caches() {
    let w = table();
    let mut d = Domain::new_full(&w);
    let mut allowed = PatternSet::new_empty(4);
    allowed.set(PatternId(0), true);
    allowed.set(PatternId(2), true);
    let result = d.restrict(&allowed, &w);
    assert_eq!(result, RestrictResult::Reduced(2));
    assert_eq!(d.cardinality(), 2);
    assert_eq!(d.sum_w(), 5.0);
    d.debug_assert_consistent(&w);
}

#[test]
fn restrict_unchanged_when_already_subset() {
    let w = table();
    let mut d = Domain::new_full(&w);
    let mut allowed = PatternSet::new_empty(4);
    allowed.set(PatternId(0), true);
    d.assign(PatternId(0), &w);
    let result = d.restrict(&allowed, &w);
    assert_eq!(result, RestrictResult::Unchanged);
}

#[test]
fn reduced_count_is_removed_not_remaining() {
    // 6 patterns, remove exactly 1 -> remaining 5. Reduced(_) must report 1, not 5.
    let w = WeightTable::new(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0]).unwrap();
    let mut d = Domain::new_full(&w);
    let result = d.remove(PatternId(0), &w);
    assert_eq!(result, RestrictResult::Reduced(1));
    assert_eq!(d.cardinality(), 5);

    // restrict from 6 down to 3 (remove 3) -> remaining 3. Reduced(_) must report 3.
    let mut d2 = Domain::new_full(&w);
    let mut allowed = PatternSet::new_empty(6);
    allowed.set(PatternId(0), true);
    allowed.set(PatternId(1), true);
    allowed.set(PatternId(2), true);
    let result2 = d2.restrict(&allowed, &w);
    assert_eq!(result2, RestrictResult::Reduced(3));
    assert_eq!(d2.cardinality(), 3);
}

#[test]
fn remove_to_singleton_and_wipeout() {
    let w = WeightTable::new(&[1.0, 1.0]).unwrap();
    let mut d = Domain::new_full(&w);
    let r1 = d.remove(PatternId(0), &w);
    assert_eq!(r1, RestrictResult::Singleton(PatternId(1)));
    let r2 = d.remove(PatternId(1), &w);
    assert_eq!(r2, RestrictResult::Wipeout);
    assert!(d.is_wiped());
}

#[test]
fn assign_forces_single_pattern() {
    let w = table();
    let mut d = Domain::new_full(&w);
    let result = d.assign(PatternId(2), &w);
    assert_eq!(result, RestrictResult::Singleton(PatternId(2)));
    assert_eq!(d.singleton(), Some(PatternId(2)));
    assert_eq!(d.sum_w(), 4.0);
    d.debug_assert_consistent(&w);
}

#[test]
fn re_add_exactly_reverses_remove() {
    let w = table();
    let mut d = Domain::new_full(&w);
    d.remove(PatternId(1), &w);
    assert_eq!(d.cardinality(), 3);
    d.re_add(PatternId(1), &w);
    assert_eq!(d.cardinality(), 4);
    assert_eq!(d.sum_w(), 15.0);
    d.debug_assert_consistent(&w);
}

#[test]
fn singleton_has_zero_entropy_regardless_of_weight() {
    let w = WeightTable::new(&[1.0, 100.0]).unwrap();
    let mut d = Domain::new_full(&w);
    d.assign(PatternId(1), &w);
    assert!(d.entropy().abs() < 1e-9);
}

#[test]
fn wiped_domain_has_zero_entropy() {
    let w = WeightTable::new(&[1.0]).unwrap();
    let mut d = Domain::new_full(&w);
    d.remove(PatternId(0), &w);
    assert_eq!(d.entropy(), 0.0);
}

#[test]
fn uniform_weights_entropy_matches_ln_cardinality() {
    let w = WeightTable::new(&[1.0, 1.0, 1.0, 1.0]).unwrap();
    let d = Domain::new_full(&w);
    assert!((d.entropy() - 4.0f64.ln()).abs() < 1e-9);
}

#[test]
fn domain_store_all_singleton() {
    let w = table();
    let mut store = DomainStore::new_full(2, &w);
    assert!(!store.all_singleton());
    store.get_mut(crate::wfc_engine::ids::NodeId(0)).assign(PatternId(0), &w);
    store.get_mut(crate::wfc_engine::ids::NodeId(1)).assign(PatternId(1), &w);
    assert!(store.all_singleton());
    assert!(!store.any_wiped());
}

mod quick {
    use super::*;

    #[test]
    fn random_remove_re_add_sequences_preserve_invariants() {
        let w = WeightTable::new(&[1.0, 3.0, 5.0, 2.0, 7.0, 1.0, 9.0, 4.0]).unwrap();
        let mut rng = semio_framework_geometry::random::Rng::from_seed(999);
        for _ in 0..100 {
            let mut d = Domain::new_full(&w);
            let mut removed_stack: Vec<PatternId> = Vec::new();
            for _ in 0..50 {
                if d.cardinality() > 1 && rng.next_bool(0.7) {
                    let idx = rng.next_range(0, w.len() as u64) as usize;
                    let p = PatternId::from_index(idx);
                    if d.bits().get(p) {
                        d.remove(p, &w);
                        removed_stack.push(p);
                    }
                } else if let Some(p) = removed_stack.pop() {
                    d.re_add(p, &w);
                }
                d.debug_assert_consistent(&w);
            }
        }
    }

    #[test]
    fn resync_boundary_does_not_change_observable_state() {
        let w = WeightTable::new(&(0..70).map(|i| 1.0 + i as f64).collect::<Vec<_>>()).unwrap();
        let mut d = Domain::new_full(&w);
        // Cross the RESYNC_INTERVAL boundary via single-pattern removals on a large domain.
        for i in 0..70u32 {
            if i % 2 == 0 {
                continue;
            }
            d.remove(PatternId(i), &w);
            d.debug_assert_consistent(&w);
        }
    }
}
