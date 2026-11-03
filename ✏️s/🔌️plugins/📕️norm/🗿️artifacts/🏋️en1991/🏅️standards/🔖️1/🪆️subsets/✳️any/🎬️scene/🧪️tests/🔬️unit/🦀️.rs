extern crate semio_s_artifact_norm_en1991_inferences as inferences;

use super::*;
use scene::projection_kit::test_support::{revision, entity, assert_handle_laws, assert_scene_laws, assert_diagram_laws};
use model::{AccidentalCase, AccidentalExplosion, AccidentalImpact, BridgeLane, ConstructionLoad, ConstructionStage, DerailmentCase, DynamicElement, ExecutionImperfection, FallingLoad, HelicopterLanding, FireMode, FloorArea, ImpactAction, InternalExplosion, RoofArea, RoofSurface, SelfWeightElement, SpecialVehicle, StructureKind, ThermalSection, TieRun, WindBridge, WindFace};
use norm::AnnexChoice;
use scene::{canonical_json, Validate};
use std::collections::BTreeSet;

fn scene_of(document: &En1991Snapshot, placements: &[Placement]) -> (StructuralScene, CheckReport) {
    let report = inferences::evaluate(document);
    let scene = project(&SceneInput { document, report: &report, placements, revision: revision() });
    (scene, report)
}

fn floor(id: &str, category: &str, area: f64, assumed: f64) -> FloorArea {
    FloorArea { id: id.into(), category: category.into(), area, assumed_qk: assumed, assumed_qk_concentrated: 4_000.0, assumed_partitions: 800.0 }
}

fn roof(id: &str, kind: &str, pitch: f64) -> RoofArea {
    RoofArea { id: id.into(), roof_type: kind.into(), pitch_deg: pitch, c_e: 1.0, c_t: 1.0, has_parapet: false, parapet_height: 0.0, drift_obstruction_height: 0.0, multi_span: false, assumed_sk: 2_000.0 }
}

fn face(id: &str, zone: &str, z: f64, c10: f64, c1: f64) -> WindFace {
    WindFace { id: id.into(), zone: zone.into(), z, c_pe10: c10, c_pe1: c1, c_pi: 0.2, c_s: 1.0, c_d: 1.0, loaded_area: 10.0, assumed_wp: 5_000.0 }
}

fn rich() -> En1991Snapshot {
    En1991Snapshot {
        height: 18.0,
        width: 30.0,
        depth: 24.0,
        storey_count: 4,
        floors: vec![floor("office-l1", "B1", 300.0, 3_000.0), floor("shop-l0", "C3", 500.0, 5_000.0)],
        self_weight_elements: vec![
            SelfWeightElement { id: "slab".into(), material: "reinforced_concrete".into(), thickness: 0.25, assumed_gk: 6_250.0 },
            SelfWeightElement { id: "screed".into(), material: "screed".into(), thickness: 0.06, assumed_gk: 1_320.0 },
        ],
        roofs: vec![RoofArea { has_parapet: true, parapet_height: 0.6, drift_obstruction_height: 1.2, ..roof("roof-main", "duopitch", 20.0) }, roof("roof-annex", "flat", 0.0)],
        wind_faces: vec![face("wall-a", "A", 6.0, -1.2, -1.4), face("wall-d", "D", 8.0, 0.77, 1.0), face("wall-e", "E", 8.0, -0.4, -0.4), face("roof-main-f", "F", 18.0, -0.9, -2.0), face("roof-main-h", "H", 18.0, -0.3, -0.3)],
        fire_mode: FireMode::Nominal,
        accidental_cases: vec![
            AccidentalCase { id: "crash".into(), impact: vec![AccidentalImpact { vehicle_mass: 7_500.0, vehicle_speed: 20.0, assumed_force: 1.0e6 }], explosion: vec![] },
            AccidentalCase { id: "blast".into(), impact: vec![], explosion: vec![AccidentalExplosion { explosion_mass: 50.0, standoff: 12.0, assumed_pressure: 5.0e4 }] },
        ],
        structure_kind: StructureKind::Bridge,
        bridge_span: 24.0,
        bridge_tandem_x: 12.0,
        bridge_lane_width: 9.0,
        crane_claimed: true,
        silo_claimed: true,
        ..En1991Snapshot::example(AnnexChoice::De)
    }
}

fn european() -> En1991Snapshot {
    En1991Snapshot { annex: AnnexChoice::En, floors: vec![floor("office-l1", "B", 240.0, 3_000.0)], ..rich() }
}

fn has(scene: &StructuralScene, id: &str) -> bool {
    scene.entities.iter().any(|e| e.entity.id == id)
}

fn letters(scene: &StructuralScene) -> String {
    scene.entities.iter().filter_map(|e| e.entity.id.strip_prefix("wind-zone-")).filter(|rest| rest.len() == 1).collect::<Vec<_>>().concat()
}

fn pressure_zones(scene: &StructuralScene, id: &str) -> Vec<scene::PressureZone> {
    let scene::Geometry::Load(scene::LoadGlyph::Pressure { zones, .. }) = &entity(scene, id).geometry else { panic!("{id} is no pressure glyph") };
    zones.clone()
}

fn area_intensity(scene: &StructuralScene, id: &str) -> f64 {
    let scene::Geometry::Load(scene::LoadGlyph::Area { intensity, .. }) = &entity(scene, id).geometry else { panic!("{id} is no area load") };
    intensity.value
}

fn roofs_of(kind: &str, pitch: f64) -> En1991Snapshot {
    En1991Snapshot { roofs: vec![roof("roof-main", kind, pitch)], ..En1991Snapshot::example(AnnexChoice::De) }
}

fn qp(d: &En1991Snapshot, z: f64) -> f64 {
    let vb = checks::part_1_4::design_vb(d.annex, d.wind_zone, d.en_vb).expect("vb");
    checks::part_1_4::peak_velocity_pressure_pa(d.annex, d.wind_zone, d.terrain_category, z, vb, d.air_density, d.orography_factor, d.mixed_terrain_upwind, d.mixed_terrain_distance, d.coast_or_island).expect("qp")
}

#[test]
fn the_scene_of_every_fixture_validates() {
    let hip = roofs_of("hip", 25.0);
    let mono = roofs_of("monopitch", 10.0);
    let flat = roofs_of("flat", 0.0);
    let bare = En1991Snapshot { roofs: vec![], wind_faces: vec![], floors: vec![], self_weight_elements: vec![], ..En1991Snapshot::example(AnnexChoice::De) };
    let failing = En1991Snapshot { floors: vec![floor("office-l1", "B1", 240.0, 10.0)], ..rich() };
    for document in [En1991Snapshot::example(AnnexChoice::De), rich(), european(), hip, mono, flat, bare, failing] {
        let (scene, _) = scene_of(&document, &[]);
        assert_scene_laws(&scene);
        assert_eq!(scene.validate(), Ok(()), "violations");
        assert_eq!(scene.schema, scene::SCENE_SCHEMA_ID);
    }
}

#[test]
fn ids_are_unique_and_layers_exist() {
    let (scene, _) = scene_of(&rich(), &[]);
    assert_scene_laws(&scene);
    let ids: BTreeSet<_> = scene.entities.iter().map(|e| e.entity.id.clone()).collect();
    assert_eq!(ids.len(), scene.entities.len());
    let layers: BTreeSet<_> = scene.layers.iter().map(|l| l.id.as_str()).collect();
    assert!(scene.entities.iter().all(|e| layers.contains(e.layer.as_str())));
    for hidden in ["snowAlternatives", "trafficAlternatives"] {
        assert!(scene.layers.iter().any(|l| l.id == hidden && !l.visible), "{hidden} starts hidden");
    }
    assert!(scene.layers.iter().filter(|l| l.visible).count() >= 10);
}

#[test]
fn projection_is_deterministic_and_bilingual() {
    let (a, _) = scene_of(&rich(), &[]);
    assert_scene_laws(&a);
    let (b, _) = scene_of(&rich(), &[]);
    assert_scene_laws(&b);
    assert_eq!(canonical_json(&a), canonical_json(&b));
    assert!(a.entities.iter().all(|e| !e.accessible_name.en.is_empty() && !e.accessible_name.de.is_empty() && !e.accessible_detail.de.is_empty()));
    assert!(a.description.de.contains("Windflächen") && a.description.en.contains("wind faces"));
}

#[test]
fn the_wind_zone_map_follows_the_roof_form() {
    let (default_scene, _) = scene_of(&En1991Snapshot::example(AnnexChoice::De), &[]);
    assert_scene_laws(&default_scene);
    assert_eq!(letters(&default_scene), "ABCDEFGHIJ");
    let (flat_scene, _) = scene_of(&roofs_of("flat", 0.0), &[]);
    assert_scene_laws(&flat_scene);
    assert_eq!(letters(&flat_scene), "ABCDEFGHI");
    let (monopitch_scene, _) = scene_of(&roofs_of("monopitch", 10.0), &[]);
    assert_scene_laws(&monopitch_scene);
    assert_eq!(letters(&monopitch_scene), "ABCDEFGH");
    let (hip_scene, _) = scene_of(&roofs_of("hip", 25.0), &[]);
    assert_scene_laws(&hip_scene);
    assert_eq!(letters(&hip_scene), "ABCDEFGHIKL");
    for letter in ["F", "G", "H", "I", "K", "L"] {
        let zone = entity(&hip_scene, &format!("wind-zone-{letter}"));
        assert!(matches!(zone.geometry, scene::Geometry::Surface(_)), "hipped zone {letter} draws geometry only while Table 7.5 is unset");
        assert!(zone.accessible_detail.en.contains("awaiting values") && zone.accessible_detail.de.contains("Werte ausstehend"), "{letter}");
    }
    let narrow = En1991Snapshot { width: 30.0, depth: 24.0, height: 18.0, ..En1991Snapshot::example(AnnexChoice::De) };
    let (narrow_scene, _) = scene_of(&narrow, &[]);
    assert_scene_laws(&narrow_scene);
    assert!(!letters(&narrow_scene).contains('C'), "zone C vanishes when e ≥ d");
}

#[test]
fn a_pressure_zone_carries_q_p_times_the_tabulated_c_pe() {
    let d = En1991Snapshot::example(AnnexChoice::De);
    let (scene, _) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let ze = checks::part_1_4::reference_height_ze(8.0, d.height, d.width);
    let c10 = checks::part_1_4::wall_cpe("D", d.height / d.depth).expect("c_pe").0;
    let zones = pressure_zones(&scene, "wind-zone-D");
    assert_eq!(zones.len(), 1);
    assert!((zones[0].value.value - qp(&d, ze) * c10).abs() < 1e-6);
    let side = pressure_zones(&scene, "wind-zone-A");
    assert_eq!(side.len(), 2, "a side-wall zone lies on both side walls");
    let wall_area: f64 = side.iter().map(|z| z.polygon.outer.points.windows(2).map(|w| w[0].distance(&w[1])).sum::<f64>()).sum();
    assert!(wall_area > 0.0);
}

#[test]
fn both_internal_pressure_cases_get_net_pressure_arrows() {
    let d = En1991Snapshot::example(AnnexChoice::De);
    let (scene, _) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let ze = checks::part_1_4::reference_height_ze(8.0, d.height, d.width);
    let c10 = checks::part_1_4::wall_cpe("D", d.height / d.depth).expect("c_pe").0;
    for (suffix, c_pi) in [("pi+0.2", 0.2), ("pi-0.3", -0.3)] {
        let expected = (qp(&d, ze) * (c10 - c_pi)).abs();
        assert!((area_intensity(&scene, &format!("wind-zone-D:{suffix}")) - expected).abs() < 1e-6, "{suffix}");
    }
    let scene::Geometry::Load(scene::LoadGlyph::Area { direction, .. }) = &entity(&scene, "wind-zone-D:pi+0.2").geometry else { panic!("area") };
    assert!(direction.x > 0.99, "windward pressure pushes along +X");
    let scene::Geometry::Load(scene::LoadGlyph::Area { direction, .. }) = &entity(&scene, "wind-zone-A:pi+0.2").geometry else { panic!("area") };
    assert!(direction.y.abs() > 0.99, "side-wall suction acts along the wall normal");
}

#[test]
fn the_q_p_profile_beside_the_building_rises_with_height() {
    let d = En1991Snapshot::example(AnnexChoice::De);
    let (scene, _) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let scene::Geometry::Load(scene::LoadGlyph::Line { profile: scene::LoadProfile::Sampled { samples }, .. }) = &entity(&scene, "wind-qp").geometry else { panic!("profile") };
    assert!(samples.windows(2).all(|w| w[1].value.value >= w[0].value.value - 1e-9));
    let top = d.height + roofs_top(&d);
    assert!((samples.last().expect("sample").value.value - qp(&d, top)).abs() < 1e-6);
}

fn roofs_top(d: &En1991Snapshot) -> f64 {
    d.depth / 2.0 * d.roofs[0].pitch_deg.to_radians().tan()
}

#[test]
fn snow_loads_follow_mu_one_with_alternative_arrangements() {
    let d = En1991Snapshot::example(AnnexChoice::De);
    let (scene, _) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let sk = checks::part_1_3::design_ground_snow_pa(d.annex, &d.snow_zone, d.altitude, d.en_sk).expect("s_k");
    let expected = sk * checks::part_1_3::mu_1(15.0);
    assert!((area_intensity(&scene, "roof-main:left") - expected).abs() < 1e-6);
    assert!((area_intensity(&scene, "roof-main:right") - expected).abs() < 1e-6);
    assert!((area_intensity(&scene, "roof-main:alt1-left") - 0.5 * expected).abs() < 1e-6);
    assert!((area_intensity(&scene, "roof-main:alt2-right") - 0.5 * expected).abs() < 1e-6);
    assert_eq!(entity(&scene, "roof-main:alt1-left").layer, "snowAlternatives");
}

#[test]
fn parapets_and_obstructions_raise_walls_and_drifts() {
    let d = rich();
    let (scene, _) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let sk = checks::part_1_3::design_ground_snow_pa(d.annex, &d.snow_zone, d.altitude, d.en_sk).expect("s_k");
    assert!(matches!(entity(&scene, "roof-main:parapet").geometry, scene::Geometry::Solid(_)));
    assert!(matches!(entity(&scene, "roof-main:obstruction").geometry, scene::Geometry::Solid(_)));
    let (_, mu, length) = checks::part_1_3::obstruction_drift(1.2, sk, d.annex);
    assert!((area_intensity(&scene, "roof-main:drift-obstruction") - sk * mu.max(checks::part_1_3::mu_1(20.0))).abs() < 1e-6);
    let scene::Geometry::Load(scene::LoadGlyph::Line { profile: scene::LoadProfile::Linear { start, end }, .. }) = &entity(&scene, "roof-main:drift-obstruction-profile").geometry else { panic!("profile") };
    assert!(start.value > end.value);
    let scene::Geometry::Load(scene::LoadGlyph::Area { polygon, .. }) = &entity(&scene, "roof-main:drift-obstruction").geometry else { panic!("area") };
    assert!(polygon.outer.points.len() == 4 && length > 0.0);
    assert!(has(&scene, "roof-main:drift-parapet"));
    assert!(has(&scene, "roof-annex:flat") && has(&scene, "roof-annex:body-flat"), "a second roof is a detached roof plane with its own snow");
}

