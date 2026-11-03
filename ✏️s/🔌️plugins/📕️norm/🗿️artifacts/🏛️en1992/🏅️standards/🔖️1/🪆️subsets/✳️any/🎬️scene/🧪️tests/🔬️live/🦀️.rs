use super::*;
use scene::projection::{ProjectionContext, SceneProjector};
use scene::tessellation::Lod;
use std::time::Instant;

struct Rng(u64);

impl Rng {
    fn next(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n as u64) as usize
    }
}
extern crate semio_s_artifact_norm_en1992_checks as checks;

fn report_of_document(document: &En1992Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    for member in &document.members {
        report.extend(checks::evaluate_member(document, member));
    }
    for anchor in &document.anchors {
        report.extend(checks::evaluate_anchor(document, anchor));
    }
    report
}

fn full(document: &En1992Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = scene::projection_kit::placements_of(document);
    let mut scene = project(&SceneInput { document, report, placements: &placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } });
    scene.revision = SceneRevision { document_seq: 0, report_seq: 0 };
    scene
}

fn incremental(projector: &mut SceneProjector<Kit<En1992Kit>>, index: &mut norm_index::Held, document: &En1992Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = scene::projection_kit::placements_of(document);
    let evaluation = index.follow(report);
    let mut scene = projector.project(&KitDoc { snapshot: document, placements: &placements, view: scene::projection::ViewOptions::none() }, &evaluation, &ProjectionContext::new(1, Lod::Lod1)).expect("scene");
    scene.revision = SceneRevision { document_seq: 0, report_seq: 0 };
    scene
}

mod norm_index {
    use scene::projection::{Evaluation, ReportIndex};
    use super::CheckReport;

    #[derive(Default)]
    pub struct Held(ReportIndex, u64);

    impl Held {
        pub fn follow(&mut self, report: &CheckReport) -> Evaluation {
            self.0.follow(report);
            self.1 += 1;
            Evaluation { report_seq: self.1, reports: self.0.clone(), ..Evaluation::default() }
        }
    }
}

fn report_of(document: &En1992Snapshot) -> CheckReport {
    report_of_document(document)
}

fn start() -> En1992Snapshot {
    En1992Snapshot::compliant_office_frame()
}

fn clone_entity(document: &mut En1992Snapshot, id: String) {
    let mut member = document.members[0].clone();
    member.id = id;
    document.members.push(member);
}

fn first_id(document: &En1992Snapshot) -> String {
    document.members[0].id.clone()
}

fn place(document: &mut En1992Snapshot, id: String, x: f64, yaw: f64) {
    match document.placements.iter_mut().find(|p| p.id == id) {
        Some(row) => (row.x, row.yaw) = (x, yaw),
        None => document.placements.push(model::Placement { id, x, y: 0.0, z: 0.0, yaw }),
    }
}


fn edit(document: &mut En1992Snapshot, rng: &mut Rng, counter: &mut usize) {
    match rng.next(7) {
        0 => {
            let i = rng.next(document.members.len());
            document.members[i].span = 2.0 + rng.next(80) as f64 / 10.0;
        }
        1 => {
            let i = rng.next(document.members.len());
            document.members[i].height = 0.2 + rng.next(80) as f64 / 100.0;
        }
        2 => {
            *counter += 1;
            clone_entity(document, format!("added-{counter}"));
        }
        3 if document.members.len() > 1 => {
            if let Some(i) = document.members.iter().position(|m| m.id.starts_with("added-")) {
                document.members.remove(i);
            }
        }
        4 => {
            let i = rng.next(document.members.len());
            document.members[i].width = 0.15 + rng.next(60) as f64 / 100.0;
        }
        5 if !document.anchors.is_empty() => {
            let id = document.anchors[rng.next(document.anchors.len())].id.clone();
            place(document, id, 0.5 + rng.next(10) as f64, 0.0);
        }
        _ => {
            let id = document.members[rng.next(document.members.len())].id.clone();
            place(document, id, rng.next(40) as f64, rng.next(6) as f64 / 4.0);
        }
    }
}

