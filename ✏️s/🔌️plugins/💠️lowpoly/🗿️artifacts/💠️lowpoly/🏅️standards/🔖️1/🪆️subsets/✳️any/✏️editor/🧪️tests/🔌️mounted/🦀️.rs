//! 🔌️ Mounted end-to-end journeys (ticket 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS, 2026-09-17):
//! a registry-backed, instance-bound app driven through the SAME string action lane the React shell uses
//! (`handle_action` → `command_from_action` → retained job → settle), so the arg bridge, the mesh-domain
//! selection, the persisted mesh content and undo are exercised together rather than per unit.

use super::*;
use crate::editor::lowpoly::unit_tests::context::{act, app_with_registry, LowpolyApp};

type Mounted = LowpolyApp;

/// 🔌️ The shared unit-test harness IS the mounted harness: registry-backed, instance-bound, settling
/// through the same loop, closing its stores on drop.
fn mounted() -> Mounted {
    semio_framework_plugin::resolve_ready(app_with_registry())
}

fn select(targets: &[(&str, &str)]) -> serde_json::Value {
    let targets: Vec<serde_json::Value> = targets.iter().map(|(granularity, id)| serde_json::json!({ "granularity": granularity, "id": id })).collect();
    serde_json::json!({ "domainId": MESH_INTERACTION_DOMAIN, "targets": serde_json::to_string(&targets).expect("targets"), "merge": "replace", "method": "pick" })
}

fn object(app: &Mounted, id: &str) -> LowpolyObject {
    app.0.snapshot().expect("snapshot").objects.iter().find(|object| object.id == id).cloned().unwrap_or_else(|| panic!("object {id} missing"))
}

fn face_count(object: &LowpolyObject) -> usize {
    semio_framework_3d::mesh::HalfedgeMesh::from_json(&object.mesh_content).expect("persisted mesh content parses").face_count()
}

/// ✂️ Face pick → extrude → undo → extrude again: geometry lives in the document, so undo restores the
/// boot mesh and the next edit still runs (it used to fail closed as `StaleMeshWorkspace`, a silent no-op).
/// The second extrude repeats the FIRST one exactly (same face, same distance) — the react playground
/// does that with the default distance, and `mesh_edit` used to diff the result against the scratch
/// transient's undone mesh, see no change and land nothing (2026-09-17).
#[semio_framework_async_macros::async_test]
async fn face_pick_extrude_undo_and_extrude_again() {
    let mut app = mounted();
    let before = object(&app, "obj-1");
    assert!(!before.mesh_content.is_empty(), "the boot mesh persists its mesh content");
    let boot_faces = face_count(&before);
    assert!(boot_faces > 6, "concrete forest boot mesh: {boot_faces}");
    act(&mut app, "interactionSelect", select(&[("face", "lowpoly-document.obj-1.face.0")])).await;
    act(&mut app, "extrude", serde_json::json!({ "extrudeDistance": 0.5 })).await;
    let extruded = object(&app, "obj-1");
    assert!(face_count(&extruded) > boot_faces, "extrude adds side faces: {}", face_count(&extruded));
    assert_ne!(extruded.mesh, before.mesh, "the mesh handle follows the content");
    act(&mut app, "undo", serde_json::json!({})).await;
    let undone = object(&app, "obj-1");
    assert_eq!(undone.mesh, before.mesh, "undo restores the prior handle");
    assert_eq!(face_count(&undone), boot_faces, "undo restores the prior geometry");
    act(&mut app, "extrude", serde_json::json!({ "extrudeDistance": 0.5 })).await;
    let again = object(&app, "obj-1");
    assert!(face_count(&again) > boot_faces, "a mesh edit after undo still applies");
    assert_eq!(again.mesh, extruded.mesh, "the repeated extrude lands the identical geometry again");
}

