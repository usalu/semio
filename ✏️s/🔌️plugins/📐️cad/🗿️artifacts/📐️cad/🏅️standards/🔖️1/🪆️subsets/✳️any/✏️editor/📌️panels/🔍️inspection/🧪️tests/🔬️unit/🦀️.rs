use super::*;
use crate::editor::cad::terminology::cad_labels;
use crate::editor::cad::unit_tests::context::*;
use crate::editor::cad::{forest_working_scene, CadPlayRuntime};
use crate::standards::v1::subsets::any::schema::inferences::CAD_EXAMPLE_FOREST_LEFT;
use crate::standards::v1::subsets::any::schema::inferences::{default_document, forest_play_scene, CAD_MODEL_DEFINITION_ENERGY};
use semio_framework_plugin::{Locale, Terminology, TreeWindows, ViewModel};

fn projected(panel: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree { root: panel }).expect("CAD panel projection")
}

fn forest_object_panel(view_state: &ViewModel, ids: &[&str]) -> String {
    let view = view_with_interaction(forest_play_scene(), CadPlayRuntime::default(), selecting(ids));
    projected(build_properties_panel(&view, cad_labels(view_state), None, &TreeWindows::unhosted()).expect("CAD properties panel assembly"))
}

#[semio_framework_async_macros::async_test]
async fn summary_counts_every_pane_object_without_a_selection() {
    let scene = forest_play_scene();
    let expected: usize = CadPaneId::all().into_iter().map(|pane| edit::cad_pane_working_scene(&scene, pane).map_or(0, |working| edit::cad_pane_working_objects(&working, pane).0.len())).sum();
    assert!(expected > 20, "the forest document lists objects in every pane");
    let json = projected(build_properties_panel(&view(scene, CadPlayRuntime::default()), cad_labels(&ViewModel::default()), Some("dislocate"), &TreeWindows::unhosted()).expect("summary"));
    assert!(json.contains(&format!("Objects: {expected}")), "{json}");
    assert!(json.contains("Utility: dislocate"));
    let empty = projected(build_properties_panel(&view(default_document(), CadPlayRuntime::default()), cad_labels(&ViewModel::default()), None, &TreeWindows::unhosted()).expect("summary"));
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
    let json = projected(build_properties_panel(&view(scene, runtime), cad_labels(&ViewModel::default()), None, &TreeWindows::unhosted()).expect("reference section"));
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
    let json = projected(build_properties_panel(&view(scene, runtime), cad_labels(&ViewModel::default()), None, &TreeWindows::unhosted()).expect("node section"));
    assert!(json.contains(&node.label), "{json}");
    assert!(json.contains("cad-play-inspector.node.id"));
}

//#region 🪟️WindowLaws
/// 🪟️ The selected-id list is a windowed section of its OWN, keyed by the RAW object id: a wide
/// selection stamps its whole extent, materialises exactly the host's slice, and never pushes the
/// object's own (author-fixed) field group out of the panel.
#[semio_framework_async_macros::async_test]
async fn a_wide_selection_windows_its_ids_by_raw_id_without_starving_the_field_group() {
    let working = forest_working_scene();
    let ids: Vec<&str> = working.building_objects.iter().map(|object| object.id.as_str()).collect();
    assert!(ids.len() >= 4, "this law needs a multi-object pane, found {}", ids.len());
    let (offset, rows) = (1u32, 2u32);
    let view_state = ViewModel {
        tree_windows: vec![semio_framework_plugin::TreeWindowRequest { body_key: CAD_PLAY_BODY_PROPERTIES.into(), node_key: IDS_SECTION.into(), open: Some(true), offset, rows }],
        ..ViewModel::default()
    };
    let view = view_with_interaction(forest_play_scene(), CadPlayRuntime::default(), selecting(&ids));
    let panel = build_properties_panel(&view, cad_labels(&ViewModel::default()), None, &TreeWindows::for_body(&view_state, CAD_PLAY_BODY_PROPERTIES)).expect("CAD properties panel assembly");

    let section = panel.children.iter().find(|child| child.key.as_str() == IDS_SECTION).expect("the ids section");
    let semio_framework_plugin::Component::TreeSection(props) = &section.component else { panic!("the ids block is a tree section") };
    let window = props.window.expect("the ids section stamps its window");
    assert_eq!(window.total as usize, ids.len(), "the ids section reports the WHOLE selection");
    assert_eq!(window.offset, offset, "and where the materialised slice starts");
    let keys: Vec<String> = section.children.iter().map(|row| row.key.as_str().to_string()).collect();
    let expected: Vec<String> = ids[offset as usize..(offset + rows) as usize].iter().map(|id| format!("cad-play-inspector.ids.{id}")).collect();
    assert_eq!(keys, expected, "exactly the requested slice, keyed by the RAW object id — never renumbered by the offset");

    // 🧾️ The object's own nine fields are an author-fixed group beside the window, not its tail.
    let fields = panel.children.iter().find(|child| child.key.as_str() == "cad-play-inspector.object").expect("the object field group");
    assert_eq!(fields.children.len(), 9, "the field group is complete however wide the selection is");
    let json = projected(panel);
    assert!(!json.contains(".more"), "no continuation row: {json}");
    assert!(!json.contains("\"+"), "no `+N` label: {json}");
}
//#endregion 🪟️WindowLaws

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
