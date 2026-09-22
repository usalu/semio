//! 🧪️ The World3d preview: the mesh catalogue is per TILE (never per slot), instances carry the
//! slot's own box, and an unresolvable tile still gets a body.

use super::*;
use crate::editor::wfc3d::transient::solved_transient;
use serde_json::Value;

fn document() -> Wfc3dSnapshot {
    crate::examples::tower_stack::snapshot()
}

fn array(text: &str) -> Vec<Value> {
    serde_json::from_str::<Value>(text).expect("json").as_array().cloned().expect("array")
}

#[test]
fn the_window_declares_a_world3d_surface_and_authors_nothing() {
    let definition = definition();
    assert_eq!(definition.id, WFC_3D_PREVIEW_WINDOW);
    assert_eq!(definition.body_key, WFC_3D_PREVIEW_BODY);
    assert_eq!(definition.surface_kind, SurfaceKind::World3d);
    assert!(definition.actions.is_empty(), "a derived view authors nothing");
}

/// 🥽️ THE instancing contract: the catalogue is bounded by the TILE count (plus one placeholder), and
/// the instance lane by the SOLVED slot count. A catalogue that grew with the slots would defeat
/// instancing.
#[test]
fn the_mesh_catalogue_is_per_tile_and_the_instance_lane_is_per_solved_slot() {
    let document = document();
    let transient = solved_transient(&document);
    let meshes = array(&meshes_json(&document));
    let instances = array(&instances_json(&document, &transient));
    assert_eq!(meshes.len(), document.tiles.len() + 1, "one entry per tile plus the shared placeholder");
    assert_eq!(transient.assignments.len(), document.slots.len(), "this example solves completely");
    assert_eq!(instances.len(), transient.assignments.len());
}

#[test]
fn every_instance_references_a_mesh_the_catalogue_declares() {
    let document = document();
    let transient = solved_transient(&document);
    let ids: Vec<String> = array(&meshes_json(&document)).iter().map(|mesh| mesh["id"].as_str().unwrap_or_default().to_string()).collect();
    for instance in array(&instances_json(&document, &transient)) {
        let mesh = instance["meshId"].as_str().unwrap_or_default().to_string();
        assert!(ids.contains(&mesh), "instance {} references an undeclared mesh {mesh}", instance["id"]);
    }
}

/// 📐️ An instance is `position = slot origin`, `scale = slot extent`, which is what lets slots of
/// different sizes share one tile mesh.
#[test]
fn an_instance_carries_its_own_slots_box() {
    let document = document();
    let transient = solved_transient(&document);
    let instances = array(&instances_json(&document, &transient));
    let cantilever = instances.iter().find(|instance| instance["id"] == "cantilever").expect("the cantilever is placed");
    assert_eq!(cantilever["position"], serde_json::json!([1.5, 3.0, 0.0]));
    assert_eq!(cantilever["scale"], serde_json::json!([2.0, 1.0, 1.0]));
}

/// 🩺 An UNSATISFIABLE document draws NOTHING and says why. This window paints the SOLUTION, so a slot
/// the solve never reached is omitted from the lane rather than drawn as a placeholder box that would
/// read as a result.
#[test]
fn an_unsatisfiable_document_places_no_instances_and_says_so() {
    let mut document = document();
    document.rules.clear();
    let transient = solved_transient(&document);
    assert!(transient.contradiction, "clearing every rule leaves an allow-list that admits nothing");
    assert!(array(&instances_json(&document, &transient)).is_empty(), "an unsolved slot is omitted, never placeholder-drawn");
    let delta: Value = serde_json::from_str(&instances_delta_json(&document, &transient)).expect("delta json");
    assert_eq!(delta["count"].as_u64(), Some(0));
    let status: Value = serde_json::from_str(&status_json(&document, &transient)).expect("status json");
    assert!(status["message"].as_str().unwrap_or_default().contains("Contradiction"), "the verdict must be visible: {status}");
    render(&document, &transient, 1.0).expect("an unsatisfiable document still assembles a surface");
}

/// 📦️ A SOLVED slot whose tile media resolves to no geometry still gets a body — the one case the
/// shared placeholder mesh exists for.
#[test]
fn a_solved_slot_whose_tile_has_no_geometry_borrows_the_placeholder() {
    let mut document = document();
    for tile in &mut document.tiles {
        tile.media = TileMedia3d::Mesh { positions: Vec::new(), indices: Vec::new(), color: None };
    }
    let transient = solved_transient(&document);
    let instances = array(&instances_json(&document, &transient));
    assert_eq!(instances.len(), transient.assignments.len());
    assert!(instances.iter().all(|instance| instance["meshId"] == WFC_3D_PLACEHOLDER_MESH));
}

