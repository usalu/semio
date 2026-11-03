use super::*;
use scene::projection::{ProjectionContext, SceneProjector};
use scene::projection_kit::placements_of;
use scene::tessellation::Lod;
use std::collections::BTreeSet;
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
extern crate semio_s_artifact_norm_en1991_inferences as inferences;

fn full(document: &En1991Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = placements_of(document);
    let mut scene = project(&SceneInput { document, report, placements: &placements, revision: SceneRevision { document_seq: 0, report_seq: 0 } });
    scene.revision = SceneRevision { document_seq: 0, report_seq: 0 };
    scene
}

fn incremental(projector: &mut SceneProjector<Kit<En1991Kit>>, index: &mut norm_index::Held, document: &En1991Snapshot, report: &CheckReport) -> StructuralScene {
    let placements = placements_of(document);
    let evaluation = index.follow(&scene_report(report));
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

fn report_of(document: &En1991Snapshot) -> CheckReport {
    inferences::evaluate(document)
}

fn start() -> En1991Snapshot {
    En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)
}

fn clone_entity(document: &mut En1991Snapshot, id: String) {
    let mut floor = document.floors[0].clone();
    floor.id = id;
    document.floors.push(floor);
}

fn first_id(document: &En1991Snapshot) -> String {
    document.floors[0].id.clone()
}

fn place(document: &mut En1991Snapshot, id: String, x: f64, yaw: f64) {
    match document.placements.iter_mut().find(|p| p.id == id) {
        Some(p) => (p.x, p.yaw) = (x, yaw),
        None => document.placements.push(model::Placement { id, x, y: 0.0, z: 0.0, yaw }),
    }
}

fn face(id: &str, zone: &str) -> model::WindFace {
    model::WindFace { id: id.into(), zone: zone.into(), z: 8.0, c_pe10: 0.77, c_pe1: 1.0, c_pi: 0.2, c_s: 1.0, c_d: 1.0, loaded_area: 10.0, assumed_wp: 5_000.0 }
}

fn roof(id: &str, kind: &str, pitch: f64) -> model::RoofArea {
    model::RoofArea { id: id.into(), roof_type: kind.into(), pitch_deg: pitch, c_e: 1.0, c_t: 1.0, has_parapet: false, parapet_height: 0.0, drift_obstruction_height: 0.0, multi_span: false, assumed_sk: 2_000.0 }
}

fn layer(id: &str, thickness: f64) -> model::SelfWeightElement {
    model::SelfWeightElement { id: id.into(), material: "reinforced_concrete".into(), thickness, assumed_gk: 5_000.0 }
}

fn lane(id: &str) -> model::BridgeLane {
    model::BridgeLane { id: id.into(), width: 3.0, offset: 0.0, lm1_lane: 1 }
}

fn stage(id: &str, days: f64) -> model::ConstructionStage {
    model::ConstructionStage { id: id.into(), activity: "scaffolding".into(), duration_days: days, return_years: 2.0, assumed_qk: 1_500.0 }
}

fn surface(id: &str, face: &str) -> model::RoofSurface {
    model::RoofSurface { id: id.into(), face: face.into(), shape: "duopitch".into(), pitch_deg: 15.0, extent: 5.0 }
}

fn vehicle(id: &str) -> model::SpecialVehicle {
    model::SpecialVehicle { id: id.into(), designation: "600/150".into(), x: 10.0, y: 2.0 }
}

fn wind_bridge(id: &str) -> model::WindBridge {
    model::WindBridge { id: id.into(), deck_width: 12.0, deck_depth: 1.0, parapet_kind: 0, parapet_height: 0.0, traffic_kind: 0, height_above_ground: 10.0, span: 30.0, assumed_force: 0.0 }
}

fn dynamic(id: &str) -> model::DynamicElement {
    model::DynamicElement { id: id.into(), height: 60.0, width: 3.0, first_frequency: 0.0, log_decrement: 0.05, mass_per_length: 500.0, strouhal: 0.2, assumed_cscd: 1.0 }
}

fn impact(id: &str) -> model::ImpactAction {
    model::ImpactAction { id: id.into(), kind: 0, mass: 1_500.0, speed: 10.0, stiffness: 1.0e6, height: 1.0, x: 0.0, y: 5.0, assumed_force: 0.0 }
}

fn explosion(id: &str) -> model::InternalExplosion {
    model::InternalExplosion { id: id.into(), volume: 100.0, vent_area: 2.0, vent_pressure: 5_000.0, assumed_pressure: 0.0 }
}

