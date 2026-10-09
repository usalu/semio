use super::*;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::compute_element_solids;
use crate::{ModelInference, ModelSnapshot};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::attach::testing::{self, close};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use serde_json::{json, Value};

fn model(sweeps: Value, extra: Value) -> ModelSnapshot {
    let mut members = json!({ "wall_sweeps": sweeps });
    members.as_object_mut().expect("members object").extend(extra.as_object().expect("extra object").clone());
    testing::model(json!({ "w": testing::wall([0.0, 0.0, 8.0, 0.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({})) }), members)
}

fn baseboard(extra: Value) -> Value {
    let mut row = json!({ "host": "w", "side": "Left", "profile": { "Rectangle": { "width": 0.02, "depth": 0.1 } }, "height": 0.0, "inset": 0.0, "material": "paint", "name": "Baseboard" });
    row.as_object_mut().expect("a sweep object").extend(extra.as_object().expect("extra object").clone());
    row
}

fn door(offset: f64) -> Value {
    json!({ "host": "w", "kind": { "Door": { "door_type": "dt" } }, "offset": offset, "flip_hand": false, "flip_facing": false, "name": "" })
}

fn window(offset: f64) -> Value {
    json!({ "host": "w", "kind": { "Window": { "window_type": "wi" } }, "offset": offset, "flip_hand": false, "flip_facing": false, "name": "" })
}

#[semio_framework_async_macros::async_test]
async fn a_baseboard_stands_on_the_left_face_of_the_wall() {
    let snapshot = model(json!({ "ws": baseboard(json!({})) }), json!({}));
    let solid = &compute_element_solids(&snapshot)["ws"];
    assert!(close(solid.volume, 8.0 * 0.02 * 0.1), "volume {}", solid.volume);
    let (min, max) = (solid.bounds.min, solid.bounds.max);
    assert!(close(min.x, 0.0) && close(max.x, 8.0) && close(min.y, 0.15) && close(max.y, 0.17) && close(min.z, 0.0) && close(max.z, 0.1), "{:?}", solid.bounds);
    assert!(solid.mesh().is_watertight(), "a closed body");
    assert_eq!(solid.groups.len(), 1);
    assert_eq!(solid.groups[0].material, "paint");
}

#[semio_framework_async_macros::async_test]
async fn the_right_face_takes_the_sweep_outwards_on_the_other_side() {
    let snapshot = model(json!({ "ws": baseboard(json!({ "side": "Right" })) }), json!({}));
    let solid = &compute_element_solids(&snapshot)["ws"];
    assert!(close(solid.bounds.min.y, -0.17) && close(solid.bounds.max.y, -0.15), "{:?}", solid.bounds);
    assert!(close(solid.volume, 8.0 * 0.02 * 0.1), "{}", solid.volume);
}

#[semio_framework_async_macros::async_test]
async fn height_and_inset_place_the_profile() {
    let snapshot = model(json!({ "ws": baseboard(json!({ "profile": { "Rectangle": { "width": 0.04, "depth": 0.05 } }, "height": 0.9, "inset": 0.01 })) }), json!({}));
    let solid = &compute_element_solids(&snapshot)["ws"];
    assert!(close(solid.bounds.min.z, 0.9) && close(solid.bounds.max.z, 0.95), "{:?}", solid.bounds);
    assert!(close(solid.bounds.min.y, 0.14) && close(solid.bounds.max.y, 0.18), "the profile sinks 1 cm into the face: {:?}", solid.bounds);
}

#[semio_framework_async_macros::async_test]
async fn a_door_interrupts_a_baseboard_but_a_window_above_it_does_not() {
    let snapshot = model(json!({ "ws": baseboard(json!({})) }), json!({ "openings": { "o-door": door(2.0), "o-window": window(6.0) } }));
    let wall = &snapshot.walls["w"];
    let inference = ModelInference::infer(&snapshot).expect("the model infers");
    let layout = &inference.wall_layout["w"];
    let frames: Vec<&crate::standards::v1::subsets::any::schema::inferences::opening_frames::OpeningFrame> = inference.opening_frames.values().collect();
    let cuts = crate::standards::v1::subsets::any::schema::inferences::element_solids::walls::cuts_of(frames.iter().copied());
    assert_eq!(cuts.len(), 2, "both openings are valid cuts");
    let runs = runs(&snapshot.wall_sweeps["ws"], wall, layout, &cuts);
    assert_eq!(runs.len(), 2, "{runs:?}");
    assert!(close(runs[0].0, 0.0) && close(runs[0].1, 1.55) && close(runs[1].0, 2.45) && close(runs[1].1, 8.0), "{runs:?}");
    let quantity = &inference.quantities.elements["ws"];
    assert!(close(quantity.length, 1.55 + 5.55), "{}", quantity.length);
    assert!(close(inference.element_solids["ws"].volume, 0.002 * 7.1), "{}", inference.element_solids["ws"].volume);
}

#[semio_framework_async_macros::async_test]
async fn the_quantities_of_a_sweep_measure_its_profile_and_path() {
    let snapshot = model(json!({ "ws": baseboard(json!({})) }), json!({}));
    let inference = ModelInference::infer(&snapshot).expect("the model infers");
    let quantity = &inference.quantities.elements["ws"];
    assert_eq!(quantity.kind, crate::standards::v1::subsets::any::schema::inferences::quantities::QuantityKind::WallSweep);
    assert!(close(quantity.length, 8.0) && close(quantity.width, 0.02) && close(quantity.height, 0.1), "{quantity:?}");
    assert!(close(quantity.gross_area, 0.002) && close(quantity.gross_volume, 0.016) && close(quantity.net_volume, 0.016));
    assert!(close(quantity.perimeter, 0.14) && close(quantity.surface_area, 0.14 * 8.0) && close(quantity.net_area, 0.14 * 8.0), "{quantity:?}");
    assert!(close(quantity.mass, 0.016 * 1200.0) && quantity.layers.len() == 1 && quantity.layers[0].material == "paint");
    assert_eq!(quantity.storey, "st");
}

#[semio_framework_async_macros::async_test]
async fn a_sweep_on_a_sloped_base_follows_it_with_the_volume_of_the_straight_one() {
    let slab = json!({ "sl": { "storey": "st", "slab_type": "slt", "boundary": [{ "point": { "x": 0.0, "y": -5.0 }, "bulge": 0.0 }, { "point": { "x": 10.0, "y": -5.0 }, "bulge": 0.0 }, { "point": { "x": 10.0, "y": 5.0 }, "bulge": 0.0 }, { "point": { "x": 0.0, "y": 5.0 }, "bulge": 0.0 }], "holes": [], "offset": 0.0, "slope": { "direction": 0.0, "angle": 0.1 }, "phase": "New", "name": "" } });
    let mut snapshot = model(json!({ "ws": baseboard(json!({})) }), json!({ "slabs": slab }));
    snapshot.walls.get_mut("w").expect("the wall").base_slab = Some("sl".to_string());
    snapshot.walls.get_mut("w").expect("the wall").axis = crate::Axis::Line { start: crate::Point2 { x: 1.0, y: 0.0 }, end: crate::Point2 { x: 9.0, y: 0.0 } };
    let solid = &compute_element_solids(&snapshot)["ws"];
    let fall = 0.1f64.tan();
    assert!(close(solid.bounds.min.z, -9.0 * fall) && close(solid.bounds.max.z, -fall + 0.1), "{:?}", solid.bounds);
    assert!(close(solid.volume, 0.002 * 8.0), "a sheared prism keeps the volume of the straight one: {}", solid.volume);
    assert!(solid.mesh().is_watertight());
}

#[semio_framework_async_macros::async_test]
async fn a_sweep_whose_host_is_missing_has_no_solid() {
    let mut snapshot = model(json!({ "ws": baseboard(json!({ "host": "w-gone" })) }), json!({}));
    snapshot.walls.get_mut("w").expect("the wall").name = "kept".to_string();
    assert!(!compute_element_solids(&snapshot).contains_key("ws"));
    assert!(ModelInference::infer(&snapshot).expect("the model infers").quantities.elements.get("ws").is_none());
}

#[test]
fn the_section_is_counter_clockwise_and_starts_at_the_origin() {
    let sweep: WallSweep = from_json_str(&baseboard(json!({ "profile": { "Custom": { "outline": [{ "point": { "x": 0.0, "y": 0.0 }, "bulge": 0.0 }, { "point": { "x": 0.0, "y": 0.04 }, "bulge": 0.0 }, { "point": { "x": 0.03, "y": 0.0 }, "bulge": 0.0 }] } } })).to_string(), JsonMemberPolicy::Reject).expect("the sweep decodes");
    let section = section_of(&sweep);
    assert!(section_area(&sweep) > 0.0 && close(section_area(&sweep), 0.5 * 0.03 * 0.04));
    assert!(section.iter().map(|p| p.x).fold(f64::INFINITY, f64::min).abs() < 1e-12 && section.iter().map(|p| p.y).fold(f64::INFINITY, f64::min).abs() < 1e-12);
    assert!(close(extents_of(&sweep).0, 0.03) && close(extents_of(&sweep).1, 0.04));
}
