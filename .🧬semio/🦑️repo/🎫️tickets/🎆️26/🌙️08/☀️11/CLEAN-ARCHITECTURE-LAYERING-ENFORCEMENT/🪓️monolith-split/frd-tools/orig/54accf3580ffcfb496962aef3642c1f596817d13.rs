#[test]
fn answer_validation_matches_shared_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../../🧪️tests/🧫️fixtures/✅️validation/🔣️answers.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let question = semio_framework_pack_json::from_json_str(&case["question"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let value = semio_framework_pack_json::from_json_str(&case["value"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(crate::standards::v1::subsets::any::schema::validation::answer_error(&question, &value), case["error"].as_str(), "{}", case["name"]);
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
