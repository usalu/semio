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

#[test]
fn bounded_resolution_keeps_original_indices_and_applies_parent_overrides_before_selection() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/👁️layer-order/🔣️.json")).unwrap();
    let case = &corpus["cases"][0];
    let document: LayoutSnapshot = semio_framework_pack_json::from_json_str(&case["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let selection = &case["boundedResolution"];
    let minimum = selection["minimumX"].as_f64().unwrap();
    let rows = resolve_page_where(&document, &document.pages[0], |_, bounds| bounds.x >= minimum);
    let projected: Vec<_> = rows.iter().map(|row| (row.frame.id(), row.frame_index)).collect();
    assert_eq!(serde_json::to_value(projected).unwrap(), selection["rows"]);
}
