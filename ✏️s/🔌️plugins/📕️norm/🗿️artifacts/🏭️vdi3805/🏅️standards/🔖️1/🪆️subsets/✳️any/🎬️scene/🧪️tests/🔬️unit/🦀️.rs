use super::*;
use scene::projection_kit::test_support::{assert_diagram_laws, assert_handle_laws, assert_scene_laws, entity, revision};
use scene::{canonical_json, Series, Validate};
use std::collections::BTreeSet;

fn scene_of(document: &Vdi3805Snapshot) -> StructuralScene {
    let report = inferences::evaluate(document);
    project(&SceneInput { document, report: &report, revision: revision() })
}

fn fixtures() -> Vec<(&'static str, Vdi3805Snapshot)> {
    vec![
        ("valve", model::conforming_valve_dataset()),
        ("valve-fail", model::nonconforming_valve_dataset()),
        ("radiator", model::conforming_blatt_3_dataset()),
        ("pump", model::conforming_blatt_5_dataset()),
        ("generator", model::conforming_blatt_6_dataset()),
        ("empty", Vdi3805Snapshot { catalog: model::ManufacturerCatalog { products: vec![], ..model::conforming_valve_dataset().catalog }, geometry: Default::default(), curves: Default::default(), ..model::conforming_valve_dataset() }),
    ]
}

#[test]
fn the_scene_of_every_fixture_validates() {
    for (name, document) in fixtures() {
        let scene = scene_of(&document);
        assert_scene_laws(&scene);
        assert_eq!(scene.validate(), Ok(()), "violations in {name}");
    }
}

#[test]
fn ids_are_unique_and_layers_exist() {
    for (name, document) in fixtures() {
        let scene = scene_of(&document);
        let ids: BTreeSet<_> = scene.entities.iter().map(|e| e.entity.id.clone()).collect();
        assert_eq!(ids.len(), scene.entities.len(), "{name}");
        let layers: BTreeSet<_> = scene.layers.iter().map(|l| l.id.as_str()).collect();
        assert!(scene.entities.iter().all(|e| layers.contains(e.layer.as_str())), "{name}");
    }
}

#[test]
fn projection_is_deterministic() {
    let document = model::conforming_valve_dataset();
    assert_eq!(canonical_json(&scene_of(&document)), canonical_json(&scene_of(&document)));
}

#[test]
fn every_row_of_the_document_is_a_scene_entity() {
    for (name, document) in fixtures() {
        let scene = scene_of(&document);
        let ids: BTreeSet<_> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
        for entity in entities_of(&document) {
            assert!(ids.contains(entity.id.as_str()), "{name}: {} is not drawn", entity.id);
        }
    }
}

#[test]
fn a_product_is_drawn_at_the_size_of_its_geometry() {
    let document = model::conforming_valve_dataset();
    let scene = scene_of(&document);
    let scene::Geometry::Solid(scene::SolidGeometry { shape: scene::SolidShape::Box { size, .. } }) = &entity(&scene, "VLV-50-001").geometry else { panic!("the valve is no box") };
    let bbox = document.geometry["geom-valve-50"].evaluate_bbox();
    assert!((size.x - (bbox.max_x - bbox.min_x)).abs() < 1e-12 && (size.y - (bbox.max_y - bbox.min_y)).abs() < 1e-12 && (size.z - (bbox.max_z - bbox.min_z)).abs() < 1e-12);
}

#[test]
fn a_radiator_without_geometry_is_drawn_from_its_extent() {
    let document = model::conforming_blatt_3_dataset();
    let scene = scene_of(&document);
    let scene::Geometry::Solid(scene::SolidGeometry { shape: scene::SolidShape::Box { size, .. } }) = &entity(&scene, "RAD-600").geometry else { panic!("the radiator is no box") };
    assert_eq!((size.x, size.y, size.z), (0.6, 0.1, 0.6));
}

#[test]
fn a_pump_is_drawn_as_a_cylinder() {
    let document = model::conforming_blatt_5_dataset();
    let id = document.catalog.products[0].id.clone();
    let scene = scene_of(&document);
    assert!(matches!(entity(&scene, &id).geometry, scene::Geometry::Solid(scene::SolidGeometry { shape: scene::SolidShape::Cylinder { .. } })));
}

#[test]
fn a_failing_check_colours_its_product() {
    let document = model::nonconforming_valve_dataset();
    let report = inferences::evaluate(&document);
    let failing: BTreeSet<String> = report.checks.iter().filter(|c| c.status == norm::CheckStatus::Fail).map(|c| c.subject.entity_id.clone()).collect();
    let scene = scene_of(&document);
    assert!(scene.entities.iter().any(|e| failing.contains(&e.entity.id) && e.appearance.style == scene::StyleState::Error));
}

#[test]
fn diagrams_obey_the_laws() {
    for (_, document) in fixtures() {
        let report = inferences::evaluate(&document);
        assert_diagram_laws(&diagrams(&document, &report, None));
    }
}