fn section(id: &str) -> model::ThermalSection {
    model::ThermalSection { id: id.into(), deck_type: 1, surfacing: 1, length: 30.0, depth: 1.5, alpha_micro: 10.0, assumed_range: 0.0 }
}

fn toggle<T>(rows: &mut Vec<T>, rng: &mut Rng, fresh: impl FnOnce() -> T) {
    if rows.is_empty() || rng.next(2) == 0 {
        rows.push(fresh());
    } else {
        rows.remove(rng.next(rows.len()));
    }
}

fn edit(document: &mut En1991Snapshot, rng: &mut Rng, counter: &mut usize) {
    *counter += 1;
    let id = format!("r{counter}");
    match rng.next(24) {
        0 => document.width = 10.0 + rng.next(300) as f64 / 10.0,
        1 => document.height = 4.0 + rng.next(200) as f64 / 10.0,
        2 => clone_entity(document, format!("added-{counter}")),
        3 if document.floors.len() > 1 => {
            if let Some(i) = document.floors.iter().position(|f| f.id.starts_with("added-")) {
                document.floors.remove(i);
            }
        }
        4 if !document.floors.is_empty() => {
            let i = rng.next(document.floors.len());
            document.floors[i].area = 20.0 + rng.next(500) as f64;
        }
        5 if !document.roofs.is_empty() => {
            let i = rng.next(document.roofs.len());
            document.roofs[i].pitch_deg = rng.next(45) as f64;
        }
        6 => {
            document.structure_kind = if rng.next(2) == 0 { model::StructureKind::Bridge } else { En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De).structure_kind };
            document.crane_claimed = rng.next(2) == 0;
            document.silo_claimed = rng.next(2) == 0;
            document.silo_kind = "silo".into();
        }
        7 => document.fire_duration = [900.0, 1800.0, 3600.0][rng.next(3)],
        8 => {
            let mut ids = vec!["building", "bridge", "silo", "crane", "selfWeight"].into_iter().map(str::to_owned).collect::<Vec<_>>();
            ids.extend(document.roofs.iter().map(|r| r.id.clone()));
            ids.push(first_id(document));
            let id = ids[rng.next(ids.len())].clone();
            place(document, id, rng.next(40) as f64, rng.next(6) as f64 / 4.0);
        }
        9 => {
            let zone = ["A", "D", "E", "F", "H", "J"][rng.next(6)];
            toggle(&mut document.wind_faces, rng, || face(&id, zone));
        }
        10 if !document.wind_faces.is_empty() => {
            let i = rng.next(document.wind_faces.len());
            document.wind_faces[i].c_s = 0.5 + rng.next(10) as f64 / 10.0;
        }
        11 => {
            document.structure_kind = model::StructureKind::Bridge;
            toggle(&mut document.bridge_lanes, rng, || lane(&id));
        }
        12 => {
            let days = 10.0 + rng.next(60) as f64;
            toggle(&mut document.construction_stages, rng, || stage(&id, days));
            if let Some(last) = document.construction_stages.last_mut() {
                last.duration_days = 10.0 + rng.next(60) as f64;
            }
        }
        13 => {
            let face_id = document.wind_faces.first().map_or_else(String::new, |f| f.id.clone());
            toggle(&mut document.roof_surfaces, rng, || surface(&id, &face_id));
        }
        14 => {
            document.en_vb = 22.0 + rng.next(10) as f64;
            document.wind_zone = 1 + rng.next(4) as u8;
            document.terrain_category = rng.next(5) as u8;
        }
        15 => {
            document.en_sk = 500.0 + rng.next(1500) as f64;
            document.altitude = rng.next(1200) as f64;
        }
        16 => {
            document.bridge_span = 10.0 + rng.next(60) as f64;
            document.bridge_tandem_x = rng.next(30) as f64;
            document.bridge_lane_width = 3.0 + rng.next(12) as f64;
        }
        17 => {
            document.crane_x = rng.next(30) as f64;
            document.silo_height = 4.0 + rng.next(20) as f64;
            document.silo_diameter = 2.0 + rng.next(10) as f64;
        }
        18 => match rng.next(6) {
            0 => toggle(&mut document.special_vehicles, rng, || vehicle(&id)),
            1 => toggle(&mut document.wind_bridges, rng, || wind_bridge(&id)),
            2 => toggle(&mut document.dynamic_elements, rng, || dynamic(&id)),
            3 => toggle(&mut document.impact_actions, rng, || impact(&id)),
            4 => toggle(&mut document.internal_explosions, rng, || explosion(&id)),
            _ => toggle(&mut document.thermal_sections, rng, || section(&id)),
        },
        19 => document.annex = if rng.next(2) == 0 { norm::AnnexChoice::De } else { norm::AnnexChoice::En },
        20 => {
            let (kind, pitch) = (["flat", "duopitch", "monopitch", "hipped"][rng.next(4)], rng.next(40) as f64);
            toggle(&mut document.roofs, rng, || roof(&id, kind, pitch));
        }
        21 => {
            let thickness = 0.05 + rng.next(30) as f64 / 100.0;
            toggle(&mut document.self_weight_elements, rng, || layer(&id, thickness));
        }
        22 => {
            document.t_max = 30.0 + rng.next(15) as f64;
            document.fire_mode = if rng.next(2) == 0 { model::FireMode::Nominal } else { model::FireMode::None };
        }
        _ => {
            let id = ["building", "bridge", "silo", "crane", "selfWeight"][rng.next(5)].to_owned();
            place(document, id, rng.next(40) as f64, rng.next(6) as f64 / 4.0);
        }
    }
}