/// ➕️ A catalogue `addPrimitive`, an object pick on it and a gumball translate in the World3d host's own
/// argument shape (`mode` + string `ids`), then a host `setCamera` — none may be refused by the bridge.
#[semio_framework_async_macros::async_test]
async fn add_primitive_pick_translate_and_camera() {
    let mut app = mounted();
    act(&mut app, "addPrimitive", serde_json::json!({ "kind": "cylinder" })).await;
    let snapshot = app.0.snapshot().expect("snapshot");
    assert_eq!(snapshot.objects.len(), 2, "addPrimitive creates an object");
    let id = snapshot.objects[1].id.clone();
    let before = object(&app, &id);
    act(&mut app, "interactionSelect", select(&[("object", &crate::editor::lowpoly::view::document_object_row_id(&id))])).await;
    act(&mut app, "transformBegin", serde_json::json!({})).await;
    act(&mut app, "translateSelection", serde_json::json!({ "mode": "mesh", "ids": [crate::editor::lowpoly::view::document_object_row_id(&id)], "dx": 1.0, "dy": 0.0, "dz": 0.0 })).await;
    act(&mut app, "transformEnd", serde_json::json!({})).await;
    assert_ne!(object(&app, &id).mesh_content, before.mesh_content, "the picked object moved");
    assert_eq!(object(&app, "obj-1").mesh_content, object(&app, "obj-1").mesh_content);
    act(&mut app, "setCamera", serde_json::json!({ "windowId": "lowpoly-main", "camera": { "position": [3.0, 2.0, 5.0], "target": [0.0, 0.0, 0.0], "zoom": 1.0 } })).await;
}

/// 🧲️ The composable gumball's three handle groups, exactly as `World3dHost` dispatches a drag end:
/// `transformBegin`, ONE absolute delta in the host shape (`mode` + string `ids`, `rotateSelection`'s
/// axis + angle, `scaleSelection`'s factors), `transformEnd` — each an undoable document edit; and a
/// face-level translate moves only the picked face's vertices.
#[semio_framework_async_macros::async_test]
async fn gumball_rotate_scale_and_face_translate_land_through_the_host_shape() {
    let mut app = mounted();
    let before = object(&app, "obj-1");
    let row = crate::editor::lowpoly::view::document_object_row_id("obj-1");
    act(&mut app, "interactionSelect", select(&[("object", &row)])).await;
    act(&mut app, "transformBegin", serde_json::json!({})).await;
    act(&mut app, "rotateSelection", serde_json::json!({ "mode": "mesh", "ids": [row], "ax": 0.0, "ay": 1.0, "az": 0.0, "angle": 0.5 })).await;
    act(&mut app, "transformEnd", serde_json::json!({})).await;
    let rotated = object(&app, "obj-1");
    assert_ne!(rotated.mesh, before.mesh, "a gumball rotate lands");
    act(&mut app, "transformBegin", serde_json::json!({})).await;
    act(&mut app, "scaleSelection", serde_json::json!({ "mode": "mesh", "ids": [row], "sx": 2.0, "sy": 1.0, "sz": 1.0 })).await;
    act(&mut app, "transformEnd", serde_json::json!({})).await;
    let scaled = object(&app, "obj-1");
    assert_ne!(scaled.mesh, rotated.mesh, "a gumball scale lands");
    act(&mut app, "undo", serde_json::json!({})).await;
    assert_eq!(object(&app, "obj-1").mesh, rotated.mesh, "undo pops the scale as one edit");
    act(&mut app, "undo", serde_json::json!({})).await;
    assert_eq!(object(&app, "obj-1").mesh, before.mesh, "undo pops the rotate as one edit");
    // ✂️ A face pick, then the gumball's translate over that component selection.
    act(&mut app, "interactionSelect", select(&[("face", "lowpoly-document.obj-1.face.0")])).await;
    let mesh_before = semio_framework_3d::mesh::HalfedgeMesh::from_json(&object(&app, "obj-1").mesh_content).expect("mesh");
    act(&mut app, "transformBegin", serde_json::json!({})).await;
    act(&mut app, "translateSelection", serde_json::json!({ "mode": "face", "ids": [row, "lowpoly-document.obj-1.face.0"], "dx": 0.0, "dy": 0.0, "dz": 2.0 })).await;
    act(&mut app, "transformEnd", serde_json::json!({})).await;
    let mesh_after = semio_framework_3d::mesh::HalfedgeMesh::from_json(&object(&app, "obj-1").mesh_content).expect("mesh");
    assert_eq!(mesh_after.face_count(), mesh_before.face_count(), "a face translate keeps the topology");
    let moved = (0..mesh_before.vertex_count() as u32).filter(|index| {
        let a = mesh_before.vertex_position(semio_framework_3d::mesh::VertexId(*index)).expect("position");
        let b = mesh_after.vertex_position(semio_framework_3d::mesh::VertexId(*index)).expect("position");
        (a.z() - b.z()).abs() > 1e-4
    }).count();
    let face_vertices = mesh_before.face_vertex_ids(semio_framework_3d::mesh::FaceId(0)).expect("face 0").len();
    assert!(moved > 0 && moved <= face_vertices.max(4), "only the picked face's vertices move: {moved} of {} (face has {face_vertices})", mesh_before.vertex_count());
}

