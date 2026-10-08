use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, BIM_EXAMPLE_TEXT};

fn demo() -> ModelSnapshot {
    parse_dsl(BIM_EXAMPLE_TEXT).expect("the committed demo parses")
}

fn json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("scene lane is JSON")
}

fn scene_of(snapshot: &ModelSnapshot, orbit: Option<&store::Viewport3dOrbit>, hidden: &[String], selected: &[String]) -> World3dScene {
    let solids = crate::render::solids(snapshot);
    scene(snapshot, &solids, &WorldView { orbit, projection: &WorldProjectionConfig::default(), hidden_storeys: hidden, selected, hovered: &[] })
}

#[test]
fn every_solid_becomes_one_mesh_and_one_instance_keyed_by_the_element_id() {
    let scene = scene_of(&demo(), None, &[], &[]);
    let meshes = json(&scene.meshes_json);
    let instances = json(&scene.instances_json);
    let mesh_ids: Vec<&str> = meshes.as_array().expect("meshes").iter().map(|mesh| mesh["id"].as_str().expect("mesh id")).collect();
    let instance_ids: Vec<&str> = instances.as_array().expect("instances").iter().map(|instance| instance["id"].as_str().expect("instance id")).collect();
    assert_eq!(mesh_ids, vec!["w-east", "w-north", "w-south", "w-west"]);
    assert_eq!(instance_ids, mesh_ids);
    for instance in instances.as_array().expect("instances") {
        assert_eq!(instance["meshId"], instance["id"]);
        assert_eq!(instance["interactionId"], instance["id"]);
        assert_eq!(instance["interactionGranularityId"], ELEMENT_GRANULARITY);
    }
}

#[test]
fn the_scene_binds_the_element_interaction_domain() {
    let scene = scene_of(&demo(), None, &[], &[]);
    assert_eq!(scene.domain_id.as_deref(), Some(ELEMENT_DOMAIN));
    assert_eq!(scene.domain_granularity_id.as_deref(), Some(ELEMENT_GRANULARITY));
}

#[test]
fn mesh_arrays_are_consistent_and_coloured_per_vertex() {
    let model = demo();
    let scene = scene_of(&model, None, &[], &[]);
    for mesh in json(&scene.meshes_json).as_array().expect("meshes") {
        let data = &mesh["data"];
        let vertices = data["positions"].as_array().expect("positions").len() / 3;
        assert!(vertices > 0 && vertices.is_multiple_of(3), "one vertex triple per triangle");
        assert_eq!(data["normals"].as_array().expect("normals").len(), vertices * 3);
        assert_eq!(data["colors"].as_array().expect("colors").len(), vertices * 4);
        assert_eq!(data["indices"].as_array().expect("indices").len(), vertices);
        assert_eq!(data["faceIds"].as_array().expect("face ids").len(), vertices / 3);
    }
}

#[test]
fn a_solid_group_takes_the_colour_of_its_material() {
    let model = demo();
    let (id, material) = model.materials.iter().next().expect("a demo material");
    let group = SolidGroup { part: parts::LAYER.into(), material: id.clone(), layer: 0 };
    assert_eq!(group_color(&model, SolidFamily::Wall, &group), [material.color.r as f32, material.color.g as f32, material.color.b as f32, 1.0]);
    let unknown = SolidGroup { part: parts::BODY.into(), material: "none".into(), layer: 0 };
    assert_eq!(group_color(&model, SolidFamily::Roof, &unknown), family_color(SolidFamily::Roof));
    let glass = SolidGroup { part: parts::GLASS.into(), material: String::new(), layer: 0 };
    assert_eq!(group_color(&model, SolidFamily::Window, &glass)[3], 0.35);
}

#[test]
fn hidden_storeys_leave_their_elements_out() {
    let model = demo();
    assert!(json(&scene_of(&model, None, &["st-ground".to_string()], &[]).instances_json).as_array().expect("instances").is_empty());
    assert_eq!(json(&scene_of(&model, None, &["st-first".to_string()], &[]).instances_json).as_array().expect("instances").len(), 4);
}

#[test]
fn selection_marks_reach_instances_and_the_selection_lane() {
    let scene = scene_of(&demo(), None, &[], &["w-north".to_string()]);
    let instances = json(&scene.instances_json);
    let selected: Vec<&str> = instances.as_array().expect("instances").iter().filter(|instance| instance["selected"] == true).map(|instance| instance["id"].as_str().expect("id")).collect();
    assert_eq!(selected, vec!["w-north"]);
    assert_eq!(json(&scene.selection_json)["ids"], serde_json::json!(["w-north"]));
}