#[test]
fn incremental_equals_full_on_random_edit_sequences() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let (mut projector, mut held, mut document, mut counter) = (SceneProjector::new(Kit::<En1991Kit>::default()), norm_index::Held::default(), start(), 0);
    for step in 0..120 {
        edit(&mut document, &mut rng, &mut counter);
        let report = report_of(&document);
        let want = full(&document, &report);
        let got = incremental(&mut projector, &mut held, &document, &report);
        assert_eq!(got, want, "incremental equals full at step {step}");
    }
}

fn counted() -> En1991Snapshot {
    let mut floors = start().floors;
    clone_entity_into(&mut floors, "shop-l0");
    En1991Snapshot {
        floors,
        self_weight_elements: vec![layer("slab", 0.25), layer("screed", 0.06)],
        roofs: vec![roof("roof-main", "duopitch", 20.0), roof("roof-annex", "flat", 0.0)],
        wind_faces: vec![face("wall-a", "A"), face("wall-d", "D"), face("wall-d2", "D"), face("roof-main-f", "F")],
        accidental_cases: vec![model::AccidentalCase { id: "crash".into(), impact: vec![model::AccidentalImpact { vehicle_mass: 7_500.0, vehicle_speed: 20.0, assumed_force: 1.0e6 }], explosion: vec![] }],
        bridge_lanes: vec![lane("lane-1")],
        construction_stages: vec![stage("stage-1", 30.0), stage("stage-2", 60.0)],
        roof_surfaces: vec![surface("surf-1", "wall-a"), surface("surf-2", "wall-d")],
        special_vehicles: vec![vehicle("veh-1")],
        wind_bridges: vec![wind_bridge("wb-1")],
        dynamic_elements: vec![dynamic("dyn-1")],
        impact_actions: vec![impact("imp-1")],
        internal_explosions: vec![explosion("expl-1")],
        thermal_sections: vec![section("sec-1")],
        structure_kind: model::StructureKind::Bridge,
        fire_mode: model::FireMode::Nominal,
        crane_claimed: true,
        silo_claimed: true,
        ..start()
    }
}

fn clone_entity_into(floors: &mut Vec<model::FloorArea>, id: &str) {
    let mut floor = floors[0].clone();
    floor.id = id.into();
    floors.push(floor);
}

fn misses_after(change: fn(&mut En1991Snapshot)) -> usize {
    let mut document = counted();
    let report = report_of(&document);
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1991Kit>::default()), norm_index::Held::default());
    incremental(&mut projector, &mut held, &document, &report);
    projector.cache_mut().reset_counters();
    change(&mut document);
    incremental(&mut projector, &mut held, &document, &report);
    projector.cache().misses()
}

