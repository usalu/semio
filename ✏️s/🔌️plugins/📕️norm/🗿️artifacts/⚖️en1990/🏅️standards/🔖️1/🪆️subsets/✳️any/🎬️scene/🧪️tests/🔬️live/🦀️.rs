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

fn full(document: &En1990Snapshot, report: &CheckReport, combination: Option<&str>) -> StructuralScene {
    let placements = placements_of(document);
    let mut scene = project(&SceneInput { document, report, placements: &placements, revision: SceneRevision { document_seq: 0, report_seq: 0 }, combination, traffic_group: None });
    scene.revision = SceneRevision { document_seq: 0, report_seq: 0 };
    scene
}

fn incremental(projector: &mut SceneProjector<Kit<En1990Kit>>, index: &mut norm_index::Held, document: &En1990Snapshot, report: &CheckReport, combination: Option<&str>) -> StructuralScene {
    let placements = placements_of(document);
    let evaluation = index.follow(report);
    let mut scene = projector.project(&KitDoc { snapshot: document, placements: &placements, view: &view_of(combination, None) }, &evaluation, &ProjectionContext::new(1, Lod::Lod1)).expect("scene");
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

fn edit(document: &mut En1990Snapshot, rng: &mut Rng, counter: &mut usize) {
    match rng.next(7) {
        0 if !document.members.is_empty() => {
            let i = rng.next(document.members.len());
            document.members[i].span = 3.0 + rng.next(120) as f64 / 10.0;
        }
        1 if !document.members.is_empty() => {
            let i = rng.next(document.members.len());
            document.members[i].rd_str = 1.0 + rng.next(2000) as f64;
        }
        2 => {
            *counter += 1;
            let mut member = document.members[0].clone();
            member.id = format!("added-{counter}");
            document.members.push(member);
        }
        3 if document.members.len() > 1 => {
            if let Some(i) = document.members.iter().position(|m| m.id.starts_with("added-")) {
                document.members.remove(i);
            }
        }
        4 if !document.permanents.is_empty() => {
            let i = rng.next(document.permanents.len());
            document.permanents[i].gk += 1.5;
        }
        5 if !document.variables.is_empty() => {
            let i = rng.next(document.variables.len());
            document.variables[i].qk += 0.5;
        }
        _ => {
            let id = document.members[rng.next(document.members.len())].id.clone();
            let (x, yaw) = (rng.next(40) as f64, rng.next(6) as f64 / 4.0);
            match document.placements.iter_mut().find(|p| p.id == id) {
                Some(p) => (p.x, p.yaw) = (x, yaw),
                None => document.placements.push(model::Placement { id, x, y: 0.0, z: 0.0, yaw }),
            }
        }
    }
}

#[test]
fn incremental_equals_full_on_random_edit_sequences() {
    for (name, start) in [("office", En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))] {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        let (mut projector, mut held, mut document, mut counter) = (SceneProjector::new(Kit::<En1990Kit>::default()), norm_index::Held::default(), start, 0);
        for step in 0..60 {
            edit(&mut document, &mut rng, &mut counter);
            let report = inferences::evaluate(&document);
            let combination = (step % 5 == 0).then(|| combinations::columns(&document).last().map(|c| c.id.clone())).flatten();
            let want = full(&document, &report, combination.as_deref());
            let got = incremental(&mut projector, &mut held, &document, &report, combination.as_deref());
            assert_eq!(got, want, "incremental equals full at step {step} of {name}");
        }
    }
}

#[test]
fn an_unrelated_edit_rebuilds_only_the_edited_fragment() {
    let mut document = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    for i in 0..20 {
        let mut member = document.members[0].clone();
        member.id = format!("many-{i}");
        document.members.push(member);
    }
    let report = inferences::evaluate(&document);
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1990Kit>::default()), norm_index::Held::default());
    incremental(&mut projector, &mut held, &document, &report, None);
    projector.cache_mut().reset_counters();
    let last = document.members.len() - 1;
    document.members[last].span += 0.5;
    incremental(&mut projector, &mut held, &document, &report, None);
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
    let mut document = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    for i in 0..200 {
        let mut member = document.members[0].clone();
        member.id = format!("many-{i}");
        document.members.push(member);
    }
    let report = inferences::evaluate(&document);
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1990Kit>::default()), norm_index::Held::default());
    let started = Instant::now();
    incremental(&mut projector, &mut held, &document, &report, None);
    gate_scenario("en1990", "cold", vec![started.elapsed().as_secs_f64() * 1e3], 1000.0);
    let samples: Vec<f64> = (0..30)
        .map(|step| {
            let id = document.members[10].id.clone();
            match document.placements.iter_mut().find(|p| p.id == id) {
                Some(p) => p.x = step as f64 + 1.0,
                None => document.placements.push(model::Placement { id, x: 1.0, y: 0.0, z: 0.0, yaw: 0.0 }),
            }
            let started = Instant::now();
            incremental(&mut projector, &mut held, &document, &report, None);
            started.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    gate_scenario("en1990", "drag", samples, 12.0);
    let (mut rng, mut counter) = (Rng(0x9e37_79b9_7f4a_7c15), 0);
    let commits: Vec<f64> = (0..12)
        .map(|_| {
            edit(&mut document, &mut rng, &mut counter);
            let started = Instant::now();
            let report = inferences::evaluate(&document);
            incremental(&mut projector, &mut held, &document, &report, None);
            started.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    gate_scenario("en1990", "commit", commits, 400.0);
}
