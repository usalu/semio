//! ⏱️ Latency gate for the architecture §5.7 evaluation rows on a 1 000-member document: the cold first evaluation (own budget, one sample), the commit of one entity whose load changes in every round, and the paint over a settled session.

use super::tests::*;
use super::*;
use std::time::Instant;

#[test]
fn one_entity_commit_meets_the_commit_budget() {
    let mut doc = base();
    doc.members = (0..1000).map(|i| Member { id: format!("m{i}"), section: format!("s{}", i % 4), load: 10.0 }).collect();
    let started = Instant::now();
    let mut session = EvaluationSession::<Mock>::open(doc.clone());
    let plan = session.plan(&ChangedSet::all(), &[], session.token());
    session.run(&plan, &mut |_| {});
    let cold = started.elapsed().as_secs_f64() * 1000.0;
    eprintln!("[latency] one-entity-commit/cold p50_ms={cold:.3} p95_ms={cold:.3} budget_ms=1000.0");
    assert!(cold < 1000.0, "cold first evaluation {cold} ms exceeds the 1000 ms cold budget");
    let mut samples = Vec::new();
    for round in 0..40 {
        doc.members[500].load += 1.0;
        let started = Instant::now();
        let plan = session.update(doc.clone(), &ChangedSet::at(["members.500.load"], vec![]), &[]);
        session.run(&plan, &mut |_| {});
        samples.push(started.elapsed().as_secs_f64() * 1000.0);
        assert_eq!(plan.units.len(), 1, "round {round}");
    }
    samples.sort_by(f64::total_cmp);
    let at = |percent: usize| samples[(samples.len() * percent / 100).min(samples.len() - 1)];
    let (p50, p95) = (at(50), at(95));
    eprintln!("[latency] one-entity-commit/commit p50_ms={p50:.3} p95_ms={p95:.3} budget_ms=400.0");
    assert!(p95 < 400.0, "p95 {p95} ms exceeds the 400 ms commit budget");
}

#[test]
fn a_responsive_paint_over_a_settled_session_is_far_below_the_commit_budget() {
    let shared = Arc::new(SharedEvaluation::<Mock>::new(Doc::default()));
    let doc = base();
    shared.report_for(&doc);
    let mut samples = Vec::new();
    for _ in 0..100 {
        let started = Instant::now();
        assert!(shared.background(&doc).is_none() && shared.settled());
        let _ = shared.report_now();
        samples.push(started.elapsed().as_secs_f64() * 1000.0);
    }
    samples.sort_by(f64::total_cmp);
    let p95 = samples[95];
    let budget = if cfg!(debug_assertions) { 100.0 } else { 5.0 };
    eprintln!("[latency] settled-paint/paint p50_ms={:.3} p95_ms={p95:.3} budget_ms={budget:.1}", samples[50]);
    assert!(p95 < budget, "p95 {p95} ms: a paint over an unchanged document must not re-evaluate");
}

#[test]
fn a_repeated_read_of_a_settled_thousand_member_session_copies_and_merges_nothing() {
    let mut doc = base();
    doc.members = (0..1000).map(|i| Member { id: format!("m{i}"), section: format!("s{}", i % 4), load: 10.0 }).collect();
    let shared = SharedEvaluation::<Mock>::new(Doc::default());
    let settled = shared.report_for(&doc);
    let mut samples = Vec::new();
    for _ in 0..100 {
        let started = Instant::now();
        let again = shared.report_for(&doc);
        samples.push(started.elapsed().as_secs_f64() * 1000.0);
        assert!(Arc::ptr_eq(&settled, &again), "the same allocation, no merge and no clone per tick");
    }
    samples.sort_by(f64::total_cmp);
    let p95 = samples[95];
    let budget = if cfg!(debug_assertions) { 100.0 } else { 2.0 };
    eprintln!("[latency] settled-report-read-1000-members/paint p50_ms={:.3} p95_ms={p95:.3} budget_ms={budget:.1}", samples[50]);
    assert!(p95 < budget, "p95 {p95} ms: reading a settled report must be a handle clone plus the hash checks");
}

#[derive(Clone, PartialEq)]
struct Thousand(Doc);

impl ToValue for Thousand {
    fn to_value(&self) -> DslValue {
        self.0.to_value()
    }
}

#[test]
fn a_further_reader_of_a_thousand_member_revision_pays_a_compare_not_a_conversion() {
    let mut doc = base();
    doc.members = (0..1000).map(|i| Member { id: format!("m{i}"), section: format!("s{}", i % 4), load: 10.0 + i as f64 }).collect();
    let doc = Thousand(doc);
    let first = shared_value_tree(&doc);
    let mut samples = Vec::new();
    for _ in 0..100 {
        let reader = doc.clone();
        let started = Instant::now();
        let again = shared_value_tree(&reader);
        samples.push(started.elapsed().as_secs_f64() * 1000.0);
        assert!(Arc::ptr_eq(&first, &again), "the scene fingerprints read the tree the session built");
    }
    samples.sort_by(f64::total_cmp);
    let p95 = samples[95];
    let budget = if cfg!(debug_assertions) { 50.0 } else { 1.0 };
    eprintln!("[latency] shared-value-tree-1000-members/paint p50_ms={:.3} p95_ms={p95:.3} budget_ms={budget:.1}", samples[50]);
    assert!(p95 < budget, "p95 {p95} ms: a second reader of one revision must not convert the document again");
}
