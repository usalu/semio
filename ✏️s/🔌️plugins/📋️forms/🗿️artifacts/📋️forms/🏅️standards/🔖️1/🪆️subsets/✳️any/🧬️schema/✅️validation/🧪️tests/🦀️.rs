#[test]
fn answer_validation_matches_shared_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️answers.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let question = semio_framework_pack_json::from_json_str(&case["question"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let value = semio_framework_pack_json::from_json_str(&case["value"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(super::answer_error(&question, &value), case["error"].as_str(), "{}", case["name"]);
    }
}
