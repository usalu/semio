use super::response::{respond, site};
use super::scene::*;
use super::*;
use scene::projection_kit::test_support::assert_handle_laws;
use model::En1998Snapshot;
use semio_s_artifact_norm_en1998_inferences::check_full_seismic;
use serde_json::{json, Value};
use std::collections::BTreeSet;

const REVISION: SceneRevision = SceneRevision { document_seq: 1, report_seq: 1 };

fn fixtures() -> Vec<(&'static str, En1998Snapshot)> {
    vec![
        ("compliant_de_office", En1998Snapshot::compliant_de_office()),
        ("noncompliant_de_office", En1998Snapshot::noncompliant_de_office()),
        ("compliant_de_multipart", En1998Snapshot::compliant_de_multipart()),
        ("noncompliant_de_multipart", En1998Snapshot::noncompliant_de_multipart()),
        ("compliant_en_office", En1998Snapshot::compliant_en_office()),
        ("noncompliant_en_office", En1998Snapshot::noncompliant_en_office()),
        ("compliant_de_torsion_regular", En1998Snapshot::compliant_de_torsion_regular()),
        ("noncompliant_de_torsion_irregular", En1998Snapshot::noncompliant_de_torsion_irregular()),
        ("full_showcase", En1998Snapshot::full_showcase()),
    ]
}

fn scene_of(document: &En1998Snapshot) -> (StructuralScene, CheckReport) {
    let report = check_full_seismic(document);
    let scene = project(&SceneInput { document, report: &report, placements: &[], revision: REVISION });
    (scene, report)
}

fn within(a: f64, b: f64, relative: f64, what: &str) {
    assert!((a - b).abs() <= relative * b.abs(), "{what}: {a} != {b}");
}

fn near(a: f64, b: f64, what: &str) {
    assert!((a - b).abs() <= 1e-6 * b.abs().max(1.0), "{what}: {a} != {b}");
}

fn digest(scene: &StructuralScene) -> Vec<String> {
    scene.entities.iter().map(|e| format!("{}|{:?}|{}|{}", e.entity.id, e.entity.kind, e.layer, e.content_hash)).chain(scene.annotations.iter().map(|a| format!("annotation|{}", a.id()))).collect()
}

fn golden(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧪️tests/🧫️fixtures").join(format!("{name}.txt"))
}

fn oracle(case: &Value) -> Value {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧪️tests/🔮️oracle/🐍️.py");
    let out = norm::python_oracle::run_ok(script, &[], Some(case.to_string().as_bytes()));
    serde_json::from_slice(&out.stdout).expect("oracle json")
}

fn numbers(v: &Value) -> Vec<f64> {
    v.as_array().expect("array").iter().map(|x| x.as_f64().expect("number")).collect()
}

fn arrows(scene: &StructuralScene, building: &str, system: &str) -> Vec<f64> {
    scene
        .entities
        .iter()
        .filter(|e| e.entity.id.starts_with(&format!("{building}:")) && e.entity.id.ends_with(&format!(":force-{system}")))
        .filter_map(|e| match &e.geometry {
            Geometry::Load(LoadGlyph::Point { magnitude, .. }) => Some(magnitude.value),
            _ => None,
        })
        .collect()
}

#[test]
fn every_fixture_projects_a_valid_scene() {
    for (name, document) in fixtures() {
        let (scene, _) = scene_of(&document);
        assert!(scene.validate().is_ok(), "{name}");
        assert!(!scene.entities.is_empty(), "{name}");
        assert!(scene.entities.iter().all(|e| e.content_hash == e.compute_hash()), "{name}");
    }
}

#[test]
fn projection_is_deterministic_and_locale_free() {
    let document = En1998Snapshot::compliant_de_multipart();
    let (a, _) = scene_of(&document);
    let (b, _) = scene_of(&document);
    assert_eq!(canonical_json(&a), canonical_json(&b));
    assert!(a.description.en.contains("EN 1998") && a.description.de.contains("EN-1998"));
}

#[test]
fn golden_digests_match_the_committed_files() {
    for name in ["compliant_de_office", "noncompliant_de_office", "compliant_de_multipart", "full_showcase"] {
        let document = fixtures().into_iter().find(|f| f.0 == name).expect("fixture").1;
        let text = digest(&scene_of(&document).0).join("\n") + "\n";
        let path = golden(name);
        if std::env::var_os("SEMIO_UPDATE_GOLDEN").is_some() || !path.exists() {
            std::fs::create_dir_all(path.parent().expect("dir")).expect("create");
            std::fs::write(&path, &text).expect("write golden");
        }
        assert_eq!(std::fs::read_to_string(&path).unwrap_or_default(), text, "{name}: rerun with SEMIO_UPDATE_GOLDEN=1 after an intended change");
    }
}

#[test]
fn golden_scene_matches_committed_bytes() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json");
    let actual = canonical_json(&scene_of(&En1998Snapshot::compliant_de_multipart()).0);
    if std::env::var("SEMIO_UPDATE_GOLDEN").is_ok() {
        std::fs::write(path, &actual).expect("write golden");
    }
    let expected = std::fs::read_to_string(path).expect("golden fixture");
    assert_eq!(actual.trim(), expected.trim(), "golden scene changed; rerun with SEMIO_UPDATE_GOLDEN=1 after reviewing the diff");
}

#[test]
fn every_utilisation_binding_names_a_report_check() {
    for (name, document) in fixtures() {
        let (scene, report) = scene_of(&document);
        let ids: BTreeSet<&str> = report.checks.iter().map(|c| c.id.as_str()).collect();
        for e in &scene.entities {
            if let Some(b) = &e.utilisation {
                assert!(ids.contains(b.check_id.as_str()), "{name}: {}", b.check_id);
            }
        }
    }
}

#[test]
fn every_listed_handle_exists_and_writes_the_current_value_at_zero() {
    let document = En1998Snapshot::compliant_de_multipart();
    let (scene, _) = scene_of(&document);
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    let ids: BTreeSet<&str> = all.iter().map(|h| h.id.as_str()).collect();
    assert!(!all.is_empty());
    assert!(all.iter().all(|h| h.validate().is_ok()));
    for e in &scene.entities {
        for h in &e.handles {
            assert!(ids.contains(h.as_str()), "{h}");
        }
    }
    let height = all.iter().find(|h| h.id == "bldg-office:s1:h" || h.id.ends_with(":s1:h")).expect("storey height handle");
    let writes = height.writes_at(0.0);
    near(writes[0].value, document.buildings[0].storeys[0].height_m, "height");
    assert!(writes[0].path.ends_with("storeys[id=s1].heightM"));
}

#[test]
fn failing_documents_show_error_styles() {
    let (scene, _) = scene_of(&En1998Snapshot::noncompliant_de_office());
    assert!(scene.entities.iter().any(|e| e.appearance.style == StyleState::Error));
    let (good, _) = scene_of(&En1998Snapshot::compliant_de_office());
    assert!(!good.entities.iter().any(|e| e.utilisation.as_ref().is_some_and(|u| u.status == scene::CheckStatus::Fail)));
}

#[test]
fn every_variable_action_row_is_a_load_glyph_with_a_magnitude_grip_writing_its_typed_leaf() {
    let mut expected = 0;
    for document in [En1998Snapshot::compliant_de_multipart(), En1998Snapshot::full_showcase(), En1998Snapshot::compliant_de_office()] {
        let (scene, _) = scene_of(&document);
        let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
        assert_handle_laws(&all);
        let rows: Vec<(String, String, f64)> = document
            .buildings
            .iter()
            .flat_map(|b| b.storeys.iter().flat_map(move |s| s.variables.iter().map(move |v| (format!("{}:{}:var-{}", b.id, s.id, v.id), format!("buildings[id={}].storeys[id={}].variables[id={}].qkN", b.id, s.id, v.id), v.qk_n))))
            .chain(document.bridges.iter().flat_map(|b| b.variables.iter().map(move |v| (format!("{}:var-{}", b.id, v.id), format!("bridges[id={}].variables[id={}].qkN", b.id, v.id), v.qk_n))))
            .chain(document.towers.iter().flat_map(|t| t.variables.iter().map(move |v| (format!("{}:var-{}", t.id, v.id), format!("towers[id={}].variables[id={}].qkN", t.id, v.id), v.qk_n))))
            .collect();
        for (id, path, qk) in rows {
            expected += 1;
            let glyph = scene.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("{id}"));
            assert!(matches!(&glyph.geometry, Geometry::Load(LoadGlyph::Point { direction, magnitude, case, .. }) if direction.z < 0.0 && magnitude.value == qk && case.kind == LoadKind::Variable), "{id}");
            let grip = all.iter().find(|h| h.id == format!("{id}:qk")).unwrap_or_else(|| panic!("{id}:qk"));
            assert!(glyph.handles.contains(&grip.id));
            assert_eq!((grip.writes[0].path.as_str(), grip.writes[0].map.base, grip.writes[0].unit.as_str()), (path.as_str(), qk, "N"));
            let at = match &glyph.geometry {
                Geometry::Load(LoadGlyph::Point { at, .. }) => at.clone(),
                _ => unreachable!(),
            };
            assert!((grip.anchor.z - at.z - qk / grip.writes[0].map.k.abs()).abs() < 1e-9, "the grip stands at the magnitude length above the level: {id}");
        }
    }
    assert!(expected >= 3);
}

#[test]
fn every_storey_has_slab_columns_mass_centres_and_one_drift_bar_per_system() {
    let document = En1998Snapshot::compliant_de_office();
    let (scene, _) = scene_of(&document);
    let b = &document.buildings[0];
    for s in &b.storeys {
        let prefix = format!("{}:{}", b.id, s.id);
        let has = |tail: &str| scene.entities.iter().any(|e| e.entity.id == format!("{prefix}{tail}"));
        assert!(has("") && has(":mass") && has(":com") && has(":cos"), "{prefix}");
        assert!(scene.entities.iter().any(|e| e.entity.id.starts_with(&format!("{prefix}:col-"))));
        assert!(scene.entities.iter().any(|e| e.entity.id.starts_with(&format!("{prefix}:joint-"))));
        for sys in &b.systems {
            assert!(has(&format!(":drift-{}", sys.id)) && has(&format!(":force-{}", sys.id)), "{prefix} {}", sys.id);
        }
    }
}

#[test]
fn drift_bar_length_is_proportional_to_the_utilisation() {
    let document = En1998Snapshot::compliant_de_office();
    let (scene, report) = scene_of(&document);
    for e in scene.entities.iter().filter(|e| e.entity.id.contains(":drift-")) {
        let Geometry::Solid(SolidGeometry { shape: SolidShape::Box { size, .. } }) = &e.geometry else { panic!("drift bar is a box") };
        let util = e.utilisation.as_ref().map_or(0.0, |u| u.ratio).min(3.0);
        near(size.x, (util * 2.0).max(0.05), &e.entity.id);
        assert!(report.checks.iter().any(|c| Some(&c.id) == e.utilisation.as_ref().map(|u| &u.check_id)));
    }
}

#[test]
fn seismic_joint_turns_red_when_the_gap_is_too_small() {
    let mut document = En1998Snapshot::compliant_de_office();
    let mut second = document.buildings[0].clone();
    second.id = "bldg-b".into();
    document.buildings.push(second);
    let report = check_full_seismic(&document);
    let tight = [Placement { entity_id: "bldg-b".into(), origin: Vec3::new(24.01, 0.0, 0.0), yaw: 0.0 }];
    let scene = project(&SceneInput { document: &document, report: &report, placements: &tight, revision: REVISION });
    let joint = scene.entities.iter().find(|e| e.entity.id.ends_with(":joint") && e.entity.id.starts_with("bldg-office:bldg-b")).expect("seismic joint");
    let needed = response::joint_demand(&document.buildings[0], &document.buildings[1]);
    assert_eq!(joint.appearance.style == StyleState::Error, 0.01 < needed);
    let wide = project(&SceneInput { document: &document, report: &report, placements: &[], revision: REVISION });
    assert!(wide.entities.iter().find(|e| e.entity.id.ends_with(":joint")).is_some_and(|e| e.appearance.style != StyleState::Error));
}

#[test]
fn placements_move_entities_and_the_default_layout_is_documented() {
    let document = En1998Snapshot::compliant_de_multipart();
    let layout = Layout::of(&document, &[]);
    assert_eq!(layout.origin(&document.buildings[0].id), Vec3::ZERO);
    assert_eq!(layout.origin(&document.silos[0].id), Vec3::new(0.0, layout::ROW_SILOS, 0.0));
    let moved = Layout::of(&document, &[Placement { entity_id: document.silos[0].id.clone(), origin: Vec3::new(5.0, 6.0, 0.0), yaw: 0.0 }]);
    assert_eq!(moved.origin(&document.silos[0].id), Vec3::new(5.0, 6.0, 0.0));
}

#[test]
fn registry_resolves_every_collection_and_dependencies_link_foundations_to_buildings() {
    let document = En1998Snapshot::compliant_de_multipart();
    for (id, collection) in [("silo", "silos"), ("tank", "tanks"), ("tower", "towers"), ("foundation", "foundations")] {
        let hit = ENTITY_KINDS.iter().find(|k| k.id == id).expect("kind");
        assert_eq!(hit.collection, collection);
    }
    let foundation = &document.foundations[0];
    assert_eq!(entity_kind_of(&document, &foundation.id).map(|k| k.id), Some("foundation"));
    assert!(entity_dependencies(&document, &foundation.id).contains(&foundation.supported_building_id));
    assert!(entity_dependencies(&document, &foundation.supported_building_id).contains(&foundation.id));
}

#[test]
fn vessels_towers_foundations_walls_and_bridges_are_projected() {
    let document = En1998Snapshot::compliant_de_multipart();
    let (scene, _) = scene_of(&document);
    for id in [&document.silos[0].id, &document.tanks[0].id, &document.towers[0].id, &document.foundations[0].id, &document.retaining_walls[0].id, &document.bridges[0].id] {
        assert!(scene.entities.iter().any(|e| &e.entity.id == id), "{id}");
    }
}

