use semio_framework_test_fixtures::fixture_text;
use serde_json::{Number, Value};
use semio_s_artifact_layout_layout_io::export::serializers::artifacts::json::v_rfc8259::any::serialize_text;

/// 🔢 Treats exactly representable integer and decimal tokens as the same JSON number.
fn normalize_exact_numbers(value: &mut Value) {
    match value {
        Value::Number(number) => {
            const MAX_EXACT_INTEGER: i64 = 1 << 53;
            if let Some(integer) = number.as_i64().filter(|integer| (-MAX_EXACT_INTEGER..=MAX_EXACT_INTEGER).contains(integer)) {
                *number = Number::from_f64(integer as f64).unwrap();
            }
        }
        Value::Array(values) => values.iter_mut().for_each(normalize_exact_numbers),
        Value::Object(values) => values.values_mut().for_each(normalize_exact_numbers),
        _ => {}
    }
}

#[test]
fn json_text_round_trip_preserves_the_language_neutral_snapshot() {
    let fixture = fixture_text!("../../../🧪️tests/🧫️fixtures/🧬️mutations/🔀reorder-pages/🔀️moves/📸️snapshot/⬅️before/🔣️.json");
    let snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire2_codec::parse_layout_document(fixture).unwrap();
    let actual = serialize_text(&snapshot).unwrap();
    let mut oracle: Value = serde_json::from_str(fixture).unwrap();
    let mut parsed: Value = serde_json::from_str(&actual).unwrap();
    normalize_exact_numbers(&mut oracle);
    normalize_exact_numbers(&mut parsed);
    for (key, value) in oracle.as_object().unwrap() {
        assert_eq!(&parsed[key], value, "field {key}");
    }
}
