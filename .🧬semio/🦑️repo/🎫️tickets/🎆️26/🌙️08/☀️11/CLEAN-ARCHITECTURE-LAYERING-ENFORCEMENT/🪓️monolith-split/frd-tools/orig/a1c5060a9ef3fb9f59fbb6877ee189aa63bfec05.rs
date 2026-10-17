use super::{sqlite_schema_bytes, sqlite_schema_len, sqlite_schema_text};

const SEGMENTS: &[&str] = &["CREATE TABLE a(x INTEGER);", "\n", "CREATE TABLE b(y TEXT);"];
const BYTES: [u8; sqlite_schema_len(SEGMENTS)] = sqlite_schema_bytes(SEGMENTS);
const SCHEMA: &str = sqlite_schema_text(&BYTES);

#[test]
fn const_composition_equals_the_ordered_concatenation() {
    assert_eq!(SCHEMA, SEGMENTS.concat());
}

#[test]
fn composition_matches_the_language_neutral_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../🧵️composition/🧫️fixtures/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let segments: Vec<&str> = case["segments"].as_array().unwrap().iter().map(|segment| segment.as_str().unwrap()).collect();
        let total = sqlite_schema_len(&segments);
        let mut composed = vec![0u8; total];
        let mut at = 0;
        for segment in &segments {
            composed[at..at + segment.len()].copy_from_slice(segment.as_bytes());
            at += segment.len();
        }
        assert_eq!(std::str::from_utf8(&composed).unwrap(), case["schema"].as_str().unwrap(), "{}", case["id"]);
        assert_eq!(total, case["schema"].as_str().unwrap().len(), "{}", case["id"]);
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
