use super::*;

#[test]
fn question_drop_indices_match_shared_final_orders() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️drop.json")).unwrap();
    let definition: crate::schema::definition::FormsDefinition = dsl::json::from_json_str(&vectors["definition"].to_string()).unwrap();
    for case in vectors["cases"].as_array().unwrap() {
        let input = &case["input"];
        let result = question_insert_index(&definition, input["stepId"].as_str().unwrap(), input["targetId"].as_str().unwrap(), input["position"].as_str().unwrap(), input["movingId"].as_str());
        if let Some(error) = case["error"].as_str() { assert_eq!(result.unwrap_err(), error, "{}", case["name"]); }
        else {
            let index = result.unwrap();
            assert_eq!(index as u64, case["index"].as_u64().unwrap(), "{}", case["name"]);
            let mut order: Vec<String> = definition.steps.iter().find(|step| step.id == input["stepId"].as_str().unwrap()).unwrap().blocks.iter().map(|question| question.id.clone()).filter(|id| Some(id.as_str()) != input["movingId"].as_str()).collect();
            order.insert(index, input["movingId"].as_str().unwrap_or("new").into());
            assert_eq!(serde_json::to_value(order).unwrap(), case["order"], "{}", case["name"]);
        }
    }
}

#[test]
fn move_commands_apply_the_requested_sibling_order() {
    use crate::editor::forms::commands::move_question::{handle, MoveQuestion};
    use crate::editor::forms::config::FormsConfig;
    use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView};
    use protocol::Mutation;
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️drop.json")).unwrap();
    let definition: crate::schema::definition::FormsDefinition = dsl::json::from_json_str(&vectors["definition"].to_string()).unwrap();
    let history = HistoryView::empty();
    let config = FormsConfig::default();
    for case in vectors["cases"].as_array().unwrap() {
        let input = &case["input"];
        let Some(question_id) = input["movingId"].as_str() else { continue; };
        let mut snapshot = crate::forms_snapshot_with_state(crate::FORMS_DOCUMENT_SCHEMA.into(), "placement".into(), "1".into(), None, &definition.steps);
        let command = MoveQuestion { question_id: question_id.into(), to_step_id: input["stepId"].as_str().unwrap().into(), target_id: Some(input["targetId"].as_str().unwrap().into()), position: input["position"].as_str().unwrap().into(), index: None };
        let result = handle(&command, &ArtifactView::new(&snapshot, &history), &ConfigView { snapshot: &config, window: None });
        if case.get("error").is_some() { assert!(result.is_err(), "{}", case["name"]); }
        else {
            for mutation in result.unwrap().artifact_mutations { mutation.diff(&snapshot).apply_to(&mut snapshot); }
            let order: Vec<&str> = snapshot.definition.steps.iter().find(|step| step.id == command.to_step_id).unwrap().blocks.iter().map(|question| question.id.as_str()).collect();
            assert_eq!(serde_json::to_value(order).unwrap(), case["order"], "{}", case["name"]);
        }
    }
    println!("[DEBUG] Forms move commands preserved requested sibling order and rejected stale targets");
}

#[test]
fn question_placement_matches_shared_events() {
    use protocol::Mutation;
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for case in vectors["cases"].as_array().unwrap() {
        let input = &case["input"];
        let definition: crate::schema::definition::FormsDefinition = dsl::json::from_json_str(&input["definition"].to_string()).unwrap();
        let question: crate::FormQuestion = dsl::json::from_json_str(&input["question"].to_string()).unwrap();
        let result = create_question_event(&definition, question, input["stepId"].as_str(), input["newStepId"].as_str().unwrap());
        if let Some(error) = case["error"].as_str() { assert_eq!(result.unwrap_err(), error, "{}", case["name"]); }
        else {
            let expected: crate::FormMutation = dsl::json::from_json_str(&case["event"].to_string()).unwrap();
            assert_eq!(result.unwrap(), expected, "{}", case["name"]);
            let mut snapshot = crate::forms_snapshot_with_state(crate::FORMS_DOCUMENT_SCHEMA.into(), "placement".into(), "1".into(), None, &definition.steps);
            expected.diff(&snapshot).apply_to(&mut snapshot);
            assert_eq!(snapshot.definition.steps.iter().flat_map(|step| &step.blocks).filter(|question| question.id == "q").count(), 1);
        }
    }
}
