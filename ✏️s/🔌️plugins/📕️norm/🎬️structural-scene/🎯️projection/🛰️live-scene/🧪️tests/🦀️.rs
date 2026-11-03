use super::*;
use crate::mock_projection::*;
use norm::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, SubjectRef};

fn check(entity: &str, utilisation: f64) -> CheckResult {
    let subject = SubjectRef::new(entity, format!("members[id={entity}]"), LocalizedCopy::new(format!("Member {entity}"), format!("Bauteil {entity}")));
    CheckResult::assess(format!("{entity}.bending"), "3", ClauseId::new("EN 1993-1-1", "6", "6.2.5"), subject, LocalizedCopy::new("Bending", "Biegung"))
        .utilization(Quantity::new(QuantityKind::Moment, utilisation * 100.0), Quantity::new(QuantityKind::Moment, 100.0))
        .annex(AnnexChoice::De)
        .explanation(LocalizedCopy::new("x", "x"))
        .build()
}

fn report(utilisations: &[(&str, f64)]) -> CheckReport {
    let mut report = CheckReport::default();
    utilisations.iter().for_each(|(entity, utilisation)| report.push(check(entity, *utilisation)));
    report
}

fn ids(delta: &SceneDelta) -> std::collections::BTreeSet<String> {
    delta.entities.iter().map(|patch| patch.id().to_owned()).collect()
}

#[test]
fn a_report_maps_onto_the_subject_entities() {
    let evaluation = evaluation_of(&report(&[("m0", 0.64), ("m1", 1.2)]), 7);
    assert_eq!(evaluation.report_seq, 7);
    assert_eq!(evaluation.of("m0").worst().map(|sample| (sample.check_id.as_str(), sample.utilisation, sample.status)), Some(("m0.bending", 0.64, CheckStatus::Pass)));
    assert_eq!(evaluation.of("m1").worst().map(|sample| sample.status), Some(CheckStatus::Fail));
    assert!(evaluation.of("m2").checks.is_empty());
}

#[test]
fn an_unchanged_report_keeps_the_sequence_and_every_fragment() {
    let mut live = LiveScene::new(MockProjection::default());
    let (d, r) = (doc(3), report(&[("m0", 0.5)]));
    let first = live.scene(&d, &r, &ctx(1)).expect("scene");
    let builds = live.projector().cache().misses();
    let second = live.scene(&d, &r, &ctx(1)).expect("scene");
    assert_eq!(canonical_json(&first), canonical_json(&second));
    assert_eq!(live.projector().cache().misses(), builds);
}

#[test]
fn a_drag_preview_rebuilds_only_the_edited_entity_and_the_ones_reading_it() {
    let mut live = LiveScene::new(MockProjection::default());
    let (mut d, r) = (doc(4), report(&[]));
    live.preview(&d, &ChangedSet::all(), &r, &ctx(1)).expect("first");
    d.members[2].length += 0.5;
    let (scene, delta) = live.preview(&d, &ChangedSet::of(["m2"]), &r, &ctx(2)).expect("preview");
    assert_eq!(scene.validate(), Ok(()));
    let touched = ids(&delta);
    assert!(touched.contains("m2") && !touched.contains("m0") && !touched.contains("m1") && !touched.contains("m3"), "{touched:?}");
}

#[test]
fn a_moved_utilisation_repaints_only_the_entity_whose_checks_changed() {
    let mut live = LiveScene::new(MockProjection::default());
    let d = doc(3);
    live.preview(&d, &ChangedSet::all(), &report(&[("m0", 0.2), ("m1", 0.2)]), &ctx(1)).expect("first");
    let (_, delta) = live.preview(&d, &ChangedSet::none(), &report(&[("m0", 0.2), ("m1", 0.9)]), &ctx(1)).expect("repaint");
    let touched = ids(&delta);
    assert!(touched.contains("m1") && !touched.contains("m0") && !touched.contains("m2"), "{touched:?}");
}

live_scene!(mod retained = MockProjection);