#[test]
fn the_radiator_curve_follows_the_en_442_closed_form() {
    let document = model::conforming_blatt_3_dataset();
    let report = inferences::evaluate(&document);
    let all = diagrams(&document, &report, Some("RAD-600"));
    let output = all.iter().find(|d| d.id == "output:RAD-600").expect("output curve");
    let Series::Polyline { points, .. } = &output.series[0] else { panic!("polyline") };
    assert_eq!(points.len(), diagrams::SAMPLES);
    for point in points {
        let expected = 1200.0 * (point.x / diagrams::DELTA_T_NOMINAL_K).powf(1.3);
        assert!((point.y - expected).abs() < 1e-9 * expected.max(1.0), "{point:?}");
    }
    assert!((diagrams::radiator_output_w(1200.0, 1.3, diagrams::DELTA_T_NOMINAL_K) - 1200.0).abs() < 1e-9);
    assert!(points.windows(2).all(|w| w[0].y < w[1].y), "the output grows with the excess temperature");
}

#[test]
fn the_valve_drop_follows_the_square_law() {
    let document = model::conforming_valve_dataset();
    let report = inferences::evaluate(&document);
    let all = diagrams(&document, &report, Some("VLV-50-001"));
    let drop = all.iter().find(|d| d.id == "drop:VLV-50-001").expect("drop curve");
    assert!((diagrams::valve_pressure_drop_bar(4.5, 4.5) - 1.0).abs() < 1e-12);
    assert!((diagrams::valve_pressure_drop_bar(4.5, 9.0) - 4.0).abs() < 1e-12);
    assert_eq!(drop.table_twin.columns.len(), 2);
}

#[test]
fn a_curve_diagram_has_one_draggable_marker_per_point() {
    let document = model::conforming_valve_dataset();
    let report = inferences::evaluate(&document);
    let all = diagrams(&document, &report, Some("VLV-50-001"));
    let curve = all.iter().find(|d| d.id == "curve:VLV-50-001:curve-kvs").expect("curve diagram");
    assert_eq!(curve.markers.len(), document.curves["curve-kvs"].points.len());
    assert_eq!(curve.markers[1].binding.as_ref().map(|b| b.path.as_str()), Some("curves[id=curve-kvs].points[1].y"));
}

#[test]
fn an_unknown_focus_has_no_diagrams() {
    let document = model::conforming_valve_dataset();
    assert!(diagrams(&document, &inferences::evaluate(&document), Some("nope")).is_empty());
}

#[test]
fn handles_are_lawful_and_write_document_leaves() {
    let document = model::conforming_valve_dataset();
    let all = handles(&document, &entities_of(&document));
    assert_eq!(all.len(), 7, "three face grips on the valve and on its geometry, the nominal diameter grip of the accessory valve");
    assert_handle_laws(&all);
    let width = all.iter().find(|h| h.id == "VLV-50-001:width").expect("width grip");
    assert_eq!(width.writes[0].path, "geometry[id=geom-valve-50].bbox.maxX");
    assert!((width.writes[0].map.apply(0.15) - 0.15).abs() < 1e-12, "the resting grip reads the stored extent back");
}

#[test]
fn every_check_is_drawn_on_its_row_or_the_badge() {
    for (name, document) in fixtures() {
        let report = inferences::evaluate(&document);
        for check in &report.checks {
            assert!(route_of(&document, check).is_some(), "{name}: {} has no route", check.id);
        }
    }
}

#[test]
fn the_golden_scene_matches_the_committed_bytes() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧫️fixtures/🏛️scene/🔣️.json");
    scene::projection_kit::test_support::assert_golden(path, &canonical_json(&scene_of(&model::conforming_valve_dataset())));
}

#[test]
fn the_independent_oracle_agrees() {
    let document = model::conforming_blatt_3_dataset();
    let report = inferences::evaluate(&document);
    let all = diagrams(&document, &report, Some("RAD-600"));
    let output = all.iter().find(|d| d.id == "output:RAD-600").expect("output curve");
    let columns: Vec<&Vec<f64>> = output.table_twin.columns.iter().map(|c| &c.values).collect();
    let valve = model::conforming_valve_dataset();
    let valve_report = inferences::evaluate(&valve);
    let valve_all = diagrams(&valve, &valve_report, Some("VLV-50-001"));
    let drop = valve_all.iter().find(|d| d.id == "drop:VLV-50-001").expect("drop curve");
    let drop_columns: Vec<&Vec<f64>> = drop.table_twin.columns.iter().map(|c| &c.values).collect();
    let layout = Layout::of(&valve);
    let grips: Vec<serde_json::Value> = handles(&valve, &entities_of(&valve)).iter().filter(|h| h.id.starts_with("VLV-50-001:")).map(|h| {
        let pose = layout.pose("VLV-50-001").expect("pose");
        let (start, rest) = match h.id.as_str() {
            "VLV-50-001:width" => (pose.min.x, pose.min.x + pose.size.x),
            "VLV-50-001:depth" => (pose.min.y, pose.min.y + pose.size.y),
            _ => (pose.min.z, pose.min.z + pose.size.z),
        };
        let stored = valve.geometry["geom-valve-50"].bbox;
        let (low, value) = match h.id.as_str() {
            "VLV-50-001:width" => (stored.min_x, stored.max_x),
            "VLV-50-001:depth" => (stored.min_y, stored.max_y),
            _ => (stored.min_z, stored.max_z),
        };
        serde_json::json!({"id": h.id, "low": low, "start": start, "scale": 1.0, "rest": rest, "stored": value})
    }).collect();
    let input = serde_json::json!({
        "radiator": {"standardOutputW": 1200.0, "exponent": 1.3, "deltaT": columns[0], "outputW": columns[1]},
        "valve": {"kvs": 4.5, "flowM3H": drop_columns[0], "dropBar": drop_columns[1]},
        "grips": grips,
    });
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔮️oracles/🐍️.py");
    let output = norm::python_oracle::run_ok(script, &[], Some(input.to_string().as_bytes()));
    let verdict = String::from_utf8_lossy(&output.stdout);
    assert!(verdict.contains("\"ok\": true"), "oracle verdict: {verdict}");
}

