use super::*;
use crate::wfc_engine::weights::WeightTable;

#[test]
fn undo_to_restores_removed_patterns() {
    let w = WeightTable::new(&[1.0, 2.0, 3.0]).unwrap();
    let mut domains = DomainStore::new_full(1, &w);
    let mut trail = Trail::new();
    domains.get_mut(NodeId(0)).remove(PatternId(0), &w);
    trail.record_removed(NodeId(0), PatternId(0));
    domains.get_mut(NodeId(0)).remove(PatternId(1), &w);
    trail.record_removed(NodeId(0), PatternId(1));
    assert_eq!(domains.get(NodeId(0)).cardinality(), 1);

    trail.undo_to(0, &mut domains, &w);
    assert_eq!(domains.get(NodeId(0)).cardinality(), 3);
    domains.get(NodeId(0)).debug_assert_consistent(&w);
}

#[test]
fn undo_to_partial_mark_restores_only_later_entries() {
    let w = WeightTable::new(&[1.0, 1.0, 1.0]).unwrap();
    let mut domains = DomainStore::new_full(1, &w);
    let mut trail = Trail::new();
    domains.get_mut(NodeId(0)).remove(PatternId(0), &w);
    trail.record_removed(NodeId(0), PatternId(0));
    let mark = trail.len();
    domains.get_mut(NodeId(0)).remove(PatternId(1), &w);
    trail.record_removed(NodeId(0), PatternId(1));

    trail.undo_to(mark, &mut domains, &w);
    assert_eq!(domains.get(NodeId(0)).cardinality(), 2); // pattern 1 restored, pattern 0 still gone
    assert!(!domains.get(NodeId(0)).bits().get(PatternId(0)));
    assert!(domains.get(NodeId(0)).bits().get(PatternId(1)));
}

#[test]
fn decision_frames_push_and_pop() {
    let mut trail = Trail::new();
    trail.push_frame(DecisionId(0), NodeId(1), PatternId(2), [1, 2, 3, 4]);
    assert_eq!(trail.depth(), 1);
    let frame = trail.pop_frame().unwrap();
    assert_eq!(frame.node, NodeId(1));
    assert_eq!(frame.candidate, PatternId(2));
    assert_eq!(trail.depth(), 0);
    assert!(trail.pop_frame().is_none());
}

#[test]
fn active_decisions_lists_every_frame_in_order() {
    let mut trail = Trail::new();
    assert!(trail.active_decisions().is_empty());
    trail.push_frame(DecisionId(0), NodeId(1), PatternId(2), [0; 4]);
    trail.push_frame(DecisionId(1), NodeId(3), PatternId(0), [0; 4]);
    assert_eq!(trail.active_decisions(), vec![(NodeId(1), PatternId(2)), (NodeId(3), PatternId(0))]);
    trail.pop_frame();
    assert_eq!(trail.active_decisions(), vec![(NodeId(1), PatternId(2))]);
}
