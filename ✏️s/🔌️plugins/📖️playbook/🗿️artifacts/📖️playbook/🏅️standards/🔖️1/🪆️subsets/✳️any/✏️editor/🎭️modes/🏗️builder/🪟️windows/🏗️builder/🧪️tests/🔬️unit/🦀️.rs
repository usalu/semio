use super::*;
use crate::editor::playbook::unit_tests::context::playbook_app;
use crate::editor::playbook::PLAYBOOK_PLAY_BODY_BUILDER as BODY_BUILDER;
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_block_list_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, PLAYBOOK_PLAY_BODY_BUILDER);
    assert!(matches!(definition.surface_kind, SurfaceKind::BlockList));
}

/// 🗂️ The open `playbook.blockKind` topic shape must surface the palette entry.
#[semio_framework_async_macros::async_test]
async fn render_builder_palette_includes_topic_contributed_block_kinds() {
    use crate::editor::playbook::config::PlaybookConfig;
    use semio_framework::{ProgramContributionEntry, TopicContribution};
    let mut config = PlaybookConfig::default();
    let entry = ProgramContributionEntry {
        plugin_id: "playbook-module-procedural".into(),
        topic_contribution: Some(TopicContribution::new(
            "playbook.blockKind",
            semio_framework_os_kernel::DslValue::object([
                ("blockKind".to_string(), semio_framework_os_kernel::DslValue::String("buildingComponent".to_string())),
                ("label".to_string(), semio_framework_os_kernel::DslValue::String("Building Component".to_string())),
                ("iconId".to_string(), semio_framework_os_kernel::DslValue::String("building".to_string())),
            ]),
        )),
    };
    config.contributions_json = protocol::json::to_json_string(&vec![entry]);
    let palette = build_palette(&config);
    assert!(palette.iter().any(|entry| entry.block_kind == "buildingComponent"));
}

#[semio_framework_async_macros::async_test]
async fn render_builder_emits_playbook_list_component_scene() {
    let mut app = playbook_app().await;
    let tree = app.render(BODY_BUILDER, None, &semio_framework_plugin::ViewModel::default()).await.expect("builder surface");
    let semio_framework_ui_contract::Component::Surface(props) = tree.root.component else { panic!("builder must render a semantic surface") };
    let scene: semio_framework_ui_scene::BlockListScene = semio_framework_ui_scene::decode(&props).expect("block-list payload");
    let expected = app.snapshot().expect("snapshot").as_kernel();
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.steps_json).unwrap(), serde_json::to_value(&expected.steps).unwrap());
    assert_eq!(serde_json::from_str::<Vec<serde_json::Value>>(&scene.palette_json).unwrap().len(), PLAYBOOK_BUILTIN_KINDS.len());
}
