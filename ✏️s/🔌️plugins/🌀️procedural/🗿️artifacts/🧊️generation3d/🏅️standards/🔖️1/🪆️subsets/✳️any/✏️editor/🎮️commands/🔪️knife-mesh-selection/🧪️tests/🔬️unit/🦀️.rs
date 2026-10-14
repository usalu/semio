#[test]
fn knife_selection_action_declares_point_controls_and_the_editor_owner() {
    use semio_s_artifact_procedural_generation3d::manifest::create_generation3d_app;
    use semio_s_artifact_procedural_3d_editor::Generation3dBoundedCommandJobFactory;
    use semio_s_artifact_procedural_3d_mode_edit::windows::preview as edit_preview;
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let definition = create_generation3d_app();
    for window in &definition.window_kinds {
        let action = window.actions.iter().find(|action| action.id == "knifeMeshSelection");
        assert_eq!(action.is_some(), window.id == edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW);
        if let Some(action) = action {
            assert_eq!(action.args.iter().map(|arg| arg.id.as_str()).collect::<Vec<_>>(), ["start", "end"]);
            assert!(action.args.iter().all(|arg| arg.required && matches!(arg.schema, semio_framework::ArgSchema::Vector { dims: 3, .. })));
        }
    }
    let publication = Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == "knifeMeshSelection").unwrap();
    assert_eq!(publication.lanes, [semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Interaction]);
}
