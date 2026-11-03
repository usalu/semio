use super::*;
use scene::projection::{ProjectionContext, SceneProjector};
use scene::projection_kit::placements_of;
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

fn full(document: &En1997Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = placements_of(document);
    let mut scene = project(&SceneInput { document, report, placements: &placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } });
    scene.revision = SceneRevision { document_seq: 0, report_seq: 0 };
    scene
}

fn incremental(projector: &mut SceneProjector<Kit<En1997Kit>>, index: &mut norm_index::Held, document: &En1997Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = placements_of(document);
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

fn report_of(document: &En1997Snapshot) -> CheckReport {
    checks::check_project(document)
}

fn start() -> En1997Snapshot {
    En1997Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)
}

fn clone_entity(document: &mut En1997Snapshot, id: String) {
    let mut footing = document.footings[0].clone();
    footing.id = id;
    document.footings.push(footing);
}

fn first_id(document: &En1997Snapshot) -> String {
    document.footings[0].id.clone()
}

fn place(document: &mut En1997Snapshot, id: String, x: f64, yaw: f64) {
    match document.placements.iter_mut().find(|p| p.id == id) {
        Some(p) => (p.x, p.yaw) = (x, yaw),
        None => document.placements.push(model::Placement { id, x, y: 0.0, z: 0.0, yaw }),
    }
}


fn edit(document: &mut En1997Snapshot, rng: &mut Rng, counter: &mut usize) {
    match rng.next(7) {
        0 => {
            let i = rng.next(document.footings.len());
            document.footings[i].width = 1.0 + rng.next(30) as f64 / 10.0;
        }
        1 if !document.layers.is_empty() => {
            let i = rng.next(document.layers.len());
            document.layers[i].depth_bottom += 0.5;
        }
        2 => {
            *counter += 1;
            clone_entity(document, format!("added-{counter}"));
        }
        3 if document.footings.len() > 1 => {
            if let Some(i) = document.footings.iter().position(|f| f.id.starts_with("added-")) {
                document.footings.remove(i);
            }
        }
        4 => document.groundwater_level = rng.next(40) as f64 / 10.0,
        5 => {
            let i = rng.next(document.footings.len());
            document.footings[i].length = 1.0 + rng.next(40) as f64 / 10.0;
        }
        _ => {
            let id = document.footings[rng.next(document.footings.len())].id.clone();
            place(document, id, rng.next(40) as f64, rng.next(6) as f64 / 4.0);
        }
    }
}

#[test]
fn incremental_equals_full_on_random_edit_sequences() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let (mut projector, mut held, mut document, mut counter) = (SceneProjector::new(Kit::<En1997Kit>::default()), norm_index::Held::default(), start(), 0);
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
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1997Kit>::default()), norm_index::Held::default());
    incremental(&mut projector, &mut held, &document, &report);
    projector.cache_mut().reset_counters();
    place(&mut document, "many-19".into(), 7.0, 0.5);
    incremental(&mut projector, &mut held, &document, &report);
    assert!(projector.cache().misses() <= 8, "misses {}", projector.cache().misses());
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
    for i in 0..60 {
        clone_entity(&mut document, format!("many-{i}"));
    }
    let report = report_of(&document);
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1997Kit>::default()), norm_index::Held::default());
    let started = Instant::now();
    incremental(&mut projector, &mut held, &document, &report);
    gate_scenario("en1997", "cold", vec![started.elapsed().as_secs_f64() * 1e3], 1000.0);
    let samples: Vec<f64> = (0..30)
        .map(|step| {
            place(&mut document, "many-10".into(), step as f64 + 1.0, 0.0);
            let started = Instant::now();
            incremental(&mut projector, &mut held, &document, &report);
            started.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    gate_scenario("en1997", "drag", samples, 12.0);
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
    gate_scenario("en1997", "commit", commits, 400.0);
}