#[test]
fn imposed_floor_loads_use_the_reduced_value_and_the_category_colour() {
    let d = En1991Snapshot::example(AnnexChoice::De);
    let (scene, _) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let expected = checks::part_1_1::imposed_qk_reduced_pa("B1", d.annex, 240.0, d.storey_count).expect("q_k");
    assert!((area_intensity(&scene, "office-l1:load") - expected).abs() < 1e-9);
    let slab = entity(&scene, "office-l1");
    assert!(matches!(slab.appearance.channel, scene::ColourChannel::Custom { .. }));
    let (rich_scene, _) = scene_of(&rich(), &[]);
    assert_scene_laws(&rich_scene);
    let colour = |id: &str| match &entity(&rich_scene, id).appearance.channel {
        scene::ColourChannel::Custom { rgb } => (rgb.r, rgb.g, rgb.b),
        other => panic!("{other:?}"),
    };
    assert_ne!(colour("office-l1"), colour("shop-l0"));
}

#[test]
fn self_weight_layers_stack_by_thickness_and_colour_by_unit_weight() {
    let (scene, _) = scene_of(&rich(), &[]);
    assert_scene_laws(&scene);
    let z = |id: &str| match &entity(&scene, id).geometry {
        scene::Geometry::Solid(scene::SolidGeometry { shape: scene::SolidShape::Prism { frame, height, .. } }) => (frame.origin.z, *height),
        _ => panic!("prism"),
    };
    let (z0, h0) = z("slab");
    let (z1, _) = z("screed");
    assert!((z1 - (z0 + h0)).abs() < 1e-12);
    assert!((area_intensity(&scene, "slab:load") - 25_000.0 * 0.25).abs() < 1e-9);
    assert!(scene.legends.iter().any(|l| l.id == "unit-weight"));
}

#[test]
fn thermal_fire_and_accidental_actions_have_glyphs() {
    let d = rich();
    let (scene, _) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let scene::Geometry::Load(scene::LoadGlyph::Thermal { delta_t, .. }) = &entity(&scene, "fire:gas").geometry else { panic!("thermal") };
    let expected = checks::part_1_2::nominal_gas_temp_k(d.fire_curve, d.fire_duration).expect("θ") - checks::part_1_2::AMBIENT_K;
    assert!((delta_t.value - expected).abs() < 1e-9);
    assert!(matches!(entity(&scene, "fire").geometry, scene::Geometry::Solid(_)));
    assert!(has(&scene, "thermal"));
    let scene::Geometry::Load(scene::LoadGlyph::Impact { energy_or_force, .. }) = &entity(&scene, "crash:impact0").geometry else { panic!("impact") };
    assert!((energy_or_force.value - checks::part_1_7::vehicle_impact_force_n(7_500.0, 20.0)).abs() < 1e-6);
    let scene::Geometry::Load(scene::LoadGlyph::Explosion { pressure, centre, .. }) = &entity(&scene, "blast:explosion0").geometry else { panic!("explosion") };
    assert!((pressure.value - checks::part_1_7::explosion_pressure_pa(50.0, 12.0)).abs() < 1e-6);
    assert!(centre.x < -11.9, "the explosion stands off the windward face");
}

#[test]
fn a_curved_bridge_projects_the_centrifugal_force_and_the_braking_force_follows_the_bounds() {
    let straight = rich();
    let (plain, _) = scene_of(&straight, &[]);
    assert!(!has(&plain, "bridge:centrifugal"), "a straight deck has no centrifugal force");
    let point_n = |scene: &StructuralScene, id: &str| match &entity(scene, id).geometry {
        scene::Geometry::Load(scene::LoadGlyph::Point { magnitude, .. }) => magnitude.value,
        _ => panic!("point"),
    };
    for (radius, factor) in [(150.0, 0.2), (400.0, 40.0 / 400.0)] {
        let curved = En1991Snapshot { bridge_curve_radius: radius, ..rich() };
        let (scene, report) = scene_of(&curved, &[]);
        assert_scene_laws(&scene);
        let lanes = checks::part_2::lane_layout(curved.bridge_lane_width, &curved.bridge_lanes.iter().map(|l| l.width).collect::<Vec<_>>()).0;
        let q_v = checks::part_2::tandem_weight_kn(curved.annex, lanes);
        assert!((point_n(&scene, "bridge:centrifugal") - factor * q_v * 1000.0).abs() < 1e-6, "Table 4.3 at r = {radius} m");
        assert!(report.checks.iter().any(|c| c.id == "en1991.2.centrifugal"));
    }
    let (far, _) = scene_of(&En1991Snapshot { bridge_curve_radius: 2000.0, ..rich() }, &[]);
    assert!(!has(&far, "bridge:centrifugal"), "no centrifugal force above 1500 m");
    let (low, high) = checks::part_2::braking_bounds_kn(straight.annex);
    assert!((low - 180.0).abs() < 1e-9 && (high - 900.0).abs() < 1e-9);
    let report = inferences::evaluate(&straight);
    assert!(report.checks.iter().any(|c| c.id == "en1991.2.braking"));
    let q_lk = |span: f64| checks::part_2::braking_force_kn(straight.annex, span, 3.0);
    assert!((q_lk(10.0) - (360.0 + 0.1 * 9.0 * 3.0 * 10.0)).abs() < 1e-9, "Eq. 4.4 inside the bounds");
    assert!((q_lk(400.0) - 900.0).abs() < 1e-9, "the upper bound governs a long deck");
}

#[test]
fn a_bridge_projects_lanes_tandems_udl_and_the_alternative_models() {
    let d = rich();
    let (scene, _) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let count = |prefix: &str| scene.entities.iter().filter(|e| e.entity.id.starts_with(prefix)).count();
    assert_eq!(count("bridge:lane-"), 3);
    assert_eq!(count("bridge:lm1-udl-"), 3);
    assert_eq!(count("bridge:lm1-tandem-"), 12);
    assert_eq!(count("bridge:footway-"), 2);
    assert_eq!(count("bridge:lm3-axle-"), 4);
    let wheel = |id: &str| match &entity(&scene, id).geometry {
        scene::Geometry::Load(scene::LoadGlyph::Point { magnitude, .. }) => magnitude.value,
        _ => panic!("point"),
    };
    assert!((wheel("bridge:lm1-tandem-1-00") - 150_000.0).abs() < 1e-9);
    assert!((wheel("bridge:lm1-tandem-3-11") - 50_000.0).abs() < 1e-9);
    assert!((area_intensity(&scene, "bridge:lm1-udl-1") - 9_000.0).abs() < 1e-9);
    assert!((area_intensity(&scene, "bridge:lm1-udl-2") - 2_500.0).abs() < 1e-9);
    assert_eq!(entity(&scene, "bridge:lm2-wheel-0").layer, "trafficAlternatives");
    assert_eq!(entity(&scene, "bridge:lm4").layer, "trafficAlternatives");
    assert!((wheel("bridge:braking") - checks::part_2::braking_force_of_deck_kn(d.annex, d.bridge_span.max(1.0), d.bridge_lane_width, &d.bridge_lanes.iter().map(|l| (l.lm1_lane, l.width)).collect::<Vec<_>>()) * 1000.0).abs() < 1e-6);
    assert!(matches!(entity(&scene, "bridge:support-a1").geometry, scene::Geometry::Support(_)));
    let narrow = En1991Snapshot { bridge_lane_width: 4.0, ..rich() };
    let (narrow_scene, _) = scene_of(&narrow, &[]);
    assert_scene_laws(&narrow_scene);
    assert_eq!(narrow_scene.entities.iter().filter(|e| e.entity.id.starts_with("bridge:lane-")).count(), 1);
    assert!(has(&narrow_scene, "bridge:lm1-remaining"), "a 4 m carriageway has a remaining area");
    let (plain_scene, _) = scene_of(&En1991Snapshot::example(AnnexChoice::De), &[]);
    assert_scene_laws(&plain_scene);
    assert!(!has(&plain_scene, "bridge"));
}

#[test]
fn the_crane_and_the_silo_project_their_loads() {
    let d = rich();
    let (scene, _) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let vertical = checks::part_3::design_vertical_wheel_n(&d.crane_class, &d.hoist_class, d.hoisting_speed).expect("φ₂ Q_r");
    let scene::Geometry::Load(scene::LoadGlyph::Point { magnitude, .. }) = &entity(&scene, "crane:wheel-a").geometry else { panic!("wheel") };
    assert!((magnitude.value - vertical).abs() < 1e-9);
    assert!(matches!(entity(&scene, "crane").geometry, scene::Geometry::Member(_)));
    assert!(has(&scene, "crane:horizontal") && has(&scene, "crane:wheel-b"));
    let scene::Geometry::Load(scene::LoadGlyph::Line { profile: scene::LoadProfile::Sampled { samples }, .. }) = &entity(&scene, "silo:pressure").geometry else { panic!("profile") };
    let at_bottom = checks::part_4::janssen_horizontal_pa(d.silo_bulk_density, d.silo_hydraulic_radius, d.silo_mu, d.silo_k, d.silo_height);
    assert!((samples[0].value.value - at_bottom).abs() < 1e-9 && samples.last().expect("top").value.value.abs() < 1e-9);
    assert!(matches!(entity(&scene, "silo").geometry, scene::Geometry::Shell(_)) && has(&scene, "silo:patch") && has(&scene, "silo:friction"));
    let tank = En1991Snapshot { silo_kind: "tank".into(), ..rich() };
    let (tank_scene, _) = scene_of(&tank, &[]);
    assert_scene_laws(&tank_scene);
    assert!(has(&tank_scene, "silo:pressure") && !has(&tank_scene, "silo:patch"));
}