#[test]
fn a_wall_with_retained_water_projects_the_water_volume_its_thrust_and_a_live_grip() {
    let mut document = En1998Snapshot::compliant_de_multipart();
    let id = document.retaining_walls[0].id.clone();
    document.retaining_walls[0].water_height_m = 2.0;
    document.retaining_walls[0].permeability_m_s = 1.0e-3;
    let (scene, report) = scene_of(&document);
    assert!(scene.entities.iter().any(|e| e.entity.id == format!("{id}:water")), "water volume");
    assert!(report.checks.iter().any(|c| c.id == format!("en1998.5.{id}.water")), "water check");
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert!(all.iter().any(|h| h.id == format!("{id}:water") && h.writes_at(0.0)[0].path.ends_with(".waterHeightM")), "water height grip");
    let mut dry = document.clone();
    dry.retaining_walls[0].water_height_m = 0.0;
    assert!(!scene_of(&dry).0.entities.iter().any(|e| e.entity.id == format!("{id}:water")), "no water without a height");
}

fn tie_and_slope_document() -> En1998Snapshot {
    let mut document = En1998Snapshot::full_showcase();
    let mut second = document.foundations[0].clone();
    second.id = "fd-2".into();
    second.supported_building_id = document.buildings[1].id.clone();
    document.foundations.push(second);
    let first = document.foundations[0].id.clone();
    document.tie_beams = vec![model::En1998TieBeam { id: "tb-1".into(), foundation_a_id: first, foundation_b_id: "fd-2".into(), span: 6.0, width: 0.4, depth: 0.6, steel_area: 0.0012, steel_design_strength: 434.8e6, relative_displacement: 0.01, displacement_limit: 0.03 }];
    let layer = document.soil_layers[0].id.clone();
    document.slopes = vec![model::En1998Slope { id: "sl-1".into(), governing_layer_id: layer, height: 8.0, angle: 30.0, friction_angle: 24.0, cohesion: 0.0, topographic_amplification: 1.2, circle_centre_x: 4.0, circle_centre_z: 12.0, circle_radius: 13.0 }];
    document
}

#[test]
fn a_tie_beam_joins_its_footings_carries_the_tie_force_at_both_ends_and_has_live_grips() {
    let document = tie_and_slope_document();
    let (scene, report) = scene_of(&document);
    let beam = scene.entities.iter().find(|e| e.entity.id == "tb-1").expect("tie beam");
    assert!(beam.utilisation.is_some(), "coloured by its checks");
    let force = report.checks.iter().find(|c| c.id == "en1998.5.tb-1.tie").map(|c| c.computed.value);
    for end in ["a", "b"] {
        assert!(scene.entities.iter().any(|e| e.entity.id == format!("tb-1:tie-force-{end}")), "tie force at end {end}");
    }
    assert!(force.is_some_and(|n| n > 0.0));
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    for (grip, field) in [("tb-1:w", ".width"), ("tb-1:d", ".depth"), ("tb-1:u", ".relativeDisplacement")] {
        assert!(all.iter().any(|h| h.id == grip && h.writes_at(0.0)[0].path.ends_with(field)), "{grip}");
    }
    let mut dangling = document.clone();
    dangling.tie_beams[0].foundation_b_id = "fd-9".into();
    assert!(!scene_of(&dangling).0.entities.iter().any(|e| e.entity.id == "tb-1"), "no beam without its second footing");
}

#[test]
fn a_slope_projects_its_body_both_slip_circles_the_seismic_forces_and_circle_grips_and_a_factor_diagram() {
    let document = tie_and_slope_document();
    let (scene, report) = scene_of(&document);
    for id in ["sl-1", "sl-1:slip-circle", "sl-1:critical-circle", "sl-1:fh", "sl-1:fv-down", "sl-1:fv-up"] {
        assert!(scene.entities.iter().any(|e| e.entity.id == id), "{id}");
    }
    let check = report.checks.iter().find(|c| c.id == "en1998.5.sl-1.slope").expect("slope check");
    let body = scene.entities.iter().find(|e| e.entity.id == "sl-1").expect("slope body");
    assert_eq!(body.utilisation.as_ref().map(|u| u.check_id.as_str()), Some(check.id.as_str()));
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    for (grip, field) in [("sl-1:h", ".height"), ("sl-1:slip-circle:x", ".circleCentreX"), ("sl-1:slip-circle:z", ".circleCentreZ"), ("sl-1:slip-circle:r", ".circleRadius")] {
        assert!(all.iter().any(|h| h.id == grip && h.writes_at(0.0)[0].path.ends_with(field)), "{grip}");
    }
    let diagram = diagrams(&document, &report, None).into_iter().find(|d| d.id == "slope-factor:sl-1").expect("factor diagram");
    assert!(diagram.markers.iter().any(|m| m.bound_check.as_deref() == Some(check.id.as_str())));
    let factors = diagram.table_twin.columns.last().expect("factor column").values.clone();
    assert!(factors.windows(2).all(|w| w[1] <= w[0] + 1e-9), "the factor falls with the seismic coefficient");
    let mut flat = document.clone();
    flat.slopes[0].angle = 0.5;
    assert!(!scene_of(&flat).0.entities.iter().any(|e| e.entity.id == "sl-1"), "a flat ground is no slope");
}

#[test]
fn diagrams_validate_and_carry_the_period_markers() {
    for (name, document) in fixtures() {
        let (_, report) = scene_of(&document);
        let set = diagrams(&document, &report, None);
        assert!(set.iter().any(|d| d.id == "plateau-de:document"), "{name}");
        for d in &set {
            assert!(d.validate().is_ok(), "{name} {}", d.id);
        }
        if let Some(s) = set.iter().find(|d| d.id == "spectra:document") {
            for corner in ["reference:corner-TB", "reference:corner-TC", "reference:corner-TD"] {
                let m = s.markers.iter().find(|m| m.id == corner).expect(corner);
                let Series::Polyline { points, .. } = &s.series[1] else { panic!("design series") };
                assert!(points.iter().any(|p| (p.x - m.at.x).abs() < 1e-12 && (p.y - m.at.y).abs() < 1e-9), "{corner}");
            }
        }
    }
}

#[test]
fn scene_matches_the_numpy_oracle_for_spectra_forces_and_modes() {
    for name in ["compliant_de_office", "compliant_de_multipart", "compliant_en_office"] {
        let document = fixtures().into_iter().find(|f| f.0 == name).expect("fixture").1;
        let (scene, report) = scene_of(&document);
        let site = site(&document);
        let building = &document.buildings[0];
        let r = respond(&site, building);
        let spectra = diagrams(&document, &report, None).into_iter().find(|d| d.id == "spectra:document").expect("spectra");
        let (Series::Polyline { points: elastic, .. }, Series::Polyline { points: design, .. }) = (&spectra.series[0], &spectra.series[1]) else { panic!("series") };
        let periods: Vec<f64> = elastic.iter().map(|p| p.x).collect();
        let system = &r.systems[0];
        let case = json!({
            "spectrum": {"ag": site.gamma_i * site.a_gr, "s": site.s, "tb": site.tb, "tc": site.tc, "td": site.td, "q": system.q, "beta": 0.2},
            "site": {"a_gr": site.a_gr, "s": site.s, "tb": site.tb, "tc": site.tc, "td": site.td, "gamma_i": site.gamma_i},
            "q": system.q, "t1": r.t1, "periods": periods, "masses": r.masses, "elevations": r.elevations,
            "stiffness": building.storeys.iter().map(|s| s.stiffness_x).collect::<Vec<_>>(),
        });
        let expected = oracle(&case);
        for (got, want) in elastic.iter().zip(numbers(&expected["elastic"])) {
            near(got.y, want, "elastic");
        }
        for (got, want) in design.iter().zip(numbers(&expected["design"])) {
            near(got.y, want, "design");
        }
        let torsion = system.delta;
        for (got, want) in arrows(&scene, &building.id, &system.id).iter().zip(numbers(&expected["forces"])) {
            near(*got, want * torsion, "storey force");
        }
        near(system.f_b, expected["baseShear"].as_f64().expect("fb"), "base shear");
        let mode = numbers(&expected["mode"]);
        if !mode.is_empty() {
            near(r.mode_period, expected["modePeriod"].as_f64().expect("period"), "mode period");
            let first = &expected["modes"][0];
            let amplitude = first["amplitude"].as_f64().expect("amplitude");
            near(r.modes.iter().find(|m| !m.along_y && m.index == 1).expect("mode 1").gamma, first["gamma"].as_f64().expect("gamma"), "participation");
            near(r.modes.iter().find(|m| !m.along_y && m.index == 1).expect("mode 1").m_eff_ratio, first["meff"].as_f64().expect("meff"), "effective mass");
            near(r.modes.iter().find(|m| !m.along_y && m.index == 1).expect("mode 1").amplitude_m, amplitude, "modal amplitude");
            for (storey, want) in building.storeys.iter().zip(&mode) {
                let slab = scene.entities.iter().find(|e| e.entity.id == format!("{}:{}:mode-x1", building.id, storey.id)).expect("mode slab");
                let overlay = slab.appearance.deformation.as_ref().expect("mode overlay");
                near(overlay.samples[0].delta.x, want * amplitude, "mode shape");
            }
        }
    }
}

#[test]
fn torsion_uses_the_plan_dimension_perpendicular_to_the_action() {
    let mut document = fixtures().into_iter().find(|f| f.0 == "compliant_de_office").expect("fixture").1;
    let site = site(&document);
    let building = &mut document.buildings[0];
    building.plan_width_m = 30.0;
    building.plan_length_m = 18.0;
    for storey in &mut building.storeys {
        storey.centre_of_mass_x_m = 15.0;
        storey.centre_of_stiffness_x_m = 13.0;
        storey.centre_of_mass_y_m = 9.0;
        storey.centre_of_stiffness_y_m = 8.0;
    }
    for direction in ["x", "y"] {
        building.systems[0].direction = direction.into();
        let r = respond(&site, building);
        let (perpendicular, offset) = if direction == "y" { (30.0, 2.0) } else { (18.0, 1.0) };
        near(r.systems[0].delta, 1.0 + 0.6 * (0.5 * perpendicular + offset) / perpendicular, "torsion factor");
    }
}

#[test]
fn eigenmodes_are_ascending_and_mass_participation_sums_below_one() {
    let document = fixtures().into_iter().find(|f| f.0 == "compliant_de_office").expect("fixture").1;
    let r = respond(&site(&document), &document.buildings[0]);
    let x: Vec<_> = r.modes.iter().filter(|m| !m.along_y).collect();
    assert!(x.windows(2).all(|w| w[0].period >= w[1].period), "periods descend with the mode index");
    assert!(x.iter().map(|m| m.m_eff_ratio).sum::<f64>() <= 1.0 + 1e-9);
}

#[test]
fn mode_and_phase_tracks_follow_the_mode_layers() {
    let mut seen = 0;
    for (name, document) in fixtures() {
        let (scene, _) = scene_of(&document);
        scene.validate().unwrap_or_else(|v| panic!("{name}: {v:?}"));
        let layers: Vec<&str> = kit::MODE_LAYERS.iter().map(|l| l.0).filter(|id| scene.entities.iter().any(|e| e.layer == *id)).collect();
        let track = scene.tracks.iter().find(|t| t.id == "mode");
        assert_eq!(track.is_some(), layers.len() >= 2, "{name}");
        assert_eq!(scene.tracks.iter().any(|t| t.id == "phase"), layers.len() >= 2, "{name}");
        let Some(track) = track else { continue };
        seen += 1;
        assert_eq!(track.kind, TrackKind::Mode);
        assert_eq!(track.keyframes.iter().filter_map(|k| k.layer.as_deref()).collect::<Vec<_>>(), layers, "{name}");
        assert!(scene.layers.iter().all(|l| l.visible), "{name}: track layers must start visible");
    }
    assert!(seen > 0, "at least one fixture offers a mode track");
    assert_eq!(tracks::cantilever_shape(1, 1.0), 1.0);
    assert!((tracks::cantilever_shape(2, 1.0) + 1.0).abs() < 1e-12);
}

fn rich_document() -> En1998Snapshot {
    let mut d = En1998Snapshot::compliant_de_multipart();
    let a = &mut d.assessments[0];
    (a.section_depth_m, a.section_width_m, a.effective_depth_m, a.shear_span_m, a.axial_force_n) = (0.5, 0.3, 0.45, 2.0, 100.0e3);
    (a.tension_steel_area_m2, a.compression_steel_area_m2, a.bar_diameter_mm, a.steel_yield_mpa, a.concrete_strength_mpa) = (1.8e-3, 0.9e-3, 20.0, 500.0, 30.0);
    (a.stirrup_leg_area_m2, a.stirrup_spacing_m, a.stirrup_yield_mpa, a.core_width_m, a.core_depth_m, a.restrained_bar_count, a.restrained_bar_spacing_m) = (1.57e-4, 0.1, 500.0, 0.24, 0.44, 4.0, 0.12);
    a.chord_rotation_demand_rad = 0.01;
    let f = &mut d.foundations[0];
    (f.liq_n1_60, f.liq_fines_percent, f.liq_depth_m, f.liq_water_depth_m, f.liq_unit_weight_n_m3, f.liq_magnitude, f.liq_layer_thickness_m) = (12.0, 20.0, 6.0, 2.0, 18000.0, 6.5, 3.0);
    (f.footing_width_m, f.bearing_n_max_n, f.soil_friction_angle_deg, f.bearing_axial_n, f.bearing_shear_n, f.bearing_moment_nm) = (2.0, 1.0e6, 35.0, 0.3e6, 0.05e6, 0.08e6);
    (f.pile_diameter_m, f.pile_length_m, f.pile_modulus_pa, f.pile_subgrade_modulus_n_m3, f.pile_head_shear_n, f.pile_head_fixed) = (0.6, 20.0, 30.0e9, 20.0e6, 100.0e3, 0.0);
    (f.pile_interface_depth_m, f.pile_upper_vs_m_s, f.pile_lower_vs_m_s, f.pile_moment_rd_nm) = (6.0, 100.0, 300.0, 1.0e6);
    let b = &mut d.bridges[0];
    (b.pier_height_m, b.pier_moment_rd_nm, b.pier_shear_rd_n) = (8.0, 4.0e6, 1.0e6);
    d.towers[0].ei_nm2 = 2.0e10;
    d
}

