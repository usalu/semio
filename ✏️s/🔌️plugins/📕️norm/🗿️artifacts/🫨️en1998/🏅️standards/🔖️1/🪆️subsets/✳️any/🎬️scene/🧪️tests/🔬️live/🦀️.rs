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

fn full(document: &En1998Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = placements_of(document);
    let mut scene = project(&SceneInput { document, report, placements: &placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } });
    scene.revision = SceneRevision { document_seq: 0, report_seq: 0 };
    scene
}

fn incremental(projector: &mut SceneProjector<Kit<En1998Kit>>, index: &mut norm_index::Held, document: &En1998Snapshot, report: &CheckReport) -> StructuralScene {
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

fn report_of(document: &En1998Snapshot) -> CheckReport {
    semio_s_artifact_norm_en1998_inferences::check_full_seismic(document)
}

fn start() -> En1998Snapshot {
    En1998Snapshot::compliant_de_multipart()
}

fn clone_entity(document: &mut En1998Snapshot, id: String) {
    let mut building = document.buildings[0].clone();
    building.id = id;
    document.buildings.push(building);
}

fn first_id(document: &En1998Snapshot) -> String {
    document.buildings[0].id.clone()
}

fn place(document: &mut En1998Snapshot, id: String, x: f64, yaw: f64) {
    match document.placements.iter_mut().find(|p| p.id == id) {
        Some(p) => (p.x, p.yaw) = (x, yaw),
        None => document.placements.push(model::En1998Placement { id, x, y: 0.0, z: 0.0, yaw }),
    }
}


fn edit(document: &mut En1998Snapshot, rng: &mut Rng, counter: &mut usize) {
    match rng.next(8) {
        0 => {
            let i = rng.next(document.buildings.len());
            document.buildings[i].plan_width_m = 6.0 + rng.next(120) as f64 / 10.0;
        }
        1 => {
            let i = rng.next(document.buildings.len());
            document.buildings[i].joint_gap_m = rng.next(30) as f64 / 100.0;
        }
        2 => {
            *counter += 1;
            clone_entity(document, format!("added-{counter}"));
        }
        3 if document.buildings.len() > 1 => {
            if let Some(i) = document.buildings.iter().position(|b| b.id.starts_with("added-")) {
                document.buildings.remove(i);
            }
        }
        4 if !document.foundations.is_empty() => {
            let i = rng.next(document.foundations.len());
            document.foundations[i].area_m2 = 1.0 + rng.next(40) as f64;
        }
        5 if !document.towers.is_empty() => {
            let i = rng.next(document.towers.len());
            document.towers[i].height_m += 1.0;
        }
        6 => {
            let i = rng.next(document.buildings.len());
            document.buildings[i].plan_length_m = 6.0 + rng.next(120) as f64 / 10.0;
        }
        _ => {
            let id = document.buildings[rng.next(document.buildings.len())].id.clone();
            place(document, id, rng.next(40) as f64, rng.next(6) as f64 / 4.0);
        }
    }
}

#[test]
fn incremental_equals_full_on_random_edit_sequences() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let (mut projector, mut held, mut document, mut counter) = (SceneProjector::new(Kit::<En1998Kit>::default()), norm_index::Held::default(), start(), 0);
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
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1998Kit>::default()), norm_index::Held::default());
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
    for i in 0..60 {
        clone_entity(&mut document, format!("many-{i}"));
    }
    let report = report_of(&document);
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1998Kit>::default()), norm_index::Held::default());
    let started = Instant::now();
    incremental(&mut projector, &mut held, &document, &report);
    gate_scenario("en1998", "cold", vec![started.elapsed().as_secs_f64() * 1e3], 1000.0);
    let samples: Vec<f64> = (0..30)
        .map(|step| {
            place(&mut document, "many-10".into(), step as f64 + 1.0, 0.0);
            let started = Instant::now();
            incremental(&mut projector, &mut held, &document, &report);
            started.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    gate_scenario("en1998", "drag", samples, 12.0);
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
    gate_scenario("en1998", "commit", commits, 400.0);
}

#[test]
fn live_drags_of_storey_masses_stiffness_site_bridge_bearings_and_tank_filling_keep_incremental_equal_to_full_and_move_the_seismic_views() {
    let mut document = En1998Snapshot::compliant_de_structures();
    document.buildings[0].t1_method = model::T1Method::Rayleigh;
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1998Kit>::default()), norm_index::Held::default());
    let spectra_of = |document: &En1998Snapshot, report: &CheckReport| diagrams(document, report, None).into_iter().find(|d| d.id == "spectra:document").expect("spectra");
    let t1_of = |spectra: &Diagram2d| spectra.markers.iter().find(|m| m.id == "t1").map(|m| m.at.x).expect("t1 marker");
    let edits: [(&str, fn(&mut En1998Snapshot)); 6] = [
        ("storey mass", |d| d.buildings[0].storeys[0].permanent_gk_n *= 1.5),
        ("storey stiffness", |d| d.buildings[0].storeys.iter_mut().for_each(|s| s.stiffness_x *= 2.0)),
        ("site plateau", |d| d.site.s_ap_r += 0.4),
        ("bearing capacity", |d| d.bearings[0].d_rd_m += 0.01),
        ("bearing seat", |d| d.bearings[1].seat_length_m += 0.1),
        ("tank filling", |d| d.tanks[0].filling_ratio = (d.tanks[0].filling_ratio * 0.8).max(0.1)),
    ];
    let initial = report_of(&document);
    assert_eq!(incremental(&mut projector, &mut held, &document, &initial), full(&document, &initial));
    for (name, apply) in edits {
        let before = spectra_of(&document, &report_of(&document));
        apply(&mut document);
        let report = report_of(&document);
        assert_eq!(incremental(&mut projector, &mut held, &document, &report), full(&document, &report), "incremental equals full after {name}");
        let after = spectra_of(&document, &report);
        if name == "storey stiffness" {
            assert!(t1_of(&after) < t1_of(&before), "a stiffer building has a shorter period");
        }
        if name == "storey mass" {
            assert!(t1_of(&after) > t1_of(&before), "a heavier building has a longer period");
        }
    }
}

