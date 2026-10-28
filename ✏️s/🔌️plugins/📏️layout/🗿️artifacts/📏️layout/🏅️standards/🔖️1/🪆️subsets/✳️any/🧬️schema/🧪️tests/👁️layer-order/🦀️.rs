use super::*;

#[test]
fn resolved_frames_follow_portable_layer_order_and_keep_overrides() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/👁️layer-order/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let document: LayoutSnapshot = semio_framework_pack_json::from_json_str(&case["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let actual = resolve_page(&document, &document.pages[0]);
        let expected = case["expected"].as_array().unwrap();
        assert_eq!(actual.len(), expected.len(), "{}", case["name"]);
        for (row, expected) in actual.iter().zip(expected) {
            assert_eq!(row.frame.id(), expected["id"].as_str().unwrap());
            assert_eq!(row.inherited, expected["inherited"].as_bool().unwrap());
            assert_eq!(row.parent, expected["parent"].as_bool().unwrap());
            assert_eq!(row.frame_index, expected["frameIndex"].as_u64().unwrap()as usize);
            assert_eq!(row.frame.bounds().x, expected["x"].as_f64().unwrap());
            assert_eq!(row.frame.visible(), expected["visible"].as_bool().unwrap());
        }
    }
}