#[test]
fn incremental_equals_full_on_random_edit_sequences() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let (mut projector, mut held, mut document, mut counter) = (SceneProjector::new(Kit::<En1992Kit>::default()), norm_index::Held::default(), start(), 0);
    for step in 0..60 {
        edit(&mut document, &mut rng, &mut counter);
        let report = report_of(&document);
        let want = full(&document, &report);
        let got = incremental(&mut projector, &mut held, &document, &report);
        assert_eq!(got, want, "incremental equals full at step {step}");
    }
}

#[test]
fn an_unrelated_edit_rebuilds_only_the_edited_fragment() {
    let mut document = start();
    for i in 0..20 {
        clone_entity(&mut document, format!("many-{i}"));
    }
    let report = report_of(&document);
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1992Kit>::default()), norm_index::Held::default());
    incremental(&mut projector, &mut held, &document, &report);
    projector.cache_mut().reset_counters();
    place(&mut document, "many-19".into(), 7.0, 0.5);
    incremental(&mut projector, &mut held, &document, &report);
    assert!(projector.cache().misses() <= 4, "misses {}", projector.cache().misses());
}

/// ⏱️ Sorts one scenario's timings, prints its `[latency] <code>/scene-<scenario>` line (p50, p95 and the budget; debug builds get 20 times) and asserts the p95 against it; `cold` is the first projection of a fresh projector (one sample, its own budget), `drag` a placement that changes every step, `commit` a real edit followed by its checks and projection.
fn gate_scenario(code: &str, scenario: &str, mut samples: Vec<f64>, release_ms: f64) {
    samples.sort_by(f64::total_cmp);
    let at = |percent: usize| samples[(samples.len() * percent / 100).min(samples.len() - 1)];
    let (p50, p95) = (at(50), at(95));
    let budget = if cfg!(debug_assertions) { release_ms * 20.0 } else { release_ms };
    eprintln!("[latency] {code}/scene-{scenario} p50_ms={p50:.3} p95_ms={p95:.3} budget_ms={budget:.1}");
    assert!(p95 < budget, "{code}/scene-{scenario}: p95 {p95:.1} ms exceeds {budget} ms");
}

#[test]
fn a_drag_preview_stays_within_the_budget_on_a_large_document() {
    let mut document = start();
    for i in 0..100 {
        clone_entity(&mut document, format!("many-{i}"));
    }
    let report = report_of(&document);
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1992Kit>::default()), norm_index::Held::default());
    let started = Instant::now();
    incremental(&mut projector, &mut held, &document, &report);
    gate_scenario("en1992", "cold", vec![started.elapsed().as_secs_f64() * 1e3], 1000.0);
    let samples: Vec<f64> = (0..30)
        .map(|step| {
            place(&mut document, "many-10".into(), step as f64 + 1.0, 0.0);
            let started = Instant::now();
            incremental(&mut projector, &mut held, &document, &report);
            started.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    gate_scenario("en1992", "drag", samples, 12.0);
    let (mut rng, mut counter) = (Rng(0x9e37_79b9_7f4a_7c15), 0);
    let commits: Vec<f64> = (0..12)
        .map(|_| {
            edit(&mut document, &mut rng, &mut counter);
            let started = Instant::now();
            let report = report_of(&document);
            incremental(&mut projector, &mut held, &document, &report);
            started.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    gate_scenario("en1992", "commit", commits, 400.0);
}

#[test]
fn opening_edits_reproject_incrementally_to_the_full_scene() {
    let (mut projector, mut held, mut document) = (SceneProjector::new(Kit::<En1992Kit>::default()), norm_index::Held::default(), En1992Snapshot::flat_slab_punching());
    let steps: [fn(&mut model::RcMember); 5] = [
        |m| m.openings.push(model::Opening { id: "op-1".into(), x: 1.0, y: 0.1, width: 0.3, depth: 0.2 }),
        |m| m.openings[0].x = 0.8,
        |m| m.openings[0].width = 0.6,
        |m| m.openings.push(model::Opening { id: "op-2".into(), x: 0.5, y: 0.9, width: 0.4, depth: 0.4 }),
        |m| m.openings.clear(),
    ];
    for (step, apply) in steps.iter().enumerate() {
        apply(document.members.iter_mut().find(|m| m.id == "slab-FS-interior").expect("interior slab"));
        let report = report_of(&document);
        let want = full(&document, &report);
        let got = incremental(&mut projector, &mut held, &document, &report);
        assert_eq!(got, want, "incremental equals full after opening edit {step}");
    }
}