/// 🚚️ The delta lane rides ALONGSIDE the authoritative set and must describe the same records.
#[test]
fn the_delta_lane_agrees_with_the_authoritative_instance_set() {
    let document = document();
    let transient = solved_transient(&document);
    let delta: Value = serde_json::from_str(&instances_delta_json(&document, &transient)).expect("delta json");
    assert_eq!(delta["count"].as_u64().expect("count"), transient.assignments.len() as u64);
    assert_eq!(delta["changed"].as_array().expect("changed").len(), transient.assignments.len());
    assert!(delta["removed"].as_array().expect("removed").is_empty());
}

#[test]
fn the_camera_frames_what_the_document_actually_holds() {
    let camera: Value = serde_json::from_str(&camera_json(&document(), 1.0)).expect("camera json");
    assert!(camera.get("position").is_some() && camera.get("target").is_some(), "a world camera needs a pose: {camera}");
}

#[test]
fn the_status_line_says_what_the_solve_concluded() {
    let document = document();
    let status: Value = serde_json::from_str(&status_json(&document, &solved_transient(&document))).expect("status json");
    let message = status["message"].as_str().expect("a message");
    assert!(message.len() <= 256, "a window's owned text is capacity-bounded");
    assert!(!message.is_empty());
}

#[test]
fn the_window_renders_a_non_empty_surface_for_every_example() {
    for document in [crate::examples::two_room_corridor::snapshot(), crate::examples::wall_roof_facade_strip::snapshot(), crate::examples::tower_stack::snapshot()] {
        let transient = solved_transient(&document);
        let built = render(&document, &transient, 1.0).expect("the preview surface assembles");
        assert!(!format!("{built:?}").is_empty());
    }
}

/// 🥽️ Every catalogue entry must bind buffers the host can actually draw: one NORMAL per position
/// (a zero-length normal attribute at item size 3 killed every instanced draw with
/// `glDrawElements: Vertex buffer is not big enough`) and RGB — not RGBA — colours, because the host
/// binds `color` at item size 3.
#[test]
fn every_catalogue_mesh_binds_one_normal_and_one_rgb_colour_per_vertex() {
    for document in [crate::examples::two_room_corridor::snapshot(), crate::examples::wall_roof_facade_strip::snapshot(), crate::examples::tower_stack::snapshot()] {
        let meshes = array(&meshes_json(&document));
        assert!(!meshes.is_empty());
        for mesh in &meshes {
            let data = &mesh["data"];
            let positions = data["positions"].as_array().expect("positions").len();
            let normals = data["normals"].as_array().expect("normals").len();
            let colors = data["colors"].as_array().expect("colors").len();
            assert_eq!(normals, positions, "mesh {} binds one normal per position", mesh["id"]);
            assert!(colors == 0 || colors == positions, "mesh {} binds RGB colours, never RGBA", mesh["id"]);
            for normal in data["normals"].as_array().expect("normals").chunks(3) {
                let length: f64 = normal.iter().filter_map(Value::as_f64).map(|value| value * value).sum::<f64>().sqrt();
                assert!((length - 1.0).abs() < 1e-9, "every normal is unit length, saw {length}");
            }
        }
    }
}

/// 📐️ A single triangle in the XY plane, wound counter-clockwise, normals to `+Z`.
#[test]
fn vertex_normals_follow_the_triangle_winding_and_never_answer_zero() {
    let normals = vertex_normals(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], &[0, 1, 2]);
    assert_eq!(normals, vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0]);
    assert_eq!(vertex_normals(&[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 2.0, 0.0, 0.0], &[0, 1, 2]), vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], "a degenerate triangle falls back to +Z, never to a zero normal");
}


#[test]
fn a_partial_fill_preview_differs_from_empty_and_finished() {
    use crate::editor::wfc3d::modes::edit::tools::fill::Wfc3dFillTickPayload;
    use crate::inferences::solve_with_job;
    use std::collections::BTreeMap;

    let document = crate::examples::tower_stack::snapshot();
    let oracle = solve_with_job(&document).expect("tower-stack solves");
    let empty = instances_json(&document, &Wfc3dTransient::default());
    let finished_transient = Wfc3dTransient {
        assignments: oracle
            .assignments
            .iter()
            .map(|(slot_id, tile_id)| crate::editor::wfc3d::transient::Wfc3dAssignment { slot_id: slot_id.clone(), tile_id: tile_id.clone() })
            .collect(),
        contradiction: false,
    };
    let finished = instances_json(&document, &finished_transient);
    let mut assignments: BTreeMap<String, Option<String>> = document.slots.iter().map(|slot| (slot.id.clone(), None)).collect();
    let half = (oracle.assignments.len() / 2).max(1);
    for (index, (slot_id, tile_id)) in oracle.assignments.iter().enumerate() {
        if index >= half {
            break;
        }
        assignments.insert(slot_id.clone(), Some(tile_id.clone()));
    }
    let partial = Wfc3dFillTickPayload { assignments, contradiction: false, done: false };
    assert!(partial.decided_count() > 0 && partial.decided_count() < oracle.assignments.len());
    let mid = instances_json(&document, &partial.into_transient());
    assert_ne!(mid, empty, "a partial board must not look empty");
    assert_ne!(mid, finished, "a partial board must not look finished");
}
