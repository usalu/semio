use semio_framework_test_fixtures::fixture_text;

#[test]
fn answer_validation_matches_shared_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/✅️validation/🔣️answers.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let question = semio_framework_pack_json::from_json_str(&case["question"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let value = semio_framework_pack_json::from_json_str(&case["value"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(crate::standards::v1::subsets::any::schema::validation::answer_error(&question, &value), case["error"].as_str(), "{}", case["name"]);
    }
}