#[test]
fn live_drags_of_records_sections_piers_silos_and_towers_keep_incremental_equal_to_full_and_move_the_diagrams() {
    let mut document = En1998Snapshot::full_showcase();
    let building = document.buildings[0].id.clone();
    document.accelerograms = vec![model::En1998Accelerogram { id: "rec-live".into(), building_id: building, label: "live".into(), samples: "0 1.5 -2.5 1 0.5 -1 0".into(), dt_s: 0.02, scale_factor: 1.0 }];
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1998Kit>::default()), norm_index::Held::default());
    let marker_y = |document: &En1998Snapshot, report: &CheckReport, diagram: &str, marker: &str| diagrams(document, report, None).into_iter().find(|d| d.id == diagram).and_then(|d| d.markers.into_iter().find(|m| m.id == marker)).map(|m| m.at.y);
    let initial = report_of(&document);
    assert_eq!(incremental(&mut projector, &mut held, &document, &initial), full(&document, &initial));
    let silo = document.silos[0].id.clone();
    let before = (marker_y(&document, &initial, "accelerogram:rec-live", "peak").expect("peak"), marker_y(&document, &initial, &format!("silo-fill:{silo}"), "fill-v"));
    document.accelerograms[0].scale_factor = 2.0;
    document.silos[0].filling_ratio = (document.silos[0].filling_ratio * 0.5).max(0.1);
    document.assessments[0].section_width_m += 0.05;
    document.assessments[0].bar_diameter_mm += 2.0;
    document.towers[0].height_m += 1.0;
    document.piers.iter_mut().for_each(|p| p.moment_rd_nm *= 1.1);
    let report = report_of(&document);
    assert_eq!(incremental(&mut projector, &mut held, &document, &report), full(&document, &report));
    let after = (marker_y(&document, &report, "accelerogram:rec-live", "peak").expect("peak"), marker_y(&document, &report, &format!("silo-fill:{silo}"), "fill-v"));
    assert!((after.0 / before.0 - 2.0).abs() < 1e-9, "doubling the scale doubles the peak");
    if let (Some(a), Some(b)) = (after.1, before.1) {
        assert!(a < b, "a lower filling ratio lowers the silo base shear");
    }
}