#[test]
fn the_crane_runway_and_the_silo_geometry_follow_their_fields_and_expose_handles() {
    let d = En1991Snapshot { crane_x: 9.0, crane_runway_length: 24.0, crane_wheel_base: 4.0, silo_diameter: 8.0, silo_eccentricity: 0.6, silo_hopper_angle_deg: 45.0, ..rich() };
    let (scene, _) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let point = |id: &str| match &entity(&scene, id).geometry { scene::Geometry::Load(scene::LoadGlyph::Point { at, .. }) => at.clone(), _ => panic!("point load {id}") };
    let (a, b) = (point("crane:wheel-a"), point("crane:wheel-b"));
    assert!(((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt() - 4.0 < 1e-9 && ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt() - 4.0 > -1e-9);
    assert!(has(&scene, "silo:hopper") && has(&scene, "silo:field-0-0") && has(&scene, "silo:field-7-3"));
    assert_eq!(scene.entities.iter().filter(|e| e.entity.id.starts_with("silo:field-")).count(), 32);
    let intensity = |id: &str| match &entity(&scene, id).geometry { scene::Geometry::Load(scene::LoadGlyph::Area { intensity, .. }) => intensity.value, _ => panic!("area load {id}") };
    assert!(intensity("silo:field-0-3") > intensity("silo:field-0-0"));
    let selection: Vec<_> = scene.entities.iter().map(|e| e.entity.clone()).collect();
    let ids: BTreeSet<_> = handles(&d, &[], &selection).iter().map(|h| h.id.clone()).collect();
    for expected in ["crane:x", "crane:runway-length", "crane:wheel-base", "silo:diameter", "silo:eccentricity", "silo:hopper-angle"] {
        assert!(ids.contains(expected), "missing {expected}");
    }
}

#[test]
fn persisted_placements_override_the_default_layout_and_rotate_about_z() {
    let d = rich();
    let default = Layout::of(&d, &[]);
    assert!(default.bridge.origin.y < 0.0 && default.crane.origin.y < default.bridge.origin.y && default.silo.origin.x > d.depth);
    assert_eq!(default.roofs[0], default.building);
    assert!(default.roofs[1].origin.y > d.width);
    let placement = Placement { entity_id: "building".into(), origin: scene::Vec3::new(100.0, 50.0, 0.0), yaw: std::f64::consts::FRAC_PI_2 };
    let moved = Layout::of(&d, &[placement.clone()]);
    assert_eq!(moved.building.origin, placement.origin);
    let corner = moved.building.at(d.depth, 0.0, 0.0);
    assert!((corner.x - 100.0).abs() < 1e-9 && (corner.y - (50.0 + d.depth)).abs() < 1e-9);
    let (scene, _) = scene_of(&d, &[placement]);
    assert_scene_laws(&scene);
    let scene::Geometry::Solid(scene::SolidGeometry { shape: scene::SolidShape::Prism { frame, .. } }) = &entity(&scene, "building").geometry else { panic!("prism") };
    assert!(frame.x.y > 0.999);
    assert_eq!(scene.validate(), Ok(()));
}

#[test]
fn a_failing_row_gets_error_style_hatch_and_a_utilisation_above_one() {
    let d = En1991Snapshot { floors: vec![floor("office-l1", "B1", 240.0, 10.0)], ..En1991Snapshot::example(AnnexChoice::De) };
    let (scene, report) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let slab = entity(&scene, "office-l1");
    assert_eq!(slab.appearance.style, scene::StyleState::Error);
    assert_eq!(slab.appearance.hatch, Some(scene::Hatch::Diagonal));
    assert!(slab.utilisation.as_ref().expect("binding").ratio > 1.0);
    let expected = report.checks.iter().filter(|c| c.subject.entity_id == "office-l1" && c.status != norm::CheckStatus::NotApplicable).map(|c| c.utilization).fold(0.0_f64, f64::max);
    assert!((slab.utilisation.as_ref().expect("binding").ratio - expected).abs() < 1e-12);
    let (compliant, _) = scene_of(&En1991Snapshot::example(AnnexChoice::De), &[]);
    assert_scene_laws(&compliant);
    assert_ne!(entity(&compliant, "office-l1").appearance.style, scene::StyleState::Error);
}

fn schema() -> serde_json::Value {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧬️schema/📸️snapshot/🔣️.json");
    serde_json::from_str(&std::fs::read_to_string(path).expect("snapshot schema")).expect("json schema")
}

fn known_path(schema: &serde_json::Value, path: &str) -> bool {
    let properties = |node: &serde_json::Value| node.get("properties").cloned().unwrap_or_default();
    let (head, leaf) = path.split_once('.').map_or((path, None), |(h, l)| (h, Some(l)));
    let (collection, _) = head.split_once('[').map_or((head, ""), |(c, rest)| (c, rest));
    let top = properties(schema);
    if top.get(collection).is_none() {
        return false;
    }
    let Some(leaf) = leaf else { return true };
    let def = match collection {
        "floors" => "FloorArea",
        "selfWeightElements" => "SelfWeightElement",
        "roofs" => "RoofArea",
        "windFaces" => "WindFace",
        "accidentalCases" => "AccidentalCase",
        "bridgeLanes" => "BridgeLane",
        "constructionStages" => "ConstructionStage",
        "roofSurfaces" => "RoofSurface",
        "specialVehicles" => "SpecialVehicle",
        "windBridges" => "WindBridge",
        "dynamicElements" => "DynamicElement",
        "impactActions" => "ImpactAction",
        "internalExplosions" => "InternalExplosion",
        "thermalSections" => "ThermalSection",
        "helicopterLandings" => "HelicopterLanding",
        "tieRuns" => "TieRun",
        "constructionLoads" => "ConstructionLoad",
        "fallingLoads" => "FallingLoad",
        "executionImperfections" => "ExecutionImperfection",
        "derailmentCases" => "DerailmentCase",
        _ => return false,
    };
    let (def, leaf) = match (collection, leaf.split_once('.')) {
        ("accidentalCases", Some((nested, rest))) => (if nested.starts_with("impact") { "AccidentalImpact" } else { "AccidentalExplosion" }, rest),
        _ => (def, leaf),
    };
    properties(&schema["$defs"][def]).get(leaf).is_some()
}

fn current(d: &En1991Snapshot, path: &str) -> f64 {
    let row = |id: &str| id.trim_start_matches(|c| c != '=').trim_start_matches('=').split(']').next().unwrap_or_default().to_string();
    let leaf = path.rsplit('.').next().unwrap_or_default();
    match (path.split('[').next().unwrap_or_default(), leaf) {
        ("height", _) => d.height,
        ("width", _) => d.width,
        ("depth", _) => d.depth,
        ("storeyCount", _) => f64::from(d.storey_count),
        ("altitude", _) => d.altitude,
        ("windZone", _) => f64::from(d.wind_zone),
        ("terrainCategory", _) => f64::from(d.terrain_category),
        ("bridgeSpan", _) => d.bridge_span,
        ("bridgeTandemX", _) => d.bridge_tandem_x,
        ("railAlpha", _) => d.rail_alpha,
        ("assumedRailLm71", _) => d.assumed_rail_lm71,
        ("bridgeLaneWidth", _) => d.bridge_lane_width,
        ("bridgeLane", _) => f64::from(d.bridge_lane),
        ("hoistingSpeed", _) => d.hoisting_speed,
        ("siloHeight", _) => d.silo_height,
        ("siloDiameter", _) => d.silo_diameter,
        ("siloEccentricity", _) => d.silo_eccentricity,
        ("siloHopperAngleDeg", _) => d.silo_hopper_angle_deg,
        ("craneX", _) => d.crane_x,
        ("craneRunwayLength", _) => d.crane_runway_length,
        ("craneWheelBase", _) => d.crane_wheel_base,
        ("fireCompartmentHeight", _) => d.fire_compartment_height,
        ("fireOpeningFactor", _) => d.fire_opening_factor,
        ("fireThermalInertia", _) => d.fire_thermal_inertia,
        ("roofs", "pitchDeg") => d.roofs.iter().find(|r| r.id == row(path)).expect("roof").pitch_deg,
        ("roofs", "parapetHeight") => d.roofs.iter().find(|r| r.id == row(path)).expect("roof").parapet_height,
        ("roofs", "driftObstructionHeight") => d.roofs.iter().find(|r| r.id == row(path)).expect("roof").drift_obstruction_height,
        ("floors", "area") => d.floors.iter().find(|r| r.id == row(path)).expect("floor").area,
        ("selfWeightElements", "thickness") => d.self_weight_elements.iter().find(|r| r.id == row(path)).expect("element").thickness,
        ("windFaces", "z") => d.wind_faces.iter().find(|r| r.id == row(path)).expect("face").z,
        ("windFaces", "loadedArea") => d.wind_faces.iter().find(|r| r.id == row(path)).expect("face").loaded_area,
        ("bridgeLanes", "width") => d.bridge_lanes.iter().find(|r| r.id == row(path)).expect("lane").width,
        ("bridgeLanes", "offset") => d.bridge_lanes.iter().find(|r| r.id == row(path)).expect("lane").offset,
        ("bridgeLanes", "lm1Lane") => f64::from(d.bridge_lanes.iter().find(|r| r.id == row(path)).expect("lane").lm1_lane),
        ("constructionStages", "durationDays") => d.construction_stages.iter().find(|r| r.id == row(path)).expect("stage").duration_days,
        ("constructionStages", "assumedQk") => d.construction_stages.iter().find(|r| r.id == row(path)).expect("stage").assumed_qk,
        ("roofSurfaces", "pitchDeg") => d.roof_surfaces.iter().find(|r| r.id == row(path)).expect("surface").pitch_deg,
        ("roofSurfaces", "extent") => d.roof_surfaces.iter().find(|r| r.id == row(path)).expect("surface").extent,
        ("specialVehicles", "x") => d.special_vehicles.iter().find(|r| r.id == row(path)).expect("vehicle").x,
        ("specialVehicles", "y") => d.special_vehicles.iter().find(|r| r.id == row(path)).expect("vehicle").y,
        ("windBridges", "deckDepth") => d.wind_bridges.iter().find(|r| r.id == row(path)).expect("case").deck_depth,
        ("windBridges", "deckWidth") => d.wind_bridges.iter().find(|r| r.id == row(path)).expect("case").deck_width,
        ("windBridges", "heightAboveGround") => d.wind_bridges.iter().find(|r| r.id == row(path)).expect("case").height_above_ground,
        ("windBridges", "span") => d.wind_bridges.iter().find(|r| r.id == row(path)).expect("case").span,
        ("dynamicElements", "height") => d.dynamic_elements.iter().find(|r| r.id == row(path)).expect("element").height,
        ("dynamicElements", "width") => d.dynamic_elements.iter().find(|r| r.id == row(path)).expect("element").width,
        ("dynamicElements", "logDecrement") => d.dynamic_elements.iter().find(|r| r.id == row(path)).expect("element").log_decrement,
        ("impactActions", "x") => d.impact_actions.iter().find(|r| r.id == row(path)).expect("impact").x,
        ("impactActions", "y") => d.impact_actions.iter().find(|r| r.id == row(path)).expect("impact").y,
        ("impactActions", "height") => d.impact_actions.iter().find(|r| r.id == row(path)).expect("impact").height,
        ("impactActions", "assumedForce") => d.impact_actions.iter().find(|r| r.id == row(path)).expect("impact").assumed_force,
        ("internalExplosions", "volume") => d.internal_explosions.iter().find(|r| r.id == row(path)).expect("room").volume,
        ("internalExplosions", "ventArea") => d.internal_explosions.iter().find(|r| r.id == row(path)).expect("room").vent_area,
        ("thermalSections", "depth") => d.thermal_sections.iter().find(|r| r.id == row(path)).expect("section").depth,
        ("thermalSections", "length") => d.thermal_sections.iter().find(|r| r.id == row(path)).expect("section").length,
        ("helicopterLandings", "x") => d.helicopter_landings.iter().find(|r| r.id == row(path)).expect("heli").x,
        ("helicopterLandings", "y") => d.helicopter_landings.iter().find(|r| r.id == row(path)).expect("heli").y,
        ("helicopterLandings", "height") => d.helicopter_landings.iter().find(|r| r.id == row(path)).expect("heli").height,
        ("helicopterLandings", "assumedForce") => d.helicopter_landings.iter().find(|r| r.id == row(path)).expect("heli").assumed_force,
        ("tieRuns", "x") => d.tie_runs.iter().find(|r| r.id == row(path)).expect("tie").x,
        ("tieRuns", "y") => d.tie_runs.iter().find(|r| r.id == row(path)).expect("tie").y,
        ("tieRuns", "span") => d.tie_runs.iter().find(|r| r.id == row(path)).expect("tie").span,
        ("tieRuns", "spacing") => d.tie_runs.iter().find(|r| r.id == row(path)).expect("tie").spacing,
        ("tieRuns", "providedForce") => d.tie_runs.iter().find(|r| r.id == row(path)).expect("tie").provided_force,
        ("constructionLoads", "magnitude") => d.construction_loads.iter().find(|r| r.id == row(path)).expect("load").magnitude,
        ("fallingLoads", "x") => d.falling_loads.iter().find(|r| r.id == row(path)).expect("drop").x,
        ("fallingLoads", "y") => d.falling_loads.iter().find(|r| r.id == row(path)).expect("drop").y,
        ("fallingLoads", "dropHeight") => d.falling_loads.iter().find(|r| r.id == row(path)).expect("drop").drop_height,
        ("fallingLoads", "assumedForce") => d.falling_loads.iter().find(|r| r.id == row(path)).expect("drop").assumed_force,
        ("executionImperfections", "height") => d.execution_imperfections.iter().find(|r| r.id == row(path)).expect("member").height,
        ("executionImperfections", "tolerance") => d.execution_imperfections.iter().find(|r| r.id == row(path)).expect("member").tolerance,
        ("executionImperfections", "x") => d.execution_imperfections.iter().find(|r| r.id == row(path)).expect("member").x,
        ("derailmentCases", "distance") => d.derailment_cases.iter().find(|r| r.id == row(path)).expect("case").distance,
        ("derailmentCases", "x") => d.derailment_cases.iter().find(|r| r.id == row(path)).expect("case").x,
        ("derailmentCases", "assumedForce") => d.derailment_cases.iter().find(|r| r.id == row(path)).expect("case").assumed_force,
        ("accidentalCases", _) => {
            let case = d.accidental_cases.iter().find(|r| r.id == row(path)).expect("accidental case");
            let index: usize = path.split_once("].").and_then(|(_, rest)| rest.split('[').nth(1)).and_then(|s| s.split(']').next()).and_then(|s| s.parse().ok()).unwrap_or(0);
            match leaf {
                "vehicleSpeed" => case.impact[index].vehicle_speed,
                "standoff" => case.explosion[index].standoff,
                "explosionMass" => case.explosion[index].explosion_mass,
                other => panic!("unknown accidental leaf {other}"),
            }
        }
        other => handles::gauges::value_at(d, path).unwrap_or_else(|| panic!("no current value for {other:?}")),
    }
}

#[test]
fn every_advertised_handle_exists_writes_a_known_leaf_and_starts_at_the_document_value() {
    let document = rich();
    let (scene, _) = scene_of(&document, &[]);
    assert_scene_laws(&scene);
    let selection: Vec<_> = scene.entities.iter().map(|e| e.entity.clone()).collect();
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    let ids: BTreeSet<_> = all.iter().map(|h| h.id.clone()).collect();
    assert_eq!(ids.len(), all.len(), "handle ids are unique");
    for e in &scene.entities {
        assert!(e.handles.iter().all(|h| ids.contains(h)), "{} advertises unknown handle", e.entity.id);
    }
    let schema = schema();
    for handle in &all {
        assert!(handle.accessible_name.en.len() > 2 && handle.accessible_name.de.len() > 2, "{}", handle.id);
        let write = &handle.writes[0];
        assert!(known_path(&schema, &write.path), "{} writes unknown leaf {}", handle.id, write.path);
        assert!((write.map.base - current(&document, &write.path)).abs() < 1e-9, "{} base differs from the document", handle.id);
        let tolerance = match &write.quantise {
            Some(scene::Quantise::Round { step }) if handle.id.contains(":gauge-") => step / 2.0 + 1e-9,
            _ => 1e-6,
        };
        assert!((handle.writes_at(0.0)[0].value - current(&document, &write.path)).abs() <= tolerance, "{} drift 0 must keep the value", handle.id);
        assert!(handle.limits.as_ref().is_some_and(|l| l.contains(0.0)), "{} limits must contain the current value", handle.id);
    }
    let tags: BTreeSet<String> = all.iter().map(|h| h.id.clone()).collect();
    for expected in ["building:height", "building:width", "building:depth", "building:storeys", "building:altitude", "building:wind-zone", "building:terrain", "roof-main:pitch", "roof-main:parapet", "roof-main:obstruction", "office-l1:area", "slab:thickness", "wall-d:z", "wall-d:area", "bridge:span", "bridge:tandem-x", "bridge:rail-alpha", "bridge:rail-lm71", "bridge:lane-width", "bridge:lane", "crane:hoisting-speed", "silo:height", "silo:radius", "fire:height"] {
        assert!(tags.contains(expected), "missing {expected}");
    }
}

#[test]
fn dragging_a_handle_writes_the_snapped_field_value() {
    let document = En1991Snapshot::example(AnnexChoice::De);
    let building = scene::EntityRef::new(scene::EntityKind::Solid, "building", "site");
    let all = handles(&document, &[], &[building]);
    assert_handle_laws(&all);
    let write = |tag: &str, s: f64| all.iter().find(|h| h.id == format!("building:{tag}")).unwrap_or_else(|| panic!("{tag}")).writes_at(s)[0].clone();
    assert!((write("height", 2.04).value - 22.0).abs() < 1e-9);
    assert_eq!(write("height", 1000.0).value, 150.0);
    assert_eq!(write("storeys", 1.4).value, 4.0);
    assert!((write("altitude", 4.1).value - 250.0).abs() < 1e-9);
    assert_eq!(write("wind-zone", 5.0).value, 4.0);
    let roof = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Surface, "roof-main", "roofs[id=roof-main]")]);
    assert_handle_laws(&roof);
    let pitch = roof.iter().find(|h| h.id == "roof-main:pitch").expect("pitch");
    let written = pitch.writes_at(10.0_f64.to_radians())[0].clone();
    assert!((written.value - 25.0).abs() < 1e-9 && written.unit == "deg");
}

#[test]
fn derived_scene_ids_resolve_to_their_document_entity() {
    let document = rich();
    let derived = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Load, "wall-d:pi+0.2", "windFaces[id=wall-d]")]);
    assert_handle_laws(&derived);
    assert!(derived.iter().any(|h| h.id == "wall-d:z"));
    let zone = handles(&document, &[], &[scene::EntityRef::new(scene::EntityKind::Load, "wind-zone-D", "windFaces[id=wall-d]")]);
    assert_handle_laws(&zone);
    assert!(zone.iter().any(|h| h.id == "wall-d:area"), "a wind zone selects the face of that zone");
    assert_eq!(entity_kind_of(&document, "roof-main:left").map(|k| k.collection), Some("roofs"));
    assert_eq!(entity_kind_of(&document, "wind-zone-A").map(|k| k.collection), Some("windFaces"));
    assert_eq!(entity_kind_of(&document, "office-l1:load").map(|k| k.collection), Some("floors"));
    assert_eq!(entity_kind_of(&document, "nothing"), None);
}

#[test]
fn dependencies_name_what_a_projection_reads() {
    let document = rich();
    assert_eq!(entity_dependencies(&document, "office-l1"), ["annex", "building"]);
    assert_eq!(entity_dependencies(&document, "roof-main"), ["annex", "building", "snow-site"]);
    assert_eq!(entity_dependencies(&document, "wall-d"), ["annex", "building", "wind-site"]);
    assert_eq!(entity_dependencies(&document, "wind-qp"), ["annex", "building", "roof-main-f", "roof-main-h", "wall-a", "wall-d", "wall-e", "wind-site"]);
    assert_eq!(entity_dependencies(&document, "selfWeight"), ["screed", "slab"]);
    assert!(entity_dependencies(&document, "bridge-deck").contains(&"bridge".to_string()));
    assert!(entity_dependencies(&document, "annex").is_empty());
    assert!(entity_dependencies(&document, "nothing").is_empty());
}