#[test]
fn pushover_reports_n2_performance_points_for_dl_sd_and_nc_in_ascending_order() {
    let (document, report) = (En1998Snapshot::compliant_de_multipart(), check_full_seismic(&En1998Snapshot::compliant_de_multipart()));
    let building = &document.buildings[0];
    let diagram = capacity::pushover(&document, &report, building).expect("pushover");
    assert_eq!(diagram.validate(), Ok(()));
    let system = &building.systems[0].id;
    let x = |state: &str| diagram.markers.iter().find(|m| m.id == format!("target-{system}-{state}")).unwrap_or_else(|| panic!("{state} point")).at.x;
    assert!(x("dl") <= x("sd") && x("sd") <= x("nc"), "{} {} {}", x("dl"), x("sd"), x("nc"));
    assert_eq!(diagram.table_twin.rows(), 3 * building.systems.iter().filter(|s| diagram.markers.iter().any(|m| m.id == format!("reference:yield-{}", s.id))).count());
    assert!(diagram.markers.iter().any(|m| m.id == format!("reference:yield-{system}")));
}

#[test]
fn the_drift_heat_map_prisms_carry_the_drift_fields_and_a_legend() {
    let (scene, _) = scene_of(&En1998Snapshot::compliant_de_multipart());
    let prisms: Vec<&SceneEntity> = scene.entities.iter().filter(|e| e.layer == kit::LAYER_DRIFT_MAP).collect();
    assert!(!prisms.is_empty());
    for e in prisms {
        let ColourChannel::Fields { active, fields } = &e.appearance.channel else { panic!("field channel") };
        assert_eq!(active, "drift-utilisation");
        assert_eq!(fields.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), ["drift-utilisation", "drift-x", "drift-y"]);
    }
    assert!(scene.legends.iter().any(|l| l.id == "drift-utilisation"));
}

#[test]
fn towers_with_a_stiffness_get_three_animated_mode_ghosts_on_the_mode_layers() {
    let document = rich_document();
    let (scene, report) = scene_of(&document);
    let t1 = report.checks.iter().find(|c| c.id == "en1998.6.tw-1.period").expect("period").computed.value;
    for n in 1..=3_u32 {
        let ghost = scene.entities.iter().find(|e| e.entity.id == format!("tw-1:mode-x{n}")).unwrap_or_else(|| panic!("mode {n}"));
        let overlay = ghost.appearance.deformation.as_ref().expect("overlay");
        assert_eq!(overlay.motion, Motion::Harmonic);
        within(overlay.frequency.as_ref().expect("frequency").value, 1.0 / (t1 * schema::part_6::cantilever_period_ratio(n)), 1.2e-2, "frequency of the kernel mode against the closed form");
        if n <= 2 {
            within(overlay.samples.last().expect("tip").delta.x / overlay.samples[overlay.samples.len() / 2].delta.x, 1.0 / schema::part_6::cantilever_mode_shape(n, 0.5), 3.0e-2, "tip over middle");
        }
    }
    assert!(scene.tracks.iter().any(|t| t.id == "mode"));
}

#[test]
fn assessed_member_pier_arrows_and_the_pile_are_projected_with_valid_handles_and_diagrams() {
    let document = rich_document();
    let (scene, report) = scene_of(&document);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    let ids: BTreeSet<&str> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    for id in ["as-1:member", "br-1:pier-a:vcd", "br-1:pier-a:vrd", "br-1:pier-a:mrd", "fd-1:pile", "fd-1:pile-interface", "fd-1:bearing-n", "fd-1:bearing-v", "fd-1:bearing-m", "fd-1:liquefiable-layer"] {
        assert!(ids.contains(id), "{id}");
    }
    let shear = scene.entities.iter().find(|e| e.entity.id == "br-1:pier-a:vcd").expect("V_CD");
    let Geometry::Load(LoadGlyph::Point { magnitude, .. }) = &shear.geometry else { panic!("point load") };
    near(magnitude.value, report.checks.iter().find(|c| c.id == "en1998.2.br-1.pier.shear").expect("pier check").computed.value, "V_CD equals the check");
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    let found: BTreeSet<&str> = all.iter().map(|h| h.id.as_str()).collect();
    for id in ["as-1:member:span", "as-1:member:depth", "br-1:pier-a:h", "br-1:pier-a:mrd:g", "br-1:pier-a:vrd:g", "fd-1:pile:l", "fd-1:pile-interface:z", "fd-1:liquefiable-layer:t"] {
        assert!(found.contains(id), "{id}");
    }
    for e in &scene.entities {
        for h in &e.handles {
            assert!(found.contains(h.as_str()), "{h}");
        }
    }
    near(all.iter().find(|h| h.id == "as-1:member:span").expect("span").writes_at(0.0)[0].value, document.assessments[0].shear_span_m, "span at zero");
    near(all.iter().find(|h| h.id == "fd-1:pile:l").expect("pile").writes_at(0.0)[0].value, document.foundations[0].pile_length_m, "pile length at zero");
    let diagrams = diagrams(&document, &report, None);
    for id in ["liquefaction-profile:fd-1", "annex-f:fd-1", "pile-moments:fd-1"] {
        let diagram = diagrams.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id}"));
        assert_eq!(diagram.validate(), Ok(()), "{id}");
        assert!(diagram.table_twin.rows() > 0, "{id}");
    }
}

#[test]
fn layered_soil_column_has_interface_handles_and_a_profile_diagram_with_a_table_twin() {
    let mut document = rich_document();
    let foundation = document.foundations[0].id.clone();
    let row = |id: &str, thickness_m: f64, n1_60: f64, v_s: f64| model::En1998SoilLayer { id: id.into(), foundation_id: foundation.clone(), thickness_m, unit_weight_n_m3: 18000.0, shear_wave_velocity_m_s: v_s, n1_60, fines_percent: 15.0 };
    document.soil_layers = vec![row("sl-1", 3.0, 0.0, 250.0), row("sl-2", 4.0, 8.0, 150.0), row("sl-3", 6.0, 22.0, 220.0)];
    let (scene, report) = scene_of(&document);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    let ids: BTreeSet<&str> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    for id in ["sl-1", "sl-2", "sl-3"] {
        assert!(ids.contains(id), "{id}");
    }
    assert!(!ids.contains("fd-1:liquefiable-layer"), "the single liquefiable layer is replaced by the soil column");
    let weak = scene.entities.iter().find(|e| e.entity.id == "sl-2").expect("weak layer");
    assert!(weak.utilisation.is_some(), "an assessed layer is bound to its own check");
    assert!(scene.entities.iter().find(|e| e.entity.id == "sl-1").expect("clay layer").utilisation.is_none());
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    for (id, thickness) in [("sl-1:interface", 3.0), ("sl-2:interface", 4.0), ("sl-3:interface", 6.0)] {
        let grip = all.iter().find(|h| h.id == id).unwrap_or_else(|| panic!("{id}"));
        near(grip.writes_at(0.0)[0].value, thickness, id);
        assert!(grip.writes_at(0.0)[0].path.ends_with(".thicknessM"));
    }
    for e in scene.entities.iter().filter(|e| e.entity.id.starts_with("sl-")) {
        for h in &e.handles {
            assert!(all.iter().any(|g| &g.id == h), "{h}");
        }
    }
    let set = diagrams(&document, &report, None);
    let profile = set.iter().find(|d| d.id == "soil-profile:fd-1").expect("soil profile");
    assert_eq!(profile.validate(), Ok(()));
    assert!(profile.table_twin.rows() >= 13, "one row per half metre down to 13 m");
    assert!(profile.markers.iter().any(|m| m.id == "layer-sl-2"));
    assert!(!profile.markers.iter().any(|m| m.id == "layer-sl-1"), "a layer with (N1)60 = 0 is not assessed");
}

fn fold_into(document: &mut En1998Snapshot, commit: &scene::projection_kit::PlacementCommit) {
    let mut rows = scene::projection_kit::placements_of(document);
    commit.apply(&mut rows);
    document.placements = rows.into_iter().map(|row| model::En1998Placement { id: row.entity_id, x: row.origin.x, y: row.origin.y, z: row.origin.z, yaw: row.yaw }).collect();
}

#[test]
fn dragging_an_entity_commits_one_undoable_event_and_the_reload_stands_at_the_same_pose() {
    use scene::projection_kit::{PlacementCommit, PlacementMove};
    let mut document = En1998Snapshot::compliant_de_office();
    let id = document.buildings[0].id.clone();
    let before = scene::projection_kit::placements_of(&document);
    assert!(before.is_empty());
    let target = scene::Vec3::new(7.5, -2.0, 0.0);
    let commit = PlacementCommit::of(&before, &[PlacementMove { entity_id: id.clone(), origin: target.clone(), yaw: 0.75 }]);
    assert_eq!(commit.ops.len(), 1);
    fold_into(&mut document, &commit);
    let reloaded = scene::projection_kit::placements_of(&document);
    assert_eq!(reloaded.len(), 1);
    assert_eq!(Layout::of(&document, &reloaded).origin(&id), target);
    let again = PlacementCommit::of(&reloaded, &[PlacementMove { entity_id: id.clone(), origin: target, yaw: 0.75 }]);
    assert!(again.is_empty());
    fold_into(&mut document, &commit.inverse(&before));
    assert_eq!(scene::projection_kit::placements_of(&document), before);
}

fn horizontal_document() -> En1998Snapshot {
    let mut document = En1998Snapshot::compliant_de_multipart();
    let building = document.buildings[0].clone();
    document.diaphragms = vec![model::En1998Diaphragm { id: "dia-1".into(), building_id: building.id.clone(), storey_id: building.storeys[1].id.clone(), material: semio_s_artifact_norm_en1998_model::DiaphragmMaterial::Concrete, thickness_m: 0.001, span_m: 0.0, depth_m: 0.0, opening_width_m: 2.4, shear_strength_mpa: 1.0, chord_area_m2: 1.0e-6, chord_strength_mpa: 435.0, gamma_d: 0.0 }];
    document.accelerograms = (0..3_i64)
        .map(|seed| model::En1998Accelerogram { id: format!("rec-{seed}"), building_id: building.id.clone(), label: format!("synthetic {seed}"), samples: (0..200_i64).map(|k| ((((k * (37 + 2 * seed) + 11 * (seed + 1)) % 101 - 50) as f64) / 25.0).to_string()).collect::<Vec<_>>().join(" "), dt_s: 0.02, scale_factor: 1.0 })
        .collect();
    document
}

#[test]
fn diaphragm_force_flow_is_a_plate_with_inertia_wall_and_chord_arrows_bound_to_its_checks() {
    let document = horizontal_document();
    let (scene, report) = scene_of(&document);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    let plate = scene.entities.iter().find(|e| e.entity.id == "dia-1").expect("diaphragm plate");
    assert_eq!(plate.entity.kind, EntityKind::Plate);
    assert_eq!(plate.layer, "diaphragms");
    assert!(plate.utilisation.as_ref().is_some_and(|u| u.status == scene::CheckStatus::Fail), "the thin diaphragm fails its shear check");
    assert!(plate.handles.iter().any(|h| h == "dia-1:t"));
    let arrows: Vec<&SceneEntity> = scene.entities.iter().filter(|e| e.entity.id.starts_with("dia-1:") && e.layer == "diaphragmFlow").collect();
    let count = |tag: &str| arrows.iter().filter(|e| e.entity.id.contains(tag)).count();
    let systems = document.buildings[0].systems.len();
    assert_eq!((count(":inertia-"), count(":edge0-"), count(":edge1-"), count(":chord-t-"), count(":chord-c-")), (6 * systems, 5 * systems, 5 * systems, systems, systems));
    let shear = report.checks.iter().find(|c| c.id.ends_with(".diaphragm.dia-1.shear") && c.id.contains(&document.buildings[0].systems[0].id)).expect("shear check");
    let edge: f64 = arrows.iter().filter(|e| e.entity.id.contains(&format!(":{}:edge0-", document.buildings[0].systems[0].id))).filter_map(|e| match &e.geometry { Geometry::Load(LoadGlyph::Point { magnitude, .. }) => Some(magnitude.value), _ => None }).sum();
    near(edge, shear.computed.value, "the wall arrows add up to V_Ed");
    assert!(scene.annotations.iter().any(|a| a.id() == "dia-1:failure"));
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    near(all.iter().find(|h| h.id == "dia-1:t").expect("thickness handle").writes_at(0.0)[0].value, 0.001, "thickness at zero");
    let dependencies = entity_dependencies(&document, "dia-1");
    assert_eq!(dependencies, vec![document.buildings[0].id.clone()]);
    assert_eq!(entity_kind_of(&document, "dia-1").map(|k| k.id), Some("diaphragm"));
    let ds = diagrams(&document, &report, None);
    let forces = ds.iter().find(|d| d.id == "diaphragm:dia-1").expect("diaphragm diagram");
    assert_eq!(forces.validate(), Ok(()));
    assert!(forces.table_twin.rows() > 0);
}

#[test]
fn base_shear_blocks_grow_from_the_roof_down_along_the_scrubbable_track() {
    let document = En1998Snapshot::compliant_de_multipart();
    let (scene, report) = scene_of(&document);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    let building = &document.buildings[0];
    let blocks: Vec<&SceneEntity> = building.storeys.iter().filter_map(|s| scene.entities.iter().find(|e| e.entity.id == format!("{}:{}:shear-block", building.id, s.id))).collect();
    assert_eq!(blocks.len(), building.storeys.len());
    let value = |e: &SceneEntity, field: &str| match &e.appearance.channel {
        ColourChannel::Fields { fields, .. } => fields.iter().find(|f| f.id == field).and_then(|f| match &f.samples { FieldSamples::Vertex { values } => values.first().copied(), _ => None }).expect("field"),
        _ => panic!("fields"),
    };
    near(value(blocks[0], "distribution-8"), 1.0, "the lowest block carries the whole base shear");
    assert!(blocks.windows(2).all(|w| value(w[0], "distribution-8") >= value(w[1], "distribution-8")));
    assert!(blocks.iter().all(|b| value(b, "distribution-0") == 0.0 || blocks.len() == 1), "nothing is shown at the start but the roof");
    let sum: f64 = blocks.iter().map(|b| value(b, "storey-force")).sum();
    near(sum, 1.0, "storey force shares add up to one");
    let track = scene.tracks.iter().find(|t| t.id == "base-shear").expect("base shear track");
    assert_eq!(track.kind, TrackKind::Time);
    assert_eq!(track.keyframes.len(), 9);
    let diagram = diagrams(&document, &report, None).into_iter().find(|d| d.id == format!("base-shear:{}", building.id)).expect("base shear diagram");
    assert_eq!(diagram.validate(), Ok(()));
    assert_eq!(diagram.table_twin.rows(), building.storeys.len());
}

