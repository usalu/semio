use super::*;
use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::terminology::lowpoly_play_labels;
use crate::editor::lowpoly::unit_tests::context::render as render_body;
use crate::{LowpolySelection, LowpolySnapshot, LowpolyTransform};
use semio_framework_3d::mesh::HalfedgeMesh;
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_DEFAULT_ROWS};
use std::collections::HashMap;

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_document_tab_to_this_body_key() {
    let definition: PanelTabDefinition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    assert_eq!(definition.body_key.as_deref(), Some(LOWPOLY_PLAY_BODY_ARTIFACT));
}

#[semio_framework_async_macros::async_test]
async fn document_tree_lists_active_object() {
    let mut a = crate::editor::lowpoly::unit_tests::context::app().await;
    assert!(render_body(&mut a, LOWPOLY_PLAY_BODY_ARTIFACT).await.contains("lowpoly-document."));
}

//#region 🪟️WindowLaws
fn object(index: usize, mesh_json: Option<&str>) -> crate::LowpolyObject {
    let id = format!("obj-{index}");
    let mesh = mesh_json.map(|json| crate::mesh_child_handle(&id, json));
    crate::LowpolyObject { id, name: format!("Object {index}"), transform: LowpolyTransform::default(), smooth_shading: false, mesh, paint_layers: Vec::new(), mesh_content: String::new() }
}

/// 🪟️ A document of `objects` objects whose ACTIVE first one carries a several-hundred-element mesh (an
/// ico sphere at three subdivisions: 642 vertices, 1920 edges, 1280 faces) — the shape that used to fail
/// the whole panel with `ui.fixed-capacity` at the thirty-third vertex row.
///
/// 🧾️ Only `obj-0` is loaded into the compute session: `LowpolyDocument` materialises a 4 MiB paint
/// buffer per object it holds, and the panel reads a missing object's counts as zero anyway — which is
/// exactly the "many cheap siblings, one dense subject" shape these laws need.
fn oversized(objects: usize) -> (LowpolySnapshot, LowpolyConfig, LowpolyDocument) {
    let dense = HalfedgeMesh::ico_sphere_prim(1.0, 3).expect("ico sphere").to_json().expect("dense mesh json");
    let mut loaded = LowpolySnapshot::default();
    loaded.objects.push(object(0, Some(&dense)));
    let mut snapshot = LowpolySnapshot::default();
    snapshot.objects = (0..objects).map(|index| object(index, None)).collect();
    let workspace = HashMap::from([("obj-0".to_string(), dense)]);
    let config = LowpolyConfig { active_object_id: "obj-0".into(), ..Default::default() };
    let doc = LowpolyDocument::with_context(loaded, "obj-0".into(), LowpolySelection::default(), workspace).expect("compute session");
    (snapshot, config, doc)
}

/// 🪟️ The panel body exactly as the host reads it, for the host-known windows in `requests`.
fn window_body(snapshot: &LowpolySnapshot, config: &LowpolyConfig, doc: &LowpolyDocument, requests: Vec<TreeWindowRequest>) -> String {
    let view_state = ViewModel { tree_windows: requests, ..Default::default() };
    let labels = lowpoly_play_labels(&ViewModel::default());
    let node = render(LowpolyView { snapshot, config }, doc, labels, &TreeWindows::for_body(&view_state, LOWPOLY_PLAY_BODY_ARTIFACT)).expect("lowpoly document tree assembly");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("lowpoly document tree projection")
}

