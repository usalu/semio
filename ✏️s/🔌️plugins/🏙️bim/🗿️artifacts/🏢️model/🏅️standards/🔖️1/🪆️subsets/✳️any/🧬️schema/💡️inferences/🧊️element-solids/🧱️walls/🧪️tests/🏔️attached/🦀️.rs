use super::*;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::compute_element_solids;
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::attach::testing::{close, gable_roof, model, rect, wall};
use crate::ModelInference;
use serde_json::json;

fn gable_end(extra: serde_json::Value) -> ModelSnapshot {
    model(json!({ "w": wall([8.0, 0.0, 8.0, 6.0], json!({ "Roof": { "roof": "r", "offset": 0.0 } }), json!({})) }), {
        let mut members = json!({ "roofs": gable_roof() });
        members.as_object_mut().expect("members").extend(extra.as_object().expect("extra object").clone());
        members
    })
}

fn window(offset: f64, extra: serde_json::Value) -> serde_json::Value {
    let mut row = json!({ "host": "w", "kind": { "Window": { "window_type": "wi" } }, "offset": offset, "flip_hand": false, "flip_facing": false, "name": "" });
    row.as_object_mut().expect("opening object").extend(extra.as_object().expect("extra object").clone());
    json!({ "o": row })
}

fn group_area(solid: &ElementSolid, part: &str) -> f64 {
    let whole = solid.mesh();
    let mut mesh = TriMesh::new();
    for (triangle, owner) in solid.face_groups.iter().enumerate() {
        if solid.groups[*owner as usize].part == part {
            let [a, b, c] = whole.triangle(triangle);
            mesh.push_triangle(a, b, c);
        }
    }
    mesh.surface_area()
}

#[semio_framework_async_macros::async_test]
async fn a_wall_under_a_gable_roof_ends_in_the_slope_of_the_roof() {
    let snapshot = gable_end(json!({}));
    let solid = &compute_element_solids(&snapshot)["w"];
    let rise = 0.5f64.tan() * 3.0;
    assert!(close(solid.bounds.max.z, 3.0 + rise) && close(solid.bounds.min.z, 0.0), "{:?}", solid.bounds);
    assert!(close(solid.volume, 0.3 * (18.0 + 9.0 * 0.5f64.tan())), "volume {}", solid.volume);
    assert!(solid.mesh().is_watertight(), "the trimmed wall is a closed body");
    let layout = &ModelInference::infer(&snapshot).expect("infers").wall_layout["w"];
    assert!(close(solid.volume, layout.volume), "the solid {} and the layout {} are the same volume", solid.volume, layout.volume);
}

#[semio_framework_async_macros::async_test]
async fn a_window_in_a_trimmed_wall_is_cut_when_it_fits_under_the_slope_and_refused_when_it_does_not() {
    let fitting = gable_end(json!({ "openings": window(1.5, json!({})) }));
    let solid = &compute_element_solids(&fitting)["w"];
    assert!(close(solid.volume, 0.3 * (18.0 + 9.0 * 0.5f64.tan()) - 0.3 * 1.2 * 1.2), "volume {}", solid.volume);
    let high = gable_end(json!({ "openings": window(0.6, json!({ "sill_override": 2.5, "height": 0.9 })) }));
    let inference = ModelInference::infer(&high).expect("infers");
    assert!(!inference.opening_frames["o"].valid && inference.opening_frames["o"].issues.contains(&crate::standards::v1::subsets::any::schema::inferences::opening_frames::OpeningIssue::AboveHostTop), "{:?}", inference.opening_frames["o"].issues);
    assert!(close(inference.element_solids["w"].volume, 0.3 * (18.0 + 9.0 * 0.5f64.tan())), "an invalid frame cuts nothing");
}