#[test]
fn a_second_face_of_a_zone_is_read_by_the_first_and_reads_nothing() {
    let document = En1991Snapshot { wind_faces: vec![face("d-first", "D", 8.0, 0.77, 1.0), face("d-second", "d", 9.0, 0.7, 1.0)], ..rich() };
    assert!(entity_dependencies(&document, "d-first").contains(&"d-second".to_string()));
    assert!(entity_dependencies(&document, "d-second").is_empty());
    let (scene, _) = scene_of(&document, &[]);
    assert_scene_laws(&scene);
    assert!(has(&scene, "wind-zone-D"));
}

#[test]
fn every_scalar_of_the_snapshot_has_an_owner_and_every_owner_is_an_entity() {
    let schema = schema();
    let properties = schema.get("properties").and_then(|p| p.as_object()).expect("properties");
    let ids: BTreeSet<String> = entities_of(&En1991Snapshot::example(AnnexChoice::De)).into_iter().map(|e| e.id).collect();
    let scalars: Vec<&String> = properties.iter().filter(|(_, def)| def.get("type").and_then(|t| t.as_str()) != Some("array")).map(|(name, _)| name).collect();
    assert!(scalars.len() > 60);
    for name in scalars {
        let owner = scalar_owner(name).unwrap_or_else(|| panic!("scalar {name} has no owner"));
        assert!(ids.contains(owner), "{owner} owning {name} is not an entity");
    }
    assert!(inert_reference("roofSurfaces", "face") && !inert_reference("roofSurfaces", "shape"));
}

#[test]
fn subjectless_checks_are_filed_under_the_entity_that_reads_them() {
    let report = inferences::evaluate(&rich());
    let scoped = scene_report(&report);
    assert!(report.checks.iter().any(|c| c.subject.entity_id.is_empty() && c.id.starts_with("en1991.2.")));
    for (before, after) in report.checks.iter().zip(&scoped.checks) {
        let want = if before.subject.entity_id.is_empty() { report_owner(&before.id) } else { before.subject.entity_id.as_str() };
        assert_eq!(after.subject.entity_id, want, "{}", before.id);
    }
    assert_eq!(report_owner("en1991.2.lm1-tandem"), "bridge-deck");
    assert_eq!(report_owner("en1991.3.crane-vertical"), "crane");
    assert_eq!(report_owner("en1991.1-1.imposed.none"), "report");
}

#[test]
fn every_diagram_validates_and_has_a_table_twin() {
    for document in [En1991Snapshot::example(AnnexChoice::De), rich(), european()] {
        let report = inferences::evaluate(&document);
        let all = diagrams(&document, &report, None);
        assert_diagram_laws(&all);
        let ids: BTreeSet<_> = all.iter().map(|d| d.id.clone()).collect();
        for expected in ["profile:qp", "cpe:area", "snow:mu", "snow:mu3"] {
            assert!(ids.contains(expected), "missing diagram {expected} in {ids:?}");
        }
        if document.structure_kind == StructureKind::Bridge {
            for expected in ["fire:curves", "bridge:influence", "rail:influence", "silo:pressure"] {
                assert!(ids.contains(expected), "missing diagram {expected} in {ids:?}");
            }
        }
        for diagram in &all {
            assert_eq!(diagram.validate(), Ok(()), "diagram {}", diagram.id);
            assert!(diagram.table_twin.rows() > 0, "diagram {} has no table twin", diagram.id);
        }
    }
}

#[test]
fn diagram_markers_sit_on_their_curves_and_write_document_leaves() {
    let document = rich();
    let report = inferences::evaluate(&document);
    let all = diagrams(&document, &report, None);
    assert_diagram_laws(&all);
    let find = |id: &str| all.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id}"));
    let profile = find("profile:qp").markers.iter().find(|m| m.id == "face-wall-d").expect("marker");
    assert!((profile.at.x - 8.0).abs() < 1e-12 && (profile.at.y - qp(&document, 8.0) / 1000.0).abs() < 1e-9);
    assert_eq!(profile.binding.as_ref().expect("binding").path, "windFaces[id=wall-d].z");
    let cpe = find("cpe:area").markers.iter().find(|m| m.id == "face-wall-d").expect("marker");
    assert!((cpe.at.x - 10.0).abs() < 1e-12 && (cpe.at.y - 0.77).abs() < 1e-12);
    assert_eq!(cpe.bound_check.as_deref(), Some("en1991.1-4.cpe10.wall-d"));
    let snow = find("snow:mu").markers.iter().find(|m| m.id == "roof-roof-main").expect("marker");
    assert!((snow.at.y - checks::part_1_3::mu_1(20.0)).abs() < 1e-12);
    assert!((snow.binding.as_ref().expect("binding").value_at(20.0) - 20.0).abs() < 1e-9);
    let fire = find("fire:curves").markers.iter().find(|m| m.id == "duration").expect("marker");
    assert!((fire.at.x - 60.0).abs() < 1e-12 && (fire.at.y - checks::part_1_2::standard_gas_temp_c(60.0)).abs() < 1e-6);
    assert!((fire.binding.as_ref().expect("binding").value_at(45.0) - 2700.0).abs() < 1e-9);
    let axle = find("bridge:influence").markers.iter().find(|m| m.id == "axle-0").expect("marker");
    assert!((axle.at.x - 11.4).abs() < 1e-12 && (axle.at.y - 5.7).abs() < 1e-12);
    let rail = find("rail:influence");
    let rail_axle = rail.markers.iter().find(|m| m.id == "axle-1").expect("marker");
    assert!((rail_axle.at.x - 11.2).abs() < 1e-12 && (rail_axle.at.y - 5.6).abs() < 1e-12);
    assert_eq!(rail.annotations.iter().find(|n| n.id == "lm71-moment").expect("note").text.en, "M_LM71 = 8298 kNm");
    let silo_marker = find("silo:pressure").markers.iter().find(|m| m.id == "height").expect("marker");
    let at_bottom = checks::part_4::janssen_horizontal_pa(document.silo_bulk_density, document.silo_hydraulic_radius, document.silo_mu, document.silo_k, document.silo_height);
    assert!((silo_marker.at.y - at_bottom / 1000.0).abs() < 1e-9);
}

#[test]
fn focus_narrows_the_diagram_set() {
    let document = rich();
    let report = inferences::evaluate(&document);
    let ids = |focus: &str| diagrams(&document, &report, Some(focus)).iter().map(|d| d.id.clone()).filter(|id| !id.starts_with("check-outcomes:")).collect::<Vec<_>>();
    assert_eq!(ids("roof-main:left"), vec!["snow:mu", "snow:mu3"]);
    assert_eq!(ids("wind-zone-D"), vec!["profile:qp", "cpe:area"]);
    assert_eq!(ids("fire:gas"), vec!["fire:curves"]);
    assert_eq!(ids("bridge:lm2-wheel-0"), vec!["bridge:influence", "bridge:travel", "bridge:envelope-moment", "bridge:envelope-shear", "rail:influence"]);
    assert_eq!(ids("silo:patch"), vec!["silo:pressure"]);
    assert_eq!(ids("office-l1"), vec!["psi:factors"]);
    assert!(ids("nothing").is_empty());
}

#[test]
fn the_cpe_diagram_uses_log_area_and_the_tabulated_reference() {
    let document = En1991Snapshot::example(AnnexChoice::De);
    let report = inferences::evaluate(&document);
    let diagram = diagrams(&document, &report, Some("facade-d")).into_iter().find(|d| d.id == "cpe:area").expect("cpe");
    assert_eq!(diagram.axes[0].scale, scene::AxisScale::Log10);
    assert!(diagram.series.iter().any(|s| s.id() == "table-facade-d"));
}

#[test]
fn golden_scene_matches_committed_bytes() {
    let (scene, _) = scene_of(&rich(), &[]);
    assert_scene_laws(&scene);
    let actual = canonical_json(&scene);
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json");
    if std::env::var("SEMIO_UPDATE_GOLDEN").is_ok() {
        std::fs::write(path, &actual).expect("write golden");
    }
    let expected = std::fs::read_to_string(path).expect("golden fixture");
    assert_eq!(actual.trim(), expected.trim(), "golden scene changed; rerun with SEMIO_UPDATE_GOLDEN=1 after reviewing the diff");
}

fn oracle_input(document: &En1991Snapshot) -> String {
    let report = inferences::evaluate(document);
    let (scene, _) = scene_of(document, &[]);
    assert_scene_laws(&scene);
    let diagram_json: Vec<String> = diagrams(document, &report, None).iter().map(canonical_json).collect();
    let r = &document.roofs[0];
    format!(
        "{{\"scene\":{},\"diagrams\":[{}],\"building\":{{\"depth\":{},\"width\":{},\"height\":{},\"roofType\":\"{}\",\"pitchDeg\":{}}},\"wind\":{{\"vb\":{},\"rho\":{},\"terrain\":{},\"orography\":{}}},\"silo\":{{\"gamma\":{},\"a\":{},\"mu\":{},\"k\":{},\"height\":{}}},\"bridgeSpan\":{}}}",
        canonical_json(&scene),
        diagram_json.join(","),
        document.depth,
        document.width,
        document.height,
        r.roof_type,
        r.pitch_deg,
        document.en_vb,
        document.air_density,
        document.terrain_category,
        document.orography_factor,
        document.silo_bulk_density,
        document.silo_hydraulic_radius,
        document.silo_mu,
        document.silo_k,
        document.silo_height,
        document.bridge_span
    )
}

#[test]
fn the_independent_oracle_agrees() {
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔮️oracles/🐍️.py");
    let flat = En1991Snapshot { roofs: vec![roof("roof-main", "flat", 0.0)], ..european() };
    let duo = european();
    for document in [duo, flat] {
        let input = oracle_input(&document);
        let output = norm::python_oracle::run_ok(script, &[], Some(input.as_bytes()));
        let verdict = String::from_utf8_lossy(&output.stdout);
        assert!(verdict.contains("\"ok\": true"), "oracle verdict: {verdict}");
    }
}

#[test]
fn projecting_a_thousand_floors_stays_interactive() {
    let document = En1991Snapshot { floors: (0..1000).map(|i| floor(&format!("f{i}"), "B1", 100.0, 3_000.0)).collect(), ..En1991Snapshot::example(AnnexChoice::De) };
    let report = CheckReport::default();
    let started = std::time::Instant::now();
    let scene = project(&SceneInput { document: &document, report: &report, placements: &[], revision: revision() });
    assert!(scene.entities.len() >= 2000);
    eprintln!("[latency] en1991/scene-cold-1000 p50_ms={0:.3} p95_ms={0:.3} budget_ms={1:.1}", started.elapsed().as_secs_f64() * 1e3, 1000.0);
    assert!(started.elapsed().as_millis() < 1000, "projection took {:?}", started.elapsed());
    assert_scene_laws(&scene);
}

#[test]
fn the_fire_compartment_is_a_room_volume_coloured_by_the_fire_load_of_the_chosen_basis() {
    let load_of = |d: &En1991Snapshot| {
        let (scene, _) = scene_of(d, &[]);
        assert_scene_laws(&scene);
        assert_eq!(scene.validate(), Ok(()));
        let scene::ColourChannel::Fields { active, fields } = &entity(&scene, "fire").appearance.channel else { panic!("fields") };
        assert_eq!(active, "fire-load-design");
        match fields.iter().find(|f| f.id == *active).map(|f| f.samples.clone()) {
            Some(scene::FieldSamples::Instance { values }) => values[0],
            other => panic!("no instance field: {other:?}"),
        }
    };
    let table = En1991Snapshot { fire_load_basis: model::FireLoadBasis::OccupancyTable, fire_load_density_qf: 100.0e6, fire_occupancy: semio_s_artifact_norm_en1991_model::FireOccupancy::Office, ..rich() };
    let user = En1991Snapshot { fire_load_basis: model::FireLoadBasis::UserSpecified, fire_load_density_qf: 800.0e6, ..table.clone() };
    let design = |d: &En1991Snapshot| checks::part_1_2::design_fire_load_j_m2(d.fire_load_basis, d.fire_load_density_qf, d.annex, d.fire_occupancy, d.fire_compartment_area, 1.0).expect("basis satisfiable");
    assert!((load_of(&table) - design(&table)).abs() < 1.0);
    assert!((load_of(&user) - design(&user)).abs() < 1.0);
    assert!(load_of(&user) > load_of(&table), "the user value above the table row shows a higher fire load");
    assert!(entity(&scene_of(&user, &[]).0, "fire").accessible_detail.en.contains("user-specified"));
    assert!(entity(&scene_of(&table, &[]).0, "fire").accessible_detail.en.contains("occupancy table E.4"));
}

#[test]
fn the_window_and_the_wall_of_the_compartment_follow_the_opening_factor_and_the_thermal_inertia_through_their_grips() {
    let d = rich();
    let width_of = |d: &En1991Snapshot| {
        let (scene, _) = scene_of(d, &[]);
        assert_scene_laws(&scene);
        let scene::Geometry::Solid(scene::SolidGeometry { shape: scene::SolidShape::WallAssembly { path, openings, .. } }) = &entity(&scene, "fire:window").geometry else { panic!("window pane") };
        assert!(openings.is_empty());
        path.points[0].distance(&path.points[1])
    };
    let wall_of = |d: &En1991Snapshot| {
        let (scene, _) = scene_of(d, &[]);
        let scene::Geometry::Solid(scene::SolidGeometry { shape: scene::SolidShape::WallAssembly { layers, openings, .. } }) = &entity(&scene, "fire:wall").geometry else { panic!("wall") };
        assert_eq!(openings.len(), 1);
        layers[0].thickness
    };
    let fire = scene::EntityRef::new(scene::EntityKind::Solid, "fire", "site");
    let all = handles(&d, &[], &[fire]);
    assert_handle_laws(&all);
    let write = |tag: &str, s: f64| all.iter().find(|h| h.id == format!("fire:{tag}")).unwrap_or_else(|| panic!("{tag}")).writes_at(s)[0].clone();
    assert_eq!(write("opening", 0.0).path, "fireOpeningFactor");
    assert_eq!(write("inertia", 0.0).path, "fireThermalInertia");
    let wider = En1991Snapshot { fire_opening_factor: write("opening", 0.5).value, ..d.clone() };
    assert!((width_of(&wider) - width_of(&d) - 0.5).abs() < 0.06, "dragging the window grip by 0.5 m widens the window by 0.5 m up to the 0.001 m½ step of the opening factor");
    let thicker = En1991Snapshot { fire_thermal_inertia: write("inertia", 0.1).value, ..d.clone() };
    assert!((wall_of(&thicker) - wall_of(&d) - 0.1).abs() < 1e-9, "dragging the wall grip by 0.1 m thickens the wall by 0.1 m");
    let (scene, _) = scene_of(&d, &[]);
    for id in ["fire:wall", "fire:window"] {
        assert!(entity(&scene, id).handles.iter().any(|h| h == "fire:opening") && entity(&scene, id).handles.iter().any(|h| h == "fire:inertia"), "{id} advertises the window and wall grips");
    }
}