#[test]
fn spectrum_surface_follows_the_elastic_spectrum_over_period_and_damping_and_marks_t1() {
    let document = En1998Snapshot::compliant_de_multipart();
    let (scene, _) = scene_of(&document);
    let building = &document.buildings[0];
    let surface = scene.entities.iter().find(|e| e.entity.id == format!("{}:spectrum-surface", building.id)).expect("surface");
    let Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }) = &surface.geometry else { panic!("grid") };
    assert_eq!((grid.rows, grid.columns), (9, 33));
    let site = site(&document);
    let a_g = site.gamma_i * site.a_gr;
    let origin = grid.points[0].clone();
    for (row, xi) in [(0, 2.0), (2, 5.0), (8, 30.0)] {
        for column in [0_usize, 4, 12, 24, 32] {
            let t = 4.0 * column as f64 / 32.0;
            let expected = schema::horizontal::spectrum_surface::ordinate(a_g, site.s, site.tb, site.tc, site.td, xi, t);
            near(grid.points[row * 33 + column].z - origin.z, expected, "surface height is the ordinate");
        }
    }
    assert!(scene.entities.iter().any(|e| e.entity.id == format!("{}:spectrum-surface:t1", building.id)));
    assert!(scene.legends.iter().any(|l| l.id == "spectrum-ordinate"));
}

#[test]
fn overturning_base_has_width_handles_a_moment_glyph_and_the_governing_check() {
    let document = En1998Snapshot::compliant_de_multipart();
    let (scene, report) = scene_of(&document);
    let building = &document.buildings[0];
    let base = scene.entities.iter().find(|e| e.entity.id == format!("{}:base", building.id)).expect("base plate");
    assert_eq!(base.layer, "overturning");
    assert!(base.utilisation.as_ref().is_some_and(|u| u.check_id.ends_with(".overturning")));
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    for (tag, field, value) in [("bx", "overturningBaseXM", building.plan_width_m), ("by", "overturningBaseYM", building.plan_length_m)] {
        let handle = all.iter().find(|h| h.id == format!("{}:base:{tag}", building.id)).expect("width handle");
        assert!(handle.writes_at(0.0)[0].path.ends_with(field));
        near(handle.writes_at(0.0)[0].value, value, "the unset base width is the plan width");
    }
    let moment = scene.entities.iter().find(|e| e.entity.id == format!("{}:base:moment", building.id)).expect("moment glyph");
    let Geometry::Load(LoadGlyph::Moment { magnitude, .. }) = &moment.geometry else { panic!("moment") };
    let governing = report.checks.iter().filter(|c| c.id.ends_with(".overturning")).max_by(|a, b| a.utilization.total_cmp(&b.utilization)).expect("check");
    near(magnitude.value, governing.computed.value, "the glyph shows the check's M_ov");
}

#[test]
fn accelerogram_traces_are_surfaces_beside_their_building_and_the_set_diagram_has_a_table_twin() {
    let document = horizontal_document();
    let (scene, report) = scene_of(&document);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    for id in ["rec-0", "rec-1", "rec-2"] {
        let trace = scene.entities.iter().find(|e| e.entity.id == id).expect("trace");
        assert_eq!((trace.entity.kind, trace.layer.as_str()), (EntityKind::Surface, "accelerograms"));
        assert_eq!(entity_dependencies(&document, id), vec![document.buildings[0].id.clone()]);
    }
    let ds = diagrams(&document, &report, None);
    let set = ds.iter().find(|d| d.id == format!("time-history:{}", document.buildings[0].id)).expect("set diagram");
    assert_eq!(set.validate(), Ok(()));
    assert_eq!(set.table_twin.rows(), 19);
    let mut broken = document.clone();
    broken.accelerograms[0].samples = "x".into();
    let (scene, _) = scene_of(&broken);
    let marker = scene.entities.iter().find(|e| e.entity.id == "rec-0").expect("marker for the unusable record");
    assert_eq!(marker.entity.kind, EntityKind::Solid);
}

#[test]
fn every_example_and_showcase_projection_obeys_the_scene_diagram_and_handle_laws() {
    for (_, document) in fixtures() {
        let (scene, report) = scene_of(&document);
        let selection: Vec<scene::EntityRef> = scene.entities.iter().map(|e| e.entity.clone()).collect();
        scene::projection_kit::test_support::assert_projection_laws(&scene, &diagrams(&document, &report, None), &handles(&document, &[], &selection));
    }
}

fn structures_document() -> En1998Snapshot {
    En1998Snapshot::compliant_de_structures()
}

#[test]
fn bridge_rows_are_piers_bearings_a_deck_and_transverse_mode_ghosts_bound_to_their_checks_with_handles_and_diagrams() {
    let document = structures_document();
    let (scene, report) = scene_of(&document);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    let ids: BTreeSet<&str> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    for id in ["br-1", "br-1:force", "br-1:mode-y1", "pier-1", "pier-2", "pier-1:mrd", "pier-1:vcd", "pier-1:vrd", "bear-a", "bear-1", "bear-2", "bear-b"] {
        assert!(ids.contains(id), "{id}");
    }
    assert!(!ids.contains("br-1:pier-a"), "the single-pier drawing gives way to the rows");
    let pier = scene.entities.iter().find(|e| e.entity.id == "pier-1").expect("pier");
    assert!(pier.utilisation.as_ref().is_some_and(|b| b.check_id.starts_with("en1998.2.br-1.pier.pier-1.")));
    let shear = scene.entities.iter().find(|e| e.entity.id == "pier-1:vcd").expect("V_CD");
    let Geometry::Load(LoadGlyph::Point { magnitude, .. }) = &shear.geometry else { panic!("point load") };
    near(magnitude.value, report.checks.iter().find(|c| c.id == "en1998.2.br-1.pier.pier-1.shear").expect("pier check").computed.value, "V_CD equals the check");
    let ghost = scene.entities.iter().find(|e| e.entity.id == "br-1:mode-y1").expect("ghost");
    let overlay = ghost.appearance.deformation.as_ref().expect("overlay");
    assert!(overlay.participation > 0.9 && overlay.samples.len() > 4, "the first transverse mode carries the deck mass");
    assert_eq!(ghost.layer, "modeY1");
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    let found: BTreeSet<&str> = all.iter().map(|h| h.id.as_str()).collect();
    for id in ["pier-1:h", "pier-1:x", "pier-1:mrd:g", "pier-1:vrd:g", "bear-1:d"] {
        assert!(found.contains(id), "{id}");
    }
    for e in &scene.entities {
        for h in &e.handles {
            assert!(found.contains(h.as_str()), "{h}");
        }
    }
    let writes = all.iter().find(|h| h.id == "pier-1:x").expect("position grip").writes_at(0.0);
    near(writes[0].value, 10.0, "position at zero");
    assert!(writes[0].path.ends_with("piers[id=pier-1].xM"));
    for id in ["deck-modes:br-1", "pier-moments:br-1"] {
        let diagram = diagrams(&document, &report, None).into_iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id}"));
        assert_eq!(diagram.validate(), Ok(()), "{id}");
        assert!(diagram.table_twin.rows() > 0, "{id}");
    }
    scene::projection_kit::test_support::assert_projection_laws(&scene, &diagrams(&document, &report, None), &all);
}

#[test]
fn a_tank_shows_the_wall_pressure_the_sloshing_surface_and_the_freeboard_as_a_draggable_filling() {
    let document = structures_document();
    let (scene, report) = scene_of(&document);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    let pressure = scene.entities.iter().find(|e| e.entity.id == "tk-1:pressure").expect("pressure");
    let Geometry::Load(LoadGlyph::Line { profile: LoadProfile::Sampled { samples }, .. }) = &pressure.geometry else { panic!("sampled line load") };
    assert_eq!(samples.len(), 21);
    let base = samples[0].value.value;
    assert!(base > 5.0e4 && samples.windows(2).all(|w| w[1].value.value <= w[0].value.value + 1e-6), "the total pressure falls from the base");
    assert!(pressure.utilisation.as_ref().is_some_and(|b| b.check_id == "en1998.4.tk-1.pressure"));
    let slosh = scene.entities.iter().find(|e| e.entity.id == "tk-1:slosh").expect("slosh surface");
    let Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }) = &slosh.geometry else { panic!("grid") };
    assert_eq!(grid.points.len(), (grid.rows * grid.columns) as usize);
    let wave = report.checks.iter().find(|c| c.id == "en1998.4.tk-1.slosh").expect("slosh check").computed.value;
    let level = grid.points[0].z;
    let crest = grid.points.iter().map(|p| p.z - level).fold(f64::MIN, f64::max);
    near(crest, wave, "the crest of the surface is the wave height of the check");
    let freeboard = scene.annotations.iter().find(|a| a.id() == "tk-1:freeboard").expect("freeboard dimension");
    let Annotation::Dimension { binding: Some(binding), value, .. } = freeboard else { panic!("dimension") };
    assert!(binding.path.ends_with("tanks[id=tk-1].fillingRatio"));
    near(value.value, document.tanks[0].height_m * (1.0 - document.tanks[0].filling_ratio), "freeboard");
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert!(all.iter().any(|h| h.id == "tk-1:t" && h.writes_at(0.0)[0].path.ends_with("tanks[id=tk-1].shellThicknessM")));
    let diagram = diagrams(&document, &report, None).into_iter().find(|d| d.id == "wall-pressure:tk-1").expect("pressure diagram");
    assert_eq!(diagram.validate(), Ok(()));
    assert_eq!(diagram.table_twin.columns.len(), 5);
}

#[test]
fn a_tower_on_a_foundation_spring_shows_the_kernel_modes_the_spring_support_and_the_modal_diagrams() {
    let document = structures_document();
    let (scene, report) = scene_of(&document);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    let t1 = report.checks.iter().find(|c| c.id == "en1998.6.tw-1.period").expect("period").computed.value;
    let ghost = scene.entities.iter().find(|e| e.entity.id == "tw-1:mode-x1").expect("first mode");
    let overlay = ghost.appearance.deformation.as_ref().expect("overlay");
    near(overlay.frequency.as_ref().expect("frequency").value, 1.0 / t1, "the ghost oscillates at the period of the check");
    assert!(overlay.samples.len() > 20 && overlay.samples[0].delta.x == 0.0);
    let base = scene.entities.iter().find(|e| e.entity.id == "tw-1:base").expect("base");
    let Geometry::Support(SupportGeometry { kind: SupportKind::Spring { .. }, .. }) = &base.geometry else { panic!("the foundation is a spring") };
    assert!(base.utilisation.as_ref().is_some_and(|b| b.check_id == "en1998.6.tw-1.interaction"));
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    let grip = all.iter().find(|h| h.id == "tw-1:base:k").expect("stiffness grip");
    near(grip.writes_at(0.0)[0].value, 1.0e10, "stiffness at zero");
    assert!(grip.writes_at(0.0)[0].path.ends_with("towers[id=tw-1].baseRotationalStiffnessNm"));
    let diagrams = diagrams(&document, &report, None);
    for id in ["tower-modes:tw-1", "tower-forces:tw-1"] {
        let diagram = diagrams.iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id}"));
        assert_eq!(diagram.validate(), Ok(()), "{id}");
        assert!(diagram.table_twin.rows() > 20, "{id}");
    }
    let mut fixed = document.clone();
    fixed.towers[0].base_rotational_stiffness_nm = 0.0;
    let (fixed_scene, _) = scene_of(&fixed);
    let Geometry::Support(SupportGeometry { kind, .. }) = &fixed_scene.entities.iter().find(|e| e.entity.id == "tw-1:base").expect("base").geometry else { panic!("support") };
    assert!(matches!(kind, SupportKind::Fixed));
}

#[test]
fn masonry_wall_rows_become_walls_with_a_density_label_per_storey_and_direction() {
    let mut document = rich_document();
    let building = &mut document.buildings[0];
    let template = building.members[0].clone();
    let first = building.storeys[0].id.clone();
    let wall = |id: &str, length: f64, direction: f64, storey: &str| model::En1998Member { id: id.into(), material: semio_s_artifact_norm_en1998_model::StructuralMaterial::Masonry, role: model::MemberKind::Wall, wall_length_m: length, wall_thickness_m: 0.3, wall_direction_rad: direction, wall_storey_id: storey.into(), ..template.clone() };
    building.members.push(wall("w-x1", 6.0, 0.0, &first));
    building.members.push(wall("w-y1", 5.0, std::f64::consts::FRAC_PI_2, ""));
    let mut column = wall("w-column", 9.0, 0.0, "");
    column.role = model::MemberKind::Column;
    building.members.push(column);
    let id = building.id.clone();
    let (scene, _) = scene_of(&document);
    assert!(!scene.entities.iter().any(|e| e.entity.id.contains(":wall:w-column:")), "only the wall kind makes wall bodies");
    for entity in [format!("{id}:wall:w-x1:{first}"), format!("{id}:wall:w-y1:{first}")] {
        assert!(scene.entities.iter().any(|e| e.entity.id == entity), "{entity}");
    }
    let label = scene.annotations.iter().find(|a| a.id() == format!("{id}:wall-density:x:{first}")).expect("density label");
    let Annotation::Label { text, .. } = label else { panic!("label") };
    assert!(text.en.contains("direction x") && text.de.contains("Richtung x"), "{}", text.en);
    document.buildings[0].members.retain(|m| !m.material == semio_s_artifact_norm_en1998_model::StructuralMaterial::Masonry);
    let (plain, _) = scene_of(&document);
    assert!(!plain.annotations.iter().any(|a| a.id().contains(":wall-density:")));
}

