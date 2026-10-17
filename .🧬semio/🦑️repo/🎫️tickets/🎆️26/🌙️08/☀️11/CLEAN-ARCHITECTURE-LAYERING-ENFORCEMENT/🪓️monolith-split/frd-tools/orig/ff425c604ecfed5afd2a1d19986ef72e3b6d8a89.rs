//! 📷️ Artifact-level framing shared by editor and viewer.
use super::*;
#[test]
fn drawing_scene_framing_fixtures() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/🧮️geometry/📷️framing/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let nodes: Vec<semio_s_artifact_draw_drawing_model::DrawingSceneNode> = semio_framework_pack_json::from_json_str(&case["nodes"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let artboard: Option<semio_s_artifact_draw_drawing_model::DrawingArtboard> = semio_framework_pack_json::from_json_str(&case["artboard"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let actual = drawing_scene_bounds(artboard.as_ref(),&nodes);
        for (index,value) in actual.iter().enumerate() { assert!((value-case["expected"][index].as_f64().unwrap()).abs()<1e-8,"{}: {actual:?}",case["name"]); }
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
