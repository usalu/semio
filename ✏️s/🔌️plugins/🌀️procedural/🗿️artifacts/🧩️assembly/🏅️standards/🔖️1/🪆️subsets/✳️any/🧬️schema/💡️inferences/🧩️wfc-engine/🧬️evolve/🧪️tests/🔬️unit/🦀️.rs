
use super::*;
use crate::wfc_engine::model::ModelBuilder;
use crate::wfc_engine::outcome::SolveOutcome;
use crate::wfc_engine::search::{self, SearchConfig};
use crate::wfc_engine::soft::ScoreFn;
use crate::wfc_engine::topology::GraphTopologyBuilder;

fn checkerboard(n: usize) -> (crate::wfc_engine::model::CompiledModel, crate::wfc_engine::topology::GraphTopology) {
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let adj = b.add_relation("adjacent");
    b.allow_mirrored(adj, black, white);
    let model = b.compile().unwrap();
    let mut tb = GraphTopologyBuilder::new(n);
    for i in 0..n.saturating_sub(1) {
        tb.arc(crate::wfc_engine::ids::NodeId::from_index(i), crate::wfc_engine::ids::NodeId::from_index(i + 1), adj);
        tb.arc(crate::wfc_engine::ids::NodeId::from_index(i + 1), crate::wfc_engine::ids::NodeId::from_index(i), adj);
    }
    (model, tb.build().unwrap())
}

#[test]
fn derive_seed_is_deterministic_and_varies_by_salt() {
    assert_eq!(derive_seed(1, 2), derive_seed(1, 2));
    assert_ne!(derive_seed(1, 2), derive_seed(1, 3));
    assert_ne!(derive_seed(1, 2), derive_seed(4, 2));
}

#[test]
fn evolve_finds_a_solution_and_tracks_evaluated_count() {
    let (model, topo) = checkerboard(6);
    let config = SearchConfig::default();
    let scorer = ScoreFn { name: "count_black", f: |a: &[PatternId]| a.iter().filter(|&&p| p == PatternId(0)).count() as f64 };
    let evolve_config = EvolveConfig { population_size: 4, generations: 3, elite_count: 2 };

    let result = evolve(1, evolve_config, &scorer, |seed| match search::solve(&model, &topo, &config, seed, None, &[]) {
        SolveOutcome::Solved(sol) => Some(sol.assignment),
        _ => None,
    });

    let best = result.best.expect("checkerboard is always satisfiable, every seed should solve");
    assert!(result.evaluated > 0);
    assert!(best.score >= 0.0);
}

#[test]
fn evolve_prefers_higher_scores_across_generations() {
    // A scorer with a clear maximum (3 black nodes is the best achievable on this 5-node
    // path — see the identical checkerboard fixture in 🦀️search.rs's own tests) lets us assert
    // the loop actually converges toward it rather than just returning whatever it finds
    // first.
    let (model, topo) = checkerboard(5);
    let config = SearchConfig::default();
    let scorer = ScoreFn { name: "count_black", f: |a: &[PatternId]| a.iter().filter(|&&p| p == PatternId(0)).count() as f64 };
    let evolve_config = EvolveConfig { population_size: 6, generations: 10, elite_count: 3 };

    let result = evolve(7, evolve_config, &scorer, |seed| match search::solve(&model, &topo, &config, seed, None, &[]) {
        SolveOutcome::Solved(sol) => Some(sol.assignment),
        _ => None,
    });

    let best = result.best.expect("expected at least one solve to succeed");
    assert_eq!(best.score, 3.0, "3 black nodes is the maximum achievable on this 5-node path");
}

#[test]
fn evolve_reports_no_best_when_every_attempt_fails() {
    let scorer = ScoreFn { name: "zero", f: |_: &[PatternId]| 0.0 };
    let evolve_config = EvolveConfig { population_size: 3, generations: 2, elite_count: 1 };

    let result = evolve(1, evolve_config, &scorer, |_seed| -> Option<Vec<PatternId>> { None });
    assert!(result.best.is_none());
    assert_eq!(result.evaluated, 0);
}
