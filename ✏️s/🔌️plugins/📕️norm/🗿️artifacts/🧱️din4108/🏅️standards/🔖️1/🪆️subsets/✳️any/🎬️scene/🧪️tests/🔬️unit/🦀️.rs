use super::*;
use scene::projection_kit::test_support::{assert_diagram_laws, assert_handle_laws, assert_scene_laws, entity, revision};
use scene::{canonical_json, Diagram2d, Series, Validate};
use std::collections::BTreeSet;

fn scene_of(document: &Din4108Snapshot) -> StructuralScene {
    let report = inferences::evaluate(document);
    project(&SceneInput { document, report: &report, revision: revision() })
}

fn fixtures() -> Vec<(&'static str, Din4108Snapshot)> {
    let mut thin = Din4108Snapshot::default();
    thin.elements[0].layers[2].thickness_m = 0.01;
    vec![("compliant", Din4108Snapshot::default()), ("thin-insulation", thin), ("empty", Din4108Snapshot { zones: vec![], elements: vec![], thermal_bridges: vec![], ..Din4108Snapshot::default() })]
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
    let document = Din4108Snapshot::default();
    assert_eq!(canonical_json(&scene_of(&document)), canonical_json(&scene_of(&document)));
}

#[test]
fn every_row_of_the_document_is_a_scene_entity() {
    let document = Din4108Snapshot::default();
    let scene = scene_of(&document);
    let ids: BTreeSet<_> = scene.entities.iter().map(|e| e.entity.id.as_str()).collect();
    for entity in entities_of(&document) {
        assert!(ids.contains(entity.id.as_str()), "{} is not drawn", entity.id);
    }
}

#[test]
fn an_element_draws_its_layers_with_their_real_thicknesses() {
    let document = Din4108Snapshot::default();
    let scene = scene_of(&document);
    let scene::Geometry::Solid(scene::SolidGeometry { shape: scene::SolidShape::WallAssembly { layers, .. } }) = &entity(&scene, "wall-north").geometry else { panic!("wall-north is no wall assembly") };
    let drawn: Vec<f64> = layers.iter().map(|l| l.thickness).collect();
    let stored: Vec<f64> = document.elements[0].layers.iter().map(|l| l.thickness_m).collect();
    assert_eq!(drawn, stored);
}

#[test]
fn the_failing_check_colours_its_element() {
    let mut document = Din4108Snapshot::default();
    document.elements[0].layers[2].thickness_m = 0.005;
    let scene = scene_of(&document);
    assert_eq!(entity(&scene, "wall-north").appearance.style, scene::StyleState::Error);
}

#[test]
fn diagrams_obey_the_laws_and_follow_the_closed_forms() {
    for (_, document) in fixtures() {
        let report = inferences::evaluate(&document);
        let all = diagrams(&document, &report, None);
        assert_diagram_laws(&all);
    }
    let document = Din4108Snapshot::default();
    let report = inferences::evaluate(&document);
    let all = diagrams(&document, &report, Some("wall-north"));
    let temperature = all.iter().find(|d| d.id == "temperature:wall-north").expect("temperature profile");
    let Series::Polyline { points, .. } = &temperature.series[0] else { panic!("polyline") };
    assert_eq!(points.len(), document.elements[0].layers.len() + 1);
    assert!(points.windows(2).all(|w| w[0].y >= w[1].y), "the temperature falls from the interior to the exterior");
    let glaser = all.iter().find(|d| d.id == "glaser:wall-north").expect("glaser profile");
    assert_eq!(glaser.table_twin.columns.len(), 3);
}

#[test]
fn an_unknown_focus_has_no_diagrams() {
    let document = Din4108Snapshot::default();
    assert!(diagrams(&document, &inferences::evaluate(&document), Some("nope")).is_empty());
}

