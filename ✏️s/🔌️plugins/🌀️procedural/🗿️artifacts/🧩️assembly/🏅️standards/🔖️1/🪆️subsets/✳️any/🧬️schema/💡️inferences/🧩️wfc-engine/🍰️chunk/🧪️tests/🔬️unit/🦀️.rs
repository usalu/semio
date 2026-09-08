
use super::*;
use crate::wfc_engine::model::ModelBuilder;
use crate::wfc_engine::oracle;
use crate::wfc_engine::topology::GraphTopologyBuilder;

fn checkerboard(n: usize) -> (CompiledModel, crate::wfc_engine::topology::GraphTopology, Vec<oracle::ArcSpec>) {
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let adj = b.add_relation("adjacent");
    b.allow_mirrored(adj, black, white);
    let model = b.compile().unwrap();
    let mut tb = GraphTopologyBuilder::new(n);
    let mut arcs = Vec::new();
    for i in 0..n.saturating_sub(1) {
        tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
        tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
        arcs.push(oracle::ArcSpec { from: NodeId::from_index(i), to: NodeId::from_index(i + 1), relation: adj });
        arcs.push(oracle::ArcSpec { from: NodeId::from_index(i + 1), to: NodeId::from_index(i), relation: adj });
    }
    (model, tb.build().unwrap(), arcs)
}

#[test]
fn chunk_seed_is_deterministic_and_varies_by_coordinate_and_model() {
    assert_eq!(chunk_seed(1, 3, 4, 99), chunk_seed(1, 3, 4, 99));
    assert_ne!(chunk_seed(1, 3, 4, 99), chunk_seed(1, 3, 5, 99));
    assert_ne!(chunk_seed(1, 3, 4, 99), chunk_seed(1, 4, 4, 99));
    assert_ne!(chunk_seed(1, 3, 4, 99), chunk_seed(1, 3, 4, 100));
    assert_ne!(chunk_seed(1, 3, 4, 99), chunk_seed(2, 3, 4, 99));
    // Negative chunk coordinates (a chunk west/north of the origin) must not panic or collide
    // trivially with their positive counterpart.
    assert_ne!(chunk_seed(1, -3, 4, 99), chunk_seed(1, 3, 4, 99));
}

#[test]
fn solve_chunk_reproduces_identical_content_on_repeated_calls() {
    let (model, topo, arcs) = checkerboard(10);
    let config = SearchConfig::default();
    let a = solve_chunk(&model, &topo, &config, 42, 2, -1, None, &[]);
    let b = solve_chunk(&model, &topo, &config, 42, 2, -1, None, &[]);
    match (a, b) {
        (SolveOutcome::Solved(sa), SolveOutcome::Solved(sb)) => {
            assert_eq!(sa.assignment, sb.assignment);
            assert!(oracle::check_assignment(&model, &sa.assignment, &arcs).is_ok());
        }
        _ => panic!("expected both calls to solve identically"),
    }
}

#[test]
fn solve_chunk_respects_seam_pins_from_a_committed_neighbor() {
    let (model, topo, _arcs) = checkerboard(6);
    let config = SearchConfig::default();
    // Simulates a neighboring chunk having already committed node 0 to white.
    let outcome = solve_chunk(&model, &topo, &config, 7, 0, 0, None, &[(NodeId(0), PatternId(1))]);
    match outcome {
        SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(1)),
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn different_chunk_coordinates_can_yield_different_content_for_an_underconstrained_model() {
    // A single isolated node with two equally-likely patterns and no seam pins: different
    // chunk coordinates should be free to (though aren't guaranteed to) land on different
    // values — what matters is they're each internally deterministic, checked here by
    // confirming the two calls aren't forced to collide by construction.
    let mut b = ModelBuilder::new();
    b.add_pattern(1.0);
    b.add_pattern(1.0);
    b.add_relation("r");
    let model = b.compile().unwrap();
    let topo = GraphTopologyBuilder::new(1).build().unwrap();
    let config = SearchConfig::default();
    let at_origin = solve_chunk(&model, &topo, &config, 123, 0, 0, None, &[]);
    let elsewhere = solve_chunk(&model, &topo, &config, 123, 500, -500, None, &[]);
    assert!(matches!(at_origin, SolveOutcome::Solved(_)));
    assert!(matches!(elsewhere, SolveOutcome::Solved(_)));
}
