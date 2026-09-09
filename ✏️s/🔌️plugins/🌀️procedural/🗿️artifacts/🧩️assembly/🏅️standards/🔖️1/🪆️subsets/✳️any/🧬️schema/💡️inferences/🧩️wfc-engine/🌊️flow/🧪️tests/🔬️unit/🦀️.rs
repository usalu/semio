use super::*;
use crate::wfc_engine::ids::RegionId;
use crate::wfc_engine::model::ModelBuilder;

fn floor_wall_model() -> CompiledModel {
    let mut b = ModelBuilder::new();
    let floor = b.add_pattern(1.0);
    let wall = b.add_pattern(1.0);
    let r = b.add_relation("adj");
    b.allow_mirrored(r, floor, floor);
    b.allow_mirrored(r, floor, wall);
    b.allow_mirrored(r, wall, wall);
    b.compile().unwrap()
}

fn line_adjacency(n: usize) -> AdjacencyView {
    let mut neighbors = vec![Vec::new(); n];
    for i in 0..n.saturating_sub(1) {
        neighbors[i].push(NodeId::from_index(i + 1));
        neighbors[i + 1].push(NodeId::from_index(i));
    }
    AdjacencyView::new(neighbors, vec![RegionId(0); n])
}

/// A 2-wide, 3-long ladder: two parallel floor corridors from node0/node1 (side by side) to
/// node4/node5, letting up to 2 edge-disjoint paths exist when everything is floor.
fn ladder_adjacency() -> AdjacencyView {
    let edges = [(0, 2), (2, 4), (1, 3), (3, 5)];
    let mut neighbors = vec![Vec::new(); 6];
    for &(a, b) in &edges {
        neighbors[a].push(NodeId::from_index(b));
        neighbors[b].push(NodeId::from_index(a));
    }
    AdjacencyView::new(neighbors, vec![RegionId(0); 6])
}

#[test]
fn single_path_meets_flow_of_one() {
    let model = floor_wall_model();
    let adjacency = line_adjacency(4);
    let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0)], vec![NodeId(3)], 1);
    let all_floor = vec![PatternId(0); 4];
    assert!(c.validate_complete(&all_floor, &adjacency).is_ok());
}

#[test]
fn a_wall_blocking_the_only_path_fails_flow_of_one() {
    let model = floor_wall_model();
    let adjacency = line_adjacency(4);
    let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0)], vec![NodeId(3)], 1);
    let blocked = vec![PatternId(0), PatternId(0), PatternId(1), PatternId(0)];
    assert!(c.validate_complete(&blocked, &adjacency).is_err());
}

#[test]
fn ladder_with_both_corridors_open_meets_flow_of_two() {
    let model = floor_wall_model();
    let adjacency = ladder_adjacency();
    let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0), NodeId(1)], vec![NodeId(4), NodeId(5)], 2);
    let all_floor = vec![PatternId(0); 6];
    assert!(c.validate_complete(&all_floor, &adjacency).is_ok());
}

#[test]
fn ladder_with_one_corridor_walled_off_fails_flow_of_two() {
    let model = floor_wall_model();
    let adjacency = ladder_adjacency();
    let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0), NodeId(1)], vec![NodeId(4), NodeId(5)], 2);
    // node2 (the middle of the top corridor) is wall: only 1 edge-disjoint path survives.
    let one_corridor = vec![PatternId(0), PatternId(0), PatternId(1), PatternId(0), PatternId(0), PatternId(0)];
    assert!(c.validate_complete(&one_corridor, &adjacency).is_err());
}

#[test]
fn source_not_selected_yields_zero_flow() {
    let model = floor_wall_model();
    let adjacency = line_adjacency(3);
    let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0)], vec![NodeId(2)], 1);
    let source_is_wall = vec![PatternId(1), PatternId(0), PatternId(0)];
    assert!(c.validate_complete(&source_is_wall, &adjacency).is_err());
}

#[test]
fn max_flow_never_exceeds_the_number_of_distinct_source_sink_edges() {
    // A direct single-edge line: max possible flow is 1, regardless of how large min_flow's
    // check demands — this just exercises the network construction terminates and is exact.
    let model = floor_wall_model();
    let adjacency = line_adjacency(2);
    let c = FlowConstraint::new(model, PatternSelector::Pattern(PatternId(0)), vec![NodeId(0)], vec![NodeId(1)], 1);
    let assignment = vec![PatternId(0), PatternId(0)];
    assert_eq!(c.compute_max_flow(&assignment, &adjacency), 1);
}