#[test]
fn the_fire_track_binds_the_iso_834_gas_temperature() {
    let (scene, _) = scene_of(&rich(), &[]);
    assert_scene_laws(&scene);
    assert_eq!(scene.validate(), Ok(()));
    let track = scene.tracks.iter().find(|t| t.id == "fire-time").expect("fire track");
    assert_eq!((track.kind.clone(), track.min, track.max, track.keyframes.len()), (scene::TrackKind::Time, 0.0, 3600.0, 7));
    let scene::ColourChannel::Fields { fields, .. } = &entity(&scene, "fire").appearance.channel else { panic!("fields") };
    assert!(track.keyframes.iter().all(|k| fields.iter().any(|f| Some(&f.id) == k.field.as_ref())));
    let at = |minutes: f64| match fields.iter().find(|f| f.id == format!("temperature@{minutes}")).map(|f| f.samples.clone()) {
        Some(scene::FieldSamples::Instance { values }) => values[0],
        other => panic!("no instance field: {other:?}"),
    };
    assert!((at(0.0) - 20.0).abs() < 1e-9);
    assert!((at(30.0) - (20.0 + 345.0 * (8.0_f64 * 30.0 + 1.0).log10())).abs() < 1e-6);
    assert!((at(30.0) - 841.8).abs() < 1e-4);
}

#[test]
fn the_traffic_track_steps_the_tandem_along_the_deck() {
    let (scene, _) = scene_of(&rich(), &[]);
    assert_scene_laws(&scene);
    let track = scene.tracks.iter().find(|t| t.id == "traffic-position").expect("traffic track");
    assert_eq!((track.min, track.max, track.keyframes.len()), (1.0, 5.0, 5));
    let layers: BTreeSet<&str> = scene.layers.iter().map(|l| l.id.as_str()).collect();
    assert!(track.keyframes.iter().all(|k| k.layer.as_deref().is_some_and(|l| layers.contains(l))));
    let x = |id: &str| match &entity(&scene, id).geometry {
        scene::Geometry::Load(scene::LoadGlyph::Point { at, .. }) => at.x,
        other => panic!("{other:?}"),
    };
    let (mid, early) = (x("bridge:lm1-tandem-1-00"), x("bridge:lm1-position-1-tandem-1-00"));
    assert!(((mid - early).abs() - 24.0 * 0.4).abs() < 1e-6, "stop 2 stands 40 % of the span from mid-span: {mid} {early}");
    assert_eq!(entity(&scene, "bridge:lm1-tandem-1-00").layer, "traffic-position-1");
}

#[test]
fn rail_load_models_project_axle_groups_and_loaded_lengths_on_a_hidden_layer() {
    let mut document = rich();
    document.structure_kind = StructureKind::Bridge;
    let (scene, _) = scene_of(&document, &[]);
    assert_scene_laws(&scene);
    let axles = scene.entities.iter().filter(|e| e.entity.id.starts_with("bridge:rail-lm71-axle-")).count();
    assert_eq!(axles, 8);
    assert!(scene.entities.iter().any(|e| e.entity.id == "bridge:rail-sw0-0") && scene.entities.iter().any(|e| e.entity.id == "bridge:rail-sw2-1"));
    let layer = scene.layers.iter().find(|l| l.id == "rail").expect("rail layer");
    assert!(!layer.visible);
    assert!(scene.entities.iter().filter(|e| e.entity.id.starts_with("bridge:rail-")).all(|e| e.layer == "rail"));
}

#[test]
fn the_wind_column_and_the_snow_beam_carry_a_ramped_deflected_shape() {
    let document = En1991Snapshot::example(AnnexChoice::De);
    let (scene, _) = scene_of(&document, &[]);
    assert_scene_laws(&scene);
    for id in ["building:wind-frame", "building:snow-beam"] {
        let overlay = entity(&scene, id).appearance.deformation.as_ref().unwrap_or_else(|| panic!("{id} has no overlay"));
        assert_eq!(overlay.motion, scene::Motion::Ramp);
        assert_eq!(overlay.load_factor, Some(1.0));
        assert!(overlay.samples.len() >= 3);
        assert!(overlay.samples.first().is_some_and(|s| s.delta.length() < 1e-9) || id == "building:snow-beam");
    }
}

#[test]
fn the_psi_chart_lists_the_floor_categories_snow_and_wind_with_the_table_a1_1_values() {
    let document = En1991Snapshot::example(AnnexChoice::De);
    let report = inferences::evaluate(&document);
    let chart = diagrams(&document, &report, None).into_iter().find(|d| d.id == "psi:factors").expect("psi chart");
    assert_eq!(chart.validate(), Ok(()));
    assert!(chart.table_twin.rows() >= 3);
    assert_eq!(checks::part_1_1::psi_factors("A1", AnnexChoice::En), checks::part_1_1::psi_factors_of_key("residential", AnnexChoice::En));
    assert_eq!(checks::part_1_1::psi_factors("E", AnnexChoice::En).0, 1.0);
    assert_eq!(checks::part_1_1::psi_factors("C3", AnnexChoice::En), checks::part_1_1::psi_factors_of_key("congregation", AnnexChoice::En));
    assert!(chart.markers.iter().all(|m| m.bound_check.as_deref().is_some_and(|id| id.starts_with("en1991.1-1.imposed."))));
}

fn with_collections() -> En1991Snapshot {
    En1991Snapshot {
        bridge_lanes: vec![BridgeLane { id: "lane-a".into(), width: 3.0, offset: -1.5, lm1_lane: 1 }, BridgeLane { id: "lane-b".into(), width: 3.0, offset: 1.5, lm1_lane: 2 }],
        construction_stages: vec![
            ConstructionStage { id: "frame".into(), activity: "personnel".into(), duration_days: 30.0, return_years: 2.0, assumed_qk: 1_500.0 },
            ConstructionStage { id: "fit-out".into(), activity: "storage".into(), duration_days: 120.0, return_years: 5.0, assumed_qk: 500.0 },
        ],
        roof_surfaces: vec![RoofSurface { id: "roof-s1".into(), face: "roof-main-f".into(), shape: "duopitch".into(), pitch_deg: 15.0, extent: 8.0 }, RoofSurface { id: "roof-s2".into(), face: "roof-main-f".into(), shape: "flat".into(), pitch_deg: 0.0, extent: 6.0 }],
        placements: vec![],
        ..rich()
    }
}

#[test]
fn bridge_lanes_stages_and_roof_surfaces_are_projected_with_handles_and_diagrams() {
    let document = with_collections();
    let (scene, report) = scene_of(&document, &[]);
    assert_scene_laws(&scene);
    assert_eq!(scene.validate(), Ok(()));
    for id in ["lane-a", "lane-b", "lane-a:lm1-udl", "lane-a:lm1-tandem-00", "frame", "fit-out", "roof-s1", "roof-s2"] {
        assert!(has(&scene, id), "{id} missing");
    }
    assert!(!has(&scene, "bridge:lane-1"), "authored lanes replace the derived notional lanes");
    assert_eq!(area_intensity(&scene, "lane-a:lm1-udl"), checks::part_2::lm1_udl_pa(document.annex, 1));
    assert_eq!(area_intensity(&scene, "lane-b:lm1-udl"), checks::part_2::lm1_udl_pa(document.annex, 2));
    assert_eq!(entity_kind_of(&document, "lane-a:label").map(|k| k.collection), Some("bridgeLanes"));
    assert_eq!(entity_kind_of(&document, "frame").map(|k| k.collection), Some("constructionStages"));
    assert_eq!(entity_kind_of(&document, "roof-s1").map(|k| k.collection), Some("roofSurfaces"));
    let selection: Vec<_> = ["lane-a", "frame", "roof-s1"].iter().map(|id| scene::EntityRef::new(scene::EntityKind::Surface, *id, "site")).collect();
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    let schema = schema();
    for tag in ["lane-a:width", "lane-a:offset", "lane-a:number", "frame:duration", "frame:load", "roof-s1:pitch", "roof-s1:extent"] {
        let handle = all.iter().find(|h| h.id == tag).unwrap_or_else(|| panic!("missing {tag}"));
        let write = &handle.writes[0];
        assert!(known_path(&schema, &write.path), "{tag} writes unknown leaf {}", write.path);
        assert!((write.map.base - current(&document, &write.path)).abs() < 1e-9, "{tag} base differs from the document");
    }
    let charts = diagrams(&document, &report, None);
    assert_diagram_laws(&charts);
    for id in ["bridge:lane-lm1", "construction:stage-timeline"] {
        let chart = charts.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("missing diagram {id}"));
        assert!(chart.table_twin.columns.iter().all(|c| c.values.len() == chart.table_twin.columns[0].values.len()));
        assert!(chart.markers.iter().all(|m| m.bound_check.as_ref().is_some_and(|c| report.checks.iter().any(|r| &r.id == c))), "{id} markers bind to existing checks");
    }
    assert!(diagrams(&document, &report, Some("frame")).iter().any(|d| d.id == "construction:stage-timeline"));
    assert!(diagrams(&document, &report, Some("lane-a")).iter().any(|d| d.id == "bridge:lane-lm1"));
}

#[test]
fn a_structure_drag_commits_one_undoable_placement_event_and_reloads_to_the_same_pose() {
    use scene::projection_kit::{placements_of, PlacementCommit, PlacementMove};
    let mut d = rich();
    let before = placements_of(&d);
    assert!(before.is_empty());
    let moved = PlacementMove { entity_id: "bridge".into(), origin: scene::Vec3::new(40.0, -30.0, 0.0), yaw: 0.25 };
    let commit = PlacementCommit::of(&before, std::slice::from_ref(&moved));
    assert_eq!(commit.ops.len(), 1);
    let mut rows = before.clone();
    commit.apply(&mut rows);
    d.placements = rows.iter().map(|p| model::Placement { id: p.entity_id.clone(), x: p.origin.x, y: p.origin.y, z: p.origin.z, yaw: p.yaw }).collect();
    let reloaded = placements_of(&d);
    assert_eq!(reloaded, rows);
    let layout = Layout::of(&d, &reloaded);
    assert_eq!(layout.bridge.origin, moved.origin);
    assert_eq!(layout.bridge.yaw, moved.yaw);
    assert!(PlacementCommit::of(&reloaded, std::slice::from_ref(&moved)).is_empty());
    let grips = handles(&d, &reloaded, &[scene::EntityRef::new(scene::EntityKind::Solid, "bridge", "bridge")]);
    assert_handle_laws(&grips);
    let paths: Vec<&str> = grips.iter().filter(|h| h.id.contains(":move-")).flat_map(|h| h.writes.iter().map(|w| w.path.as_str())).collect();
    assert_eq!(paths, ["placements[id=bridge].x", "placements[id=bridge].y"]);
    let again = PlacementMove { entity_id: "bridge".into(), origin: scene::Vec3::new(41.0, -30.0, 0.0), yaw: 0.25 };
    let second = PlacementCommit::of(&reloaded, std::slice::from_ref(&again));
    assert_eq!(second.ops.len(), 1);
    let mut after = reloaded.clone();
    second.apply(&mut after);
    second.inverse(&reloaded).apply(&mut after);
    assert_eq!(after, reloaded);
    let mut undone = reloaded.clone();
    commit.inverse(&before).apply(&mut undone);
    assert_eq!(undone, before);
}

fn with_wave_3() -> En1991Snapshot {
    En1991Snapshot {
        annex: AnnexChoice::En,
        en_vb: 25.0,
        terrain_category: 2,
        structure_kind: StructureKind::Bridge,
        bridge_span: 40.0,
        bridge_lane_width: 7.0,
        assumed_bridge_lm3: 200_000.0,
        special_vehicles: vec![SpecialVehicle { id: "sv-1".into(), designation: "1200/150".into(), x: 20.0, y: 3.5 }],
        wind_bridges: vec![WindBridge { id: "wind-deck".into(), deck_width: 12.0, deck_depth: 2.0, parapet_kind: 1, parapet_height: 1.1, traffic_kind: 0, height_above_ground: 20.0, span: 40.0, assumed_force: 10_000.0 }],
        dynamic_elements: vec![DynamicElement { id: "tower".into(), height: 80.0, width: 4.0, first_frequency: 0.0, log_decrement: 0.05, mass_per_length: 500.0, strouhal: 0.18, assumed_cscd: 1.2 }],
        impact_actions: vec![ImpactAction { id: "car".into(), kind: 0, mass: 1500.0, speed: 15.0, stiffness: 0.0, height: 1.0, x: -4.0, y: 10.0, assumed_force: 400_000.0 }],
        internal_explosions: vec![InternalExplosion { id: "room".into(), volume: 100.0, vent_area: 2.0, vent_pressure: 5_000.0, assumed_pressure: 120_000.0 }],
        thermal_sections: vec![ThermalSection { id: "deck-section".into(), deck_type: 1, surfacing: 2, length: 60.0, depth: 1.5, alpha_micro: 12.0, assumed_range: 90.0 }],
        t_min: -24.0,
        t_max: 37.0,
        t_0: 10.0,
        placements: vec![],
        ..rich()
    }
}

#[test]
fn wave_3_collections_are_projected_with_pressure_maps_bands_handles_and_diagrams() {
    let document = with_wave_3();
    let (scene, report) = scene_of(&document, &[]);
    assert_scene_laws(&scene);
    assert_eq!(scene.validate(), Ok(()));
    for id in ["sv-1", "sv-1:axle-0", "sv-1:axle-7", "wind-deck", "wind-deck:field-0-0", "wind-deck:field-7-3", "wind-deck:uplift", "tower", "tower:profile-0", "tower:profile-5", "car", "car:patch", "room", "room:wall-west", "room:wall-roof", "room:vent", "deck-section", "deck-section:band-0", "deck-section:band-5"] {
        assert!(has(&scene, id), "{id} missing");
    }
    assert!(!has(&scene, "sv-1:axle-8"), "a 1200/150 vehicle has eight axle lines");
    let expected_axle = checks::part_2::special_vehicle("1200/150").expect("designation").1;
    assert!((area_like_line_total(&scene, "sv-1:axle-0") - expected_axle / checks::part_2::lm3_vehicle_width_m()).abs() < 1e-6);
    let depth_tot = checks::part_1_4::bridge_depth_m(2.0, 1, 1.1, 0).expect("open parapets");
    let c_fx = checks::part_1_4::bridge_c_fx0(12.0, depth_tot);
    assert!((area_intensity(&scene, "wind-deck:field-0-3") - qp(&document, 20.0 - depth_tot / 2.0 + 3.5 * depth_tot / 4.0) * c_fx).abs() < 1e-6, "pressure map follows q_p(z) c_f,x");
    assert_eq!(entity_kind_of(&document, "wind-deck:field-0-0").map(|k| k.collection), Some("windBridges"));
    assert_eq!(entity_kind_of(&document, "room:vent").map(|k| k.collection), Some("internalExplosions"));
    let selection: Vec<_> = ["sv-1", "wind-deck", "tower", "car", "room", "deck-section"].iter().map(|id| scene::EntityRef::new(scene::EntityKind::Surface, *id, "site")).collect();
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    let schema = schema();
    for tag in ["sv-1:x", "sv-1:y", "wind-deck:depth", "wind-deck:width", "wind-deck:height", "wind-deck:span", "tower:height", "tower:width", "tower:decrement", "car:x", "car:y", "car:height", "car:force", "room:volume", "room:vent", "deck-section:depth", "deck-section:length"] {
        let handle = all.iter().find(|h| h.id == tag).unwrap_or_else(|| panic!("missing {tag}"));
        let write = &handle.writes[0];
        assert!(known_path(&schema, &write.path), "{tag} writes unknown leaf {}", write.path);
        assert!((write.map.base - current(&document, &write.path)).abs() < 1e-9, "{tag} base differs from the document");
        assert!(handle.limits.as_ref().is_some_and(|l| l.contains(0.0)), "{tag} limits must contain the current value");
    }
    let charts = diagrams(&document, &report, None);
    assert_diagram_laws(&charts);
    for id in ["bridge:wind-force", "dynamic:cscd", "dynamic:cscd-frequency", "vortex:velocity", "impact:force", "gas-explosion:pressure", "section:temperature-profile"] {
        let chart = charts.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("missing diagram {id}"));
        assert!(chart.table_twin.columns.iter().all(|c| c.values.len() == chart.table_twin.columns[0].values.len()), "{id} table twin is rectangular");
        assert!(chart.markers.iter().all(|m| m.bound_check.as_ref().is_none_or(|c| report.checks.iter().any(|r| &r.id == c))), "{id} markers bind to existing checks");
    }
    for (focus, id) in [("wind-deck", "bridge:wind-force"), ("tower", "dynamic:cscd"), ("car", "impact:force"), ("room", "gas-explosion:pressure"), ("deck-section", "section:temperature-profile")] {
        assert!(diagrams(&document, &report, Some(focus)).iter().any(|d| d.id == id), "{focus} focuses {id}");
    }
}

