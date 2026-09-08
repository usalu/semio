
use super::*;
use crate::wfc_engine::model::ModelBuilder;

fn two_pattern_model() -> CompiledModel {
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let r = b.add_relation("adj");
    b.allow_mirrored(r, black, white);
    b.allow_mirrored(r, black, black);
    b.compile().unwrap()
}

fn adjacency_line(n: usize) -> AdjacencyView {
    let mut neighbors = vec![Vec::new(); n];
    for i in 0..n.saturating_sub(1) {
        neighbors[i].push(NodeId::from_index(i + 1));
        neighbors[i + 1].push(NodeId::from_index(i));
    }
    AdjacencyView::new(neighbors, vec![RegionId(0); n])
}

#[test]
fn rejects_invalid_bounds() {
    let model = two_pattern_model();
    assert!(CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 5, 2).is_err());
}

#[test]
fn validate_complete_accepts_matching_count() {
    let model = two_pattern_model();
    let adjacency = adjacency_line(4);
    let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 1, 2).unwrap();
    let assignment = vec![PatternId(0), PatternId(1), PatternId(0), PatternId(1)];
    assert!(c.validate_complete(&assignment, &adjacency).is_ok());
}

#[test]
fn validate_complete_rejects_out_of_bounds_count() {
    let model = two_pattern_model();
    let adjacency = adjacency_line(4);
    let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 0, 1).unwrap();
    let assignment = vec![PatternId(0), PatternId(0), PatternId(0), PatternId(1)];
    assert!(c.validate_complete(&assignment, &adjacency).is_err());
}

#[test]
fn initialize_forces_pattern_when_min_equals_possible() {
    let model = two_pattern_model();
    let adjacency = adjacency_line(2);
    let weights = model.weights().clone();
    let mut domains = DomainStore::new_full(2, &weights);
    // Restrict node1 so only node0 can possibly satisfy "at least 1 pattern0".
    let mut white_only = PatternSet::new_empty(2);
    white_only.set(PatternId(1), true);
    domains.get_mut(NodeId(1)).restrict(&white_only, &weights);

    let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 1, 2).unwrap();
    let restrictions = c.initialize(&domains, &weights, &adjacency).unwrap();
    assert!(restrictions.iter().any(|(n, set)| *n == NodeId(0) && set.get(PatternId(0)) && !set.get(PatternId(1))));
}

#[test]
fn initialize_signals_infeasible_min_via_empty_domain() {
    let model = two_pattern_model();
    let adjacency = adjacency_line(1);
    let weights = model.weights().clone();
    let domains = DomainStore::new_full(1, &weights);
    // Impossible: need at least 2 nodes matching pattern0, but only 1 node total.
    let c = CardinalityConstraint::new(model, PatternSelector::Pattern(PatternId(0)), Scope::All, 2, 2).unwrap();
    let restrictions = c.initialize(&domains, &weights, &adjacency).unwrap();
    assert!(restrictions.iter().any(|(_, set)| set.is_all_zero()));
}