fn request(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> TreeWindowRequest {
    TreeWindowRequest { body_key: LOWPOLY_PLAY_BODY_ARTIFACT.into(), node_key: node_key.into(), open, offset, rows }
}

fn vertex_group_key() -> String {
    "lowpoly-document.obj-0.vertex.group".to_string()
}

/// 🪟️ Law (a): every container at ALL THREE levels stamps its full extent — the meshes section over the
/// objects, the active object row over its three groups, and the vertex group over its 642 raw ids —
/// while the first paint materialises about one viewport of rows and invents no `+N`.
#[test]
fn an_oversized_document_stamps_every_level_and_never_a_continuation_row() {
    let (snapshot, config, doc) = oversized(250);
    let vertices = doc.mesh_at(0).expect("the dense mesh").vertex_count();
    assert!(vertices >= 300, "the law (a) fixture must exercise the third level: {vertices} vertices");
    let json = window_body(
        &snapshot,
        &config,
        &doc,
        vec![request("lowpoly-play-document.meshes", Some(true), 0, 2), request("lowpoly-document.obj-0", Some(true), 0, 3), request(&vertex_group_key(), Some(true), 0, 12)],
    );
    assert!(json.contains("\"total\":250"), "the meshes section stamps the whole document: {json}");
    assert!(json.contains("\"total\":3"), "the open object row stamps its three component groups: {json}");
    assert!(json.contains(&format!("\"total\":{vertices}")), "the vertex group stamps every raw element id: {json}");
    assert!(!json.contains(".more"), "no continuation row survives: {json}");
    assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
    assert!(json.matches("\"key\":\"lowpoly-document.").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "a window materialises about one viewport: {json}");
    let cold = window_body(&snapshot, &config, &doc, Vec::new());
    assert!(cold.contains("\"total\":250"), "the first paint stamps the whole document too: {cold}");
    assert!(cold.matches("\"key\":\"lowpoly-document.").count() <= TREE_WINDOW_DEFAULT_ROWS as usize, "the first paint materialises about one viewport: {cold}");
}

/// 🪟️ Law (b): a container the host closed stamps its total and materialises nothing — the component
/// groups are authored closed, so an unopened one costs nothing but its own row.
#[test]
fn a_closed_container_stamps_its_total_and_materialises_no_children() {
    let (snapshot, config, doc) = oversized(4);
    let vertices = doc.mesh_at(0).expect("the dense mesh").vertex_count();
    let json = window_body(&snapshot, &config, &doc, vec![request(&vertex_group_key(), Some(false), 0, 0)]);
    assert!(json.contains(&format!("\"total\":{vertices}")), "a closed group still stamps its extent: {json}");
    assert!(!json.contains("\"key\":\"lowpoly-document.obj-0.vertex.0\""), "a closed group materialises no element row: {json}");
    let closed_section = window_body(&snapshot, &config, &doc, vec![request("lowpoly-play-document.meshes", Some(false), 0, 0)]);
    assert!(closed_section.contains("\"total\":4"), "a closed section still stamps its extent: {closed_section}");
    assert!(!closed_section.contains("\"key\":\"lowpoly-document.obj-0\""), "a closed section materialises no object row: {closed_section}");
}

/// 🪟️ Law (c): a host window materialises exactly `[offset, offset + rows)`, keyed by the RAW mesh
/// target id the interaction domain itself addresses.
#[test]
fn a_host_window_materialises_exactly_its_slice() {
    let (snapshot, config, doc) = oversized(4);
    let json = window_body(&snapshot, &config, &doc, vec![request(&vertex_group_key(), Some(true), 100, 10)]);
    assert!(json.contains("\"offset\":100"), "the group reports its offset: {json}");
    for id in 100..110 {
        assert!(json.contains(&format!("\"key\":\"lowpoly-document.obj-0.vertex.{id}\"")), "vertex {id} is inside the window: {json}");
    }
    assert!(!json.contains("\"key\":\"lowpoly-document.obj-0.vertex.99\""), "the row before the window stays out: {json}");
    assert!(!json.contains("\"key\":\"lowpoly-document.obj-0.vertex.110\""), "the row after the window stays out: {json}");
}

/// 🪟️ Law (d): the tree binds the "mesh" domain ONCE and every pick row carries only its granularity —
/// the per-row `interactionSelect` argument map is gone. A face row keeps its own `flipFaces` ROW
/// action, which is not a pick and therefore untouched by the domain migration.
#[test]
fn pick_rows_carry_granularity_while_the_tree_carries_the_one_interaction_select() {
    let (snapshot, config, doc) = oversized(2);
    let json = window_body(&snapshot, &config, &doc, vec![request("lowpoly-document.obj-0.face.group", Some(true), 0, 6)]);
    assert!(json.contains("\"interactionDomain\":\"mesh\""), "the tree binds the mesh domain: {json}");
    assert_eq!(json.matches("interactionSelect").count(), 1, "exactly one tree-level interactionSelect binding: {json}");
    assert!(json.contains("\"granularity\":\"object\""), "object rows are pick targets: {json}");
    assert!(json.contains("\"granularity\":\"face\""), "face rows are pick targets: {json}");
    assert!(json.contains("flipFaces"), "a face row keeps its own flip-normal row action: {json}");
}
//#endregion 🪟️WindowLaws