#[test]
fn the_fit_is_requested_only_while_no_orbit_is_stored() {
    let model = demo();
    assert_eq!(json(scene_of(&model, None, &[], &[]).fit_json.as_deref().expect("fit")), serde_json::json!({ "enabled": true, "revision": 0, "padding": 1.25 }));
    let stored = store::Viewport3dOrbit { position: [20.0, -20.0, 10.0], target: [4.0, 3.0, 1.0], zoom: 1.0, up: None };
    let scene = scene_of(&model, Some(&stored), &[], &[]);
    assert_eq!(json(scene.fit_json.as_deref().expect("fit"))["enabled"], false);
    let camera = json(&scene.camera_json);
    assert_eq!(camera["position"], serde_json::json!([20.0, -20.0, 10.0]));
    assert_eq!(camera["target"], serde_json::json!([4.0, 3.0, 1.0]));
}

#[test]
fn the_overview_camera_looks_at_the_centre_of_the_drawn_solids() {
    let model = demo();
    let solids = crate::render::solids(&model);
    let (lo, hi) = world_bounds(solids.values()).expect("drawn solids have bounds");
    let orbit = overview_orbit(Some((lo, hi)));
    assert_eq!(orbit.target, [(lo[0] + hi[0]) / 2.0, (lo[1] + hi[1]) / 2.0, (lo[2] + hi[2]) / 2.0]);
    assert!(orbit.position[0] > orbit.target[0] && orbit.position[1] < orbit.target[1] && orbit.position[2] > orbit.target[2], "from the south-east above");
    let camera = json(&scene_of(&model, None, &[], &[]).camera_json);
    assert_eq!(camera["target"], serde_json::json!(orbit.target));
}

#[test]
fn world_bounds_follow_the_building_placement() {
    let mut model = demo();
    let plain = world_bounds(crate::render::solids(&model).values()).expect("bounds");
    model.buildings.get_mut("bldg-1").expect("the building").origin = crate::Point2 { x: 100.0, y: -50.0 };
    let moved = world_bounds(crate::render::solids(&model).values()).expect("bounds");
    assert!((moved.0[0] - plain.0[0] - 100.0).abs() < 1e-9 && (moved.0[1] - plain.0[1] + 50.0).abs() < 1e-9);
}

#[test]
fn an_empty_model_renders_an_empty_scene() {
    let scene = scene_of(&ModelSnapshot::default(), None, &[], &[]);
    assert_eq!(scene.meshes_json, "[]");
    assert_eq!(scene.instances_json, "[]");
}

const CASES: [&str; 2] = ["demo", "placed-demo"];

fn case_model(name: &str) -> ModelSnapshot {
    let mut model = demo();
    if name == "placed-demo" {
        let building = model.buildings.get_mut("bldg-1").expect("the building");
        building.origin = crate::Point2 { x: 100.0, y: -50.0 };
        building.rotation = 0.5;
    }
    model
}

fn rounded(value: f64) -> f64 {
    (value * 1e6).round() / 1e6
}

fn case_projection(name: &str) -> serde_json::Value {
    let model = case_model(name);
    let solids = crate::render::solids(&model);
    let scene = scene_of(&model, None, &[], &[]);
    let meshes: Vec<serde_json::Value> = json(&scene.meshes_json)
        .as_array()
        .expect("meshes")
        .iter()
        .map(|mesh| serde_json::json!({ "id": mesh["id"], "positions": mesh["data"]["positions"].as_array().expect("positions").iter().map(|value| rounded(value.as_f64().expect("a number"))).collect::<Vec<f64>>(), "indices": mesh["data"]["indices"] }))
        .collect();
    let instances: Vec<serde_json::Value> = json(&scene.instances_json).as_array().expect("instances").iter().map(|instance| serde_json::json!({ "id": instance["id"], "position": instance["position"], "rotation": instance["rotation"] })).collect();
    let bounds: serde_json::Map<String, serde_json::Value> = solids
        .iter()
        .map(|(id, solid)| {
            let (min, max) = world_bounds([solid]).expect("a solid has bounds");
            (id.clone(), serde_json::json!({ "min": min.map(rounded), "max": max.map(rounded) }))
        })
        .collect();
    serde_json::json!({ "name": name, "meshes": meshes, "instances": instances, "bounds": bounds })
}

#[test]
fn the_committed_fixture_is_the_scene_vector_the_three_oracle_measures() {
    let fixture = json(include_str!("../../🧫️fixtures/🔣️.json"));
    let committed = fixture["cases"].as_array().expect("cases");
    assert_eq!(committed.len(), CASES.len(), "run the bless test (BIM_BLESS=1) after changing the scene");
    for (row, name) in committed.iter().zip(CASES) {
        assert_eq!(*row, case_projection(name), "case {name}");
    }
}

/// 🖨️ `BIM_BLESS=1` rewrites the committed vector the three.js oracle in `🟦️.ts` measures.
#[test]
fn bless_the_world_fixture() {
    if std::env::var_os("BIM_BLESS").is_none() {
        return;
    }
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖌️render/🧊️world/🧫️fixtures/🔣️.json");
    let cases: Vec<serde_json::Value> = CASES.iter().map(|name| case_projection(name)).collect();
    std::fs::write(path, serde_json::to_string_pretty(&serde_json::json!({ "cases": cases })).expect("fixture serialises") + "\n").expect("fixture is written");
}