/// 🔢️ A fragment rebuilds when its row, a scalar it owns, a declared dependency or its stamp changes: the expected count is the edited entity, its transitive dependents and the merged global part (one more miss).
#[test]
fn one_leaf_edit_rebuilds_exactly_the_dependent_fragments() {
    let cases: [(&str, fn(&mut En1991Snapshot), usize); 36] = [
        ("floor area", |d| d.floors[1].area = 99.0, 2),
        ("floor drag", |d| place(d, "office-l1".into(), 7.0, 0.5), 2),
        ("top self-weight layer", |d| d.self_weight_elements[1].thickness = 0.08, 3),
        ("bottom self-weight layer", |d| d.self_weight_elements[0].thickness = 0.30, 4),
        ("later roof pitch", |d| d.roofs[1].pitch_deg = 5.0, 3),
        ("first roof pitch", |d| d.roofs[0].pitch_deg = 25.0, 10),
        ("second face of a zone", |d| d.wind_faces[2].c_s = 0.9, 5),
        ("face of a lone zone", |d| d.wind_faces[3].c_s = 0.9, 4),
        ("roof surface", |d| d.roof_surfaces[0].extent = 6.0, 3),
        ("stage duration", |d| d.construction_stages[0].duration_days = 45.0, 3),
        ("lane width", |d| d.bridge_lanes[0].width = 3.5, 3),
        ("special vehicle", |d| d.special_vehicles[0].x = 12.0, 3),
        ("thermal section", |d| d.thermal_sections[0].depth = 1.8, 3),
        ("impact", |d| d.impact_actions[0].x = 2.0, 2),
        ("slender element", |d| d.dynamic_elements[0].height = 70.0, 3),
        ("bridge wind case", |d| d.wind_bridges[0].span = 40.0, 3),
        ("accidental case", |d| d.accidental_cases[0].impact[0].vehicle_speed = 25.0, 2),
        ("explosion room", |d| d.internal_explosions[0].volume = 120.0, 2),
        ("bridge span", |d| d.bridge_span = 30.0, 9),
        ("carriageway width", |d| d.bridge_lane_width = 9.0, 10),
        ("scalar no projection reads", |d| d.assumed_bridge_udl = 1.0, 2),
        ("wind site", |d| d.en_vb = 27.0, 11),
        ("snow site", |d| d.en_sk = 1_000.0, 6),
        ("annex", |d| d.annex = norm::AnnexChoice::En, 22),
        ("building depth", |d| d.depth = 30.0, 24),
        ("building drag", |d| place(d, "building".into(), 3.0, 0.0), 23),
        ("silo height", |d| d.silo_height = 15.0, 2),
        ("crane position", |d| d.crane_x = 10.0, 2),
        ("fire duration", |d| d.fire_duration = 1800.0, 2),
        ("thermal range", |d| d.t_max = 40.0, 2),
        ("added floor", |d| clone_entity(d, "added".into()), 2),
        ("removed floor", |d| drop(d.floors.pop()), 1),
        ("added face of a new zone", |d| d.wind_faces.push(face("wall-h", "H")), 5),
        ("added stage", |d| d.construction_stages.push(stage("stage-3", 15.0)), 4),
        ("added lane", |d| d.bridge_lanes.push(lane("lane-2")), 3),
        ("added slender element", |d| d.dynamic_elements.push(dynamic("dyn-2")), 3),
    ];
    for (name, change, expected) in cases {
        assert_eq!(misses_after(change), expected, "rebuilt fragments after: {name}");
    }
}

#[test]
fn an_untouched_document_rebuilds_nothing() {
    let document = counted();
    let report = report_of(&document);
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1991Kit>::default()), norm_index::Held::default());
    incremental(&mut projector, &mut held, &document, &report);
    projector.cache_mut().reset_counters();
    incremental(&mut projector, &mut held, &document, &report);
    assert_eq!(projector.cache().misses(), 0);
}

#[test]
fn the_dependency_graph_names_only_entities_and_has_no_cycle() {
    fn reaches(document: &En1991Snapshot, from: &str, target: &str, seen: &mut BTreeSet<String>) -> bool {
        entity_dependencies(document, from).iter().any(|d| d == target || (seen.insert(d.clone()) && reaches(document, d, target, seen)))
    }
    let document = counted();
    let ids: BTreeSet<String> = entities_of(&document).into_iter().map(|e| e.id).collect();
    assert_eq!(ids.len(), entities_of(&document).len(), "entity ids are unique");
    for id in &ids {
        assert!(entity_dependencies(&document, id).iter().all(|d| ids.contains(d)), "{id} depends on an unknown entity");
        assert!(!reaches(&document, id, id, &mut BTreeSet::new()), "{id} depends on itself");
    }
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
    let (mut projector, mut held) = (SceneProjector::new(Kit::<En1991Kit>::default()), norm_index::Held::default());
    let started = Instant::now();
    incremental(&mut projector, &mut held, &document, &report);
    gate_scenario("en1991", "cold", vec![started.elapsed().as_secs_f64() * 1e3], 1000.0);
    let samples: Vec<f64> = (0..30)
        .map(|step| {
            place(&mut document, "many-10".into(), step as f64 + 1.0, 0.0);
            let started = Instant::now();
            incremental(&mut projector, &mut held, &document, &report);
            started.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    gate_scenario("en1991", "drag", samples, 12.0);
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
    gate_scenario("en1991", "commit", commits, 400.0);
}
