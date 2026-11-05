use crate::viewer::forms::*;
use semio_plugin::ArtifactViewer;

#[test]
fn viewer_host_command_is_bounded_and_configuration_only() {
    use crate::forms_config::{FormsConfig, FormsConfigMutation};
    use semio_framework_value::DslValue;
    use protocol::{ColdMutation, Mutation};
    let args = DslValue::object([("json".into(), DslValue::String("[]".into()))]);
    let command = FormsViewer::command_from_action("setContributions", &mut Some(args), &mut None).unwrap();
    let wire = protocol::OpBinary::encode_op(&command).unwrap();
    let command = <FormsViewCommand as protocol::OpBinary>::decode_op(&wire).unwrap();
    assert_eq!(command.command_id(), "setContributions");
    let snapshot = crate::schema::empty_forms_snapshot();
    let history = semio_plugin::HistoryView::default();
    let doc = semio_plugin::ArtifactView::new(&snapshot, &history);
    let config = FormsConfig::default();
    let cfg = semio_plugin::ConfigView { snapshot: &config, window: None };
    let emit = command.dispatch(&doc, &cfg).unwrap();
    assert_eq!(emit.config_mutations.len(), 1);
    let FormsConfigMutation::SetContributions(_) = &emit.config_mutations[0] else { panic!("configuration projection only"); };
    let next: FormsConfig = protocol::apply_diff_cold(emit.config_mutations[0].diff(&config).diff(), &config).unwrap();
    assert_eq!(next.contributions_json, "[]");
    let definition = create_forms_viewer();
    assert_eq!(definition.commands.iter().filter(|row| row.id == "setContributions").count(), 1);
    let proofs = FormsViewer::bounded_first_step_tool_proofs();
    assert_eq!(proofs.len(), 1);
    assert_eq!(proofs[0].tool_id(), "setContributions");
}

#[test]
fn viewer_installs_and_mounts_every_contributed_question_kind_without_answer_authority() {
    use crate::forms_config::FormsConfig;
    use semio_framework_value::{DslValue, ToValue};
    use protocol::{ColdMutation, Mutation};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let metadata: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️extensions/🧪️tests/🧫️fixtures/🔣️contribution.json")).unwrap();
    for route in fixture["routes"].as_array().unwrap() {
        let mut question = fixture["question"].clone();
        question["kind"] = route["kind"].clone();
        let question = semio_framework_pack_json::from_json_str(&question.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let snapshot = crate::forms_snapshot_with_state(crate::FORMS_DOCUMENT_SCHEMA.into(), "readonly-forms".into(), "1".into(), None, &[crate::FormStep { id: "step".into(), title: "Questions".into(), description: None, blocks: vec![question] }]);
        let mut payload = metadata["payload"].clone();
        for key in ["questionKind", "paramsBodyKey", "previewBodyKey"] {
            payload[key] = route[if key == "questionKind" { "kind" } else { key }].clone();
        }
        let payload: crate::question_extensions::QuestionKindContribution = semio_framework_pack_json::from_json_str(&payload.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let contributions = vec![semio_framework::ProgramContributionEntry { plugin_id: "forms-module-procedural".into(), topic_contribution: Some(semio_plugin::TopicContribution::new("forms.questionKind", payload.to_value())) }];
        let json = semio_framework_pack_json::to_json_string(&contributions);
        let command = FormsViewer::command_from_action("setContributions", &mut Some(DslValue::object([("json".into(), DslValue::String(json))])), &mut None).unwrap();
        let base = FormsConfig::default();
        let history = semio_plugin::HistoryView::default();
        let doc = semio_plugin::ArtifactView::new(&snapshot, &history);
        let emit = command.dispatch(&doc, &semio_plugin::ConfigView { snapshot: &base, window: None }).unwrap();
        let config: FormsConfig = protocol::apply_diff_cold(emit.config_mutations[0].diff(&base).diff(), &base).unwrap();
        for locale in [semio_ui_locale::Locale::En, semio_ui_locale::Locale::De] {
            let view = semio_plugin::ViewModel::new(locale, semio_ui_locale::Terminology::Native);
            let tree = FormsViewer::render(crate::viewer::forms::modes::view::windows::try_wizard::BODY_KEY, &doc, &semio_plugin::ConfigView { snapshot: &config, window: None }, &view).unwrap();
            let json = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap();
            let value: serde_json::Value = serde_json::from_str(&json).unwrap();
            let mut slots = Vec::new();
            viewer_extension_payloads(&value, &mut slots);
            assert_eq!(slots.len(), 2);
            for (body, payload) in slots {
                assert!(body == route["paramsBodyKey"] || body == route["previewBodyKey"]);
                assert_eq!(payload, fixture["expected"]);
            }
        }
    }
}

fn viewer_extension_payloads(value: &serde_json::Value, slots: &mut Vec<(String, serde_json::Value)>) {
    if value["component"]["type"] == "extension" {
        let props = &value["component"]["props"];
        let source: String = props["paramsJsonChunks"].as_array().unwrap().iter().map(|chunk| chunk.as_str().unwrap()).collect();
        slots.push((props["bodyKey"].as_str().unwrap().into(), serde_json::from_str(&source).unwrap()));
    }
    if let Some(children) = value["children"].as_array() { for child in children { viewer_extension_payloads(child, slots); } }
}
