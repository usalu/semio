
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

fn k_graph(n: usize, k: usize) -> (CompiledModel, crate::wfc_engine::topology::GraphTopology) {
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
    for i in 0..n {
        for j in (i + 1)..n {
            tb.arc(NodeId::from_index(i), NodeId::from_index(j), ne);
            tb.arc(NodeId::from_index(j), NodeId::from_index(i), ne);
        }
    }
    (model, tb.build().unwrap())
}

#[test]
fn derive_seed_is_deterministic_and_varies_by_index() {
    assert_eq!(derive_seed(42, 3), derive_seed(42, 3));
    assert_ne!(derive_seed(42, 0), derive_seed(42, 1));
    assert_ne!(derive_seed(42, 5), derive_seed(43, 5));
}

#[test]
fn multi_start_finds_a_valid_solution() {
    let (model, topo, arcs) = checkerboard(20);
    let config = SearchConfig::default();
    let outcome = multi_start(&model, &topo, &config, 1, None, &[], 4);
    match outcome {
        SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn multi_start_proves_unsat_when_every_attempt_would() {
    let (model, topo) = k_graph(5, 4); // pigeonhole unsat regardless of seed
    let config = SearchConfig::default();
    let outcome = multi_start(&model, &topo, &config, 7, None, &[], 6);
    match outcome {
        SolveOutcome::Unsatisfiable(rep) => assert!(rep.proven),
        other => panic!("expected Unsatisfiable, got {other:?}"),
    }
}

#[test]
fn multi_start_is_deterministic_across_repeated_calls() {
    let (model, topo, _arcs) = checkerboard(12);
    let config = SearchConfig::default();
    let a = multi_start(&model, &topo, &config, 99, None, &[], 8);
    let b = multi_start(&model, &topo, &config, 99, None, &[], 8);
    match (a, b) {
        (SolveOutcome::Solved(sa), SolveOutcome::Solved(sb)) => assert_eq!(sa.assignment, sb.assignment),
        _ => panic!("expected both calls to solve"),
    }
}

#[test]
fn single_attempt_matches_a_plain_solve_call() {
    let (model, topo, _arcs) = checkerboard(10);
    let config = SearchConfig::default();
    let direct = search::solve(&model, &topo, &config, derive_seed(5, 0), None, &[]);
    let via_multi_start = multi_start(&model, &topo, &config, 5, None, &[], 1);
    match (direct, via_multi_start) {
        (SolveOutcome::Solved(sa), SolveOutcome::Solved(sb)) => assert_eq!(sa.assignment, sb.assignment),
        _ => panic!("expected both to solve identically"),
    }
}