#[test]
fn an_edit_changes_only_the_fingerprints_of_the_entities_it_touches() {
    use scene::projection::{KitDoc, StructuralProjection, ViewOptions};
    let before = model::conforming_valve_dataset();
    let mut after = before.clone();
    after.geometry.get_mut("geom-valve-50").expect("geometry").bbox.max_x = 0.30;
    let entities = entities_of(&before);
    let projection = Vdi3805Projection::default();
    let print = |d: &Vdi3805Snapshot| projection.fingerprints(&KitDoc { snapshot: d, placements: &[], view: ViewOptions::none() }, &entities);
    let (a, b) = (print(&before), print(&after));
    let changed: Vec<&str> = entities.iter().zip(a.iter().zip(&b)).filter(|(_, (x, y))| x != y).map(|(e, _)| e.id.as_str()).collect();
    assert_eq!(changed, ["VLV-50-001", "ACT-01", "geom-valve-50"], "the widened valve pushes its neighbour along the row, the neighbour pose is part of its fingerprint");
    let mut renamed = before.clone();
    renamed.catalog.products[1].title = model::bilingual("Stellantrieb 2", "Actuator 2");
    let c = print(&renamed);
    let touched: Vec<&str> = entities.iter().zip(a.iter().zip(&c)).filter(|(_, (x, y))| x != y).map(|(e, _)| e.id.as_str()).collect();
    assert_eq!(touched, ["ACT-01"]);
}

#[test]
fn a_radiator_without_geometry_has_extent_grips_on_its_attributes() {
    let document = model::conforming_blatt_3_dataset();
    let all = handles(&document, &entities_of(&document));
    assert_eq!(all.len(), 3);
    let length = all.iter().find(|h| h.id == "RAD-600:width").expect("length grip");
    assert_eq!(length.writes[0].path, "catalog.products[id=RAD-600].configuration.attributes.lengthM");
    assert!((length.writes[0].map.apply(0.6) - 0.6).abs() < 1e-12);
}

#[test]
fn attribute_grips_rest_at_the_stored_values() {
    let pump = model::conforming_blatt_5_dataset();
    let generator = model::conforming_blatt_6_dataset();
    let valve = model::conforming_valve_dataset();
    for (document, id, tag, stored) in [
        (&pump, pump.catalog.products[0].id.clone(), "head", match &pump.catalog.products[0].configuration.attributes { model::SheetAttributes::PumpHeating(a) => a.nominal_head_m, _ => panic!("pump") }),
        (&pump, pump.catalog.products[0].id.clone(), "dn", match &pump.catalog.products[0].configuration.attributes { model::SheetAttributes::PumpHeating(a) => f64::from(a.dn_discharge), _ => panic!("pump") }),
        (&generator, generator.catalog.products[0].id.clone(), "output", match &generator.catalog.products[0].configuration.attributes { model::SheetAttributes::HeatGenerator(a) => a.nominal_heat_output_w, _ => panic!("generator") }),
        (&valve, "ACT-01".to_owned(), "dn", 50.0),
    ] {
        let grips = handles(document, &entities_of(document));
        let grip = grips.iter().find(|h| h.id == format!("{id}:{tag}")).unwrap_or_else(|| panic!("{id}:{tag}"));
        let rest = match &grip.kind { scene::HandleKind::ScalarAlong { axis, .. } => grip.anchor.x * axis.x + grip.anchor.y * axis.y + grip.anchor.z * axis.z, _ => panic!("scalar grip") };
        let read = grip.writes[0].map.apply(rest);
        assert!((read - stored).abs() <= 1e-6 * stored.abs().max(1.0), "{id}:{tag} reads {read}, stored {stored}");
        assert!(grip.writes[0].path.contains(&format!("catalog.products[id={id}].configuration.attributes.")));
    }
}
