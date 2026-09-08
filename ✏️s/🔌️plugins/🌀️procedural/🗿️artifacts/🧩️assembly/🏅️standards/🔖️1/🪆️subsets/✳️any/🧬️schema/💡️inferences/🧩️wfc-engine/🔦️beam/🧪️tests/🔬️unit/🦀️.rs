
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

fn k_graph(n: usize, k: usize) -> (CompiledModel, crate::wfc_engine::topology::GraphTopology, Vec<oracle::ArcSpec>) {
    let mut b = ModelBuilder::new();
    let patterns: Vec<_> = (0..k).map(|_| b.add_pattern(1.0)).collect();
    let ne = b.add_relation("ne");
    for &a in &patterns {
        for &c in &patterns {
            if a != c {
                b.allow(ne, a, c);
            }
        }
    }
    let model = b.compile().unwrap();
    let mut tb = GraphTopologyBuilder::new(n);
    let mut arcs = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let a = NodeId::from_index(i);
            let c = NodeId::from_index(j);
            tb.arc(a, c, ne);
            tb.arc(c, a, ne);
            arcs.push(oracle::ArcSpec { from: a, to: c, relation: ne });
            arcs.push(oracle::ArcSpec { from: c, to: a, relation: ne });
        }
    }
    (model, tb.build().unwrap(), arcs)
}

#[test]
fn finds_a_valid_solution_on_a_satisfiable_instance() {
    let (model, topo, arcs) = checkerboard(8);
    let config = BeamConfig::default();
    for seed in 0..10 {
        match beam_search(&model, &topo, config, seed, None, &[]) {
            SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok(), "seed {seed}: invalid solution"),
            other => panic!("seed {seed}: expected Solved, got {other:?}"),
        }
    }
}

#[test]
fn finds_a_valid_solution_on_a_harder_coloring_instance() {
    let (model, topo, arcs) = k_graph(4, 4);
    let config = BeamConfig { width: 6, ..Default::default() };
    match beam_search(&model, &topo, config, 7, None, &[]) {
        SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn reports_contradiction_not_a_panic_on_an_unsatisfiable_instance() {
    // K5 needs 5 colors, only 4 available: genuinely unsatisfiable. Beam search must still
    // terminate cleanly and report Contradiction, never claim Solved or panic.
    let (model, topo, _arcs) = k_graph(5, 4);
    let config = BeamConfig { width: 4, max_steps: 50, ..Default::default() };
    let outcome = beam_search(&model, &topo, config, 1, None, &[]);
    assert!(matches!(outcome, SolveOutcome::Contradiction(_)), "expected Contradiction, got {outcome:?}");
}

#[test]
fn width_one_still_finds_a_solution_when_no_backtrack_is_needed() {
    // A checkerboard path never needs more than one live branch at a time (propagation alone
    // forces every other node), so width=1 should still succeed here.
    let (model, topo, arcs) = checkerboard(10);
    let config = BeamConfig { width: 1, ..Default::default() };
    match beam_search(&model, &topo, config, 3, None, &[]) {
        SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn respects_fixed_pins() {
    let (model, topo, _arcs) = checkerboard(4);
    let config = BeamConfig::default();
    match beam_search(&model, &topo, config, 1, None, &[(NodeId(0), PatternId(0))]) {
        SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(0)),
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn contradictory_fixed_pins_report_contradiction_immediately() {
    let (model, topo, _arcs) = checkerboard(2);
    let config = BeamConfig::default();
    // Both nodes pinned to the same pattern is impossible under the mirrored adjacency rule.
    let outcome = beam_search(&model, &topo, config, 1, None, &[(NodeId(0), PatternId(0)), (NodeId(1), PatternId(0))]);
    assert!(matches!(outcome, SolveOutcome::Contradiction(_)));
}
