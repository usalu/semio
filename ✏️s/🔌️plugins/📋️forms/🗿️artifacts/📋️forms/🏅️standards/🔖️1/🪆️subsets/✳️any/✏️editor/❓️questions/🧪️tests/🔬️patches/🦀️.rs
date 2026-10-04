use super::*;

#[test]
fn question_patches_match_shared_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️patches.json")).unwrap();
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️choices.json")).unwrap();
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
