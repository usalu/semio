use super::*;
use crate::wfc_engine::ids::RegionId;

fn line_adjacency(n: usize) -> AdjacencyView {
    let mut neighbors = vec![Vec::new(); n];
    for i in 0..n.saturating_sub(1) {
        neighbors[i].push(NodeId::from_index(i + 1));
        neighbors[i + 1].push(NodeId::from_index(i));
    }
    AdjacencyView::new(neighbors, vec![RegionId(0); n])
}

#[test]
fn halo_radius_zero_is_just_the_centers() {
    let adj = line_adjacency(5);
    let mut h = halo(&adj, &[NodeId(2)], 0);
    h.sort();
    assert_eq!(h, vec![NodeId(2)]);
}

#[test]
fn halo_expands_by_relation_hops_and_stays_in_bounds() {
    let adj = line_adjacency(5);
    let mut h = halo(&adj, &[NodeId(2)], 1);
    h.sort();
    assert_eq!(h, vec![NodeId(1), NodeId(2), NodeId(3)]);

    // A large radius from an edge node must not panic or duplicate — it just saturates at
    // the whole graph.
    let mut whole = halo(&adj, &[NodeId(0)], 100);
    whole.sort();
    assert_eq!(whole, (0..5).map(NodeId::from_index).collect::<Vec<_>>());
}

#[test]
fn halo_from_multiple_centers_unions_their_neighborhoods() {
    let adj = line_adjacency(7);
    let mut h = halo(&adj, &[NodeId(0), NodeId(6)], 1);
    h.sort();
    assert_eq!(h, vec![NodeId(0), NodeId(1), NodeId(5), NodeId(6)]);
}

#[test]
fn repair_region_reproduces_a_solved_checkerboard_when_reopened_around_one_node() {
    use crate::wfc_engine::model::ModelBuilder;
    use crate::wfc_engine::oracle;
    use crate::wfc_engine::topology::GraphTopologyBuilder;

    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let adj_rel = b.add_relation("adjacent");
    b.allow_mirrored(adj_rel, black, white);
    let model = b.compile().unwrap();

    let n = 6;
    let mut tb = GraphTopologyBuilder::new(n);
    let mut arcs = Vec::new();
    for i in 0..n - 1 {
        tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj_rel);
        tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj_rel);
        arcs.push(oracle::ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(i + 1), relation: adj_rel });
        arcs.push(oracle::ArcSpec { from: NodeId::from_index(i + 1), to: NodeId::from_index(i), relation: adj_rel });
    }
    let topo = tb.build().unwrap();
    let adjacency = crate::wfc_engine::constraint::build_adjacency_view(&topo);

    let config = SearchConfig::default();
    let previous = match search::solve(&model, &topo, &config, 1, None, &[]) {
        SolveOutcome::Solved(sol) => sol.assignment,
        other => panic!("expected an initial Solved baseline, got {other:?}"),
    };

    // Repair around the middle node with a radius wide enough to actually leave it open.
    let outcome = repair_region(&model, &topo, &adjacency, &previous, &[NodeId::from_index(3)], 1, &config, 2);
    match outcome {
        SolveOutcome::Solved(sol) => {
            assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok());
            // Every node strictly outside the radius-1 halo around node 3 must be byte-identical
            // to the pre-repair assignment — repair must never touch what it didn't reopen.
            for i in [0usize, 1, 5] {
                assert_eq!(sol.assignment[i], previous[i], "node {i} outside the halo must be unchanged");
            }
        }
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn repair_region_with_radius_zero_pins_everything_and_reproduces_the_input_exactly() {
    use crate::wfc_engine::model::ModelBuilder;
    use crate::wfc_engine::topology::GraphTopologyBuilder;

    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let adj_rel = b.add_relation("adjacent");
    b.allow_mirrored(adj_rel, black, white);
    let model = b.compile().unwrap();

    let n = 4;
    let mut tb = GraphTopologyBuilder::new(n);
    for i in 0..n - 1 {
        tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj_rel);
        tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj_rel);
    }
    let topo = tb.build().unwrap();
    let adjacency = crate::wfc_engine::constraint::build_adjacency_view(&topo);
    let config = SearchConfig::default();

    let previous = match search::solve(&model, &topo, &config, 1, None, &[]) {
        SolveOutcome::Solved(sol) => sol.assignment,
        other => panic!("expected an initial Solved baseline, got {other:?}"),
    };

    // Radius 0 around a single center still leaves that one node open, but every other node
    // is pinned — with only two nodes total here, node 1 pinned forces node 0 right back to
    // its original value too.
    let outcome = repair_region(&model, &topo, &adjacency, &previous, &[NodeId::from_index(0)], 0, &config, 3);
    match outcome {
        SolveOutcome::Solved(sol) => assert_eq!(sol.assignment, previous),
        other => panic!("expected Solved, got {other:?}"),
    }
}
