use crate::FormExpr;
use semio_framework_pack_json::Value;
use crate::editor::forms::questions::visibility::*;

#[test]
fn visibility_edits_match_shared_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../✏️editor/❓️questions/🫥️visibility/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let before: Option<FormExpr> = semio_framework_pack_json::from_json_str(&case["before"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let value = semio_framework_pack_json::parse(&case["value"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let result = patch_condition(before.as_ref(), case["path"].as_str().unwrap(), case["field"].as_str().unwrap(), &value);
        if let Some(error) = case["error"].as_str() { assert_eq!(result.unwrap_err(), error, "{}", case["name"]); }
        else {
            let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&result.unwrap())).unwrap();
            assert_eq!(actual, case["after"], "{}", case["name"]);
        }
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