/// 📐️ Vertex and edge picks select through the same `<object>.<granularity>.<id>` targets, and the
/// Model window scene publishes the domain binding, the component ids and a gumball anchor.
#[semio_framework_async_macros::async_test]
async fn component_selection_reaches_the_model_scene() {
    let mut app = mounted();
    act(&mut app, "interactionSelect", select(&[("vertex", "lowpoly-document.obj-1.vertex.0"), ("vertex", "lowpoly-document.obj-1.vertex.1")])).await;
    act(&mut app, "snap", serde_json::json!({})).await;
    act(&mut app, "interactionSelect", select(&[("edge", "lowpoly-document.obj-1.edge.0")])).await;
    act(&mut app, "bevel", serde_json::json!({ "bevelAmount": 0.1 })).await;
    let domain_selection = protocol::DomainSelection { granularity: "face".into(), ids: vec!["lowpoly-document.obj-1.face.2".into()], ..Default::default() };
    let snapshot = app.0.snapshot().expect("snapshot");
    let config = LowpolyConfig::default();
    let world = crate::editor::lowpoly::view::world_selection_from_state(&snapshot, &config, &domain_selection, Some("face"));
    assert_eq!(world.component_ids, vec![2]);
    let scratch = LowpolyScratch::default();
    let loaded = crate::editor::lowpoly::view::build_doc(&snapshot, &config, &scratch).expect("document loads from persisted content");
    assert_eq!(world.granularity, "face");
    assert_eq!(world.active_object_id, "obj-1");
    edit::windows::model::render(LowpolyView { snapshot: &snapshot, config: &config }, Some(&loaded), "move", &HashMap::new(), &world).expect("the model scene renders from persisted content");
}

/// 🔬️ Diagnostic twin: a transient-only threaded command (`snap` with nothing selected) before the first
/// artifact-publishing one.
#[semio_framework_async_macros::async_test]
async fn warmed_extrude_commits() {
    let mut app = mounted();
    act(&mut app, "snap", serde_json::json!({})).await;
    act(&mut app, "interactionSelect", select(&[("face", "lowpoly-document.obj-1.face.0")])).await;
    act(&mut app, "extrude", serde_json::json!({ "extrudeDistance": 0.5 })).await;
    assert!(face_count(&object(&app, "obj-1")) > 6);
}

/// 🔬️ Diagnostic: a SMALL artifact mutation (rename) through the same retained route.
#[semio_framework_async_macros::async_test]
async fn small_artifact_mutation_commits() {
    let mut app = mounted();
    act(&mut app, "patchObject", serde_json::json!({ "objectId": "obj-1", "field": "name", "value": "Hull" })).await;
    assert_eq!(object(&app, "obj-1").name, "Hull");
}

/// 🔬️ Diagnostic: is the publication threshold the MUTATION's own size? A plane is a small mesh.
#[semio_framework_async_macros::async_test]
async fn add_plane_primitive_commits() {
    let mut app = mounted();
    act(&mut app, "addPrimitive", serde_json::json!({ "kind": "plane" })).await;
    assert_eq!(app.0.snapshot().expect("snapshot").objects.len(), 2);
}