#[test]
fn wall_rows_stand_at_their_plan_positions_with_grips_and_drive_the_stiffness_centre() {
    let mut document = rich_document();
    let building = &mut document.buildings[0];
    let template = building.members[0].clone();
    let first = building.storeys[0].id.clone();
    let wall = |id: &str, x: f64, y: f64, length: f64, direction: f64| model::En1998Member { id: id.into(), material: semio_s_artifact_norm_en1998_model::StructuralMaterial::Rc, role: model::MemberKind::Wall, wall_length_m: length, wall_thickness_m: 0.3, wall_direction_rad: direction, wall_start_x_m: x, wall_start_y_m: y, wall_storey_id: String::new(), ..template.clone() };
    building.members.push(wall("w-a", 1.0, 2.0, 6.0, 0.0));
    building.members.push(wall("w-b", 9.0, 1.0, 5.0, std::f64::consts::FRAC_PI_2));
    let id = building.id.clone();
    let (scene, _) = scene_of(&document);
    let Geometry::Plate(PlateGeometry { frame: plan, .. }) = &scene.entities.iter().find(|e| e.entity.id == id).expect("plan").geometry else { panic!("plate") };
    let body = scene.entities.iter().find(|e| e.entity.id == format!("{id}:wall:w-a:{first}")).expect("wall body");
    let Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame, size } }) = &body.geometry else { panic!("box") };
    near(frame.origin.x - plan.origin.x, 4.0, "wall a centre x");
    near(frame.origin.y - plan.origin.y, 2.0, "wall a centre y");
    near(size.x, 6.0, "wall a length");
    near(size.y, 0.3, "wall a thickness");
    let across = scene.entities.iter().find(|e| e.entity.id == format!("{id}:wall:w-b:{first}")).expect("wall b body");
    let Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame, .. } }) = &across.geometry else { panic!("box") };
    near(frame.x.y, 1.0, "wall b runs along plan y");
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    for tag in ["start-x", "start-y", "length", "direction"] {
        let grip = all.iter().find(|h| h.id == format!("{id}:wall:w-a:{first}:{tag}")).unwrap_or_else(|| panic!("{tag}"));
        assert!(body.handles.iter().any(|h| *h == grip.id), "{tag}");
        assert!(grip.writes_at(0.0)[0].path.contains("members[id=w-a].wall"), "{tag}");
    }
    near(all.iter().find(|h| h.id == format!("{id}:wall:w-a:{first}:start-x")).expect("start-x").writes_at(0.0)[0].value, 1.0, "start x");
    assert!(!all.iter().any(|h| h.id == format!("{id}:{first}:cos:x")), "the walls drive the centre of stiffness, no grip writes the entered value");
    assert!(scene.annotations.iter().any(|a| a.id() == format!("{id}:{first}:walls-cos")), "centre of stiffness label from the walls");
}

#[test]
fn joint_rows_stand_at_their_plan_positions_coloured_by_their_capacity_design_check_with_plan_grips() {
    let mut document = rich_document();
    let (building, storey) = (document.buildings[0].id.clone(), document.buildings[0].storeys[1].id.clone());
    let row = |id: &str, x: f64, column: f64| model::En1998Joint { id: id.into(), building_id: building.clone(), storey_id: storey.clone(), column_id: "col-c1".into(), beam_id: "beam-b1".into(), x_m: x, y_m: 4.0, column_moment_rd_nm: column, beam_moment_rd_nm: 4.0e6 };
    document.joints = vec![row("jt-ok", 6.0, 9.0e6), row("jt-weak", 12.0, 5.0e6)];
    let (scene, report) = scene_of(&document);
    assert!(!scene.entities.iter().any(|e| e.entity.id.contains(":joint-")), "joint rows replace the synthesised grid joints");
    for (id, status_fail) in [("jt-ok", false), ("jt-weak", true)] {
        let marker = scene.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("{id}"));
        let check = format!("en1998.1.{building}.capacityDesign.joint.{id}");
        assert!(marker.utilisation.as_ref().is_some_and(|u| u.check_id == check), "{id}");
        assert_eq!(report.checks.iter().find(|c| c.id == check).expect("check").utilization > 1.0, status_fail, "{id}");
        assert!(scene.annotations.iter().any(|a| a.id() == format!("{id}:ratio")), "{id}");
        assert_eq!(entity_kind_of(&document, id).map(|k| k.id), Some("joint"));
    }
    assert!(entity_dependencies(&document, "jt-ok").contains(&building) && entity_dependencies(&document, &building).contains(&"jt-ok".to_owned()));
    let Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame, .. } }) = &scene.entities.iter().find(|e| e.entity.id == "jt-ok").expect("marker").geometry else { panic!("box") };
    let Geometry::Plate(PlateGeometry { frame: plan, .. }) = &scene.entities.iter().find(|e| e.entity.id == building).expect("plan").geometry else { panic!("plate") };
    near(frame.origin.x - plan.origin.x, 6.0, "joint x");
    near(frame.origin.y - plan.origin.y, 4.0, "joint y");
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    for tag in ["x", "y"] {
        let grip = all.iter().find(|h| h.id == format!("jt-ok:{tag}")).unwrap_or_else(|| panic!("{tag}"));
        assert!(grip.writes_at(0.0)[0].path == format!("joints[id=jt-ok].{tag}M"), "{tag}");
    }
    near(all.iter().find(|h| h.id == "jt-weak:x").expect("x").writes_at(0.0)[0].value, 12.0, "joint x grip");
}

#[test]
fn soil_column_follows_the_site_ground_inputs_live_and_the_spectrum_letter_follows_the_derived_type() {
    let mut document = rich_document();
    let foundation = document.foundations[0].id.clone();
    let row = |id: &str, thickness_m: f64, v_s: f64| model::En1998SoilLayer { id: id.into(), foundation_id: foundation.clone(), thickness_m, unit_weight_n_m3: 18000.0, shear_wave_velocity_m_s: v_s, n1_60: 0.0, fines_percent: 0.0 };
    document.soil_layers = vec![row("sl-1", 30.0, 250.0)];
    document.annex = semio_s_plugin_norm_compliance::AnnexChoice::En;
    document.site.en_ground_type = semio_s_artifact_norm_en1998_model::EnGroundType::B;
    document.site.v_s30_m_s = 0.0;
    let label_of = |document: &model::En1998Snapshot| {
        let (scene, _) = scene_of(document);
        let Annotation::Label { text, tone, .. } = scene.annotations.iter().find(|a| a.id() == format!("{foundation}:ground-type")).expect("ground type label").clone() else { panic!("label") };
        (text, tone)
    };
    let (from_layers, tone) = label_of(&document);
    assert!(from_layers.en.contains("ground type C") && from_layers.en.contains("spectra follow these layers") && !from_layers.en.contains("spectra use"), "{}", from_layers.en);
    assert!(from_layers.de.contains("Spektren folgen diesen Schichten"), "{}", from_layers.de);
    assert!(matches!(tone, Tone::Info));
    document.site.v_s30_m_s = 150.0;
    let (layers_win, tone) = label_of(&document);
    assert!(layers_win.en.contains("ground type C") && layers_win.en.contains("spectra follow these layers"), "{}", layers_win.en);
    assert!(matches!(tone, Tone::Info));
    assert!(matches!(super::response::annex_params(&document), super::schema::AnnexParams::En { ground: 'C', .. }), "the foundation layers take precedence over the site v_s,30");
    document.soil_layers.clear();
    assert!(matches!(super::response::annex_params(&document), super::schema::AnnexParams::En { ground: 'D', .. }), "without layers the site v_s,30 applies");
    document.soil_layers = vec![row("sl-1", 30.0, 250.0)];
    document.site.liquefiable_layer = true;
    let (special, tone) = label_of(&document);
    assert!(special.en.contains("ground type S2") && special.en.contains("special study") && special.de.contains("Sonderuntersuchung"), "{}", special.en);
    assert!(matches!(tone, Tone::Warning));
}

#[test]
fn soil_column_names_the_ground_type_of_table_3_1_and_the_modal_participation_diagram_reaches_ninety_percent() {
    let mut document = rich_document();
    let foundation = document.foundations[0].id.clone();
    let row = |id: &str, thickness_m: f64, v_s: f64| model::En1998SoilLayer { id: id.into(), foundation_id: foundation.clone(), thickness_m, unit_weight_n_m3: 18000.0, shear_wave_velocity_m_s: v_s, n1_60: 0.0, fines_percent: 0.0 };
    document.soil_layers = vec![row("sl-1", 3.0, 250.0), row("sl-2", 4.0, 150.0), row("sl-3", 6.0, 220.0)];
    let (scene, report) = scene_of(&document);
    let label = scene.annotations.iter().find(|a| a.id() == format!("{foundation}:ground-type")).expect("ground type label");
    let Annotation::Label { text, .. } = label else { panic!("label") };
    assert!(text.en.contains("ground type C"), "{}", text.en);
    assert!(text.de.contains("Baugrundklasse C"), "{}", text.de);
    let b = &document.buildings[0];
    let diagram = diagrams(&document, &report, Some(&b.id)).into_iter().find(|d| d.id == format!("modal-participation:{}", b.id)).expect("modal participation");
    assert_eq!(diagram.validate(), Ok(()));
    assert!(diagram.table_twin.rows() > 0);
    let Some(Series::Polyline { points, .. }) = diagram.series.iter().find(|s| matches!(s, Series::Polyline { id, .. } if id == "cumulative-x")) else { panic!("x series") };
    assert!(points.windows(2).all(|w| w[1].y >= w[0].y - 1e-9) && points.last().is_some_and(|p| p.y <= 100.0 + 1e-6));
}

#[test]
fn every_placed_entity_has_move_grips_and_every_silo_a_demand_diagram() {
    let document = En1998Snapshot::full_showcase();
    let (scene, report) = scene_of(&document);
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    for id in [&document.buildings[0].id, &document.silos[0].id, &document.tanks[0].id, &document.towers[0].id, &document.retaining_walls[0].id, &document.bridges[0].id] {
        for tag in ["move-x", "move-y"] {
            let grip = all.iter().find(|h| h.id == format!("{id}:{tag}")).unwrap_or_else(|| panic!("{id}:{tag}"));
            assert_eq!(grip.writes[0].path, format!("placements[id={id}].{}", &tag[5..]));
        }
    }
    let silo = &document.silos[0].id;
    let diagram = diagrams(&document, &report, None).into_iter().find(|d| d.id == format!("silo-demand:{silo}")).expect("silo diagram");
    assert_eq!(diagram.validate(), Ok(()));
    assert!(diagram.axes.iter().all(|a| !a.unit.is_empty()) && diagram.axes.len() == 2);
    assert_eq!(diagram.table_twin.columns.len(), 3);
}

fn rayleigh_document() -> En1998Snapshot {
    let mut document = En1998Snapshot::compliant_de_office();
    document.buildings[0].t1_method = model::T1Method::Rayleigh;
    document
}

#[test]
fn the_spectrum_diagram_marks_gross_and_cracked_t1_with_their_elastic_and_design_points() {
    let document = rayleigh_document();
    let (_, report) = scene_of(&document);
    let building = &document.buildings[0];
    let response = respond(&site(&document), building);
    let factor = schema::cracked_stiffness::building_factor(building);
    near(response.t1 / response.t1_gross, (1.0 / factor).sqrt(), "Rayleigh period scales with the inverse root of the stiffness factor");
    let spectra = diagrams(&document, &report, None).into_iter().find(|d| d.id == "spectra:document").expect("spectra");
    assert_eq!(spectra.validate(), Ok(()));
    let marker = |id: &str| spectra.markers.iter().find(|m| m.id == id).unwrap_or_else(|| panic!("{id}"));
    near(marker("t1").at.x, response.t1.min(4.0), "cracked T1");
    near(marker("t1-se").at.x, response.t1.min(4.0), "elastic point at cracked T1");
    near(marker("t1-gross").at.x, response.t1_gross.min(4.0), "gross T1");
    near(marker("t1-gross-se").at.x, response.t1_gross.min(4.0), "elastic point at gross T1");
    assert!(marker("t1-se").at.y >= marker("t1").at.y - 1e-9 && marker("t1-gross-se").at.y >= marker("t1-gross").at.y - 1e-9, "Se is never below Sd");
    assert!(spectra.series.iter().any(|s| matches!(s, Series::Limit { id, .. } if id == "t1-gross-line")));
}

#[test]
fn a_dragged_storey_stiffness_moves_the_cracked_t1_marker_while_the_gross_marker_follows_the_stiffness_too() {
    let mut document = rayleigh_document();
    let t1_of = |document: &En1998Snapshot| {
        let (_, report) = scene_of(document);
        let spectra = diagrams(document, &report, None).into_iter().find(|d| d.id == "spectra:document").expect("spectra");
        let x = |id: &str| spectra.markers.iter().find(|m| m.id == id).map(|m| m.at.x).unwrap_or_else(|| panic!("{id}"));
        (x("t1"), x("t1-gross"))
    };
    let before = t1_of(&document);
    document.buildings[0].storeys.iter_mut().for_each(|s| (s.stiffness_x, s.stiffness_y) = (s.stiffness_x * 4.0, s.stiffness_y * 4.0));
    let after = t1_of(&document);
    near(after.0, before.0 / 2.0, "four times the stiffness halves the period");
    near(after.1, before.1 / 2.0, "gross period halves too");
}

