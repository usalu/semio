use super::*;
use crate::wfc_engine::model_vectors::*;

#[test]
fn checkerboard_path_is_satisfiable_and_alternates() {
    let fx = checkerboard_path(4);
    let result = enumerate(&fx.model, fx.node_count, &fx.arcs, &fx.init_domains, 100);
    assert!(result.complete);
    assert!(!result.solutions.is_empty());
    for sol in &result.solutions {
        assert!(check_assignment(&fx.model, sol, &fx.arcs).is_ok());
        for w in sol.windows(2) {
            assert_ne!(w[0], w[1]);
        }
    }
    // Exactly 2 solutions on a path with 2 colors: BWBW... or WBWB...
    assert_eq!(result.solutions.len(), 2);
}

#[test]
fn unsat_odd_cycle_has_no_solutions() {
    let fx = unsat_odd_cycle(5);
    let result = enumerate(&fx.model, fx.node_count, &fx.arcs, &fx.init_domains, 100);
    assert!(result.complete);
    assert!(result.solutions.is_empty());
}

#[test]
fn even_cycle_is_satisfiable() {
    let mut fx = checkerboard_path(6);
    let adj = RelationId(0);
    fx.arcs.push(ArcSpec { from: NodeId::from_index(5), to: NodeId::from_index(0), relation: adj });
    fx.arcs.push(ArcSpec { from: NodeId::from_index(0), to: NodeId::from_index(5), relation: adj });
    let result = enumerate(&fx.model, fx.node_count, &fx.arcs, &fx.init_domains, 100);
    assert!(result.complete);
    assert_eq!(result.solutions.len(), 2);
}

#[test]
fn complete_graph_coloring_matches_chromatic_condition() {
    let sat = complete_graph_coloring(4, 4);
    let r1 = enumerate(&sat.model, sat.node_count, &sat.arcs, &sat.init_domains, 1000);
    assert!(r1.complete);
    assert!(!r1.solutions.is_empty());
    assert_eq!(r1.solutions.len(), 24); // 4! proper colorings of K4 with exactly 4 colors

    let unsat = complete_graph_coloring(5, 4);
    let r2 = enumerate(&unsat.model, unsat.node_count, &unsat.arcs, &unsat.init_domains, 1000);
    assert!(r2.complete);
    assert!(r2.solutions.is_empty());
}

#[test]
fn check_assignment_detects_violation() {
    let fx = checkerboard_path(3);
    let bad = vec![PatternId(0), PatternId(0), PatternId(1)];
    assert!(check_assignment(&fx.model, &bad, &fx.arcs).is_err());
    let good = vec![PatternId(0), PatternId(1), PatternId(0)];
    assert!(check_assignment(&fx.model, &good, &fx.arcs).is_ok());
}

#[test]
fn limit_caps_collected_solutions() {
    let fx = complete_graph_coloring(4, 4);
    let result = enumerate(&fx.model, fx.node_count, &fx.arcs, &fx.init_domains, 5);
    assert_eq!(result.solutions.len(), 5);
}

mod quick {
    use super::*;

    #[test]
    fn random_instances_every_solution_passes_check_assignment() {
        let mut rng = semio_framework_geometry::random::Rng::from_seed(2024);
        for _ in 0..200 {
            let pattern_count = 1 + rng.next_range(0, 4) as usize;
            let node_count = 1 + rng.next_range(0, 8) as usize;
            let (model, r) = random_model(&mut rng, pattern_count, 0.5);
            let arcs = random_arcs(&mut rng, node_count, r);
            let init_domains = full_domains(&model, node_count);
            let result = enumerate(&model, node_count, &arcs, &init_domains, 50);
            for sol in &result.solutions {
                assert!(check_assignment(&model, sol, &arcs).is_ok());
            }
        }
    }
}