#[test]
fn handles_are_lawful_and_write_document_leaves() {
    let document = Din4108Snapshot::default();
    let selection: Vec<_> = entities_of(&document);
    let all = handles(&document, &selection);
    assert!(!all.is_empty());
    assert_handle_laws(&all);
    let thickness = all.iter().find(|h| h.id == "wall-north:thickness:masonry").expect("masonry grip");
    assert_eq!(thickness.writes[0].path, "elements[id=wall-north].layers[id=masonry].thicknessM");
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

fn polyline_json(diagram: &Diagram2d, id: &str) -> String {
    let Some(Series::Polyline { points, .. }) = diagram.series.iter().find(|s| s.id() == id) else { panic!("{} has no series {id}", diagram.id) };
    format!("[{}]", points.iter().map(|p| format!("[{},{}]", p.x, p.y)).collect::<Vec<_>>().join(","))
}

fn oracle_element(document: &Din4108Snapshot, element: &model::EnvelopeElement, all: &[Diagram2d]) -> String {
    let (r_si, r_se) = checks::surface_resistances_inclined(&element.kind, &element.adjacent, element.inclination_deg);
    let boundary = checks::part_3::glaser_boundary(element, document.climate_zone);
    let layers: Vec<String> = element.layers.iter().map(|l| format!("{{\"d\":{},\"lambda\":{},\"mu\":{}}}", l.thickness_m, l.lambda, l.mu)).collect();
    let temperature = all.iter().find(|d| d.id == format!("temperature:{}", element.id)).expect("temperature diagram");
    let glaser = all.iter().find(|d| d.id == format!("glaser:{}", element.id)).expect("glaser diagram");
    format!(
        "{{\"id\":\"{}\",\"r_si\":{r_si},\"r_se\":{r_se},\"t_int\":{},\"t_ext\":{},\"rh_int\":{},\"rh_ext\":{},\"layers\":[{}],\"temperature\":{},\"vapour\":{},\"saturation\":{}}}",
        element.id,
        document.t_int_c,
        boundary.t_ext_w,
        document.rh_int,
        boundary.rh_ext_w,
        layers.join(","),
        polyline_json(temperature, "temperature"),
        polyline_json(glaser, "vapour"),
        polyline_json(glaser, "saturation"),
    )
}

#[test]
fn the_independent_oracle_agrees() {
    let document = Din4108Snapshot::default();
    let report = inferences::evaluate(&document);
    let all = diagrams(&document, &report, None);
    assert_diagram_laws(&all);
    let rows: Vec<String> = document.elements.iter().filter(|e| !e.layers.is_empty()).map(|e| oracle_element(&document, e, &all)).collect();
    let input = format!("{{\"elements\":[{}]}}", rows.join(","));
    let script = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔮️oracles/🐍️.py");
    let output = norm::python_oracle::run_ok(script, &[], Some(input.as_bytes()));
    let verdict = String::from_utf8_lossy(&output.stdout);
    assert!(verdict.contains("\"ok\": true"), "oracle verdict: {verdict}");
}

#[test]
fn projecting_a_thousand_elements_stays_interactive() {
    let mut document = Din4108Snapshot::default();
    let template = document.elements[0].clone();
    document.elements = (0..1000).map(|i| model::EnvelopeElement { id: format!("e{i}"), ..template.clone() }).collect();
    let report = inferences::evaluate(&document);
    let started = std::time::Instant::now();
    let scene = project(&SceneInput { document: &document, report: &report, revision: revision() });
    assert!(scene.entities.len() >= 1000);
    assert!(started.elapsed() < std::time::Duration::from_secs(5), "projection took {:?}", started.elapsed());
}

fn vector_layer(row: &serde_json::Value) -> model::LayerDocument {
    let number = |key: &str| row[key].as_f64().expect(key);
    model::LayerDocument { id: "l".into(), material_id: String::new(), thickness_m: number("thicknessM"), lambda: number("lambda"), mu: row["mu"].as_f64().unwrap_or(1.0), density: 0.0, application_type: String::new(), compressive_class: String::new(), water_class: String::new(), tensile_class: String::new(), acoustic_class: String::new(), segments: vec![] }
}

#[test]
fn the_glaser_drying_vectors_agree_with_the_independent_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧮️glaser-drying/🔣️.json")).expect("vectors");
    let tolerance = vectors["toleranceRelative"].as_f64().expect("tolerance");
    let near = |got: f64, want: f64| (got - want).abs() <= tolerance * want.abs().max(1e-6);
    for case in vectors["cases"].as_array().expect("cases") {
        let name = case["id"].as_str().expect("id");
        let layers: Vec<model::LayerDocument> = case["layers"].as_array().expect("layers").iter().map(vector_layer).collect();
        let number = |key: &str| case[key].as_f64().expect(key);
        let boundary = |key: &str| (case[key]["tC"].as_f64().expect("tC"), case[key]["rh"].as_f64().expect("rh"));
        let (winter, summer) = (boundary("winter"), boundary("summer"));
        let condensed = checks::part_3::glaser_condensed_mass_kg_m2(&layers, number("rSi"), number("rSe"), number("tIntC"), number("rhInt"), winter.0, winter.1, number("hours"));
        let evaporated = checks::part_3::glaser_evaporated_mass_kg_m2(&layers, number("rSi"), number("rSe"), number("tIntC"), number("rhInt"), winter, summer, number("hours"));
        assert!(near(condensed, number("condensedKgM2")), "{name}: condensed {condensed} against {}", number("condensedKgM2"));
        assert!(near(evaporated, number("evaporatedKgM2")), "{name}: evaporated {evaporated} against {}", number("evaporatedKgM2"));
        assert_eq!(evaporated >= condensed, case["dries"].as_bool().expect("dries"), "{name}");
    }
}

#[test]
fn the_u_value_vectors_agree_with_the_independent_iso_6946_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧮️u-values/🔣️.json")).expect("vectors");
    let tolerance = vectors["toleranceRelative"].as_f64().expect("tolerance");
    for case in vectors["cases"].as_array().expect("cases") {
        let name = case["id"].as_str().expect("id");
        let layers: Vec<model::LayerDocument> = case["layers"]
            .as_array()
            .expect("layers")
            .iter()
            .map(|row| {
                let segments = row["segments"].as_array().expect("segments").iter().map(|s| model::LayerSegment { id: "s".into(), material_id: String::new(), fraction: s["fraction"].as_f64().expect("fraction"), lambda: s["lambda"].as_f64().expect("lambda"), mu: 1.0, density: 0.0 }).collect();
                model::LayerDocument { segments, ..vector_layer(row) }
            })
            .collect();
        let r = checks::total_resistance(&layers, case["rSi"].as_f64().expect("rSi"), case["rSe"].as_f64().expect("rSe"));
        let (want_r, want_u) = (case["rTotal"].as_f64().expect("rTotal"), case["u"].as_f64().expect("u"));
        assert!((r - want_r).abs() <= tolerance * want_r.abs().max(1.0), "{name}: R_T {r} against {want_r}");
        assert!((checks::u_value(r) - want_u).abs() <= tolerance * want_u.abs().max(1.0), "{name}: U against {want_u}");
    }
}
