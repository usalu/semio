//! 🔢️ Native numeric refusal authority separates caller work bounds from malformed values.
use semio_framework_dsl_record::{FieldSpec, ParseOptions, RecordLayout, RecordSpec, Shape, parse_exact_controlled};
use semio_framework_value::NativeDecodeControl;

#[test]
fn controlled_numeric_token_bounds_preserve_distinct_work_and_value_refusals() {
    let corpus: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../🧪️tests/🧫️fixtures/🔢️number-refusal/🔣️.json")).unwrap();
    let spec = RecordSpec::new(Some("measure"), RecordLayout::Inline, vec![FieldSpec::new(1, "value", Shape::Float)]);
    for row in corpus["cases"].as_array().unwrap() {
        let fragment = row["fragment"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap() as usize);
        let text = format!("measure value={fragment}");
        let mut callback = |_| true;
        let mut control = NativeDecodeControl::new(1_048_576, &mut callback);
        let error = parse_exact_controlled(&text, &spec, &ParseOptions::default(), &mut control).unwrap_err();
        assert_eq!(serde_json::json!({"kind": error.kind.as_str(), "message": error.message}), row["expected"], "{}", row["id"]);
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
