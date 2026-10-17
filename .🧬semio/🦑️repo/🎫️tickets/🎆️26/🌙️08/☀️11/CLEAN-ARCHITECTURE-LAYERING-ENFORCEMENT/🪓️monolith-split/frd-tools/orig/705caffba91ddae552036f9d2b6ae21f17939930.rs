use crate::FormQuestion;
use semio_framework_pack_json::Value;
use crate::editor::forms::questions::extensions::*;

#[test]
fn extension_render_inputs_match_shared_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../✏️editor/❓️questions/🧩️extensions/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for item in vectors["cases"].as_array().unwrap() {
        let question: FormQuestion = semio_framework_pack_json::from_json_str(&item["question"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let values = semio_framework_pack_json::parse(&item["values"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let target = &item["target"];
        let surface = if target["surface"] == "blueprint" {
            ExtensionSurface::Blueprint
        } else {
            ExtensionSurface::Try { window_id: target["windowId"].as_str().unwrap() }
        };
        let result = render_payload(&question, values.as_object().unwrap(), "forms-play", surface, item["interactive"].as_bool().unwrap());
        let actual: serde_json::Value = serde_json::from_str(&result.to_string()).unwrap();
        assert_eq!(actual, item["expected"], "{}", item["name"]);
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