#[test]
fn the_declared_module_serves_editor_and_viewer_from_one_projector() {
    let (d, r) = (doc(2), report(&[("m1", 0.3)]));
    let first = retained::scene(&d, &r, &ctx(1)).expect("scene");
    let again = retained::scene(&d, &r, &ctx(1)).expect("scene");
    assert_eq!(canonical_json(&first), canonical_json(&again));
    let (previewed, _) = retained::preview(&d, &ChangedSet::none(), &r, &ctx(1)).expect("preview");
    assert_eq!(canonical_json(&first), canonical_json(&previewed));
}

#[test]
fn a_single_entity_drag_preview_on_a_thousand_member_scene_meets_the_preview_budget() {
    let mut live = LiveScene::new(MockProjection::default());
    let (mut d, r) = (doc(1000), report(&[]));
    let started = std::time::Instant::now();
    live.preview(&d, &ChangedSet::all(), &r, &ctx(1)).expect("first");
    let cold = started.elapsed().as_secs_f64() * 1000.0;
    let cold_budget = if cfg!(debug_assertions) { 20_000.0 } else { 1000.0 };
    eprintln!("[latency] live-scene-1000-members/cold p50_ms={cold:.3} p95_ms={cold:.3} budget_ms={cold_budget:.1}");
    assert!(cold < cold_budget, "cold first preview {cold} ms exceeds the {cold_budget} ms cold budget");
    let mut samples = Vec::new();
    for round in 0..40u64 {
        d.members[500].length += 0.01;
        let started = std::time::Instant::now();
        let (_, delta) = live.preview(&d, &ChangedSet::of(["m500"]), &r, &ctx(2 + round)).expect("preview");
        samples.push(started.elapsed().as_secs_f64() * 1000.0);
        assert!(ids(&delta).len() <= 2, "round {round}: only the member and its load change");
    }
    samples.sort_by(f64::total_cmp);
    let p95 = samples[samples.len() * 95 / 100];
    let budget = if cfg!(debug_assertions) { 250.0 } else { 12.0 };
    eprintln!("[latency] live-scene-1000-members/drag p50_ms={:.3} p95_ms={p95:.3} budget_ms={budget:.1}", samples[samples.len() / 2]);
    assert!(p95 < budget, "p95 {p95} ms exceeds the {budget} ms drag-preview budget (architecture 10: 12 ms, release; debug builds get 20x)");
}

#[test]
fn the_diagram_memo_costs_a_drag_step_and_a_repaint_far_less_than_the_budgets() {
    let scale = if cfg!(debug_assertions) { 20.0 } else { 1.0 };
    let r = report(&[("m1", 0.3)]);
    let mut document: Vec<f64> = (0..20_000).map(f64::from).collect();
    let ms = |work: &mut dyn FnMut()| {
        let started = std::time::Instant::now();
        work();
        started.elapsed().as_secs_f64() * 1000.0
    };
    let scenario = |name: &str, mut samples: Vec<f64>, release_ms: f64| {
        samples.sort_by(f64::total_cmp);
        let (p50, p95) = (samples[samples.len() / 2], samples[samples.len() * 95 / 100]);
        eprintln!("[latency] diagram-memo/{name} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={:.1}", release_ms * scale);
        assert!(p95 < release_ms * scale, "diagram-memo/{name}: p95 {p95:.3} ms exceeds {} ms", release_ms * scale);
    };
    let cold = ms(&mut || drop(crate::projection::memo_diagrams("latency-memo", &document, &r, "k", Vec::new)));
    scenario("cold", vec![cold], 1000.0);
    let drag: Vec<f64> = (0..40)
        .map(|step| {
            document[0] = f64::from(step) + 0.5;
            ms(&mut || drop(crate::projection::memo_diagrams("latency-memo", &document, &r, "k", Vec::new)))
        })
        .collect();
    scenario("drag", drag, 12.0);
    let repaint: Vec<f64> = (0..40).map(|_| ms(&mut || drop(crate::projection::memo_diagrams("latency-memo", &document, &r, "k", || unreachable!("a repaint of an unchanged document must hit the memo"))))).collect();
    scenario("repaint", repaint, 12.0);
}