#[semio_framework_async_macros::async_test]
async fn a_wall_standing_on_a_sloped_slab_follows_it_with_its_base() {
    let slab = json!({ "sl": { "storey": "st", "slab_type": "slt", "boundary": rect(0.0, -5.0, 10.0, 5.0), "holes": [], "offset": 0.0, "slope": { "direction": 0.0, "angle": 0.1 }, "phase": "New", "name": "" } });
    let snapshot = model(json!({ "w": wall([1.0, 0.0, 9.0, 0.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({ "base_slab": "sl" })) }), json!({ "slabs": slab }));
    let solid = &compute_element_solids(&snapshot)["w"];
    let fall = 0.1f64.tan();
    assert!(close(solid.bounds.min.z, -9.0 * fall) && close(solid.bounds.max.z, 3.0), "{:?}", solid.bounds);
    assert!(close(solid.volume, 0.3 * (3.0 * 8.0 + fall * 40.0)), "volume {}", solid.volume);
    assert!(solid.mesh().is_watertight());
}

#[semio_framework_async_macros::async_test]
async fn the_reveal_material_covers_the_jambs_inside_the_reveal_and_the_volume_stays() {
    let plain = model(json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({})) }), json!({ "openings": window(4.0, json!({})) }));
    let revealed = model(json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({})) }), json!({ "openings": window(4.0, json!({ "reveal_depth": 0.1, "reveal_material": "paint" })) }));
    let (before, after) = (&compute_element_solids(&plain)["w"], &compute_element_solids(&revealed)["w"]);
    assert!(close(before.volume, after.volume), "the reveal re-materials faces, it removes nothing: {} {}", before.volume, after.volume);
    assert!(after.area > before.area, "the layer is split at the reveal depth, so the interface adds two faces: {} {}", before.area, after.area);
    assert!(close(group_area(after, "reveal"), 2.0 * (1.2 + 1.2) * 0.1), "jambs, head and sill over 10 cm: {}", group_area(after, "reveal"));
    assert!(after.groups.iter().any(|group| group.part == "reveal" && group.material == "paint"));
    assert!(before.groups.iter().all(|group| group.part != "reveal"));
}

#[semio_framework_async_macros::async_test]
async fn a_reveal_on_the_right_face_and_a_full_depth_reveal() {
    let right = model(json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({})) }), json!({ "openings": window(4.0, json!({ "flip_facing": true, "reveal_depth": 0.1, "reveal_material": "paint" })) }));
    let solid = &compute_element_solids(&right)["w"];
    assert!(close(group_area(solid, "reveal"), 2.0 * (1.2 + 1.2) * 0.1), "{}", group_area(solid, "reveal"));
    let reveal_y: Vec<f64> = {
        let whole = solid.mesh();
        solid.face_groups.iter().enumerate().filter(|(_, owner)| solid.groups[**owner as usize].part == "reveal").flat_map(|(triangle, _)| whole.triangle(triangle)).map(|corner| corner[1]).collect()
    };
    assert!(reveal_y.iter().all(|y| *y <= -0.05 + 1e-9 && *y >= -0.15 - 1e-9), "the reveal of the right face lies between y = -0.15 and -0.05: {reveal_y:?}");
    let full = model(json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({})) }), json!({ "openings": window(4.0, json!({ "reveal_depth": 0.3, "reveal_material": "paint" })) }));
    assert!(close(group_area(&compute_element_solids(&full)["w"], "reveal"), 2.0 * (1.2 + 1.2) * 0.3));
}

#[semio_framework_async_macros::async_test]
async fn the_frame_of_a_window_sits_back_from_the_front_face_by_the_reveal_depth() {
    let centred = model(json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({})) }), json!({ "openings": window(4.0, json!({})) }));
    let set_back = model(json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({})) }), json!({ "openings": window(4.0, json!({ "reveal_depth": 0.1 })) }));
    let (middle, back) = (&compute_element_solids(&centred)["o"], &compute_element_solids(&set_back)["o"]);
    assert!(close(middle.bounds.min.y, -0.04) && close(middle.bounds.max.y, 0.04), "centred: {:?}", middle.bounds);
    assert!(close(back.bounds.max.y, 0.15 - 0.1) && close(back.bounds.min.y, 0.15 - 0.1 - 0.08), "front plane 10 cm behind the face at y = 0.15: {:?}", back.bounds);
    let frame = &ModelInference::infer(&set_back).expect("infers").opening_frames["o"];
    assert_eq!((frame.setback, frame.reveal_depth), (Some(0.1), 0.1));
}