#[test]
fn the_storey_shear_and_drift_diagrams_match_the_forces_and_the_damage_limitation_limits() {
    for (name, document) in fixtures() {
        let (_, report) = scene_of(&document);
        for building in &document.buildings {
            let set = diagrams(&document, &report, Some(&building.id));
            let response = respond(&site(&document), building);
            if let Some(shear) = set.iter().find(|d| d.id == format!("storey-shear:{}", building.id)) {
                assert_eq!(shear.validate(), Ok(()), "{name}");
                let system = &response.systems[0];
                let Series::Polyline { points, .. } = &shear.series[0] else { panic!("shear series") };
                near(points[0].x, system.forces.iter().sum::<f64>() / 1000.0, "the base storey shear is the sum of the storey forces");
                assert!(points.windows(2).all(|w| w[1].x <= w[0].x + 1e-9), "{name}: the storey shear never grows upwards");
            }
            if let Some(drift) = set.iter().find(|d| d.id == format!("storey-drift:{}", building.id)) {
                assert_eq!(drift.validate(), Ok(()), "{name}");
                let limits = storey_response::drift_limits_m(building);
                let Some(Series::Polyline { points, .. }) = drift.series.iter().find(|s| matches!(s, Series::Polyline { id, .. } if id == "limit")) else { panic!("limit series") };
                for (point, limit) in points.iter().zip(&limits) {
                    near(point.x, limit * 1000.0, "limit in mm");
                }
                for m in drift.markers.iter().filter(|m| m.binding.is_some()) {
                    let binding = m.binding.as_ref().expect("binding");
                    assert!(binding.path.contains(&format!("buildings[id={}].storeys[id=", building.id)) && (binding.path.ends_with(".driftXM") || binding.path.ends_with(".driftYM")), "{}", binding.path);
                    let limit = building.storeys.iter().zip(&limits).find(|(s, _)| m.id.ends_with(&format!("-{}", s.id))).map_or(0.0, |(_, l)| *l);
                    near(m.at.x, m.utilisation.expect("utilisation") * limit * 1000.0, "drift is the utilisation times the limit");
                }
            }
        }
    }
}

#[test]
fn storey_shears_accumulate_from_the_top_down() {
    let shears = storey_response::storey_shears(&[1.0, 2.0, 3.0]);
    assert_eq!(shears, vec![6.0, 5.0, 3.0]);
}

#[test]
fn bridge_piers_carry_the_capacity_design_ratio_bearings_the_demand_arrow_and_the_deck_a_longitudinal_ghost() {
    let document = structures_document();
    let (scene, report) = scene_of(&document);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    for pier in ["pier-1", "pier-2"] {
        let entity = scene.entities.iter().find(|e| e.entity.id == pier).expect(pier);
        let check = report.checks.iter().find(|c| c.id == format!("en1998.2.br-1.pier.{pier}.shear")).expect("shear check");
        let binding = entity.utilisation.as_ref().expect("binding");
        assert_eq!(binding.check_id, check.id, "the pier is coloured by V_CD / V_Rd");
        near(binding.ratio, check.utilization, "ratio equals the check utilisation");
        assert!(scene.annotations.iter().any(|a| a.id() == format!("{pier}:capacity-ratio")), "{pier}");
    }
    let mut demands = 0;
    for bearing in &document.bearings {
        let demand = report.checks.iter().find(|c| c.id == format!("en1998.2.br-1.bearing.{}.displacement", bearing.id)).map(|c| c.computed.value.abs()).unwrap_or(0.0);
        let arrow = scene.entities.iter().find(|e| e.entity.id == format!("{}:demand", bearing.id));
        assert_eq!(arrow.is_some(), demand > 0.0, "{}", bearing.id);
        if let Some(arrow) = arrow {
            demands += 1;
            assert!(arrow.utilisation.as_ref().is_some_and(|b| b.check_id.ends_with(&format!("bearing.{}.displacement", bearing.id))));
            let Some(Annotation::Dimension { value, .. }) = scene.annotations.iter().find(|a| a.id() == format!("{}:demand-dim", bearing.id)) else { panic!("demand dimension") };
            near(value.value, demand, "demand arrow length");
        }
        let required = report.checks.iter().find(|c| c.id == format!("en1998.2.br-1.bearing.{}.seat", bearing.id)).map(|c| c.computed.value).unwrap_or(0.0);
        assert_eq!(scene.annotations.iter().any(|a| a.id() == format!("{}:seat-required", bearing.id)), required > 0.0, "{}", bearing.id);
    }
    assert!(demands > 0, "at least one bearing takes a displacement");
    let view = structure_views::bridge_view(&document, &document.bridges[0], &site(&document)).expect("view");
    let ghost = scene.entities.iter().find(|e| e.entity.id == "br-1:mode-x1");
    assert_eq!(ghost.is_some(), view.longitudinal.is_some());
    if let Some(ghost) = ghost {
        assert_eq!(ghost.layer, kit::LAYER_MODE_X1);
        let overlay = ghost.appearance.deformation.as_ref().expect("overlay");
        assert!(overlay.samples.iter().all(|s| s.delta.y == 0.0), "the longitudinal mode moves along the deck axis");
        near(overlay.participation, view.longitudinal.as_ref().expect("longitudinal").effective_mass_ratio[0], "participation");
    }
}

#[test]
fn a_tank_splits_its_wall_pressure_into_impulsive_and_convective_parts_and_marks_the_sloshing_height() {
    let document = structures_document();
    let (scene, report) = scene_of(&document);
    let total = scene.entities.iter().find(|e| e.entity.id == "tk-1:pressure").expect("total");
    let Geometry::Load(LoadGlyph::Line { profile: LoadProfile::Sampled { samples: total_samples }, .. }) = &total.geometry else { panic!("sampled") };
    for (tag, name) in [("impulsive", "impulsive"), ("convective", "convective")] {
        let part = scene.entities.iter().find(|e| e.entity.id == format!("tk-1:pressure-{tag}")).unwrap_or_else(|| panic!("{name}"));
        let Geometry::Load(LoadGlyph::Line { profile: LoadProfile::Sampled { samples }, .. }) = &part.geometry else { panic!("sampled") };
        assert_eq!(samples.len(), total_samples.len(), "{name}");
        assert!(samples.iter().zip(total_samples).all(|(p, t)| p.value.value >= 0.0 && p.value.value <= t.value.value + 1e-6), "{name} part never exceeds the total");
        assert!(part.utilisation.as_ref().is_some_and(|b| b.check_id == "en1998.4.tk-1.pressure"));
    }
    let wave = report.checks.iter().find(|c| c.id == "en1998.4.tk-1.slosh").expect("slosh check").computed.value;
    assert_eq!(scene.annotations.iter().any(|a| a.id() == "tk-1:slosh-height"), wave > 0.0);
    if wave > 0.0 {
        let Some(Annotation::Dimension { value, .. }) = scene.annotations.iter().find(|a| a.id() == "tk-1:slosh-dim") else { panic!("slosh dimension") };
        near(value.value, wave, "sloshing height equals the check");
    }
}

fn diagram_named(document: &En1998Snapshot, report: &CheckReport, id: &str) -> Diagram2d {
    let found = diagrams(document, report, None).into_iter().find(|d| d.id == id).unwrap_or_else(|| panic!("{id}"));
    assert_eq!(found.validate(), Ok(()), "{id}");
    assert!(found.table_twin.rows() > 0, "{id}");
    found
}

#[test]
fn the_assessed_section_has_width_depth_bar_and_steel_grips_and_a_draggable_demand_point() {
    let document = rich_document();
    let (scene, report) = scene_of(&document);
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    let a = &document.assessments[0];
    for (tag, field, value) in [("width", "sectionWidthM", a.section_width_m), ("effective-depth", "effectiveDepthM", a.effective_depth_m), ("bar-diameter", "barDiameterMm", a.bar_diameter_mm), ("tension-steel", "tensionSteelAreaM2", a.tension_steel_area_m2)] {
        let grip = all.iter().find(|h| h.id == format!("as-1:member:{tag}")).unwrap_or_else(|| panic!("{tag}"));
        assert_eq!(grip.writes[0].path, format!("assessments[id=as-1].{field}"));
        near(grip.writes_at(0.0)[0].value, value, tag);
    }
    let member = scene.entities.iter().find(|e| e.entity.id == "as-1:member").expect("member");
    assert!(["width", "effective-depth", "bar-diameter", "tension-steel"].iter().all(|t| member.handles.contains(&format!("as-1:member:{t}"))));
    let ladder = diagram_named(&document, &report, "chord-rotation:as-1");
    let demand = ladder.markers.iter().find(|m| m.id == "demand-as-1").expect("demand marker");
    near(demand.at.y, a.chord_rotation_demand_rad, "demand marker height");
    assert_eq!(demand.binding.as_ref().map(|b| b.path.as_str()), Some("assessments[id=as-1].chordRotationDemandRad"));
}

#[test]
fn the_assessed_section_shows_its_moment_curvature_and_is_coloured_by_the_chord_rotation_ratio() {
    let document = rich_document();
    let (scene, report) = scene_of(&document);
    let a = &document.assessments[0];
    let diagram = diagram_named(&document, &report, "moment-curvature:as-1");
    let marker = |id: &str| diagram.markers.iter().find(|m| m.id == id).unwrap_or_else(|| panic!("{id}"));
    let (yield_point, ultimate_point, demand) = (marker("reference:yield-as-1"), marker("reference:ultimate-as-1"), marker("demand-as-1"));
    let k = super::schema::part_3::AnnexA::from_registry(super::response::annex_params(&document).choice());
    let cf = super::schema::part_3::confidence_factor(a.knowledge_level);
    let member = super::schema::part_3::RcMember::of(a).expect("section").over_cf(cf);
    let response = member.section_response(&k).expect("response");
    near(yield_point.at.x, response.kappa_y, "yield curvature");
    near(yield_point.at.y, response.m_y / 1000.0, "yield moment");
    near(ultimate_point.at.x, response.kappa_u, "ultimate curvature");
    assert!(ultimate_point.at.x > yield_point.at.x && ultimate_point.at.y >= yield_point.at.y && yield_point.at.y > 0.0);
    let theta_y = super::schema::part_3::chord_rotation_yield_rad(&member, &k);
    near(demand.at.x, (response.kappa_y * a.chord_rotation_demand_rad / theta_y).min(response.kappa_u), "equivalent demand curvature");
    assert_eq!(demand.bound_check.as_deref(), Some("en1998.3.as-1.chordRotation"));
    let entity = scene.entities.iter().find(|e| e.entity.id == "as-1:member").expect("member");
    let ColourChannel::Fields { active, fields } = &entity.appearance.channel else { panic!("field channel") };
    assert_eq!(active, "assessment-dc");
    assert_eq!(fields[0].id, "chord-rotation-utilisation");
    let FieldSamples::Station { values } = &fields[0].samples else { panic!("station samples") };
    let ultimate = super::schema::part_3::design_capacities(a, &k, cf).ultimate_rad;
    assert!(values.iter().all(|v| (v.value - a.chord_rotation_demand_rad / ultimate).abs() < 1e-9));
    assert!(scene.legends.iter().any(|l| l.id == "chord-rotation-utilisation"));
    assert!(scene.legends.iter().any(|l| l.id == "assessment-dc"));
}

#[test]
fn hoops_are_placed_one_per_spacing_along_the_shear_span() {
    assert_eq!(buildings::hoop_positions(2.0, 0.5), vec![0.25, 0.75, 1.25, 1.75]);
    assert!(buildings::hoop_positions(2.0, 0.0).is_empty());
    assert!(buildings::hoop_positions(0.0, 0.1).is_empty());
    assert_eq!(buildings::hoop_positions(1000.0, 0.01).len(), buildings::HOOPS_MAX);
}

#[test]
fn the_assessed_member_carries_a_three_dimensional_hoop_layout_of_its_confined_core() {
    let document = rich_document();
    let a = &document.assessments[0];
    let (scene, _) = scene_of(&document);
    let hoops: Vec<&SceneEntity> = scene.entities.iter().filter(|e| e.entity.id.starts_with(&format!("{}:hoop:", a.id))).collect();
    assert_eq!(hoops.len(), buildings::hoop_positions(a.shear_span_m, a.stirrup_spacing_m).len());
    assert!(!hoops.is_empty());
    assert!(hoops.iter().all(|e| e.layer == kit::LAYER_ASSESSED && e.accessible_detail.en.contains("confinement effectiveness")));
    let ids: std::collections::BTreeSet<&str> = hoops.iter().map(|e| e.entity.id.as_str()).collect();
    assert_eq!(ids.len(), hoops.len());
}

#[test]
fn the_assessed_member_has_a_demand_grip_a_dc_diagram_a_dc_colour_and_a_knowledge_badge() {
    let mut document = rich_document();
    document.assessments[0].shear_force_demand_n = 2.0e5;
    let (scene, report) = scene_of(&document);
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    let grip = all.iter().find(|h| h.id == "as-1:member:shear-demand").expect("shear demand grip");
    assert_eq!(grip.writes[0].path, "assessments[id=as-1].shearForceDemandN");
    near(grip.writes_at(0.0)[0].value, 2.0e5, "shear demand grip");
    let a = &document.assessments[0];
    let ratios = super::schema::part_3_state::member_ratios(a, super::response::annex_params(&document).choice());
    let diagram = diagram_named(&document, &report, "assessment-dc:as-1");
    assert_eq!(diagram.validate(), Ok(()));
    near(diagram.markers.iter().find(|m| m.id == "shear-as-1").expect("shear marker").at.y, ratios.shear.expect("shear ratio"), "shear D/C");
    near(diagram.markers.iter().find(|m| m.id == "chord-as-1").expect("chord marker").at.y, ratios.chord.expect("chord ratio"), "chord D/C");
    let entity = scene.entities.iter().find(|e| e.entity.id == "as-1:member").expect("member");
    let ColourChannel::Fields { active, fields } = &entity.appearance.channel else { panic!("field channel") };
    assert_eq!(active, "assessment-dc");
    let FieldSamples::Station { values } = &fields.iter().find(|f| f.id == "assessment-dc").expect("D/C field").samples else { panic!("station samples") };
    near(values[0].value, ratios.governing().expect("governing"), "member colour");
    let Annotation::Label { text, .. } = scene.annotations.iter().find(|x| x.id() == "as-1:badge").expect("badge").clone() else { panic!("label") };
    let basis = super::schema::part_3_state::basis(a, super::response::annex_params(&document).choice());
    assert!(text.en.starts_with(&format!("KL2 \u{b7} CF {:.2}", basis.cf)) && text.en.contains("D/C") && text.en.contains("E/R"), "{}", text.en);
}

