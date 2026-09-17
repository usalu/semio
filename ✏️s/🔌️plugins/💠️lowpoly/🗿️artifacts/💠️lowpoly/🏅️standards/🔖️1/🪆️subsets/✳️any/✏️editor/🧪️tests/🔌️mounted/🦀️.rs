//! 🔌️ Mounted end-to-end journeys (ticket 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS, 2026-09-17):
//! a registry-backed, instance-bound app driven through the SAME string action lane the React shell uses
//! (`handle_action` → `command_from_action` → retained job → settle), so the arg bridge, the mesh-domain
//! selection, the persisted mesh content and undo are exercised together rather than per unit.

use super::super::*;
use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry, settle_registered_typed_operation};
use semio_framework_plugin::{App, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

const INSTANCE: u32 = 1;

struct Mounted(VcsArtifactApp<EditorApp<LowpolyPlayApp>>);

impl Drop for Mounted {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            close_registered_fixture_app(&mut self.0);
        }
    }
}

fn manifest() -> App {
    App { definition: create_lowpoly_app(), examples: Vec::new() }
}

fn mounted() -> Mounted {
    let mut app = semio_framework_plugin::resolve_ready(new_app_with_registry::<EditorApp<LowpolyPlayApp>>(manifest));
    semio_framework_plugin::resolve_ready(app.bind_instance_id(INSTANCE));
    Mounted(app)
}

fn action_meta() -> semio_framework_plugin::ActionMeta {
    let id = edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN;
    let mut action = meta("local");
    action.view_state = Some(ViewModel { window_id: Some(id.into()), window_instances: vec![ViewWindowInstance { id: id.into(), window_kind_id: id.into() }], ..Default::default() });
    action
}

async fn act(app: &mut Mounted, action: &str, args: serde_json::Value) {
    let args = protocol::DslValue::from(&args);
    app.0.handle_action(action, Some(&args), &action_meta()).await.unwrap_or_else(|fault| panic!("{action} refused: {fault:?}"));
    settle_registered_typed_operation(&mut app.0, INSTANCE).await.unwrap_or_else(|fault| panic!("{action} did not settle: {fault:?}"));
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
/// cube and the next edit still runs (it used to fail closed as `StaleMeshWorkspace`, a silent no-op).
#[semio_framework_async_macros::async_test]
async fn face_pick_extrude_undo_and_extrude_again() {
    let mut app = mounted();
    let before = object(&app, "obj-1");
    assert!(!before.mesh_content.is_empty(), "the boot box persists its mesh content");
    assert_eq!(face_count(&before), 6);
    act(&mut app, "interactionSelect", select(&[("face", "lowpoly-document.obj-1.face.0")])).await;
    act(&mut app, "extrude", serde_json::json!({ "extrudeDistance": 0.5 })).await;
    let extruded = object(&app, "obj-1");
    assert!(face_count(&extruded) > 6, "extrude adds side faces: {}", face_count(&extruded));
    assert_ne!(extruded.mesh, before.mesh, "the mesh handle follows the content");
    act(&mut app, "undo", serde_json::json!({})).await;
    let undone = object(&app, "obj-1");
    assert_eq!(undone.mesh, before.mesh, "undo restores the prior handle");
    assert_eq!(face_count(&undone), 6, "undo restores the prior geometry");
    act(&mut app, "extrude", serde_json::json!({ "extrudeDistance": 0.25 })).await;
    assert!(face_count(&object(&app, "obj-1")) > 6, "a mesh edit after undo still applies");
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
    let node = edit::windows::model::render(LowpolyView { snapshot: &snapshot, config: &config }, Some(&loaded), "move", &HashMap::new(), &world).expect("render");
    let json = serde_json::to_string(&semio_framework_plugin::built_to_component_tree(node).root).expect("tree json");
    for needle in ["componentIds", "gumballTarget", "lowpoly-document.obj-1"] {
        assert!(json.contains(needle), "model scene lacks {needle}");
    }
}
