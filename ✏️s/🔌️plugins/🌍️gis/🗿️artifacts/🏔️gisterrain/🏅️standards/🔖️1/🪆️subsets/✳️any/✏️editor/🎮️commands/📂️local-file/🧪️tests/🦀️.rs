//! 🧪️ Checks the local file picker callback rules against the language-agnostic fixture that the Ajv cross-check also reads.

use super::*;
use semio_plugin::ArtifactEditor;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🎮️commands/📂️local-file/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()
}

fn callback(case: &Value) -> (Option<DslValue>, Option<String>) {
    let mut args = case["args"].clone();
    let repeated = case["repeat"].as_object().map(|repeat| repeat["text"].as_str().unwrap().repeat(repeat["count"].as_u64().unwrap() as usize));
    if let Some(text) = &repeated { args["payload"] = Value::String(text.clone()); }
    let dsl = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&args.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    (Some(dsl), repeated.or_else(|| args["payload"].as_str().map(str::to_owned)))
}

#[test]
fn every_callback_is_accepted_or_refused_as_the_fixture_states() {
    let fixture = fixture();
    let maximum = fixture["limits"]["geoJsonBytes"].as_u64().unwrap() as u32;
    assert_eq!(maximum, MAX_FILE_BYTES);
    for case in fixture["callbacks"].as_array().unwrap() {
        let (mut args, payload) = callback(case);
        let received = receive(&mut args, maximum);
        if case["accepted"].as_bool().unwrap() {
            assert_eq!(received, payload, "{}", case["name"]);
            assert_eq!(args.as_ref().and_then(|args| args.get("payload")).and_then(DslValue::as_str), Some(""), "the payload moves out of the callback: {}", case["name"]);
        } else {
            assert_eq!(received, None, "{}", case["name"]);
        }
        assert_eq!(valid_length(payload.as_deref().unwrap_or(""), maximum), payload.as_ref().is_some_and(|text| !text.is_empty() && text.len() <= maximum as usize), "{}", case["name"]);
    }
}

#[test]
fn the_file_actions_and_wire_envelope_are_the_fixture_limits() {
    let fixture = fixture();
    assert_eq!(FILE_ACTIONS.to_vec(), fixture["fileActions"].as_array().unwrap().iter().map(|action| action.as_str().unwrap()).collect::<Vec<_>>());
    assert_eq!(import_elevation::MAX_FILE_BYTES as u64, fixture["limits"]["elevationBytes"].as_u64().unwrap());
    assert_eq!(MAX_WIRE_BYTES as u64, fixture["limits"]["wireBytes"].as_u64().unwrap());
    let mut args = Some(DslValue::object([("payload".into(), DslValue::String("0".into()))]));
    let elevation = Gis3dPlayApp::command_from_action(import_elevation::FILE_ACTION, &mut args, &mut None).unwrap();
    assert_eq!(command_wire_maximum(&elevation), MAX_WIRE_BYTES);
    let mut args = Some(DslValue::object([("payload".into(), DslValue::String("{}".into()))]));
    let geojson = Gis3dPlayApp::command_from_action(geojson::FILE_ACTION, &mut args, &mut None).unwrap();
    assert_eq!(command_wire_maximum(&geojson), semio_plugin::RUNTIME_COMMAND_WIRE_MAXIMUM);
}