#[test]
fn an_accelerogram_has_a_time_history_with_a_peak_marker_and_a_scale_grip_that_agree() {
    let document = horizontal_document();
    let (scene, report) = scene_of(&document);
    let record = &document.accelerograms[0];
    let history = diagram_named(&document, &report, "accelerogram:rec-0");
    let raw = schema::horizontal::time_history::parse_samples(&record.samples).expect("samples");
    let raw_peak = raw.iter().fold(0.0_f64, |p, a| p.max(a.abs()));
    let peak = history.markers.iter().find(|m| m.id == "peak").expect("peak");
    near(peak.at.y.abs(), raw_peak * record.scale_factor, "peak equals the scaled record peak");
    let binding = peak.binding.as_ref().expect("binding");
    assert_eq!(binding.path, "accelerograms[id=rec-0].scaleFactor");
    near(binding.map.base + binding.map.k * peak.at.y, record.scale_factor, "dragging the peak where it is keeps the factor");
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    let grip = all.iter().find(|h| h.id == "rec-0:scale").expect("scale grip");
    assert_eq!(grip.writes[0].path, "accelerograms[id=rec-0].scaleFactor");
    near(grip.writes_at(0.0)[0].value, record.scale_factor, "scale at zero");
    assert!(scene.entities.iter().find(|e| e.entity.id == "rec-0").expect("trace").handles.contains(&"rec-0:scale".to_owned()));
}

#[test]
fn pier_capacity_curves_peak_at_the_hinge_force_and_the_tank_marks_its_fill_level() {
    let document = structures_document();
    let (_, report) = scene_of(&document);
    let capacity = diagram_named(&document, &report, "pier-capacity:br-1");
    for pier in document.piers.iter().filter(|p| p.bridge_id == "br-1" && p.height_m > 0.0 && p.moment_rd_nm > 0.0) {
        let Some(yield_marker) = capacity.markers.iter().find(|m| m.id == format!("reference:yield-{}", pier.id)) else { continue };
        near(yield_marker.at.y, pier.moment_rd_nm / pier.height_m, "hinge force is M_Rd over H");
        let binding = yield_marker.binding.as_ref().expect("binding");
        assert_eq!(binding.path, format!("piers[id={}].momentRdNm", pier.id));
        near(binding.map.k * yield_marker.at.y, pier.moment_rd_nm, "the marker height maps back to M_Rd");
        let curve = capacity.series.iter().find_map(|s| match s { Series::Polyline { id, points, .. } if *id == format!("capacity-{}", pier.id) => Some(points), _ => None }).expect("curve");
        assert!(curve.iter().all(|p| p.y <= yield_marker.at.y + 1e-6) && curve.windows(2).all(|w| w[1].x >= w[0].x));
        assert!(capacity.markers.iter().any(|m| m.id == format!("demand-{}", pier.id)));
    }
    let tank = &document.tanks[0];
    let pressure = diagram_named(&document, &report, &format!("wall-pressure:{}", tank.id));
    let fill = pressure.markers.iter().find(|m| m.id == "reference:fill-level").expect("fill level");
    near(fill.at.y, schema::tank::fluid_height_m(tank.height_m, tank.filling_ratio), "fill level equals the fluid height");
    let binding = fill.binding.as_ref().expect("binding");
    assert_eq!(binding.path, format!("tanks[id={}].fillingRatio", tank.id));
    near(binding.map.base + binding.map.k * fill.at.y, tank.filling_ratio.clamp(0.0, 1.0), "dragging the marker where it is keeps the ratio");
}

#[test]
fn silo_fill_and_tower_level_profiles_are_consistent_with_their_kernels() {
    let document = En1998Snapshot::full_showcase();
    let (_, report) = scene_of(&document);
    let silo = &document.silos[0];
    let fill = diagram_named(&document, &report, &format!("silo-fill:{}", silo.id));
    let demand = diagram_named(&document, &report, &format!("silo-demand:{}", silo.id));
    let at_design = |d: &Diagram2d, id: &str| d.markers.iter().find(|m| m.id == id).expect(id).at.y;
    if (silo.filling_ratio.clamp(0.0, 1.0) - silo.filling_ratio).abs() < 1e-12 {
        near(at_design(&fill, "reference:fill-n"), at_design(&demand, "reference:design-n"), "axial demand agrees between the acceleration and the filling diagram");
        near(at_design(&fill, "reference:fill-v"), at_design(&demand, "reference:design-v"), "base shear agrees between the acceleration and the filling diagram");
    }
    assert_eq!(fill.markers.iter().filter_map(|m| m.binding.as_ref()).filter(|b| b.path == format!("silos[id={}].fillingRatio", silo.id)).count(), 2);
    let rich = rich_document();
    let (_, rich_report) = scene_of(&rich);
    let tower = &rich.towers[0];
    let shear = diagram_named(&rich, &rich_report, &format!("tower-shear:{}", tower.id));
    let drift = diagram_named(&rich, &rich_report, &format!("tower-drift:{}", tower.id));
    let base = shear.markers.iter().find(|m| m.id == "reference:base-shear").expect("base shear");
    assert!(base.at.x > 0.0);
    let Series::Polyline { points, .. } = &shear.series[0] else { panic!("polyline") };
    assert!(points.len() > 2 && points.windows(2).all(|w| w[1].y > w[0].y) && points.last().expect("top").x <= points[0].x, "shear at the top never exceeds the base shear");
    let tip = drift.markers.iter().find(|m| m.id == "reference:tip").expect("tip");
    assert_eq!(tip.binding.as_ref().map(|b| b.path.clone()), Some(format!("towers[id={}].heightM", tower.id)));
    near(tip.at.y, tower.height_m, "tip elevation equals the height");
}

#[test]
fn a_computed_pier_is_coloured_by_ei_eff_over_ei_gross_and_shows_its_moment_curvature_and_a_reinforcement_grip() {
    let mut document = structures_document();
    let pier = &mut document.piers[0];
    (pier.stiffness_basis, pier.reinforcement_ratio, pier.cover_m, pier.concrete_fck_mpa, pier.steel_fyk_mpa, pier.axial_load_n) = (model::PierStiffnessBasis::Computed, 0.008, 0.08, 30.0, 500.0, 1.0e6);
    let id = document.piers[0].id.clone();
    let (scene, report) = scene_of(&document);
    scene.validate().unwrap_or_else(|v| panic!("{v:?}"));
    let k = site(&document).section;
    let ratio = super::schema::bridge::pier_ei_ratio(&document.piers[0], super::schema::bridge::Direction::Longitudinal, document.bridges[0].pier_cracked_inertia_ratio, &k);
    assert!(ratio > 0.0 && ratio < 1.0);
    let entity = scene.entities.iter().find(|e| e.entity.id == id).expect("pier");
    let ColourChannel::Fields { fields, .. } = &entity.appearance.channel else { panic!("field channel") };
    let FieldSamples::Station { values } = &fields[0].samples else { panic!("station samples") };
    assert!(values.iter().all(|v| (v.value - ratio).abs() < 1e-12), "the colour is EI_eff / EI_gross of the computed section");
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    assert!(all.iter().any(|h| h.id == format!("{id}:rho")), "grip on the reinforcement ratio");
    assert!(!all.iter().any(|h| h.id == format!("{id}:cracked")), "the declared share grip is replaced");
    let diagram = diagrams(&document, &report, None).into_iter().find(|d| d.id == format!("pier-moment-curvature:{id}")).expect("pier moment-curvature");
    assert!(diagram.markers.iter().any(|m| m.id == format!("reference:yield-{id}")) && diagram.markers.iter().any(|m| m.id == format!("reference:ultimate-{id}")));
    assert!(diagram.series.len() == 2 && diagram.table_twin.columns.len() == 2);
    assert!(report.checks.iter().any(|c| c.id.ends_with(&format!(".pier.{id}.stiffness"))));
}

#[test]
fn tie_beams_slopes_and_placements_are_registered_kinds_with_pickable_scene_entities() {
    let mut document = tie_and_slope_document();
    document.placements = vec![model::En1998Placement { id: "sl-1".into(), x: 3.0, y: 4.0, z: 0.0, yaw: 0.0 }];
    for (id, collection) in [("tieBeam", "tieBeams"), ("slope", "slopes"), ("placement", "placements")] {
        let hit = ENTITY_KINDS.iter().find(|k| k.id == id).expect("kind");
        assert_eq!(hit.collection, collection);
        assert!(!hit.label_en.is_empty() && !hit.label_de.is_empty());
    }
    assert_eq!(entity_kind_of(&document, "tb-1").map(|k| k.id), Some("tieBeam"));
    assert_eq!(entity_kind_of(&document, "sl-1").map(|k| k.id), Some("slope"));
    assert!(entity_dependencies(&document, "tb-1").contains(&"fd-2".to_string()));
    assert!(entity_dependencies(&document, "fd-2").contains(&"tb-1".to_string()));
    let (scene, report) = scene_of(&document);
    let owned: BTreeSet<_> = entities_of(&document).into_iter().map(|e| e.id).collect();
    assert!(owned.contains("tb-1") && owned.contains("sl-1"));
    for (id, kind) in [("tb-1", EntityKind::Member), ("sl-1", EntityKind::Solid)] {
        let entity = scene.entities.iter().find(|e| e.entity.id == id).expect("drawn");
        assert_eq!(entity.entity.kind, kind);
        let bound = entity.utilisation.as_ref().expect("bound to its check");
        assert!(report.checks.iter().any(|c| c.id == bound.check_id && c.subject.entity_id == id), "the bound check names the entity as subject");
    }
}

fn region_checks(report: &CheckReport, region: &model::En1998CriticalRegion, suffixes: &[&str]) -> Vec<(String, f64)> {
    suffixes.iter().filter_map(|s| report.checks.iter().find(|c| c.id == format!("en1998.1.bldg-office.region.{}.{s}", region.id))).map(|c| (c.id.clone(), c.utilization)).collect()
}

#[test]
fn a_critical_region_carries_a_hoop_layout_coloured_by_its_hoop_checks() {
    let document = En1998Snapshot::full_showcase();
    let region = document.critical_regions.iter().find(|r| r.member_id == "col-c1").expect("column region");
    let (scene, report) = scene_of(&document);
    let hoops: Vec<&SceneEntity> = scene.entities.iter().filter(|e| e.entity.id.starts_with(&format!("{}:hoop:", region.id))).collect();
    assert_eq!(hoops.len(), buildings::hoop_positions(region.hooped_length_m, region.hoop_spacing_m).len());
    assert!(!hoops.is_empty());
    assert!(hoops.iter().all(|e| e.layer == kit::LAYER_REGIONS && e.accessible_detail.en.contains("effectiveness") && e.accessible_detail.de.contains("Wirksamkeit")));
    let governing = region_checks(&report, region, &["hoopDiameter", "hoopSpacing", "barSpacing", "confinement", "omegaWdMin"]).into_iter().fold(("".to_owned(), 0.0), |best, next| if next.1 > best.1 { next } else { best });
    assert!(!governing.0.is_empty());
    for hoop in &hoops {
        let binding = hoop.utilisation.as_ref().expect("coloured by the hoop checks");
        assert_eq!(binding.check_id, governing.0);
        near(binding.ratio, governing.1, "hoop colour ratio");
    }
    let member = scene.entities.iter().find(|e| e.entity.id == format!("{}:region", region.id)).expect("member glyph");
    assert!(member.utilisation.as_ref().is_some_and(|u| u.check_id.ends_with(".length") || u.check_id.ends_with(".rhoTot")));
}

#[test]
fn a_wide_hoop_spacing_turns_the_hoops_of_the_region_non_compliant() {
    let mut document = En1998Snapshot::full_showcase();
    let id = document.critical_regions.iter().find(|r| r.member_id == "col-c1").expect("column region").id.clone();
    document.critical_regions.iter_mut().find(|r| r.id == id).expect("row").hoop_spacing_m = 0.2;
    let (scene, _) = scene_of(&document);
    let hoop = scene.entities.iter().find(|e| e.entity.id == format!("{id}:hoop:1")).expect("first hoop");
    let binding = hoop.utilisation.as_ref().expect("bound");
    assert!(binding.ratio > 1.0 && binding.status == scene::CheckStatus::Fail, "{binding:?}");
}

#[test]
fn the_region_member_has_grips_for_hoop_spacing_diameter_and_hooped_length() {
    let document = En1998Snapshot::full_showcase();
    let region = document.critical_regions.iter().find(|r| r.member_id == "col-c1").expect("column region");
    let (scene, _) = scene_of(&document);
    let all = handles(&document, &[], &scene.entities.iter().map(|e| e.entity.clone()).collect::<Vec<_>>());
    assert_handle_laws(&all);
    let entity = scene.entities.iter().find(|e| e.entity.id == format!("{}:region", region.id)).expect("member glyph");
    for (tag, field, value) in [("hoop-spacing", "hoopSpacingM", region.hoop_spacing_m), ("hoop-diameter", "hoopDiameterM", region.hoop_diameter_m), ("hooped-length", "hoopedLengthM", region.hooped_length_m)] {
        let id = format!("{}:region:{tag}", region.id);
        let grip = all.iter().find(|h| h.id == id).unwrap_or_else(|| panic!("grip {id}"));
        assert_eq!(grip.writes[0].path, format!("criticalRegions[id={}].{field}", region.id));
        near(grip.writes_at(0.0)[0].value, value, tag);
        assert!(entity.handles.contains(&id), "{id} listed by the member glyph");
    }
}

