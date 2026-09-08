
use super::*;
use crate::CadPaneId;
use crate::editor::cad::config::CadConfig;
use crate::editor::cad::terminology::cad_labels;
use crate::editor::cad::testkit::*;
use crate::editor::cad::{CadPlayRuntime, make_object_for_typology};
use crate::standards::v1::subsets::any::schema::inferences::default_document;
use semio_framework_plugin::{ui_inspector_groups_to_tree, Locale, Terminology, ViewModel};
fn selected_box_panel(view_state: &ViewModel) -> String {
    let runtime = CadPlayRuntime::default();
    let panel = build_properties_panel(&view(default_document(), runtime), cad_labels(view_state), None).expect("CAD properties panel assembly");
    semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: panel }).expect("CAD panel projection")
}

#[semio_framework_async_macros::async_test]
async fn multi_selection_inspector_shows_mixed_values() {
    // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `build_properties_panel`
    // no longer resolves object selection into an inspector group (documented gap, see its own
    // doc comment — no live per-pane object list on `CadSnapshot`) — this exercises the real
    // `object_inspector_group` builder directly instead, the pure function the full render path
    // will call once resolved-child-content rendering exists.
    let mut first = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    let mut second = make_object_for_typology("spatial.shape.primitive.box", 1, CadPaneId::Shape);
    first.label = "Alpha".into();
    second.label = "Beta".into();
    first.orientation = Some([0.0, 0.0, 0.0, 1.0]);
    second.orientation = Some([0.0, 0.707, 0.0, 0.707]);
    let group = object_inspector_group(&[&first, &second], cad_labels(&ViewModel::default()));
    let json = protocol::json::to_json_string(&ui_inspector_groups_to_tree(&[group]));
    assert!(json.contains("Mixed"));
    assert!(json.contains("cad-play-inspector.object.orientation"));
}

// ⚠️ Pre-existing gap (predates this wave's app-layer pass, see `build_properties_panel`'s own
// doc comment): the full render path can no longer resolve object/primitive selection into an
// inspector group at all (no live per-pane object list on `CadSnapshot`), so `selected_box_panel`
// can no longer exercise `object_inspector_group`/`primitive_inspector_group`'s terminology
// labels — every test below that needs those groups now calls the real, still-working builder
// directly instead (same pattern `multi_selection_inspector_shows_mixed_values` already uses).
#[semio_framework_async_macros::async_test]
async fn cad_labels_resolve_native_by_default() {
    let object = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    let json = protocol::json::to_json_string(&ui_inspector_groups_to_tree(&[object_inspector_group(&[&object], cad_labels(&ViewModel::default()))]));
    assert!(json.contains("\"Object\""));
    assert!(!json.contains("Building component"));
}

#[semio_framework_async_macros::async_test]
async fn cad_labels_resolve_reuse_terminology_in_english() {
    let view_state = ViewModel { terminology: Terminology::Reuse, locale: Locale::En, ..ViewModel::default() };
    let json = selected_box_panel(&view_state);
    assert!(json.contains("Building component"));
    assert!(!json.contains("\"Object\""));
}

#[semio_framework_async_macros::async_test]
async fn cad_labels_resolve_reuse_terminology_in_german() {
    let view_state = ViewModel { terminology: Terminology::Reuse, locale: Locale::De, ..ViewModel::default() };
    assert!(selected_box_panel(&view_state).contains("Baukomponente"));
}

#[semio_framework_async_macros::async_test]
async fn cad_labels_resolve_native_terminology_in_german() {
    let view_state = ViewModel { terminology: Terminology::Native, locale: Locale::De, ..ViewModel::default() };
    let object = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    let json = protocol::json::to_json_string(&ui_inspector_groups_to_tree(&[object_inspector_group(&[&object], cad_labels(&view_state))]));
    assert!(json.contains("\"Objekt\""));
}

#[semio_framework_async_macros::async_test]
async fn cad_labels_resolve_reuse_terminology_for_primitive() {
    let view_state = ViewModel { terminology: Terminology::Reuse, locale: Locale::De, ..ViewModel::default() };
    let object = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
    let json = protocol::json::to_json_string(&ui_inspector_groups_to_tree(&[primitive_inspector_group(&object, cad_labels(&view_state), "box-solid", "solid")]));
    assert!(json.contains("Bauteil"));
}
