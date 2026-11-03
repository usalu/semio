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

fn full(document: &En1993Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = placements_of(document);
    let mut scene = project(&SceneInput { document, report, placements: &placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } });
    scene.revision = SceneRevision { document_seq: 0, report_seq: 0 };
    scene
}

fn incremental(projector: &mut SceneProjector<Kit<En1993Kit>>, index: &mut norm_index::Held, document: &En1993Snapshot, report: &CheckReport) -> StructuralScene {
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

fn report_of(document: &En1993Snapshot) -> CheckReport {
    checks::check_full_steel_structure(document)
}

fn start() -> En1993Snapshot {
    En1993Snapshot::frame_portal_showcase()
}

fn clone_entity(document: &mut En1993Snapshot, id: String) {
    let mut member = document.members[0].clone();
    member.id = id;
    document.members.push(member);
}

fn first_id(document: &En1993Snapshot) -> String {
    document.members[0].id.clone()
}

fn place(document: &mut En1993Snapshot, id: String, x: f64, yaw: f64) {
    match document.placements.iter_mut().find(|p| p.id == id) {
        Some(p) => (p.x, p.yaw) = (x, yaw),
        None => document.placements.push(model::Placement { id, x, y: 0.0, z: 0.0, yaw }),
    }
}


fn edit(document: &mut En1993Snapshot, rng: &mut Rng, counter: &mut usize) {
    match rng.next(8) {
        0 => {
            let i = rng.next(document.members.len());
            document.members[i].length = 2.0 + rng.next(80) as f64 / 10.0;
        }
        1 => {
            let i = rng.next(document.members.len());
            document.members[i].member_type = ["beam", "column"][rng.next(2)].into();
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
        4 if !document.joints.is_empty() => {
            let i = rng.next(document.joints.len());
            document.joints[i].plate_thickness = 0.008 + rng.next(20) as f64 / 1000.0;
        }
        5 if !document.frame_nodes.is_empty() => {
            let i = rng.next(document.frame_nodes.len());
            document.frame_nodes[i].x += 0.25;
        }
        6 if !document.member_actions.is_empty() => {
            let i = rng.next(document.member_actions.len());
            document.member_actions[i].action.n += 1000.0;
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
    let (mut projector, mut held, mut document, mut counter) = (SceneProjector::new(Kit::<En1993Kit>::default()), norm_index::Held::default(), start(), 0);
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
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1993Kit>::default()), norm_index::Held::default());
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
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1993Kit>::default()), norm_index::Held::default());
    let started = Instant::now();
    incremental(&mut projector, &mut held, &document, &report);
    gate_scenario("en1993", "cold", vec![started.elapsed().as_secs_f64() * 1e3], 1000.0);
    let samples: Vec<f64> = (0..30)
        .map(|step| {
            place(&mut document, "many-10".into(), step as f64 + 1.0, 0.0);
            let started = Instant::now();
            incremental(&mut projector, &mut held, &document, &report);
            started.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    gate_scenario("en1993", "drag", samples, 12.0);
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
    gate_scenario("en1993", "commit", commits, 400.0);
}

#[test]
fn plate_and_block_edits_stay_incremental_and_equal_to_the_full_projection() {
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    let (mut projector, mut held, mut document) = (SceneProjector::new(Kit::<En1993Kit>::default()), norm_index::Held::default(), En1993Snapshot::special_parts_showcase());
    for step in 0..30 {
        let plate = rng.next(document.joint_plates.len());
        match rng.next(6) {
            0 => document.joint_plates[plate].plate_thickness = 0.008 + rng.next(10) as f64 / 1000.0,
            1 => document.joint_plates[plate].bolt_rows = 1 + rng.next(4) as u32,
            2 => document.foundation_blocks[0].block_depth = 0.4 + rng.next(8) as f64 / 10.0,
            3 => document.foundation_blocks[0].anchor_count = 1 + rng.next(4) as u32,
            4 => document.joint_plates[plate].eccentricity = rng.next(10) as f64 / 100.0,
            _ => {
                if let Some(j) = document.joints.iter_mut().find(|j| j.id == "joint-bp") {
                    j.base_moment = 20_000.0 + rng.next(100) as f64 * 1_000.0;
                }
            }
        }
        let report = report_of(&document);
        let want = full(&document, &report);
        let got = incremental(&mut projector, &mut held, &document, &report);
        assert_eq!(got, want, "incremental equals full at step {step}");
    }
}

#[test]
fn design_point_diagrams_follow_edits_and_a_focused_projection_equals_the_full_one() {
    let mut rng = Rng(0x51ed_270b_a1c4_9d3f);
    let mut document = semio_s_artifact_norm_en1993_showcase::full_showcase();
    for step in 0..30 {
        match rng.next(5) {
            0 => document.piles[0].design_moment = 50_000.0 + rng.next(400) as f64 * 1_000.0,
            1 => document.bridge_fatigue[0].delta_sigma_p = (20 + rng.next(60)) as f64 * 1.0e6,
            2 => document.crane_runways[0].max_wheel_load = 100_000.0 + rng.next(200) as f64 * 1_000.0,
            3 => document.plated_panels[0].thickness = 0.008 + rng.next(10) as f64 / 1000.0,
            _ => {
                let i = rng.next(document.member_actions.len());
                document.member_actions[i].action.n += 1000.0;
            }
        }
        let report = report_of(&document);
        let all = diagrams(&document, &report, None);
        let owners: Vec<String> = document.piles.iter().map(|x| x.id.clone()).chain(document.bridge_fatigue.iter().map(|x| x.id.clone())).chain(document.crane_runways.iter().map(|x| x.id.clone())).chain(document.plated_panels.iter().map(|x| x.id.clone())).chain(document.load_cases.iter().map(|x| x.id.clone())).collect();
        for owner in owners {
            for diagram in diagrams(&document, &report, Some(&owner)) {
                assert_eq!(all.iter().find(|d| d.id == diagram.id), Some(&diagram), "step {step}: {}", diagram.id);
            }
        }
        let pile = all.iter().find(|d| d.id == "pile-mv:pile-1").expect("pile");
        assert_eq!(pile.markers[0].at.y, document.piles[0].design_moment / 1e3, "step {step}");
    }
}