fn with_wave_6() -> En1991Snapshot {
    En1991Snapshot {
        helicopter_landings: vec![HelicopterLanding { id: "heli".into(), mass: 3_000.0, x: 5.0, y: 5.0, height: 20.0, assumed_force: 200_000.0 }],
        tie_runs: vec![TieRun { id: "tie".into(), kind: 0, spacing: 5.0, span: 6.0, dead_load: 5_000.0, imposed_load: 3_000.0, psi: 0.3, x: 0.0, y: 0.0, provided_force: 80_000.0 }],
        construction_stages: vec![ConstructionStage { id: "short".into(), activity: "personnel".into(), duration_days: 30.0, return_years: 2.0, assumed_qk: 1_500.0 }],
        construction_loads: vec![ConstructionLoad { id: "load".into(), stage: "short".into(), kind: 0, magnitude: 1_000.0 }],
        falling_loads: vec![FallingLoad { id: "drop".into(), mass: 100.0, drop_height: 2.0, stopping_distance: 0.1, x: 2.0, y: 2.0, assumed_force: 25_000.0 }],
        execution_imperfections: vec![ExecutionImperfection { id: "member".into(), height: 4.0, tolerance: 0.02, vertical_load: 1.0e6, x: 1.0, y: 1.0, assumed_force: 6_000.0 }],
        derailment_cases: vec![DerailmentCase { id: "track".into(), distance: 3.0, design_force_x: 1_000_000.0, design_force_y: 300_000.0, x: 0.0, y: -6.0, assumed_force: 1_200_000.0 }],
        ..rich()
    }
}

#[test]
fn falling_loads_imperfections_and_derailments_draw_their_design_situation() {
    use projection::hazards::{block_side, exaggeration};
    let document = with_wave_6();
    let (scene, _) = scene_of(&document, &[]);
    assert_scene_laws(&scene);
    assert_eq!(scene.validate(), Ok(()));
    for id in ["drop:body", "drop:impact-zone", "drop:fall-path", "member:nominal", "member:sway", "member:bow", "track:rail-left", "track:rail-right", "track:position", "track:fdx", "track:fdy"] {
        assert!(has(&scene, id), "{id} missing");
    }
    let body = entity(&scene, "drop:body");
    let side = block_side(100.0);
    assert!(matches!(&body.geometry, scene::Geometry::Solid(solid) if matches!(&solid.shape, scene::SolidShape::Prism { frame, height, .. } if (frame.origin.z - 2.0).abs() < 1e-9 && (*height - side).abs() < 1e-9)), "the mass block stands at the drop height");
    assert!(block_side(10_000.0) > side && block_side(1.0e9) <= 2.0);
    let k = exaggeration(4.0, 0.02);
    assert!((k - 8.0).abs() < 1e-9);
    assert_eq!(exaggeration(4.0, 0.0), 1.0);
    assert!(exaggeration(4.0, 1.0e-6) <= 100.0);
    let top = |id: &str| match &entity(&scene, id).geometry {
        scene::Geometry::Surface(surface) => match &surface.shape {
            scene::SurfaceShape::Polygon { polygon } => polygon.outer.points.iter().map(|p| p.x).fold(f64::MIN, f64::max),
            _ => panic!("polygon"),
        },
        _ => panic!("surface"),
    };
    assert!((top("member:sway") - (1.0 + k * 0.02 + 0.04)).abs() < 1e-9, "the sway top is drawn k times the tolerance");
    assert!((top("member:nominal") - 1.04).abs() < 1e-9);
    let forces: Vec<f64> = ["track:fdx", "track:fdy"].iter().map(|id| match &entity(&scene, id).geometry {
        scene::Geometry::Load(scene::LoadGlyph::Point { magnitude, .. }) => magnitude.value,
        _ => panic!("point load"),
    }).collect();
    assert_eq!(forces, [1_000_000.0, 300_000.0]);
    let dimension = |id: &str| scene.annotations.iter().find_map(|a| match a {
        scene::Annotation::Dimension { id: found, value, binding, .. } if found == id => Some((value.value, binding.as_ref().map(|b| b.path.clone()))),
        _ => None,
    });
    assert_eq!(dimension("drop:drop-height"), Some((2.0, Some("fallingLoads[id=drop].dropHeight".into()))));
    assert_eq!(dimension("drop:stopping"), Some((0.1, Some("fallingLoads[id=drop].stoppingDistance".into()))));
    assert_eq!(dimension("track:distance"), Some((3.0, Some("derailmentCases[id=track].distance".into()))));
}

#[test]
fn wave_6_collections_are_projected_with_layers_grips_and_check_bindings() {
    let document = with_wave_6();
    let (scene, report) = scene_of(&document, &[]);
    assert_scene_laws(&scene);
    assert_eq!(scene.validate(), Ok(()));
    for id in ["heli:pad", "tie", "load", "drop:drop", "member:axis", "track:track"] {
        assert!(has(&scene, id), "{id} missing");
    }
    for (id, collection) in [("heli:pad", "helicopterLandings"), ("tie", "tieRuns"), ("load", "constructionLoads"), ("drop:drop", "fallingLoads"), ("member:axis", "executionImperfections"), ("track:track", "derailmentCases")] {
        assert_eq!(entity_kind_of(&document, id).map(|k| k.collection), Some(collection), "{id}");
    }
    let layers: BTreeSet<_> = scene.layers.iter().map(|l| l.id.as_str()).collect();
    assert!(["helicopterLandings", "tieRuns", "constructionLoads", "fallingLoads", "executionImperfections", "derailmentCases"].iter().all(|layer| layers.contains(layer)));
    let selection: Vec<_> = ["heli", "tie", "load", "drop", "member", "track"].iter().map(|id| scene::EntityRef::new(scene::EntityKind::Load, *id, "site")).collect();
    let all = handles(&document, &[], &selection);
    assert_handle_laws(&all);
    let schema = schema();
    let mut seen = 0;
    for handle in &all {
        let write = &handle.writes[0];
        if !["helicopterLandings[", "tieRuns[", "constructionLoads[", "fallingLoads[", "executionImperfections[", "derailmentCases["].iter().any(|prefix| write.path.starts_with(prefix)) {
            continue;
        }
        seen += 1;
        assert!(known_path(&schema, &write.path), "{} writes unknown leaf {}", handle.id, write.path);
        assert!((write.map.base - current(&document, &write.path)).abs() < 1e-9, "{} base differs from the document", handle.id);
        assert!(handle.limits.as_ref().is_some_and(|l| l.contains(write.map.base)), "{} limits must contain the current value", handle.id);
    }
    assert_eq!(seen, 20, "every grip of the six collections is advertised");
    for id in ["en1991.1-7.helicopter-force.heli", "en1991.1-7.tie-force.tie", "en1991.1-6.stage-combination.short", "en1991.1-6.falling-load.drop", "en1991.1-6.execution-imperfection.member", "en1991.1-7.derailment-force.track"] {
        assert!(report.checks.iter().any(|c| c.id == id), "{id} missing in the report");
    }
}

fn area_like_line_total(scene: &StructuralScene, id: &str) -> f64 {
    let scene::Geometry::Load(scene::LoadGlyph::Line { profile: scene::LoadProfile::Uniform { value }, .. }) = &entity(scene, id).geometry else { panic!("{id} is no uniform line load") };
    value.value
}

#[test]
fn the_selected_load_group_shows_its_alternative_load_model() {
    for (group, lm2, lm4) in [("gr1a", "trafficAlternatives", "trafficAlternatives"), ("gr1b", "traffic", "trafficAlternatives"), ("gr4", "trafficAlternatives", "traffic")] {
        let (scene, _) = scene_of(&En1991Snapshot { bridge_load_group: group.into(), ..rich() }, &[]);
        assert_eq!(entity(&scene, "bridge:lm2-wheel-0").layer, lm2, "{group}");
        assert_eq!(entity(&scene, "bridge:lm4").layer, lm4, "{group}");
    }
}

#[test]
fn a_linear_temperature_difference_is_drawn_as_a_layered_strip_with_a_gradient_through_the_depth() {
    for document in [En1991Snapshot { delta_t_m: 10.0, ..rich() }, En1991Snapshot { delta_t_m: 10.0, structure_kind: model::StructureKind::Bridge, ..rich() }] {
        let (scene, _) = scene_of(&document, &[]);
        assert_scene_laws(&scene);
        let layer = |k: usize| match &entity(&scene, &format!("thermal:profile-{k}")).appearance.channel {
            scene::ColourChannel::Fields { fields, .. } => match &fields[0].samples {
                scene::FieldSamples::Instance { values } => values[0],
                _ => panic!("instance samples"),
            },
            _ => panic!("temperature field"),
        };
        assert_eq!(scene.entities.iter().filter(|e| e.entity.id.starts_with("thermal:profile-") && e.entity.id != "thermal:profile-label").count(), 10);
        assert!((layer(0) - 4.5).abs() < 1e-9 && (layer(9) + 4.5).abs() < 1e-9, "top warmer than the bottom by ΔT_M");
        assert!((0..9).all(|k| layer(k) > layer(k + 1)), "linear through the depth");
    }
    let (cold, _) = scene_of(&En1991Snapshot { delta_t_m: 0.0, ..rich() }, &[]);
    assert!(!has(&cold, "thermal:profile-0"));
}

#[test]
fn the_lm1_tandem_travel_overlay_carries_eleven_moment_stops_the_envelope_and_a_track() {
    let (scene, _) = scene_of(&rich(), &[]);
    let overlay = entity(&scene, fields::travel::OVERLAY);
    assert_eq!(overlay.layer, "trafficTravel");
    let scene::ColourChannel::Fields { active, fields: stops } = &overlay.appearance.channel else { panic!("travel fields") };
    assert_eq!(active, "travel@envelope");
    assert_eq!(stops.len(), fields::travel::STOPS + 2);
    let value = |id: &str| match &stops.iter().find(|f| f.id == id).expect("field").samples {
        scene::FieldSamples::Instance { values } => values[0],
        _ => panic!("instance samples"),
    };
    assert!((0..=fields::travel::STOPS).all(|k| value(&format!("travel@{k}")) <= value("travel@envelope") + 1e-6));
    assert!(scene.tracks.iter().any(|t| t.id == fields::travel::TRACK && t.keyframes.len() == fields::travel::STOPS + 1));
    let report = inferences::evaluate(&rich());
    assert!(diagrams(&rich(), &report, None).iter().any(|d| d.id == "bridge:travel"));
}

#[test]
fn wind_zones_are_coloured_by_their_net_pressure_and_roofs_carry_a_snow_profile() {
    let (scene, _) = scene_of(&rich(), &[]);
    let scene::ColourChannel::Fields { fields: zone_fields, .. } = &entity(&scene, "wind-zone-D").appearance.channel else { panic!("pressure fields") };
    assert_eq!(zone_fields.len(), 3);
    assert!(zone_fields.iter().all(|f| f.unit == scene::QuantityKind::Pressure));
    let scene::Geometry::Load(scene::LoadGlyph::Line { profile: scene::LoadProfile::Sampled { samples }, .. }) = &entity(&scene, "roof-main:profile").geometry else { panic!("sampled snow profile") };
    assert_eq!(samples.len(), 13);
    assert!(samples.iter().all(|s| s.value.value > 0.0));
    let parapet_end = samples.last().expect("sample").value.value;
    assert!(parapet_end >= samples[0].value.value - 1e-9);
}

#[test]
fn tie_runs_and_helicopter_pads_have_force_diagrams_with_axes_and_units() {
    let document = with_wave_6();
    let (_, report) = scene_of(&document, &[]);
    let charts = diagrams(&document, &report, None);
    assert_diagram_laws(&charts);
    for (focus, id) in [("tie", "tie:forces"), ("heli", "helicopter:landing-force")] {
        let chart = charts.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("missing diagram {id}"));
        assert!(chart.axes.len() == 2 && chart.axes.iter().all(|a| !a.unit.is_empty()), "{id} has axes with units");
        assert!(chart.table_twin.columns.iter().all(|c| c.values.len() == chart.table_twin.columns[0].values.len()), "{id} table twin is rectangular");
        assert!(diagrams(&document, &report, Some(focus)).iter().any(|d| d.id == id), "{focus} focuses {id}");
    }
    let tie = charts.iter().find(|d| d.id == "tie:forces").expect("tie diagram");
    let required = tie.markers.iter().find(|m| m.id == "required-tie").expect("required marker");
    assert!((required.at.y - 0.4 * (5.0 + 0.3 * 3.0) * 5.0 * 6.0).abs() < 1e-9);
}

