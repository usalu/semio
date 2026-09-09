use super::*;
use crate::wfc_engine::ids::PatternId;
use crate::wfc_engine::weights::WeightTable;

#[test]
fn mrv_picks_smallest_domain() {
    let w = WeightTable::new(&[1.0, 1.0, 1.0, 1.0]).unwrap();
    let mut store = DomainStore::new_full(3, &w);
    store.get_mut(NodeId(1)).remove(PatternId(0), &w); // node 1: cardinality 3
    store.get_mut(NodeId(2)).remove(PatternId(0), &w);
    store.get_mut(NodeId(2)).remove(PatternId(1), &w); // node 2: cardinality 2
    let picked = select_unresolved(ObserveHeuristic::Mrv, &store).unwrap();
    assert_eq!(picked, NodeId(2));
}

#[test]
fn ties_break_by_ascending_node_id() {
    let w = WeightTable::new(&[1.0, 1.0]).unwrap();
    let store = DomainStore::new_full(3, &w);
    let picked = select_unresolved(ObserveHeuristic::Mrv, &store).unwrap();
    assert_eq!(picked, NodeId(0));
}

#[test]
fn singleton_and_resolved_domains_are_skipped() {
    let w = WeightTable::new(&[1.0, 1.0]).unwrap();
    let mut store = DomainStore::new_full(2, &w);
    store.get_mut(NodeId(0)).assign(PatternId(0), &w);
    let picked = select_unresolved(ObserveHeuristic::Mrv, &store).unwrap();
    assert_eq!(picked, NodeId(1));
}

#[test]
fn none_when_all_singleton() {
    let w = WeightTable::new(&[1.0]).unwrap();
    let store = DomainStore::new_full(2, &w);
    assert!(select_unresolved(ObserveHeuristic::Mrv, &store).is_none());
}

#[test]
fn weighted_entropy_prefers_lower_entropy_domain() {
    let w = WeightTable::new(&[1.0, 1.0, 1.0, 1.0]).unwrap();
    let mut store = DomainStore::new_full(2, &w);
    // node 1 has cardinality 2 (lower entropy) after one removal; node 0 stays at cardinality 4.
    store.get_mut(NodeId(1)).remove(PatternId(0), &w);
    store.get_mut(NodeId(1)).remove(PatternId(1), &w);
    let picked = select_unresolved(ObserveHeuristic::WeightedEntropy, &store).unwrap();
    assert_eq!(picked, NodeId(1));
}
