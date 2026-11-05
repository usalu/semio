//! 🩹️ Public PatchQuestions admission and application laws share the neutral TypeScript vectors.
#![cfg(test)]

extern crate semio_os_kernel as protocol;
use semio_s_artifact_forms_forms_forms as forms;
use forms::editor::forms::{FormsCommand,FormsPlayApp,FORMS_PAGED_VALUE_RAW_BYTES};
use forms::editor::forms::commands::patch_questions;
use forms::forms_config::FormsConfig;
use forms::FORMS_DOCUMENT_SCHEMA;
use semio_plugin::{ArtifactEditor,ArtifactView,ConfigView,EditorApp};

#[test]
fn patch_question_complete_chunks_preserve_near_bound_authored_sources() {
    use protocol::{ColdMutation, Mutation};
    let vectors: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let decode = |input: &serde_json::Value| {
        let raw = semio_framework_pack_json::parse(&input.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let args = forms::standards::v1::subsets::any::io::text::snapshot::value_to_dsl(&raw);
        FormsPlayApp::command_from_action("patchQuestions", &mut Some(args), &mut None)
    };
    for row in vectors["cases"].as_array().unwrap() {
        let FormsCommand::PatchQuestions(payload) = decode(&row["input"]).unwrap() else { panic!("exact patch command"); };
        assert_eq!(serde_json::from_str::<serde_json::Value>(&payload.value_json().unwrap()).unwrap(), row["expected"]);
    }
    for row in vectors["invalid"].as_array().unwrap() { assert!(decode(row).is_err()); }
    let large: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/📄️value-source/🔣️.json")).unwrap();
    let boundary = &large["encodingBoundaries"];
    for row in boundary["cases"].as_array().unwrap() {
        let source = boundary["unit"].as_str().unwrap().repeat(boundary["repeat"].as_u64().unwrap() as usize) + row["suffix"].as_str().unwrap();
        let encoded = serde_json::to_string(&source).unwrap();
        assert_eq!(encoded.len(), row["encodedBytes"].as_u64().unwrap() as usize);
        let chunks = semio_ui_contract::UiValue::try_text_chunks(&encoded, 65_536).unwrap();
        let input = serde_json::json!({"questionIds":["q"],"field":"param","paramKey":"note","valueJsonChunks":serde_json::to_value(&chunks).unwrap()});
        let admitted = decode(&input);
        assert_eq!(admitted.is_ok(), row["accepted"].as_bool().unwrap());
        if let Ok(FormsCommand::PatchQuestions(payload)) = admitted { assert_eq!(serde_json::from_str::<String>(&payload.value_json().unwrap()).unwrap(), source); }
    }
    for row in large["largeSources"].as_array().unwrap() {
        let recipe = &row["source"];
        let source = recipe["prefix"].as_str().unwrap().to_owned() + &recipe["unit"].as_str().unwrap().repeat(recipe["repeat"].as_u64().unwrap() as usize) + recipe["suffix"].as_str().unwrap();
        assert_eq!(source.len(), row["rawBytes"].as_u64().unwrap() as usize);
        let encoded = serde_json::to_string(&source).unwrap();
        assert_eq!(encoded.len(), row["encodedBytes"].as_u64().unwrap() as usize);
        let chunks = semio_ui_contract::UiValue::try_text_chunks(&encoded, patch_questions::MAXIMUM_VALUE_BYTES).unwrap();
        let input = serde_json::json!({"questionIds":["q"],"field":"param","paramKey":"__flow","valueJsonChunks":serde_json::to_value(&chunks).unwrap()});
        let FormsCommand::PatchQuestions(payload) = decode(&input).unwrap() else { panic!("exact patch command"); };
        for chunk in &payload.value_json_chunks { assert!(chunk.chars().map(semio_framework::public_invocation_char_cost).sum::<usize>() <= semio_framework::PUBLIC_INVOCATION_STRING_BYTES); }
        let wire = serde_json::json!({"action":"patchQuestions","args":input}).to_string();
        assert!(wire.len() <= FORMS_PAGED_VALUE_RAW_BYTES);
        let mut question = forms::editor::forms::questions::question_shell("q".into(),"Flow".into(),"flow".into());
        question.params = Some(semio_framework_value::DslValue::Object(Vec::new()));
        let mut snapshot = forms::forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(),"flow-form".into(),"1".into(),None,&[semio_framework_artifact_playbook_playbook::PlaybookStep {id:"s".into(),title:"Flow".into(),description:None,blocks:vec![question]}]);
        let history = semio_plugin::HistoryView::default();
        let config = FormsConfig::default();
        let emit = patch_questions::handle(&payload,&ArtifactView::new(&snapshot,&history),&ConfigView {snapshot:&config,window:None}).unwrap();
        assert!(!emit.artifact_mutations.is_empty());
        for mutation in emit.artifact_mutations { snapshot = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).unwrap(); }
        let stored = forms::schema::locate_question(&snapshot,"q").unwrap().question.params.as_ref().unwrap();
        assert_eq!(stored.get("__flow").and_then(semio_framework_value::DslValue::as_str),Some(source.as_str()));
    }
}

