use crate::schema::response::FormsResponse;
use crate::standards::v1::subsets::any::io::text::snapshot::response::export::*;

#[test]
fn exports_match_shared_vectors_and_independent_csv() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../../🚪️io/📝️text/📸️snapshot/📨️response/📤️export/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for test in fixture["cases"].as_array().unwrap() {
        let responses: Vec<FormsResponse> = semio_framework_pack_json::from_json_str(&test["responses"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let expected: Vec<Vec<String>> = serde_json::from_value(test["rows"].clone()).unwrap();
        assert_eq!(response_rows(&responses), expected, "{}", test["name"]);
        let csv = export_responses_csv(&responses);
        assert_eq!(csv, test["csv"].as_str().unwrap(), "{}", test["name"]);
        let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(csv.as_bytes());
        let parsed: Vec<Vec<String>> = reader.records().map(|row| row.unwrap().iter().map(str::to_owned).collect()).collect();
        assert_eq!(parsed, expected, "{}", test["name"]);
        let json: serde_json::Value = serde_json::from_str(&export_responses_json(&responses)).unwrap();
        assert_eq!(json, test["responses"], "{}", test["name"]);
        for format in ["json", "csv"] {
            let mut work = ResponseExport::new(format).unwrap();
            let mut turns = 0;
            let result = loop {
                turns += 1;
                if let Some(result) = work.advance(&responses) { break result; }
                assert!(turns < 100, "the shared fixture must finish");
            };
            assert!(turns > responses.len(), "work yields between individual answer records");
            if format == "csv" { assert_eq!(result, test["csv"].as_str().unwrap()); }
            else { assert_eq!(serde_json::from_str::<serde_json::Value>(&result).unwrap(), test["responses"]); }
            assert!(work.is_empty());
            let mut cancelled = ResponseExport::new(format).unwrap();
            cancelled.advance(&responses);
            cancelled.cancel();
            assert!(cancelled.is_empty());
            assert!(cancelled.advance(&responses).is_none());
        }
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