/// 🧭️ Every check of every fixture is drawn: it resolves to a document row the scene projects, or to the global badge through a document-level family, and the focus of the outcome names the same row (modal, limitState and regularity checks reach their building, bridge, tower or assessment).
#[test]
fn every_check_outcome_routes_to_its_entity_or_the_document_badge() {
    use kit::{base_id, focus_of_outcome, route_of, CheckRoute};
    for (name, document) in fixtures() {
        let (scene, report) = scene_of(&document);
        assert!(!report.checks.is_empty(), "{name}");
        for check in &report.checks {
            let route = route_of(&document, check).unwrap_or_else(|| panic!("{name}: check {} (subject {:?}) has no owner", check.id, check.subject.entity_id));
            match route {
                CheckRoute::Entity(id) => {
                    assert!(scene.entities.iter().any(|e| base_id(&e.entity.id) == id) || document.critical_regions.iter().any(|r| r.id == id), "{name}: check {} routes to {id}, which the scene does not draw", check.id);
                    assert_eq!(focus_of_outcome(&document, &report, &check.id).as_deref(), Some(id.as_str()), "{name}");
                }
                CheckRoute::Document => assert!(focus_of_outcome(&document, &report, &check.id).is_none(), "{name}: {}", check.id),
            }
        }
    }
}

/// 🧭️ The building carries the worst of its modal, regularity and combination checks, the assessed member the worst of its limit state and analysis method checks, through the owner routing.
#[test]
fn modal_regularity_and_limit_state_checks_bind_to_their_owner_entity() {
    let document = En1998Snapshot::full_showcase();
    let (scene, report) = scene_of(&document);
    for (id, fragments) in [(document.buildings[0].id.as_str(), vec![".regularity.", ".t1", "componentCombination"]), (document.assessments[0].id.as_str(), vec![".limitState", ".analysisMethod"])] {
        let owned = report.checks.iter().filter(|c| c.utilization.is_finite() && fragments.iter().any(|f| c.id.contains(f)) && kit::owner_entity(&document, c).as_deref() == Some(id)).count();
        let bound = kit::binding_of_owner(&document, &report, id, &fragments);
        assert_eq!(bound.is_some(), owned > 0, "{id}");
        let entity = scene.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("{id}"));
        if let Some(worst) = bound {
            assert!(entity.utilisation.as_ref().is_some_and(|u| u.ratio > 0.0 || worst.ratio == 0.0), "{id}");
        }
    }
}

/// 📈️ A check of an entity with a dedicated physical diagram is drawn by that diagram, otherwise by the outcome diagram of its entity, never by both and never by neither.
#[test]
fn every_check_is_drawn_by_a_dedicated_diagram_or_the_outcome_diagram() {
    let owners = |document: &En1998Snapshot| -> BTreeSet<String> {
        let d = document;
        d.bridges.iter().map(|e| &e.id).chain(d.silos.iter().map(|e| &e.id)).chain(d.tanks.iter().map(|e| &e.id)).chain(d.towers.iter().map(|e| &e.id)).chain(d.retaining_walls.iter().map(|e| &e.id)).chain(d.piers.iter().map(|e| &e.id)).chain(d.bearings.iter().map(|e| &e.id)).chain(d.joints.iter().map(|e| &e.id)).chain(d.tie_beams.iter().map(|e| &e.id)).cloned().collect()
    };
    let mut dedicated = 0;
    for (name, document) in fixtures() {
        let (_, report) = scene_of(&document);
        let set = diagrams(&document, &report, None);
        let owners = owners(&document);
        for check in report.checks.iter().filter(|c| c.utilization.is_finite() && owners.contains(&c.subject.entity_id)) {
            let drawn = kit::drawn_diagram(&check.id).is_some_and(|id| set.iter().any(|d| d.id == id));
            let outcome = set.iter().any(|d| d.id.ends_with(&format!("-checks:{}", check.subject.entity_id)) && d.markers.iter().any(|m| m.bound_check.as_deref() == Some(check.id.as_str())));
            assert!(drawn != outcome, "{name}: {} drawn {drawn}, outcome {outcome}", check.id);
            dedicated += usize::from(drawn);
        }
    }
    assert!(dedicated > 0, "no check is drawn by a dedicated diagram");
}

/// 📈️ The spectrum diagram marks every modal period on the design spectrum with its effective mass share; the capacity-design diagram stands at M_Rc/M_Rb of every joint against 1.3; the bearing diagram carries the displacement capacity and the design point of the bridge.
#[test]
fn the_physical_diagrams_mark_modal_periods_joint_ratios_and_the_bearing_displacement() {
    let document = En1998Snapshot::full_showcase();
    let (_, report) = scene_of(&document);
    let building = &document.buildings[0];
    let response = respond(&site(&document), building);
    let spectrum = diagrams(&document, &report, Some(&building.id)).into_iter().find(|d| d.id == "spectra:document").expect("spectra");
    for mode in response.modes.iter().filter(|m| m.period > 0.0 && m.period <= 4.0) {
        let marker = spectrum.markers.iter().find(|m| m.id == format!("mode-{}{}", if mode.along_y { "y" } else { "x" }, mode.index)).unwrap_or_else(|| panic!("mode {}", mode.index));
        near(marker.at.x, mode.period, "the modal marker stands at the period");
    }
    assert!(diagrams(&document, &report, Some(&building.id)).iter().all(|d| d.id != format!("drift:{}", building.id)), "the utilisation drift plot is replaced by storey-drift");
    if let Some(joint) = document.joints.iter().find(|j| j.building_id == building.id && j.beam_moment_rd_nm > 0.0) {
        let diagram = diagram_named(&document, &report, &format!("joint-ratios:{}", building.id));
        let marker = diagram.markers.iter().find(|m| m.id == format!("joint-{}", joint.id)).expect("joint marker");
        near(marker.at.y, joint.column_moment_rd_nm / joint.beam_moment_rd_nm, "M_Rc/M_Rb");
        assert!(diagram.series.iter().any(|s| matches!(s, Series::Limit { value, .. } if (*value - 1.3).abs() < 1e-12)));
        assert!(marker.bound_check.is_some());
    }
    for bridge in document.bridges.iter().filter(|b| b.bearing_d_rd_m > 0.0) {
        let diagram = diagram_named(&document, &report, &format!("bearing-displacement:{}", bridge.id));
        assert!(diagram.series.iter().any(|s| matches!(s, Series::Limit { value, .. } if (*value - 1000.0 * bridge.bearing_d_rd_m).abs() < 1e-9)));
        assert_eq!(diagram.markers.len(), 1);
    }
}

/// 🎚️ Every numeric field of a placement, soil layer and retaining wall row has a viewport grip that writes it, or a documented exemption, and none has both.
#[test]
fn every_numeric_field_of_placements_soil_layers_and_retaining_walls_has_a_grip_or_a_documented_exemption() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧬️schema/📸️snapshot/🔣️.json");
    let schema: Value = serde_json::from_slice(&std::fs::read(path).expect("snapshot schema")).expect("json");
    let mut document = En1998Snapshot::full_showcase();
    let foundation = document.foundations[0].id.clone();
    let mut wall = document.retaining_walls.first().cloned().expect("a wall");
    wall.id = "rw-dry".into();
    wall.water_height_m = 0.0;
    let mut wet = wall.clone();
    wet.id = "rw-wet".into();
    wet.water_height_m = 1.0;
    document.retaining_walls.extend([wall, wet]);
    if document.soil_layers.is_empty() {
        document.soil_layers.push(model::En1998SoilLayer { id: "sl-1".into(), foundation_id: foundation, thickness_m: 2.0, unit_weight_n_m3: 18000.0, shear_wave_velocity_m_s: 200.0, n1_60: 12.0, fines_percent: 10.0 });
    }
    if document.placements.is_empty() {
        let id = document.buildings[0].id.clone();
        document.placements.push(model::En1998Placement { id, x: 1.0, y: 2.0, z: 0.0, yaw: 0.0 });
    }
    let layout = Layout::of(&document, &[]);
    let grips = handles::all(&document, &layout);
    assert_handle_laws(&grips);
    for (collection, def, rows) in [("retainingWalls", "En1998RetainingWall", document.retaining_walls.iter().map(|r| r.id.clone()).collect::<Vec<_>>()), ("soilLayers", "En1998SoilLayer", document.soil_layers.iter().map(|r| r.id.clone()).collect()), ("placements", "En1998Placement", document.placements.iter().map(|r| r.id.clone()).collect())] {
        let properties = schema["$defs"][def]["properties"].as_object().unwrap_or_else(|| panic!("{def}"));
        for (field, _) in properties.iter().filter(|(_, v)| matches!(v["type"].as_str(), Some("number" | "integer"))) {
            let exempt = handles::EXEMPT.contains(&(collection, field.as_str()));
            let written = |id: &str| grips.iter().any(|g| g.writes.iter().any(|w| w.path == format!("{collection}[id={id}].{field}")));
            let by_grip = rows.iter().any(|id| written(id));
            let by_move = collection == "placements" && matches!(field.as_str(), "x" | "y");
            assert!(by_grip || by_move || exempt, "{collection}.{field} has no grip");
            assert!(u8::from(by_grip || by_move) + u8::from(exempt) == 1, "{collection}.{field} is covered twice");
        }
    }
}

/// 🛞 The isolation system of the showcase is a spring support at the bridge deck with a ghost at each end of ±d_cd, a demand and a draggable capacity dimension; its hysteresis diagram holds the loop whose area is the dissipated energy and a design marker bound to the displacement check at d_cd.
#[test]
fn the_isolation_system_is_a_spring_with_displacement_ghosts_and_a_hysteresis_diagram_at_the_design_displacement() {
    let document = En1998Snapshot::full_showcase();
    let (scene, report) = scene_of(&document);
    let support = scene.entities.iter().find(|e| e.entity.id == "iso-1").expect("isolation support");
    assert!(matches!(&support.geometry, Geometry::Support(s) if matches!(&s.kind, SupportKind::Spring { stiffness } if stiffness.len() == 6)));
    let d_cd = kit::computed_of(&report, "en1998.isolation.iso-1.displacement").expect("d_cd");
    for tag in ["plus", "minus"] {
        assert!(scene.entities.iter().any(|e| e.entity.id == format!("iso-1:ghost-{tag}")), "ghost {tag}");
    }
    assert!(scene.annotations.iter().any(|a| a.id() == "iso-1:capacity") && scene.annotations.iter().any(|a| a.id() == "iso-1:demand"));
    let set = diagrams(&document, &report, None);
    let diagram = set.iter().find(|d| d.id == "isolation-loop:iso-1").expect("loop diagram");
    let marker = diagram.markers.iter().find(|m| m.id == "design").expect("design marker");
    near(marker.at.x, d_cd, "the design marker stands at d_cd");
    assert_eq!(marker.bound_check.as_deref(), Some("en1998.isolation.iso-1.displacement"));
    let system = &document.isolation_systems[0];
    let law = schema::isolation::Bilinear { strength_n: system.strength_n, post_yield_n_per_m: system.stiffness_n_per_m, yield_displacement_m: system.yield_displacement_m };
    let Some(Series::Polyline { points, .. }) = diagram.series.iter().find(|s| matches!(s, Series::Polyline { id, .. } if id == "loop")) else { panic!("loop series") };
    let area = 0.5 * points.windows(2).map(|w| w[0].x * w[1].y - w[1].x * w[0].y).sum::<f64>().abs();
    near(area, law.dissipated_energy_nm(d_cd), "the loop encloses E_D");
    assert!(diagram.markers.iter().any(|m| m.id == "reference:capacity" && m.binding.as_ref().is_some_and(|b| b.path == "isolationSystems[id=iso-1].capacityM")));
}

/// 🪨 The pier base is the Gazetas spring under its pier with the footing disc and the radius dimension, and its diagram marks the fixed-base and the lengthened period on the displacement spectrum.
#[test]
fn the_pier_base_is_a_foundation_spring_and_its_spectrum_marks_both_periods() {
    let document = En1998Snapshot::full_showcase();
    let (scene, report) = scene_of(&document);
    let support = scene.entities.iter().find(|e| e.entity.id == "pb-1").expect("pier base support");
    assert!(matches!(&support.geometry, Geometry::Support(s) if matches!(&s.kind, SupportKind::Spring { stiffness } if stiffness.len() == 6)));
    assert!(scene.entities.iter().any(|e| e.entity.id == "pb-1:footing") && scene.annotations.iter().any(|a| a.id() == "pb-1:radius"));
    let t_ssi = kit::computed_of(&report, "en1998.2.br-1.base.pb-1.period").expect("T_ssi");
    let diagram = diagrams(&document, &report, None).into_iter().find(|d| d.id == "pier-base:pb-1").expect("spectrum");
    let ssi = diagram.markers.iter().find(|m| m.id == "ssi").expect("ssi marker");
    near(ssi.at.x, t_ssi, "the lengthened marker stands at T_ssi");
    assert!(diagram.markers.iter().find(|m| m.id == "fixed").is_some_and(|m| m.at.x < ssi.at.x));
}

/// 🌊 The cyclic ratio diagram of a foundation stands CSR against the layer resistance over the depth with r_u in the table twin; the marker of the liquefied layer l7 lies on the stress curve at its governing depth.
#[test]
fn the_cyclic_ratio_diagram_marks_the_liquefied_layer_where_the_stress_exceeds_the_resistance() {
    let document = En1998Snapshot::full_showcase();
    let (_, report) = scene_of(&document);
    let diagram = diagrams(&document, &report, None).into_iter().find(|d| d.id == "liquefaction-ratios:fd-3").expect("ratio diagram");
    let marker = diagram.markers.iter().find(|m| m.id == "layer-l7").expect("l7 marker");
    assert_eq!(marker.bound_check.as_deref(), Some("en1998.5.fd-3.layer.l7.liquefaction"));
    assert!(marker.at.y < 0.0 && marker.at.x > 0.0);
    assert_eq!(diagram.table_twin.columns.len(), 5);
    let r_u = diagram.table_twin.columns.last().expect("r_u column");
    assert!(r_u.values.iter().all(|v| (0.0..=1.0).contains(v)) && r_u.values.iter().any(|v| *v >= 0.0));
}

#[test]
fn the_basis_badge_follows_the_document_annex_in_the_scene_and_every_diagram() {
    scene::projection_kit::test_support::assert_basis_follows_annex(norm::NormFamilyId::En1998, |annex| {
        let mut document = rich_document();
        document.annex = annex;
        let (scene, report) = scene_of(&document);
        (scene, diagrams(&document, &report, None))
    });
}
