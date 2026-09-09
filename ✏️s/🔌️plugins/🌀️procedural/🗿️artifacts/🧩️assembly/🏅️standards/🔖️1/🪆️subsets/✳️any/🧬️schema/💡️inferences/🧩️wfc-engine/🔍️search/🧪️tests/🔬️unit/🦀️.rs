use super::*;
use crate::wfc_engine::model::ModelBuilder;
use crate::wfc_engine::oracle;
use crate::wfc_engine::topology::GraphTopologyBuilder;

fn checkerboard_topology(n: usize) -> (CompiledModel, crate::wfc_engine::topology::GraphTopology, Vec<oracle::ArcSpec>) {
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let adj = b.add_relation("adjacent");
    b.allow_mirrored(adj, black, white);
    let model = b.compile().unwrap();
    let mut tb = GraphTopologyBuilder::new(n);
    let mut arcs = Vec::new();
    for i in 0..n.saturating_sub(1) {
        let a = NodeId::from_index(i);
        let c = NodeId::from_index(i + 1);
        tb.arc(a, c, adj);
        tb.arc(c, a, adj);
        arcs.push(oracle::ArcSpec { from: a, to: c, relation: adj });
        arcs.push(oracle::ArcSpec { from: c, to: a, relation: adj });
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
fn solves_a_satisfiable_path() {
    let (model, topo, arcs) = checkerboard_topology(6);
    let config = SearchConfig::default();
    let outcome = solve(&model, &topo, &config, 1, None, &[]);
    match outcome {
        SolveOutcome::Solved(sol) => {
            assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok());
        }
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn proves_unsat_on_odd_cycle_with_backtrack_mode() {
    let mut b = ModelBuilder::new();
    let black = b.add_pattern(1.0);
    let white = b.add_pattern(1.0);
    let adj = b.add_relation("adjacent");
    b.allow_mirrored(adj, black, white);
    let model = b.compile().unwrap();
    let mut tb = GraphTopologyBuilder::new(5);
    for i in 0..4 {
        tb.arc(NodeId::from_index(i), NodeId::from_index(i + 1), adj);
        tb.arc(NodeId::from_index(i + 1), NodeId::from_index(i), adj);
    }
    tb.arc(NodeId(4), NodeId(0), adj);
    tb.arc(NodeId(0), NodeId(4), adj);
    let topo = tb.build().unwrap();

    let config = SearchConfig { mode: SearchMode::Backtrack, ..Default::default() };
    let outcome = solve(&model, &topo, &config, 1, None, &[]);
    match outcome {
        SolveOutcome::Unsatisfiable(report) => {
            assert!(report.proven);
            assert_eq!(report.report.model_fingerprint, model.fingerprint());
        }
        other => panic!("expected Unsatisfiable, got {other:?}"),
    }
}

#[test]
fn backtracking_solves_graph_coloring_needing_multiple_decisions() {
    let (model, topo, arcs) = k_graph(4, 4);
    for seed in 0..20 {
        let config = SearchConfig::default();
        let outcome = solve(&model, &topo, &config, seed, None, &[]);
        match outcome {
            SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
            other => panic!("seed {seed}: expected Solved, got {other:?}"),
        }
    }
}

#[test]
fn unsatisfiable_k5_with_four_colors_proves_unsat() {
    let (model, topo, _arcs) = k_graph(5, 4);
    let config = SearchConfig { mode: SearchMode::Backtrack, ..Default::default() };
    let outcome = solve(&model, &topo, &config, 7, None, &[]);
    match outcome {
        SolveOutcome::Unsatisfiable(report) => assert!(report.proven),
        other => panic!("expected Unsatisfiable, got {other:?}"),
    }
}

#[test]
fn backjump_mode_matches_backtrack_completeness() {
    let (model, topo, _arcs) = k_graph(5, 4);
    let config = SearchConfig { mode: SearchMode::Backjump, ..Default::default() };
    let outcome = solve(&model, &topo, &config, 7, None, &[]);
    match outcome {
        SolveOutcome::Unsatisfiable(report) => assert!(report.proven),
        other => panic!("expected Unsatisfiable, got {other:?}"),
    }

    let (model2, topo2, arcs2) = k_graph(4, 4);
    let config2 = SearchConfig { mode: SearchMode::Backjump, ..Default::default() };
    let outcome2 = solve(&model2, &topo2, &config2, 3, None, &[]);
    match outcome2 {
        SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model2, &sol.assignment, &arcs2).is_ok()),
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn fixed_pins_are_respected() {
    let (model, topo, _arcs) = checkerboard_topology(3);
    let config = SearchConfig::default();
    let outcome = solve(&model, &topo, &config, 5, None, &[(NodeId(0), PatternId(1))]);
    match outcome {
        SolveOutcome::Solved(sol) => assert_eq!(sol.assignment[0], PatternId(1)),
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn budget_exceeded_reports_partial_state() {
    // A checkerboard path fully solves after a single decision (propagation alone forces
    // every other node), so the budget must bite before any decision is even attempted.
    let (model, topo, _arcs) = checkerboard_topology(30);
    let config = SearchConfig { budget: Budget { max_observations: Some(0), ..Default::default() }, ..Default::default() };
    let outcome = solve(&model, &topo, &config, 1, None, &[]);
    let SolveOutcome::BudgetExceeded { partial, report } = outcome else {
        panic!("expected BudgetExceeded");
    };
    assert_eq!(report.seed, 1);
    assert_eq!(report.model_fingerprint, model.fingerprint());
    assert_eq!(report.metrics.observations, 0);
    assert_eq!(partial.domains.len(), partial.decided.len());
    assert!(partial.domains.len() <= topo.node_count());
    for (domain, decided) in partial.domains.iter().zip(&partial.decided) {
        if let Some(pattern) = decided {
            assert_eq!(domain.count_ones(), 1);
            assert!(domain.get(*pattern));
        }
    }
}

#[test]
fn same_seed_is_fully_reproducible() {
    let (model, topo, _arcs) = checkerboard_topology(10);
    let config = SearchConfig::default();
    let o1 = solve(&model, &topo, &config, 123, None, &[]);
    let o2 = solve(&model, &topo, &config, 123, None, &[]);
    match (o1, o2) {
        (SolveOutcome::Solved(s1), SolveOutcome::Solved(s2)) => assert_eq!(s1.assignment, s2.assignment),
        _ => panic!("expected both solves to succeed"),
    }
}

#[test]
fn golden_replay_same_seed_reproduces_the_identical_decision_trace() {
    // Determinism at the level of the final assignment (`same_seed_is_fully_reproducible`)
    // is necessary but not sufficient — this checks the exact decision *sequence* two
    // `DiagLevel::Decisions` solves recorded is byte-identical via `TraceReplay`, catching a
    // divergence that happened to still land on the same final assignment by coincidence.
    use crate::wfc_engine::diag::TraceReplay;
    let (model, topo, _arcs) = k_graph(4, 4);
    let config = SearchConfig { diag_level: DiagLevel::Decisions, ..Default::default() };
    let o1 = solve(&model, &topo, &config, 77, None, &[]);
    let o2 = solve(&model, &topo, &config, 77, None, &[]);
    match (o1, o2) {
        (SolveOutcome::Solved(s1), SolveOutcome::Solved(s2)) => {
            let t1 = TraceReplay::from_report(&s1.report);
            let t2 = TraceReplay::from_report(&s2.report);
            assert!(!t1.decisions.is_empty(), "k_graph(4,4) needs at least one real decision");
            assert!(t1.matches(&t2));
        }
        _ => panic!("expected both solves to succeed"),
    }
}

#[test]
fn diag_off_records_no_decision_events_but_summary_and_above_do() {
    let (model, topo, _arcs) = checkerboard_topology(5);
    let off_config = SearchConfig { diag_level: DiagLevel::Off, ..Default::default() };
    let decisions_config = SearchConfig { diag_level: DiagLevel::Decisions, ..Default::default() };

    let off_outcome = solve(&model, &topo, &off_config, 1, None, &[]);
    let decisions_outcome = solve(&model, &topo, &decisions_config, 1, None, &[]);
    match (off_outcome, decisions_outcome) {
        (SolveOutcome::Solved(off_sol), SolveOutcome::Solved(dec_sol)) => {
            assert!(off_sol.report.events.is_empty());
            assert!(dec_sol.report.events.iter().any(|e| matches!(e, Event::Observed { .. })));
            assert!(dec_sol.report.events.iter().any(|e| matches!(e, Event::Solved)));
        }
        _ => panic!("expected both solves to succeed"),
    }
}

#[test]
fn cancellation_stops_search_and_reports_partial() {
    let (model, topo, _arcs) = k_graph(6, 4);
    let cancel = CancelToken::new();
    cancel.cancel();
    let config = SearchConfig::default();
    let outcome = solve_cancellable(&model, &topo, &config, 1, None, &[], &cancel);
    let SolveOutcome::Cancelled { partial, report } = outcome else {
        panic!("expected Cancelled");
    };
    assert_eq!(report.seed, 1);
    assert_eq!(report.model_fingerprint, model.fingerprint());
    assert_eq!(report.metrics.observations, 0);
    assert_eq!(partial.domains.len(), partial.decided.len());
    assert!(partial.domains.len() <= topo.node_count());
    for (domain, decided) in partial.domains.iter().zip(&partial.decided) {
        if let Some(pattern) = decided {
            assert_eq!(domain.count_ones(), 1);
            assert!(domain.get(*pattern));
        }
    }
}

#[test]
fn cancel_token_reflects_state() {
    let cancel = CancelToken::new();
    assert!(!cancel.is_cancelled());
    cancel.cancel();
    assert!(cancel.is_cancelled());
}

#[test]
fn restart_only_never_proves_unsat_on_unsatisfiable_instance() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔍️decoding-and-graphs/🔣️.json")).unwrap();
    let row = &oracle["restart"];
    let base = row["base"].as_u64().unwrap();
    let geometric = RestartSchedule::Geometric { base, factor: row["factor"].as_f64().unwrap() };
    let expected: Vec<u64> = serde_json::from_value(row["budgets"].clone()).unwrap();
    for (attempt, budget) in expected.into_iter().enumerate() {
        assert_eq!(geometric.backtrack_budget(attempt as u64), Some(budget));
    }
    let (model, topo, _arcs) = k_graph(5, 4);
    for restart_schedule in [RestartSchedule::Fixed(base), geometric] {
        let config = SearchConfig { mode: SearchMode::RestartOnly, max_restarts: Some(row["maxRestarts"].as_u64().unwrap()), restart_schedule, ..Default::default() };
        let outcome = solve(&model, &topo, &config, 1, None, &[]);
        let SolveOutcome::Contradiction(failed) = outcome else {
            panic!("expected restart exhaustion");
        };
        assert!(failed.node.index() < topo.node_count());
        assert_eq!(failed.report.seed, 1);
        assert_eq!(failed.report.model_fingerprint, model.fingerprint());
        assert!(failed.report.metrics.restarts > 0);
    }
}

#[test]
fn restart_only_still_solves_satisfiable_instances() {
    let (model, topo, arcs) = k_graph(4, 4);
    let config = SearchConfig { mode: SearchMode::RestartOnly, max_restarts: Some(50), restart_schedule: RestartSchedule::Luby(4), ..Default::default() };
    let outcome = solve(&model, &topo, &config, 1, None, &[]);
    match outcome {
        SolveOutcome::Solved(sol) => assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok()),
        other => panic!("expected Solved, got {other:?}"),
    }
}

#[test]
fn luby_sequence_matches_known_values() {
    let expected = [1, 1, 2, 1, 1, 2, 4, 1, 1, 2, 1, 1, 2, 4, 8];
    for (i, &e) in expected.iter().enumerate() {
        assert_eq!(luby((i + 1) as u64), e, "luby({})", i + 1);
    }
}

#[test]
fn solve_all_finds_every_solution_and_proves_complete() {
    let (model, topo, arcs) = k_graph(3, 3);
    let config = SearchConfig::default();
    let (solutions, complete) = solve_all(&model, &topo, &config, 1, None, &[], 1000);
    assert!(complete);
    assert_eq!(solutions.len(), 6); // 3! proper colorings of K3 with exactly 3 colors
    for sol in &solutions {
        assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok());
    }
    let mut assignments: Vec<_> = solutions.iter().map(|s| s.assignment.clone()).collect();
    assignments.sort();
    assignments.dedup();
    assert_eq!(assignments.len(), 6, "solve_all must not report the same solution twice");
}

#[test]
fn solve_all_on_unsat_instance_returns_empty_and_complete() {
    let (model, topo, _arcs) = k_graph(5, 4);
    let config = SearchConfig::default();
    let (solutions, complete) = solve_all(&model, &topo, &config, 1, None, &[], 1000);
    assert!(complete);
    assert!(solutions.is_empty());
}

#[test]
fn solve_all_respects_limit_and_reports_incomplete() {
    let (model, topo, _arcs) = k_graph(4, 4);
    let config = SearchConfig::default();
    let (solutions, complete) = solve_all(&model, &topo, &config, 1, None, &[], 3);
    assert_eq!(solutions.len(), 3);
    assert!(!complete);
}

#[test]
fn nogood_learning_still_proves_unsat_on_pigeonhole_instance() {
    use crate::wfc_engine::nogood::NogoodConfig;
    let (model, topo, _arcs) = k_graph(5, 4); // K5 needs 5 colors, only 4 available: unsat
    let config = SearchConfig { nogood: NogoodConfig { enabled: true, ..Default::default() }, ..Default::default() };
    let outcome = solve(&model, &topo, &config, 1, None, &[]);
    match outcome {
        SolveOutcome::Unsatisfiable(rep) => assert!(rep.proven),
        other => panic!("expected Unsatisfiable, got {other:?}"),
    }
}

#[test]
fn nogood_learning_survives_restarts_and_still_proves_unsat() {
    use crate::wfc_engine::nogood::NogoodConfig;
    let (model, topo, _arcs) = k_graph(5, 4);
    let config = SearchConfig { mode: SearchMode::RestartOnly, max_restarts: Some(20), restart_schedule: RestartSchedule::Fixed(10), nogood: NogoodConfig { enabled: true, ..Default::default() }, ..Default::default() };
    // RestartOnly never proves unsat by itself (it just gives up) — this exercises nogoods
    // persisting and being re-watched across every restart without ever panicking or
    // corrupting the search, over many independent seeds.
    for seed in 0..10 {
        let outcome = solve(&model, &topo, &config, seed, None, &[]);
        assert!(matches!(outcome, SolveOutcome::Contradiction(_)), "seed {seed}: expected Contradiction, got {outcome:?}");
    }
}

mod quick {
    use super::*;

    #[test]
    fn random_instances_solved_or_proven_unsat_match_oracle() {
        let mut rng = Rng::from_seed(777);
        for trial in 0..100 {
            let pattern_count = 1 + rng.next_range(0, 4) as usize;
            let node_count = 1 + rng.next_range(0, 7) as usize;
            let (model, r) = oracle::testgen::random_model(&mut rng, pattern_count, 0.5);
            let arcs = oracle::testgen::random_arcs(&mut rng, node_count, r);
            let mut tb = GraphTopologyBuilder::new(node_count);
            for a in &arcs {
                tb.arc(a.from, a.to, a.relation);
            }
            let topo = tb.build().unwrap();
            let init_domains = oracle::testgen::full_domains(&model, node_count);

            let oracle_result = oracle::enumerate(&model, node_count, &arcs, &init_domains, 1);
            let config = SearchConfig { mode: SearchMode::Backtrack, ..Default::default() };
            let outcome = solve(&model, &topo, &config, trial as u64, None, &[]);

            match outcome {
                SolveOutcome::Solved(sol) => {
                    assert!(!oracle_result.solutions.is_empty(), "trial {trial}: solver found a solution but oracle found none");
                    assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok(), "trial {trial}: solver's solution violates an arc");
                }
                SolveOutcome::Unsatisfiable(rep) => {
                    assert!(rep.proven);
                    assert!(oracle_result.solutions.is_empty(), "trial {trial}: solver proved unsat but oracle found a solution");
                }
                other => panic!("trial {trial}: unexpected outcome {other:?}"),
            }
        }
    }

    #[test]
    fn random_instances_with_nogoods_enabled_still_match_oracle() {
        use crate::wfc_engine::nogood::NogoodConfig;
        // Same sweep as `random_instances_solved_or_proven_unsat_match_oracle`, but with
        // nogood learning turned on — nogoods are supposed to be a purely redundant pruning
        // layer (see 🦀️nogood.rs's module doc), so this must reach the exact same
        // Solved-or-proven-Unsatisfiable verdict as the oracle on every trial, never a
        // different one.
        let mut rng = Rng::from_seed(778);
        for trial in 0..100 {
            let pattern_count = 1 + rng.next_range(0, 4) as usize;
            let node_count = 1 + rng.next_range(0, 7) as usize;
            let (model, r) = oracle::testgen::random_model(&mut rng, pattern_count, 0.5);
            let arcs = oracle::testgen::random_arcs(&mut rng, node_count, r);
            let mut tb = GraphTopologyBuilder::new(node_count);
            for a in &arcs {
                tb.arc(a.from, a.to, a.relation);
            }
            let topo = tb.build().unwrap();
            let init_domains = oracle::testgen::full_domains(&model, node_count);

            let oracle_result = oracle::enumerate(&model, node_count, &arcs, &init_domains, 1);
            let config = SearchConfig { mode: SearchMode::Backtrack, nogood: NogoodConfig { enabled: true, max_len: 8, max_count: 64 }, ..Default::default() };
            let outcome = solve(&model, &topo, &config, trial as u64, None, &[]);

            match outcome {
                SolveOutcome::Solved(sol) => {
                    assert!(!oracle_result.solutions.is_empty(), "trial {trial}: solver found a solution but oracle found none");
                    assert!(oracle::check_assignment(&model, &sol.assignment, &arcs).is_ok(), "trial {trial}: solver's solution violates an arc");
                }
                SolveOutcome::Unsatisfiable(rep) => {
                    assert!(rep.proven);
                    assert!(oracle_result.solutions.is_empty(), "trial {trial}: solver proved unsat but oracle found a solution");
                }
                other => panic!("trial {trial}: unexpected outcome {other:?}"),
            }
        }
    }

    #[test]
    fn solve_all_matches_oracle_solution_set_on_random_instances() {
        let mut rng = Rng::from_seed(2026);
        for trial in 0..40 {
            let pattern_count = 1 + rng.next_range(0, 4) as usize;
            let node_count = 1 + rng.next_range(0, 6) as usize;
            let (model, r) = oracle::testgen::random_model(&mut rng, pattern_count, 0.6);
            let arcs = oracle::testgen::random_arcs(&mut rng, node_count, r);
            let mut tb = GraphTopologyBuilder::new(node_count);
            for a in &arcs {
                tb.arc(a.from, a.to, a.relation);
            }
            let topo = tb.build().unwrap();
            let init_domains = oracle::testgen::full_domains(&model, node_count);

            let oracle_result = oracle::enumerate(&model, node_count, &arcs, &init_domains, 10_000);
            let config = SearchConfig::default();
            let (solutions, complete) = solve_all(&model, &topo, &config, trial as u64, None, &[], 10_000);

            assert!(complete, "trial {trial}: solve_all did not report complete");
            assert_eq!(complete, oracle_result.complete, "trial {trial}: completeness disagreement");
            let mut got: Vec<Vec<PatternId>> = solutions.iter().map(|s| s.assignment.clone()).collect();
            got.sort();
            let mut want = oracle_result.solutions.clone();
            want.sort();
            assert_eq!(got, want, "trial {trial}: solution set mismatch");
        }
    }
}
