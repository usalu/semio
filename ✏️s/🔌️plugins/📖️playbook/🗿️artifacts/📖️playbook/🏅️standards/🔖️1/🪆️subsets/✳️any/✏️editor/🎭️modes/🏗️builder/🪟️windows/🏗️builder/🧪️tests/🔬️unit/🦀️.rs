use super::*;
use crate::editor::playbook::unit_tests::context::{live_spec, playbook_app};
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
            semio_framework_value::DslValue::object([
                ("blockKind".to_string(), semio_framework_value::DslValue::String("buildingComponent".to_string())),
                ("label".to_string(), semio_framework_value::DslValue::String("Building Component".to_string())),
                ("iconId".to_string(), semio_framework_value::DslValue::String("building".to_string())),
            ]),
        )),
    };
    config.contributions_json = semio_framework_pack_json::to_json_string(&vec![entry]);
    let palette = build_palette(&config);
    assert!(palette.iter().any(|entry| entry.block_kind == "buildingComponent"));
}

#[semio_framework_async_macros::async_test]
async fn render_builder_emits_playbook_list_component_scene() {
    let mut app = playbook_app().await;
    let tree = app.render(BODY_BUILDER, None, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("builder surface");
    let semio_framework_ui_contract::Component::Surface(props) = tree.root.component else { panic!("builder must render a semantic surface") };
    let scene: semio_framework_ui_scene::BlockListScene = semio_framework_ui_scene::decode(&props).expect("block-list payload");
    let expected = live_spec(&app).await;
    assert_eq!(scene.steps.len(),expected.steps.len());
    for (actual,expected) in scene.steps.iter().zip(&expected.steps) {
        assert_eq!((&actual.id,&actual.title,&actual.description),(&expected.id,&expected.title,&expected.description));
        assert_eq!(actual.blocks.len(),expected.blocks.len());
        for (actual,expected) in actual.blocks.iter().zip(&expected.blocks) { assert_eq!((&actual.id,&actual.label,&actual.kind,&actual.description),(&expected.id,&expected.label,&expected.kind,&expected.description)); }
    }
    assert_eq!(scene.palette.len(), PLAYBOOK_BUILTIN_KINDS.len());
}
