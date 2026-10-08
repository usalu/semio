use super::*;

#[semio_framework_async_macros::async_test]
async fn block_list_gestures_reach_mounted_document_commands() {
    use crate::editor::forms::{unit_tests::context, FormsCommand, FormsPlayApp};
    use semio_framework_plugin::{artifact_app_laws, ArtifactEditor, PluginApp, ViewModel, ViewWindowInstance};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️actions.json")).unwrap();
    let steps: Vec<crate::FormStep> = serde_json::from_value(fixture["initial"]["steps"].clone()).unwrap();
    let initial = crate::forms_snapshot_with_state(crate::FORMS_DOCUMENT_SCHEMA.into(), "gestures".into(), "1".into(), None, &steps);
    let mut app = context::forms_app().await;
    context::dispatch(&mut app, FormsCommand::SetSpecJson(crate::editor::forms::commands::set_spec_json::SetSpecJson {
        json: semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&initial)),
    })).await;
    let definition = crate::editor::forms::create_forms_app();
    let view = ViewModel {
        active_mode_id: Some(definition.default_mode_id.clone()),
        window_id: Some("design-test".into()),
        window_instances: vec![ViewWindowInstance { id: "design-test".into(), window_kind_id: FORMS_PLAY_WINDOW_BLUEPRINT.into() }],
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    for case in fixture["actions"].as_array().unwrap() {
        let action = case["action"].as_str().unwrap();
        let args = context::action_args(&case["args"]);
        let command = FormsPlayApp::command_from_action(action, Some(&args)).expect("scene gesture decodes");
        assert_eq!(command.command_id(), action, "scene and retained command must share one identity");
        let invocation = semio_framework::manifest::ActionInvocation {
            address: semio_framework::manifest::ActionAddress {
                plugin_id: "forms".into(),
                app_id: definition.id.clone(),
                mode_id: definition.default_mode_id.clone(),
                window_kind_id: FORMS_PLAY_WINDOW_BLUEPRINT.into(),
                window_instance_id: "design-test".into(),
                action_id: action.into(),
            },
            arguments: args.into_object().unwrap().into_iter().collect(),
        };
        let meta = semio_framework_plugin::ActionMeta { view_state: Some(view.clone()), ..artifact_app_laws::meta("local") };
        app.handle_action_invocation(&invocation, Some(&definition.default_mode_id), &meta).await.expect("manifest admits scene gesture");
        context::settle(&mut app).await;
        let snapshot = app.snapshot().unwrap();
        let projected = semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot.definition));
        let oracle = serde_json::json!({ "steps": snapshot.definition.steps });
        assert_eq!(serde_json::from_str::<serde_json::Value>(&projected).unwrap(), oracle, "independent JSON serializer");
        let kinds: Vec<Vec<&str>> = snapshot.definition.steps.iter().map(|step| step.blocks.iter().map(|question| question.kind.as_str()).collect()).collect();
        assert_eq!(serde_json::to_value(kinds).unwrap(), case["kinds"], "{action} durable effect");
    }
}

#[semio_framework_async_macros::async_test]
async fn renders_blueprint_builder_cards() {
    let spec = FormsSnapshot::default();
    let config = FormsConfig::default();
    let node = render(&spec, &config, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), None).expect("blueprint surface");
    let semio_framework_ui_contract::Component::Surface(props) = node.component else { panic!("blueprint must render a semantic surface") };
    let scene: semio_framework_ui_scene::BlockListScene = semio_framework_ui_scene::decode(&props).expect("block-list payload");
    let expected = crate::mutations::as_playbook_spec(&spec);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.steps_json).unwrap(), serde_json::to_value(&expected.steps).unwrap());
    assert!(scene.selected_id.is_none());
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_block_list_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.body_key, FORMS_PLAY_BODY_BLUEPRINT);
    assert!(matches!(definition.surface_kind, SurfaceKind::BlockList));
}

#[test]
fn blueprint_cards_publish_exact_forms_selection_targets() {
    let spec = crate::standards::v1::subsets::any::io::text::snapshot::default_example_spec();
    let node = render(&spec, &FormsConfig::default(), &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), Some("name")).unwrap();
    let semio_framework_ui_contract::Component::Surface(props) = node.component else { panic!("blueprint surface") };
    let scene: semio_framework_ui_scene::BlockListScene = semio_framework_ui_scene::decode(&props).unwrap();
    assert_eq!(scene.domain_id.as_deref(), Some(crate::editor::forms::FORMS_INTERACTION_FIELDS));
    let steps: serde_json::Value = serde_json::from_str(&scene.steps_json).unwrap();
    assert_eq!(steps[0]["target"], serde_json::json!({ "granularity": "section", "id": "step:contact" }));
    assert_eq!(steps[0]["blocks"][0]["target"], serde_json::json!({ "granularity": "field", "id": "name" }));
    assert_eq!(scene.selected_id.as_deref(), Some("name"));
}
