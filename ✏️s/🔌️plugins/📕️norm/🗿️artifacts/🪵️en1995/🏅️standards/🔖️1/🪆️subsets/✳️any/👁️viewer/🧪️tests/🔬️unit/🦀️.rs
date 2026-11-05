use crate::*;

#[semio_framework_async_macros::async_test]
async fn create_en1995_viewer_builds_a_definition_for_this_dialect() {
    let def = create_en1995_viewer();
    assert_eq!(def.dialect, EN1995_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<En1995Viewer as ArtifactViewer>::DIALECT, EN1995_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let snapshot = En1995Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let json = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(
        <En1995Viewer as ArtifactViewer>::render("nope", &doc, &ConfigView { snapshot: &NoConfig::default(), window: None }, &semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).expect("viewer assembly"),
    )
    .expect("json");
    assert!(json.contains("Unknown body"));
}

#[test]
fn the_viewer_is_explicitly_read_only() {
    assert!(En1995Viewer::READ_ONLY);
    assert!(viewport::CONTROLLER.ends_with("#viewer"));
}

#[test]
fn a_viewer_steps_checks_and_combinations_through_its_ephemeral_lane_without_any_edit() {
    let snapshot = En1995Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let lane = crate::lanes::NormLaneTransient::default();
    let transient = semio_plugin::TransientView { snapshot: &lane, window: None };
    for action in ["viewportStepCheck", "viewportStepGoverning", "viewportStepCombination"] {
        let command = En1995ViewCommand::ViewportAct(viewport_act::ViewportAct { action: action.to_string(), text: None, number: Some(1.0) });
        assert!(command.dispatch(&doc, &cfg).expect("dispatch").config_mutations.is_empty(), "{action} never writes the config");
        assert!(semio_s_plugin_norm_app_surface::viewer_ephemeral::<En1995Viewer>(&command, &doc, &transient).1.len() <= 1, "{action} publishes at most one transient mutation");
        assert_eq!(<En1995Viewer as ArtifactViewer>::command_id(&command), "viewportAct");
    }
}

#[test]
fn the_viewer_maps_the_stepping_shortcuts_to_its_view_action() {
    assert!(<En1995Viewer as ArtifactViewer>::command_from_action("norm.shortcut.next-check", &mut None, &mut None).is_ok());
    assert!(<En1995Viewer as ArtifactViewer>::command_from_action("norm.unknown", &mut None, &mut None).is_err());
    assert!(En1995Viewer::READ_ONLY);
}
