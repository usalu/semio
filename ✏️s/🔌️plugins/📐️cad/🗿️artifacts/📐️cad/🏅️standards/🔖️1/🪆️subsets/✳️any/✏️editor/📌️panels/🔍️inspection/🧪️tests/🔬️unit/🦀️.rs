use super::*;
use crate::editor::cad::terminology::cad_labels;
use crate::editor::cad::unit_tests::context::*;
use crate::editor::cad::{forest_working_scene, CadPlayRuntime};
use crate::standards::v1::subsets::any::schema::inferences::CAD_EXAMPLE_FOREST_LEFT;
use crate::standards::v1::subsets::any::schema::inferences::{default_document, forest_play_scene, CAD_MODEL_DEFINITION_ENERGY};
use semio_framework_plugin::{Locale, Terminology, ViewModel};

fn projected(panel: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: panel }).expect("CAD panel projection")
}

fn forest_object_panel(view_state: &ViewModel, ids: &[&str]) -> String {
    let view = view_with_interaction(forest_play_scene(), CadPlayRuntime::default(), selecting(ids));
    projected(build_properties_panel(&view, cad_labels(view_state), None).expect("CAD properties panel assembly"))
}

#[semio_framework_async_macros::async_test]
async fn summary_counts_every_pane_object_without_a_selection() {
    let scene = forest_play_scene();
    let expected: usize = CadPaneId::all().into_iter().map(|pane| edit::cad_pane_working_scene(&scene, pane).map_or(0, |working| edit::cad_pane_working_objects(&working, pane).0.len())).sum();
    assert!(expected > 20, "the forest document lists objects in every pane");
    let json = projected(build_properties_panel(&view(scene, CadPlayRuntime::default()), cad_labels(&ViewModel::default()), Some("dislocate")).expect("summary"));
    assert!(json.contains(&format!("Objects: {expected}")), "{json}");
    assert!(json.contains("Utility: dislocate"));
    let empty = projected(build_properties_panel(&view(default_document(), CadPlayRuntime::default()), cad_labels(&ViewModel::default()), None).expect("summary"));
    assert!(empty.contains("Objects: 0"));
}

#[semio_framework_async_macros::async_test]
async fn live_object_selection_renders_its_fields() {
    let working = forest_working_scene();
    let object = &working.building_objects[0];
    let json = forest_object_panel(&ViewModel::default(), &[object.id.as_str()]);
    assert!(json.contains(&object.id), "{json}");
    assert!(json.contains(&object.label));
    assert!(json.contains("cad-play-inspector.object.origin"));
    assert!(json.contains("cad-play-inspector.object.primitives"));
    assert!(json.contains("building"), "the pane row names the object's home pane");
    assert!(!json.contains("Objects:"), "a live selection replaces the summary");
}

#[semio_framework_async_macros::async_test]
async fn multi_selection_inspector_titles_the_count() {
    let working = forest_working_scene();
    let ids: Vec<&str> = working.building_objects.iter().take(3).map(|object| object.id.as_str()).collect();
    let json = forest_object_panel(&ViewModel::default(), &ids);
    assert!(json.contains("3 Objects"), "{json}");
    for id in ids {
        assert!(json.contains(id));
    }
}

#[semio_framework_async_macros::async_test]
async fn unknown_selection_ids_fall_back_to_the_summary() {
    let json = forest_object_panel(&ViewModel::default(), &["object-that-was-deleted"]);
    assert!(json.contains("Objects:"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn selected_reference_renders_fields_and_bounded_patch_rows() {
    let scene = forest_play_scene();
    let reference = scene.references_by_model_definition_id.get(CAD_MODEL_DEFINITION_ENERGY).and_then(|references| references.first()).expect("energy reference").clone();
    let runtime = CadPlayRuntime { selected_reference_model_definition_id: Some(CAD_MODEL_DEFINITION_ENERGY.into()), selected_reference_id: Some(reference.id.clone()), ..CadPlayRuntime::default() };
    assert_eq!(scene.id, CAD_EXAMPLE_FOREST_LEFT);
    let json = projected(build_properties_panel(&view(scene, runtime), cad_labels(&ViewModel::default()), None).expect("reference section"));
    assert!(json.contains(&reference.source_url), "{json}");
    assert!(json.contains("cad-play-inspector.reference.width.grow"));
    assert!(json.contains("cad-play-inspector.reference.origin.x.plus"));
    assert!(json.contains("patchCadPlayReference"));
    assert!(json.contains(if reference.locked { "Unlock" } else { "Lock" }));
}

#[semio_framework_async_macros::async_test]
async fn selected_node_renders_read_only_rows() {
    let scene = forest_play_scene();
    let node = scene.nodes.first().expect("forest node").clone();
    let runtime = CadPlayRuntime { selected_node_ids: vec![node.id.clone()], ..CadPlayRuntime::default() };
    let json = projected(build_properties_panel(&view(scene, runtime), cad_labels(&ViewModel::default()), None).expect("node section"));
    assert!(json.contains(&node.label), "{json}");
    assert!(json.contains("cad-play-inspector.node.id"));
}

#[semio_framework_async_macros::async_test]
async fn cad_labels_resolve_native_by_default() {
    let working = forest_working_scene();
    let json = forest_object_panel(&ViewModel::default(), &[working.objects[0].id.as_str()]);
    assert!(json.contains("\"Object\""), "{json}");
    assert!(!json.contains("Building component"));
}

#[semio_framework_async_macros::async_test]
async fn cad_labels_resolve_reuse_terminology_in_english() {
    let working = forest_working_scene();
    let view_state = ViewModel { terminology: Terminology::Reuse, locale: Locale::En, ..ViewModel::default() };
    let json = forest_object_panel(&view_state, &[working.objects[0].id.as_str()]);
    assert!(json.contains("Building component"), "{json}");
    assert!(!json.contains("\"Object\""));
}

#[semio_framework_async_macros::async_test]
async fn cad_labels_resolve_reuse_terminology_in_german() {
    let working = forest_working_scene();
    let view_state = ViewModel { terminology: Terminology::Reuse, locale: Locale::De, ..ViewModel::default() };
    assert!(forest_object_panel(&view_state, &[working.objects[0].id.as_str()]).contains("Baukomponente"));
}

#[semio_framework_async_macros::async_test]
async fn cad_labels_resolve_native_terminology_in_german() {
    let working = forest_working_scene();
    let view_state = ViewModel { terminology: Terminology::Native, locale: Locale::De, ..ViewModel::default() };
    let json = forest_object_panel(&view_state, &[working.objects[0].id.as_str()]);
    assert!(json.contains("\"Objekt\""), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn cad_labels_resolve_reuse_terminology_for_primitive() {
    let working = forest_working_scene();
    let view_state = ViewModel { terminology: Terminology::Reuse, locale: Locale::De, ..ViewModel::default() };
    let json = forest_object_panel(&view_state, &[working.objects[0].id.as_str()]);
    assert!(json.contains("Bauteil"), "{json}");
}
