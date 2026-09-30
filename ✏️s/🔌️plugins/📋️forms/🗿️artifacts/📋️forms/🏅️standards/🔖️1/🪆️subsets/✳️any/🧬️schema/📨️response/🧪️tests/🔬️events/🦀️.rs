use crate::{FormMutation, FormsSnapshot};
use protocol::{Mutation, MutationDiff};

#[test]
fn response_events_match_shared_vectors_and_undo() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️events.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut before = FormsSnapshot::default();
        before.responses = dsl::json::from_json_str(&case["before"].to_string()).unwrap();
        let event: FormMutation = dsl::json::from_json_str(&case["event"].to_string()).unwrap();
        assert_eq!(<FormMutation as protocol::OpText>::parse_op(&protocol::OpText::print_op(&event)).unwrap(), event);
        assert_eq!(<FormMutation as protocol::OpBinary>::decode_op(&protocol::OpBinary::encode_op(&event).unwrap()).unwrap(), event);
        let outcome = event.diff(&before);
        if let Some(error) = case["error"].as_str() {
            assert!(outcome.messages().iter().any(|message| message.code.0 == error), "{}", case["name"]);
        } else {
            let after = outcome.diff().apply(&before).unwrap();
            let actual: serde_json::Value = serde_json::from_str(&dsl::os_pack::json::to_json_string(&after.responses)).unwrap();
            assert_eq!(actual, case["after"], "{}", case["name"]);
            let mut undone = after;
            for inverse in event.inverse(&before) { undone = inverse.diff(&undone).diff().apply(&undone).unwrap(); }
            assert_eq!(undone.responses, before.responses, "{}", case["name"]);
        }
    }
}

#[test]
fn submission_checks_every_step_and_keeps_visible_answers() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️submission.json")).unwrap();
    let definition = dsl::json::from_json_str(&fixture["definition"].to_string()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let values = dsl::json::from_json_str(&case["values"].to_string()).unwrap();
        let result = super::prepare_response(&definition, &values, "response-a".into(), 1790545740000, "revision-a".into());
        let actual = match result {
            Ok(response) => serde_json::json!({ "response": serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&response)).unwrap(), "errors": [] }),
            Err(errors) => serde_json::json!({ "response": null, "errors": serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&errors)).unwrap() }),
        };
        assert_eq!(actual, case["expected"], "{}", case["name"]);
    }
}

/// 🕰️ Every accepted timestamp survives JavaScript and native JSON decoding exactly.
#[test]
fn response_timestamps_match_portable_integer_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️timestamps.json")).unwrap();
    for case in vectors["cases"].as_array().unwrap() {
        let value = serde_json::json!({ "id": "response-a", "submittedAt": case["submittedAt"], "definitionVersion": "revision-a", "answers": [] });
        let decoded: Result<super::FormsResponse, _> = dsl::json::from_json_str(&value.to_string());
        let valid = decoded.is_ok_and(|response| response.validate().is_ok());
        let oracle = case["submittedAt"].as_u64().is_some_and(|timestamp| timestamp <= 9_007_199_254_740_991);
        assert_eq!(valid, case["valid"].as_bool().unwrap(), "{}", case["name"]);
        assert_eq!(valid, oracle, "independent JSON integer projection");
    }
}
