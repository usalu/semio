
use super::*;
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

/// A 5-node "H" shape: 0-1-2, 2-3, 2-4 (so node2 is a semio_hub).
fn hub_adjacency() -> AdjacencyView {
    let edges = [(0, 1), (1, 2), (2, 3), (2, 4)];
    let mut neighbors = vec![Vec::new(); 5];
    for &(a, b) in &edges {
        neighbors[a].push(NodeId::from_index(b));
        neighbors[b].push(NodeId::from_index(a));
    }
    AdjacencyView::new(neighbors, vec![crate::wfc_engine::ids::RegionId(0); 5])
}

#[test]
fn connectivity_accepts_single_connected_component() {
    let model = floor_wall_model();
    let adjacency = hub_adjacency();
    let c = ConnectivityConstraint::new(model, PatternSelector::Pattern(PatternId(0)));
    // floor at 0,1,2 (connected through the semio_hub), wall elsewhere.
    let assignment = vec![PatternId(0), PatternId(0), PatternId(0), PatternId(1), PatternId(1)];
    assert!(c.validate_complete(&assignment, &adjacency).is_ok());
}

#[test]
fn connectivity_rejects_split_components() {
    let model = floor_wall_model();
    let adjacency = hub_adjacency();
    let c = ConnectivityConstraint::new(model, PatternSelector::Pattern(PatternId(0)));
    // floor at 0 and at 3+4, split by wall at the semio_hub (node2) and node1.
    let assignment = vec![PatternId(0), PatternId(1), PatternId(1), PatternId(0), PatternId(0)];
    assert!(c.validate_complete(&assignment, &adjacency).is_err());
}

#[test]
fn connectivity_trivially_accepts_zero_or_one_selected() {
    let model = floor_wall_model();
    let adjacency = hub_adjacency();
    let c = ConnectivityConstraint::new(model, PatternSelector::Pattern(PatternId(0)));
    let all_wall = vec![PatternId(1); 5];
    assert!(c.validate_complete(&all_wall, &adjacency).is_ok());
}

#[test]
fn reachability_accepts_connected_path() {
    let model = floor_wall_model();
    let adjacency = hub_adjacency();
    let c = ReachabilityConstraint::new(model, vec![NodeId(0)], vec![NodeId(3), NodeId(4)], PatternSelector::Pattern(PatternId(0)));
    let assignment = vec![PatternId(0), PatternId(0), PatternId(0), PatternId(0), PatternId(0)];
    assert!(c.validate_complete(&assignment, &adjacency).is_ok());
}

#[test]
fn reachability_rejects_blocked_path() {
    let model = floor_wall_model();
    let adjacency = hub_adjacency();
    let c = ReachabilityConstraint::new(model, vec![NodeId(0)], vec![NodeId(3)], PatternSelector::Pattern(PatternId(0)));
    // Wall at the semio_hub (node2) blocks the only path from 0 to 3.
    let assignment = vec![PatternId(0), PatternId(0), PatternId(1), PatternId(0), PatternId(0)];
    assert!(c.validate_complete(&assignment, &adjacency).is_err());
}
