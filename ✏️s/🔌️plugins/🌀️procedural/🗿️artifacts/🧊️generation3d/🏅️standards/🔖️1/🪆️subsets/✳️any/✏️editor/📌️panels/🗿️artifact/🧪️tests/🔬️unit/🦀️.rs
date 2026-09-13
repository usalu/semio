use super::*;
use crate::editor::generation3d::unit_tests::context;
use crate::editor::generation3d::unit_tests::context::{app, render as render_body};

const DOCUMENT_ROWS_LAW: &str = include_str!("../../🧫️fixtures/🔬️unit/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn document_lists_widgets() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let rendered = render_body(&mut app, GENERATION_3D_PLAY_BODY_DOCUMENT).await;
    let fixture_widgets: Vec<String> = context::snapshot(&app).fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
    let first = fixture_widgets.first().expect("default fixture has at least one widget");
    assert!(rendered.contains(first), "document tree missing widget id {first}: {rendered}");
}

/// 🧾️ The law fixture's document tree, projected through the same retiring projection the flow
/// window's outline law uses, so both trees are read as one renderer-neutral shape.
fn law_document_projection() -> serde_json::Value {
    let law: serde_json::Value = serde_json::from_str(DOCUMENT_ROWS_LAW).expect("document rows law json");
    let fixture = semio_framework_os_flow::FlowHost::parse_fixture_json(&law["fixture"].to_string()).expect("law fixture parses");
    let labels = crate::editor::generation3d::terminology::generation3d_labels(&semio_framework_plugin::ViewModel::default());
    let tree = render(&fixture, labels).expect("document tree builds");
    fixture.retire_cold();
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).expect("document tree projects");
    serde_json::from_str(&projection).expect("document projection json")
}

/// ⏎️ LAW: every document row is a `graph`-domain pick target in its own right. A row that carries only
/// the tree's `interactionDomain` is inert on the React renderer — a real click reached the row and
/// nothing was invoked (ticket 26/09/09/PROCEDURAL-3D-END-TO-END), which also left the Inspection
/// panel permanently on its "no selection" branch.
#[test]
fn document_rows_bind_both_framework_interaction_verbs() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let law: serde_json::Value = serde_json::from_str(DOCUMENT_ROWS_LAW).expect("document rows law json");
    let projection = law_document_projection();
    assert_eq!(projection["component"]["interactionDomain"].as_str(), Some(GENERATION_3D_INTERACTION_DOMAIN));
    let sections = projection["children"].as_array().cloned().unwrap_or_default();
    assert_eq!(sections.len(), 1, "the document tree carries exactly one widgets section");
    let rows = sections[0]["children"].as_array().cloned().unwrap_or_default();
    let expected = law["expected"]["rows"].as_array().expect("law rows");
    assert_eq!(rows.len(), expected.len(), "document rows: {rows:?}");
    let granularity = law["granularity"].as_str().expect("law granularity");
    for (row, want) in rows.iter().zip(expected) {
        let id = want["id"].as_str().unwrap_or_default();
        assert_eq!(row["key"].as_str(), Some(id), "document row key");
        let bindings = row["bindings"].as_array().cloned().unwrap_or_default();
        let triggers: Vec<String> = bindings.iter().map(|binding| binding["trigger"].as_str().unwrap_or_default().to_string()).collect();
        let want_triggers: Vec<String> = want["triggers"].as_array().expect("law triggers").iter().map(|trigger| trigger.as_str().unwrap_or_default().to_string()).collect();
        assert_eq!(triggers, want_triggers, "row {id} interaction bindings");
        let select = bindings.iter().find(|binding| binding["trigger"] == "activate").expect("activate binding");
        let hover = bindings.iter().find(|binding| binding["trigger"] == "hoverPreview").expect("hoverPreview binding");
        assert!(select["action"].to_string().contains(semio_framework_plugin::INTERACTION_SELECT_ACTION_ID), "{select}");
        assert!(hover["action"].to_string().contains(semio_framework_plugin::INTERACTION_HOVER_ACTION_ID), "{hover}");
        let select_args = select["args"].to_string();
        assert!(select_args.contains(GENERATION_3D_INTERACTION_DOMAIN), "{select_args}");
        assert!(select_args.contains(&format!("\\\"granularity\\\":\\\"{granularity}\\\"")), "{select_args}");
        assert!(select_args.contains(id), "row {id} select targets: {select_args}");
        let hover_args = hover["args"].to_string();
        assert!(hover_args.contains(GENERATION_3D_INTERACTION_CHANNEL), "{hover_args}");
        assert!(hover_args.contains(id), "row {id} hover targets: {hover_args}");
    }
}
