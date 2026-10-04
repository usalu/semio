//! 🫧️ Laws of the CAD world-window transient: its wire form is exactly the shape `🧬️schema/🔣️.json` declares, a window at
//! rest is idle, and every value survives the window-transient pack.

use super::*;
use store::ArtifactPack;

const SCHEMA_JSON: &str = include_str!("../../🧬️schema/🔣️.json");

fn wire(transient: &CadWorldWindowTransient) -> serde_json::Value {
    semio_framework_value::ToValue::to_value(transient).into()
}

/// ⚖️ LAW: the wire keys are exactly the schema's required properties, nullable fields carry `null` at rest, and a window
/// at rest shows `Idle`.
#[test]
fn the_wire_form_is_the_declared_schema() {
    let schema: serde_json::Value = serde_json::from_str(SCHEMA_JSON).expect("schema parses");
    let mut required: Vec<&str> = schema["required"].as_array().expect("required").iter().map(|key| key.as_str().expect("key")).collect();
    required.sort_unstable();
    let rest = wire(&CadWorldWindowTransient::default());
    let mut keys: Vec<&str> = rest.as_object().expect("object").keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, required, "the wire carries exactly the declared properties");
    assert_eq!(rest["engagementStep"], "Idle");
    for key in ["engagementPane", "engagementSessionJson", "lastFinalizedInteractionId"] {
        assert!(rest[key].is_null(), "{key} is null at rest");
        assert_eq!(schema["properties"][key]["type"], serde_json::json!(["string", "null"]), "{key} is declared nullable");
    }
}

/// ⚖️ LAW: a live engagement survives the pack byte-for-byte in value.
#[test]
fn a_live_engagement_round_trips_through_the_pack() {
    let live = CadWorldWindowTransient { engagement_input: "b".into(), engagement_step: "Pick first corner".into(), engagement_pane: Some("shape".into()), engagement_session_json: Some("{\"interactionId\":\"box\"}".into()), last_finalized_interaction_id: Some("box".into()) };
    let bytes = live.encode_pack();
    assert_eq!(CadWorldWindowTransient::decode_pack(&bytes).expect("decodes"), live);
}

/// ⚖️ LAW: every valid row of the language-agnostic contract fixture (which Ajv and the TypeScript twin also check) decodes
/// to a transient whose wire form is that row.
#[test]
fn every_valid_fixture_row_is_its_own_wire_form() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🧫️fixtures/🪪️document/🔣️.json")).expect("fixture parses");
    for row in fixture["valid"].as_array().expect("valid rows") {
        let decoded: CadWorldWindowTransient = semio_framework_value::FromValue::from_value(row.clone().into()).expect("a valid row decodes");
        assert_eq!(&wire(&decoded), row);
    }
}