#[test]
fn the_v12_rows_project_surfaces_loads_and_grips_that_follow_their_fields() {
    use model::{BulkSolid, Footbridge, LoadDispersal, RailDynamic, RotatingMachine};
    let mut d = rich();
    d.footbridges.push(Footbridge { id: "fb-1".into(), loaded_length: 30.0, deck_width: 3.0, natural_frequency: 1.9, assumed_service_axle: 0.0 });
    d.rail_dynamics.push(RailDynamic { id: "rail-1".into(), determinant_length: 4.0, maintenance: "careful".into() });
    d.load_dispersals.push(LoadDispersal { id: "disp-1".into(), wheel_length: 0.4, wheel_width: 0.4, pavement_thickness: 0.1, slab_thickness: 0.3 });
    d.bulk_solids.push(BulkSolid { id: "bulk-1".into(), internal_friction: 30.0_f64.to_radians() });
    d.rotating_machines.push(RotatingMachine { id: "machine-1".into(), mass: 1000.0, eccentricity: 0.002, frequency: 25.0 });
    let (scene, _) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    for id in ["fb-1", "fb-1:load", "rail-1", "rail-1:label", "disp-1", "disp-1:contact", "bulk-1", "bulk-1:label", "machine-1", "machine-1:load"] {
        assert!(has(&scene, id), "{id} missing");
    }
    let scene::Geometry::Load(scene::LoadGlyph::Area { intensity, .. }) = &entity(&scene, "fb-1:load").geometry else { panic!("footbridge load") };
    assert!((intensity.value - 4000.0).abs() < 1e-9, "q_fk = 2.0 + 120/60 kN/m2");
    let scene::Geometry::Load(scene::LoadGlyph::Point { magnitude, .. }) = &entity(&scene, "machine-1:load").geometry else { panic!("machine load") };
    assert!((magnitude.value - 49_348.022).abs() < 1e-2, "F_dyn = m e omega^2");
    assert_eq!(entity(&scene, "fb-1").handles.len(), 3);
    assert_eq!(entity(&scene, "rail-1").handles.len(), 2);
    assert_eq!(entity(&scene, "disp-1").handles.len(), 4);
    assert_eq!(entity(&scene, "bulk-1").handles.len(), 1);
    assert_eq!(entity(&scene, "machine-1").handles.len(), 3);
}

#[test]
fn the_full_showcase_populates_every_collection_and_every_new_view_has_a_diagram() {
    let d = En1991Snapshot::full_showcase();
    let counts = [d.floors.len(), d.self_weight_elements.len(), d.roofs.len(), d.wind_faces.len(), d.accidental_cases.len(), d.bridge_lanes.len(), d.construction_stages.len(), d.roof_surfaces.len(), d.placements.len(), d.special_vehicles.len(), d.wind_bridges.len(), d.dynamic_elements.len(), d.impact_actions.len(), d.internal_explosions.len(), d.thermal_sections.len(), d.cases.len(), d.helicopter_landings.len(), d.tie_runs.len(), d.construction_loads.len(), d.falling_loads.len(), d.execution_imperfections.len(), d.derailment_cases.len(), d.crane_hoists.len(), d.footbridges.len(), d.rail_dynamics.len(), d.load_dispersals.len(), d.bulk_solids.len(), d.rotating_machines.len(), d.fatigue_models.len(), d.support_collisions.len(), d.local_fires.len(), d.snow_guards.len()];
    assert_eq!(counts.len(), 32);
    assert!(counts.iter().all(|n| *n > 0), "every collection of the showcase is populated: {counts:?}");
    let (scene, report) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    let charts = diagrams(&d, &report, None);
    assert_diagram_laws(&charts);
    for (focus, id) in [("facade-d", "wind:face-pressure"), ("roof-main", "roof:snow-loads"), ("lane-1", "bridge:lane-plan"), ("q-ca", "construction:stage-loads"), ("slab-rc", "self-weight:breakdown"), ("impact-1", "accidental:impacts"), ("explosion-1", "accidental:explosions"), ("car", "impact:pulse"), ("drop", "falling:load-force"), ("imperfection-1", "execution:imperfection-force"), ("derail-1", "derailment:forces")] {
        assert!(charts.iter().any(|c| c.id == id), "{id} in the unfocused list");
        assert!(diagrams(&d, &report, Some(focus)).iter().any(|c| c.id == id), "{focus} focuses {id}");
    }
}

#[test]
fn the_hazard_diagrams_carry_the_hand_computed_required_values() {
    let d = En1991Snapshot::full_showcase();
    let (_, report) = scene_of(&d, &[]);
    let charts = diagrams(&d, &report, None);
    let marker = |chart: &str, id: &str| charts.iter().find(|c| c.id == chart).and_then(|c| c.markers.iter().find(|m| m.id == id)).cloned().unwrap_or_else(|| panic!("{chart}/{id}"));
    assert!((marker("falling:load-force", "required-drop").at.y - 100.0 * 9.80665 * 2.1 / 0.1 / 1000.0).abs() < 1e-9);
    assert!((marker("execution:imperfection-force", "required-imperfection-1").at.y - 5.0).abs() < 1e-9, "N d / h = 1000 kN * 0.02 / 4 m");
    assert!((marker("derailment:forces", "assumed-derail-1").at.y - 1200.0).abs() < 1e-9);
    let pulse = charts.iter().find(|c| c.id == "impact:pulse").expect("pulse");
    let scene::Series::Polyline { points, .. } = &pulse.series[0] else { panic!("polyline") };
    let (peak, end) = (points[1], points[2]);
    let momentum = 1500.0 * 15.0;
    assert!((0.5 * (peak.y * 1000.0) * end.x - momentum).abs() < 1e-6 * momentum, "the pulse impulse equals the momentum m v");
}

#[test]
fn the_new_grips_write_the_span_arrangement_and_the_mass_per_length() {
    let d = En1991Snapshot::full_showcase();
    let roof = handles(&d, &[], &[scene::EntityRef::new(scene::EntityKind::Surface, "roof-continuous", "roofs[id=roof-continuous]")]);
    assert_handle_laws(&roof);
    assert!(roof.iter().any(|h| h.id == "roof-continuous:multi-span" && matches!(h.kind, scene::HandleKind::ChoiceStepper) && h.writes.iter().any(|w| w.path == "roofs[id=roof-continuous].multiSpan")));
    let tower = handles(&d, &[], &[scene::EntityRef::new(scene::EntityKind::Solid, "tower", "dynamicElements[id=tower]")]);
    assert_handle_laws(&tower);
    let mass = tower.iter().find(|h| h.id == "tower:mass").expect("mass grip");
    assert_eq!(mass.writes[0].path, "dynamicElements[id=tower].massPerLength");
}

#[test]
fn the_v12_collections_and_placements_are_registered_kinds_whose_rows_are_scene_owners() {
    let d = En1991Snapshot::full_showcase();
    let rows = [
        ("craneHoist", d.crane_hoists.first().map(|r| r.id.clone())),
        ("footbridge", d.footbridges.first().map(|r| r.id.clone())),
        ("railDynamic", d.rail_dynamics.first().map(|r| r.id.clone())),
        ("loadDispersal", d.load_dispersals.first().map(|r| r.id.clone())),
        ("bulkSolid", d.bulk_solids.first().map(|r| r.id.clone())),
        ("rotatingMachine", d.rotating_machines.first().map(|r| r.id.clone())),
        ("fatigueModel", d.fatigue_models.first().map(|r| r.id.clone())),
        ("supportCollision", d.support_collisions.first().map(|r| r.id.clone())),
        ("localFire", d.local_fires.first().map(|r| r.id.clone())),
        ("snowGuard", d.snow_guards.first().map(|r| r.id.clone())),
    ];
    let (scene, report) = scene_of(&d, &[]);
    let owned: BTreeSet<_> = entities_of(&d).into_iter().map(|e| e.id).collect();
    for (kind, id) in rows {
        let id = id.unwrap_or_else(|| panic!("the showcase has a {kind} row"));
        let spec = ENTITY_KINDS.iter().find(|k| k.id == kind).expect("registered kind");
        assert!(!spec.label_en.is_empty() && !spec.label_de.is_empty());
        assert_eq!(entity_kind_of(&d, &id).map(|k| k.id), Some(kind));
        assert!(owned.contains(&id), "{kind} {id} owns scene entities");
        assert!(report.checks.iter().any(|c| c.subject.entity_id == id), "{kind} {id} has a check that names it as subject, so selecting the check highlights it");
        assert!(scene.entities.iter().any(|e| e.entity.id == id || e.entity.id.starts_with(&format!("{id}:"))), "{kind} {id} is drawn");
    }
    let placement = ENTITY_KINDS.iter().find(|k| k.id == "placement").expect("placement kind");
    assert_eq!(placement.collection, "placements");
    let moved = d.placements.first().expect("showcase placement").id.clone();
    assert!(entity_kind_of(&d, &moved).is_some(), "a placement row resolves to a registry row");
}

#[test]
fn the_wave_3_rows_project_vehicles_collisions_fires_and_guards_with_hand_computed_loads_and_grips() {
    use model::{FatigueModel, LocalFire, SnowGuard, SupportCollision};
    let mut d = rich();
    d.structure_kind = model::StructureKind::Bridge;
    d.bridge_span = 30.0;
    d.fatigue_models.push(FatigueModel { id: "fl-1".into(), model: "flm1".into(), lane: 1, assumed_axle_load: 210_000.0, assumed_udl: 2_700.0 });
    d.fatigue_models.push(FatigueModel { id: "fl-3".into(), model: "flm3".into(), lane: 1, assumed_axle_load: 120_000.0, assumed_udl: 0.0 });
    d.support_collisions.push(SupportCollision { id: "co-1".into(), location: "deck".into(), assumed_force_x: 1_000_000.0, assumed_force_y: 500_000.0, height: 1.25, position_x: 10.0 });
    d.support_collisions.push(SupportCollision { id: "co-2".into(), location: "soffit".into(), assumed_force_x: 500_000.0, assumed_force_y: 0.0, height: 1.25, position_x: 20.0 });
    d.local_fires.push(LocalFire { id: "lf-1".into(), fire_diameter: 2.0, heat_release_rate: 5.0e6, ceiling_height: 3.0, horizontal_distance: 1.0, assumed_flux: 60_000.0 });
    d.local_fires.push(LocalFire { id: "lf-2".into(), fire_diameter: 2.0, heat_release_rate: 1.0e6, ceiling_height: 6.0, horizontal_distance: 1.0, assumed_flux: 0.0 });
    d.snow_guards.push(SnowGuard { id: "sg-1".into(), roof_id: "roof-main".into(), slope_length: 6.0, assumed_force: 8_000.0 });
    let (scene, report) = scene_of(&d, &[]);
    assert_scene_laws(&scene);
    for id in ["fl-1", "fl-1:axle-0", "fl-1:axle-1", "fl-1:udl", "fl-3", "fl-3:axle-3", "co-1", "co-1:cross", "co-1:label", "co-2", "co-2:label", "lf-1", "lf-1:ceiling", "lf-1:flux", "lf-1:label", "lf-2", "lf-2:ceiling", "lf-2:label", "sg-1", "sg-1:label"] {
        assert!(has(&scene, id), "{id} missing");
    }
    assert!(!has(&scene, "co-2:cross") && !has(&scene, "lf-2:flux") && !has(&scene, "fl-3:udl") && !has(&scene, "fl-3:axle-4"));
    let scene::Geometry::Load(scene::LoadGlyph::Point { magnitude, .. }) = &entity(&scene, "fl-1:axle-0").geometry else { panic!("axle load") };
    assert!((magnitude.value - 210_000.0).abs() < 1e-6, "0.7 · 300 kN");
    let scene::Geometry::Load(scene::LoadGlyph::Area { intensity, .. }) = &entity(&scene, "fl-1:udl").geometry else { panic!("fatigue udl") };
    assert!((intensity.value - 2_700.0).abs() < 1e-6, "0.3 · 9 kN/m²");
    let force_of = |id: &str| match &entity(&scene, id).geometry {
        scene::Geometry::Load(scene::LoadGlyph::Impact { energy_or_force, .. }) => energy_or_force.value,
        _ => panic!("{id} is not an impact"),
    };
    assert!((force_of("co-1") - 1_000_000.0).abs() < 1e-6 && (force_of("co-1:cross") - 500_000.0).abs() < 1e-6 && (force_of("co-2") - 500_000.0).abs() < 1e-6);
    let scene::Geometry::Load(scene::LoadGlyph::Area { intensity, .. }) = &entity(&scene, "lf-1:flux").geometry else { panic!("heat flux") };
    assert!((intensity.value - 50_907.1348).abs() < 1e-3, "Hasemi flux of D = 2 m, Q = 5 MW, H = 3 m, r = 1 m");
    for (id, count) in [("fl-1", 3), ("fl-3", 2), ("co-1", 4), ("co-2", 3), ("lf-1", 5), ("sg-1", 2)] {
        assert_eq!(entity(&scene, id).handles.len(), count, "{id} grips");
    }
    let grips = handles(&d, &[], &[scene::EntityRef::new(scene::EntityKind::Load, "co-1", "supportCollisions[id=co-1]")]);
    assert_handle_laws(&grips);
    assert!(grips.iter().any(|h| h.writes.iter().any(|w| w.path == "supportCollisions[id=co-1].assumedForceX")));
    let charts = diagrams(&d, &report, None);
    assert_diagram_laws(&charts);
    for (focus, id) in [("fl-1", "fatigue:axle-loads"), ("co-1", "collision:forces"), ("lf-1", "local-fire:flux"), ("sg-1", "snow-guard:forces")] {
        assert!(charts.iter().any(|c| c.id == id), "{id} in the unfocused list");
        assert!(diagrams(&d, &report, Some(focus)).iter().any(|c| c.id == id), "{focus} focuses {id}");
    }
    let marker = |chart: &str, id: &str| charts.iter().find(|c| c.id == chart).and_then(|c| c.markers.iter().find(|m| m.id == id)).cloned().unwrap_or_else(|| panic!("{chart}/{id}"));
    assert!((marker("fatigue:axle-loads", "required-fl-1").at.y - 210.0).abs() < 1e-9 && (marker("fatigue:axle-loads", "required-fl-3").at.y - 120.0).abs() < 1e-9);
    assert!((marker("collision:forces", "required-co-2").at.y - 500.0).abs() < 1e-9);
    assert!((marker("local-fire:flux", "required-lf-1").at.y - 50.9071348).abs() < 1e-6, "kW/m²");
}

