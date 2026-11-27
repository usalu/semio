//! 🧪️ Checks the Shell effect of `open-source` against the language-agnostic fixture that the Ajv and URL cross-check also reads.

use super::*;
use crate::editor::gis2d::{Gis2dCommand, Gis2dPlayApp};
use semio_plugin::ArtifactEditor;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()
}

fn features(rows: &Value) -> Vec<crate::MapFeature> {
    rows.as_array().unwrap().iter().map(|row| {
        let data = semio_framework_pack_json::parse(&row["data"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        crate::MapFeature { id: row["id"].as_str().unwrap().into(), data: semio_framework_pack_json::to_dsl_value(&data) }
    }).collect()
}

#[test]
fn only_a_position_with_a_textual_link_opens_its_source() {
    let fixture = fixture();
    let snapshot = GisMapSnapshot { positions: features(&fixture["positions"]), routes: features(&fixture["routes"]), ..Default::default() };
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    for case in fixture["cases"].as_array().unwrap() {
        let payload = open_source::OpenSource { feature_id: case["command"]["featureId"].as_str().unwrap().into() };
        let emit = open_source::handle(&payload, &doc, &cfg).unwrap();
        match case["url"].as_str() {
            Some(url) => assert!(matches!(emit.effects.as_slice(), [Effect::OpenExternalUrl { url: opened }] if opened == url), "{}", case["name"]),
            None => assert!(emit.effects.is_empty() && emit.artifact_mutations.is_empty(), "{}", case["name"]),
        }
    }
}

#[test]
fn the_action_arguments_address_the_payload_the_schema_describes() {
    let fixture = fixture();
    let decode = |json: &Value| {
        let mut args = Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&json.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()));
        match Gis2dPlayApp::command_from_action("openSource", &mut args, &mut None).unwrap() {
            Gis2dCommand::OpenSource(payload) => payload.feature_id,
            _ => panic!("openSource must produce OpenSource"),
        }
    };
    for case in fixture["cases"].as_array().unwrap() {
        assert_eq!(decode(&case["command"]), case["command"]["featureId"].as_str().unwrap(), "{}", case["name"]);
    }
    for invalid in fixture["invalidCommands"].as_array().unwrap() {
        assert!(decode(invalid).is_empty(), "{invalid}");
    }
}
