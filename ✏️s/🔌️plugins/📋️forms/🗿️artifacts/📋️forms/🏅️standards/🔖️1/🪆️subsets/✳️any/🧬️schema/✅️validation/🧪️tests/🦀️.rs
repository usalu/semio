#[test]
fn answer_validation_matches_shared_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️answers.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let question = dsl::json::from_json_str(&case["question"].to_string()).unwrap();
        let value = dsl::json::from_json_str(&case["value"].to_string()).unwrap();
        assert_eq!(super::answer_error(&question, &value), case["error"].as_str(), "{}", case["name"]);
    }
}