#[test]
fn the_information_collections_have_curves_with_hand_computed_row_markers() {
    let d = En1991Snapshot::full_showcase();
    let (_, report) = scene_of(&d, &[]);
    let charts = diagrams(&d, &report, None);
    assert_diagram_laws(&charts);
    let marker = |chart: &str, id: &str| charts.iter().find(|c| c.id == chart).and_then(|c| c.markers.iter().find(|m| m.id == id)).cloned().unwrap_or_else(|| panic!("{chart}/{id}"));
    let at = |chart: &str, id: &str| { let m = marker(chart, id); (m.at.x, m.at.y) };
    let (x, y) = at("footbridge:load", "fb-1");
    assert!((x - 40.0).abs() < 1e-9 && (y - (2.0 + 120.0 / 70.0)).abs() < 1e-9, "q_fk = 2.0 + 120/(40 + 30) kN/m²");
    let (x, y) = at("hoist:release-factor", "hoist-1");
    assert!((x - 0.5).abs() < 1e-9 && (y - 0.25).abs() < 1e-9, "Φ₃ = 1 − 0.5 (1 + 0.5)");
    let (_, y) = at("rail:dynamic-factor", "rd-1");
    assert!((y - (1.44 / (12f64.sqrt() - 0.2) + 0.82)).abs() < 1e-9, "Φ₂ = 1.44/(√12 − 0.2) + 0.82");
    let (x, y) = at("machine:force", "rm-1");
    assert!((x - 25.0).abs() < 1e-9 && (y - 500.0 * 0.002 * (2.0 * std::f64::consts::PI * 25.0).powi(2) / 1000.0).abs() < 1e-9 && (y - 24.674011).abs() < 1e-5, "F = m e ω² = 24.674 kN");
    let (x, y) = at("bulk:pressure-ratio", "bs-1");
    assert!((x - 0.5236f64.to_degrees()).abs() < 1e-9 && (y - 0.55).abs() < 1e-4, "K_m = 1.1 (1 − sin 30°)");
    let (x, y) = at("dispersal:footprint", "ld-1");
    assert!((x - 0.25).abs() < 1e-9 && (y - 0.85 * 0.85).abs() < 1e-9, "(0.4 + 2 (0.1 + 0.125))² m²");
    assert!(charts.iter().any(|c| c.id == "roof:surface-mu") && charts.iter().any(|c| c.id == "special:vehicle-axles") && charts.iter().any(|c| c.id == "placement:plan"));
    for (focus, id) in [("fb-1", "footbridge:load"), ("hoist-1", "hoist:release-factor"), ("rd-1", "rail:dynamic-factor"), ("rm-1", "machine:force"), ("bs-1", "bulk:pressure-ratio"), ("ld-1", "dispersal:footprint"), ("roof-west", "roof:surface-mu"), ("sv-600", "special:vehicle-axles")] {
        assert!(diagrams(&d, &report, Some(focus)).iter().any(|c| c.id == id), "{focus} focuses {id}");
    }
}

fn routed_documents() -> Vec<En1991Snapshot> {
    vec![En1991Snapshot::example(AnnexChoice::De), rich(), european(), with_collections(), with_wave_3(), with_wave_6(), En1991Snapshot::full_showcase()]
}

fn counts(check: &norm::CheckResult) -> bool {
    check.utilization.is_finite() && !matches!(check.status, norm::CheckStatus::NotApplicable | norm::CheckStatus::Info)
}

fn rank(status: scene::CheckStatus) -> u8 {
    match status {
        scene::CheckStatus::Fail => 3,
        scene::CheckStatus::Warning => 2,
        scene::CheckStatus::Pass => 1,
        _ => 0,
    }
}

/// 🧭️ Every check of every fixture document, as emitted and as filed for the live scene, routes to the scene entity that draws it or to the document badge; the entity stands in the scene and is the one `focus_of_outcome` selects.
#[test]
fn every_check_outcome_routes_to_its_entity_or_the_document_badge() {
    for document in routed_documents() {
        let raw = inferences::evaluate(&document);
        let (scene, _) = scene_of(&document, &[]);
        assert!(!raw.checks.is_empty());
        for report in [raw.clone(), scene_report(&raw)] {
            for check in &report.checks {
                let route = route_of(&document, check).unwrap_or_else(|| panic!("check {} (subject {:?}) has no owner entity and is no document-level family", check.id, check.subject.entity_id));
                match route {
                    CheckRoute::Entity(id) => {
                        if counts(check) {
                            assert!(scene.entities.iter().any(|e| kit::base_id(&e.entity.id) == id), "check {} routes to {id}, which the scene does not draw", check.id);
                        }
                        if report.checks.iter().filter(|c| c.id == check.id).count() == 1 {
                            assert_eq!(focus_of_outcome(&document, &report, &check.id).as_deref(), Some(id.as_str()), "{}", check.id);
                        }
                    }
                    CheckRoute::Document => assert!(focus_of_outcome(&document, &report, &check.id).is_none(), "{}", check.id),
                }
            }
        }
    }
}

/// 📄️ The document-level checks stay unfiled in the live report, so the scene carries the global badge exactly when an applicable one exists, toned by the most severe of them.
#[test]
fn document_level_checks_are_summed_up_by_the_global_badge() {
    for document in routed_documents() {
        let raw = inferences::evaluate(&document);
        let filed = scene_report(&raw);
        let document_checks: Vec<_> = filed.checks.iter().filter(|c| matches!(route_of(&document, c), Some(CheckRoute::Document))).collect();
        assert!(document_checks.iter().all(|c| c.subject.entity_id.is_empty()), "a document-level check was filed under an entity");
        let applicable = document_checks.iter().any(|c| c.status != norm::CheckStatus::NotApplicable);
        let (scene, _) = scene_of(&document, &[]);
        assert_eq!(scene.annotations.iter().any(|a| a.id() == scene::projection_kit::GLOBAL_BADGE), applicable);
    }
}

/// 🌡️ The 3D owner of a check carries at least the severity of the worst check it owns, so a failing outcome always shows on its entity.
#[test]
fn an_entity_is_toned_by_the_worst_check_it_owns() {
    for document in routed_documents() {
        let report = inferences::evaluate(&document);
        let (scene, _) = scene_of(&document, &[]);
        let mut worst: std::collections::BTreeMap<String, u8> = std::collections::BTreeMap::new();
        for check in report.checks.iter().filter(|c| counts(c)) {
            if let Some(CheckRoute::Entity(id)) = route_of(&document, check) {
                let severity = rank(scene::projection_kit::status(check.status));
                let known = worst.entry(id).or_insert(0);
                *known = (*known).max(severity);
            }
        }
        for (id, severity) in worst {
            let shown = scene.entities.iter().filter(|e| kit::base_id(&e.entity.id) == id).filter_map(|e| e.utilisation.as_ref()).map(|b| rank(b.status)).max().unwrap_or(0);
            assert!(shown >= severity, "{id} owns a check of severity {severity} but its entities show {shown}");
        }
    }
}

/// 📈️ Every check family is drawn by a dedicated diagram or by the outcome diagram of its entity, never by both and never by neither; the longest prefix of a check id names its family.
#[test]
fn every_check_family_is_drawn_by_a_dedicated_diagram_or_the_outcome_diagram() {
    for family in DRAWN_FAMILIES {
        assert!(!OUTCOME_FAMILIES.contains(family), "{family} is both drawn and an outcome family");
    }
    let all: Vec<&&str> = DRAWN_FAMILIES.iter().chain(OUTCOME_FAMILIES).collect();
    assert_eq!(all.iter().collect::<BTreeSet<_>>().len(), all.len(), "a family is listed twice");
    for document in routed_documents() {
        for check in &inferences::evaluate(&document).checks {
            if let Some(CheckRoute::Entity(_)) = route_of(&document, check) {
                let family = family_of(&check.id).unwrap_or_else(|| panic!("check {} belongs to no family", check.id));
                assert!(DRAWN_FAMILIES.contains(&family) != OUTCOME_FAMILIES.contains(&family), "{family}");
            }
        }
    }
}

/// 📊️ An entity with checks of undiagrammed families gets one outcome diagram with a bar per check at its utilisation, the limit line at 1, bilingual titles and a table twin; a focus narrows the set to its owner.
#[test]
fn the_outcome_diagram_draws_a_bar_per_check_of_an_undiagrammed_family() {
    let document = En1991Snapshot::full_showcase();
    let report = inferences::evaluate(&document);
    let list = diagrams(&document, &report, None);
    assert_diagram_laws(&list);
    let outcomes: Vec<&scene::Diagram2d> = list.iter().filter(|d| d.id.starts_with("check-outcomes:")).collect();
    assert!(!outcomes.is_empty(), "no outcome diagram");
    for d in &outcomes {
        assert_eq!(d.validate(), Ok(()), "{}", d.id);
        let owner = d.id.trim_start_matches("check-outcomes:");
        let owned: Vec<&norm::CheckResult> = report.checks.iter().filter(|c| kit::is_outcome(c) && owner_entity(&document, c).as_deref() == Some(owner)).collect();
        assert_eq!(d.table_twin.columns[1].values.len(), owned.len(), "{}", d.id);
        assert_eq!(d.markers.len(), owned.len());
        for (marker, check) in d.markers.iter().zip(&owned) {
            assert_eq!(marker.bound_check.as_deref(), Some(check.id.as_str()));
            assert!((marker.at.y - check.utilization.max(0.0)).abs() < 1e-12, "{}: the bar stands at the utilisation of its check", check.id);
        }
        assert!(d.series.iter().any(|s| matches!(s, scene::Series::Limit { value, .. } if (*value - 1.0).abs() < 1e-12)));
        assert!(!d.title.en.is_empty() && !d.title.de.is_empty() && d.table_twin.rows() > 0);
    }
    let owner = outcomes[0].id.trim_start_matches("check-outcomes:").to_owned();
    assert!(diagrams(&document, &report, Some(&owner)).iter().any(|d| d.id == format!("check-outcomes:{owner}")));
    assert!(diagrams(&document, &report, Some("no-such-row")).iter().all(|d| !d.id.starts_with("check-outcomes:")));
}

fn normal_path(path: &str) -> (String, String) {
    match path.split_once("[id=") {
        Some((collection, rest)) => (collection.to_owned(), rest.split_once("].").map_or_else(String::new, |(_, leaf)| leaf.to_owned())),
        None => (String::new(), path.to_owned()),
    }
}

fn written_fields(documents: &[En1991Snapshot]) -> BTreeSet<(String, String)> {
    documents.iter().flat_map(|d| handles(d, &[], &entities_of(d))).flat_map(|h| h.writes.into_iter().map(|w| normal_path(&w.path))).collect()
}

/// 🎚️ Every numeric field of the snapshot (row fields and document scalars) has a viewport grip, geometric or gauge, or a documented exemption; every gauge names a numeric schema field with the schema step and the integer type of the schema.
#[test]
fn every_numeric_field_of_a_row_has_a_viewport_grip_or_a_documented_exemption() {
    let schema = schema();
    let written = written_fields(&routed_documents());
    let gauged = handles::gauges::coverage();
    let numeric = |spec: &serde_json::Value| spec["type"].as_str().is_some_and(|t| t == "number" || t == "integer");
    let exempt = |collection: &str, field: &str| handles::gauges::EXEMPT.iter().any(|(c, f, _)| *c == collection && *f == field);
    let mut covered = 0;
    let properties = schema["properties"].as_object().expect("properties");
    for (name, spec) in properties {
        let fields: Vec<(String, String, serde_json::Value)> = match spec["items"]["$ref"].as_str() {
            Some(def) => schema["$defs"][def.rsplit('/').next().unwrap_or_default()]["properties"].as_object().expect("row definition").iter().filter(|(_, v)| numeric(v)).map(|(f, v)| (name.clone(), f.clone(), v.clone())).collect(),
            None if numeric(spec) => vec![(String::new(), name.clone(), spec.clone())],
            None => Vec::new(),
        };
        for (collection, field, def) in fields {
            let by_grip = written.contains(&(collection.clone(), field.clone()));
            let by_gauge = gauged.iter().any(|(c, f, ..)| *c == collection && *f == field);
            let by_exemption = exempt(&collection, &field);
            assert!(by_grip || by_exemption, "{collection}.{field} has no viewport grip ({def})");
            assert!(!(by_exemption && by_grip), "{collection}.{field} is exempt and has a grip");
            assert!(!by_gauge || by_grip, "{collection}.{field} has a gauge that no document advertises");
            covered += usize::from(by_grip);
        }
    }
    assert!(covered > 150, "{covered} numeric fields have a grip");
    for (collection, field, step, integer) in gauged {
        let spec = if collection.is_empty() { &schema["properties"][field] } else { &schema["$defs"][properties[collection]["items"]["$ref"].as_str().and_then(|r| r.rsplit('/').next()).unwrap_or_default()]["properties"][field] };
        assert!(numeric(spec), "gauge {collection}.{field} names no numeric schema field");
        assert_eq!(spec["type"] == "integer", integer, "{collection}.{field}: integer type");
        if let Some(schema_step) = spec["x-semio-ui"]["step"].as_f64() {
            assert!((schema_step - step).abs() < 1e-12, "{collection}.{field}: gauge step {step} against schema step {schema_step}");
        }
    }
}

/// 🎚️ A gauge writes the current value at zero drag, moves it by twenty steps per metre, writes only existing leaves, stays inside its physical range and the dragged document re-projects.
#[test]
fn gauges_write_existing_leaves_move_by_steps_and_the_dragged_document_re_projects() {
    let document = rich();
    let all = handles(&document, &[], &entities_of(&document));
    assert_handle_laws(&all);
    let gauges: Vec<&scene::Handle> = all.iter().filter(|h| h.id.contains(":gauge-")).collect();
    assert!(gauges.len() >= 30, "{} gauges", gauges.len());
    let mut ids = BTreeSet::new();
    let schema = schema();
    for h in &gauges {
        assert!(ids.insert(h.id.clone()), "duplicate {}", h.id);
        let write = &h.writes[0];
        assert!(known_path(&schema, &write.path), "{} writes unknown leaf {}", h.id, write.path);
        let value = handles::gauges::value_at(&document, &write.path).unwrap_or_else(|| panic!("{} reads no value", write.path));
        assert!((write.map.base - value).abs() < 1e-9, "{}", h.id);
        assert!(h.limits.as_ref().is_some_and(|l| l.contains(0.0)), "{}", h.id);
    }
    let wp = all.iter().find(|h| h.id == "wall-d:gauge-assumed-wp").expect("assumed wind pressure gauge");
    assert_eq!(wp.writes[0].path, "windFaces[id=wall-d].assumedWp");
    let start = document.wind_faces.iter().find(|f| f.id == "wall-d").expect("face").assumed_wp;
    assert!((wp.writes_at(0.0)[0].value - start).abs() <= 25.0);
    let dragged = wp.writes_at(1.0)[0].value;
    assert!((dragged - (start + 1000.0)).abs() <= 25.0, "twenty steps of 50 Pa per metre: {dragged}");
    assert_eq!(wp.writes_at(-1.0e6)[0].value, 0.0, "a pressure never drags below zero");
    let mut moved = document.clone();
    moved.wind_faces.iter_mut().find(|f| f.id == "wall-d").expect("face").assumed_wp = dragged;
    assert_ne!(scene_of(&moved, &[]).1.checks.iter().map(|c| c.utilization.to_bits()).collect::<Vec<_>>(), scene_of(&document, &[]).1.checks.iter().map(|c| c.utilization.to_bits()).collect::<Vec<_>>(), "the dragged pressure changes the check outcome");
    let code = all.iter().find(|h| h.id.ends_with(":gauge-mixed-terrain-upwind")).expect("code gauge");
    assert_eq!(code.writes_at(0.4)[0].value, f64::from(document.mixed_terrain_upwind).round() + 8.0, "an integer code moves by whole steps");
}

#[test]
fn the_basis_badge_follows_the_document_annex_in_the_scene_and_every_diagram() {
    scene::projection_kit::test_support::assert_basis_follows_annex(norm::NormFamilyId::En1991, |annex| {
        let mut document = rich();
        document.annex = annex;
        let (scene, report) = scene_of(&document, &[]);
        (scene, diagrams(&document, &report, None))
    });
}
