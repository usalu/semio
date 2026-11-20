use crate::*;

#[test]
fn portable_layer_order_drives_topmost_hit_testing() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/👁️layer-order/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let document: LayoutSnapshot = semio_framework_pack_json::from_json_str(&case["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let page = &document.pages[0];
        let hits = case["hits"].as_array().into_iter().flatten();
        for point in std::iter::once(&case["hit"]).chain(hits) {
            let hit = hit_test_page_frames(&document, page, point["x"].as_f64().unwrap() as f32, point["y"].as_f64().unwrap() as f32);
            assert_eq!(hit.as_deref(), point["expected"].as_str(), "{} at {point}", case["name"]);
        }
        if let Some(view) = case.get("view") {
            let list = build_interactive_display_list(&document, page, &page.id, &[], None, false, view["x"].as_f64().unwrap(), view["y"].as_f64().unwrap(), view["zoom"].as_f64().unwrap(), None);
            let expected: Vec<_> = view["expected"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect();
            assert_eq!(paint_order_ids(&list), expected, "{}", case["name"]);
        }
    }
}
