use super::*;

#[test]
fn portable_layer_order_drives_topmost_hit_testing() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/👁️layer-order/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let document: LayoutSnapshot = semio_framework_pack_json::from_json_str(&case["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let hit = hit_test_page_frames(&document, &document.pages[0], case["hit"]["x"].as_f64().unwrap() as f32, case["hit"]["y"].as_f64().unwrap() as f32);
        assert_eq!(hit.as_deref(), case["hit"]["expected"].as_str(), "{}", case["name"]);
    }
}
