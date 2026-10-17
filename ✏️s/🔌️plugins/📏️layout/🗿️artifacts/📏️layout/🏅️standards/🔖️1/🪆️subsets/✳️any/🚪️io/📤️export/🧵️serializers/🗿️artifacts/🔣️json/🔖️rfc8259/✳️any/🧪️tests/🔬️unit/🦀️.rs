use super::*;
#[test]
fn json_text_round_trip_preserves_the_language_neutral_snapshot() {
    let fixture = include_str!("../../../../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀reorder-pages/🔀️moves/📸️snapshot/⬅️before/🔣️.json");
    let snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire2_codec::parse_layout_document(fixture).unwrap();
    let actual = serialize_text(&snapshot).unwrap();
    let oracle: serde_json::Value = serde_json::from_str(fixture).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&actual).unwrap();
    for (key, value) in oracle.as_object().unwrap() {
        assert_eq!(&parsed[key], value, "field {key}");
    }
}
