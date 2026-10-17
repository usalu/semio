use crate::FormQuestion;
use semio_framework_pack_json::Value;
use crate::editor::forms::questions::*;

#[test]
fn question_patches_match_shared_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../❓️questions/🧪️tests/🧫️fixtures/🔣️patches.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let before: FormQuestion = semio_framework_pack_json::from_json_str(&case["before"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let value = semio_framework_pack_json::parse(&case["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let result = patch_question(&before, case["field"].as_str().unwrap(), &value);
        if let Some(error) = case["error"].as_str() {
            assert_eq!(result.unwrap_err(), error, "{}", case["name"]);
        } else {
            let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&result.unwrap())).unwrap();
            let expected: FormQuestion = semio_framework_pack_json::from_json_str(&case["after"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            let expected: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&expected)).unwrap();
            assert_eq!(actual, expected, "{}", case["name"]);
        }
    }
}

#[test]
fn question_choices_match_shared_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../❓️questions/🧪️tests/🧫️fixtures/🔣️choices.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let before: FormQuestion = semio_framework_pack_json::from_json_str(&case["before"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let value = semio_framework_pack_json::parse(&case["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let result = patch_choice(&before, case["option"].as_str().unwrap(), case["field"].as_str().unwrap(), &value);
        if let Some(error) = case["error"].as_str() { assert_eq!(result.unwrap_err(), error, "{}", case["name"]); }
        else {
            let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&result.unwrap())).unwrap();
            let expected: FormQuestion = semio_framework_pack_json::from_json_str(&case["after"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            let expected: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&expected)).unwrap();
            assert_eq!(actual, expected, "{}", case["name"]);
        }
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
