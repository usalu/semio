use super::*;
use semio_framework_value::{FromValue as _, NativeDecodeControl, NativeEncodeControl};

fn roundtrip<T: semio_framework_value::FromValue + semio_framework_value::ToValue + std::fmt::Debug + PartialEq>(text: &str) {
    let policy = semio_framework_pack_json::JsonMemberPolicy::Reject;
    let expected: T = semio_framework_pack_json::from_json_str(text, policy).unwrap();
    let mut callback = |_| true;
    let actual: T = semio_framework_pack_json::from_json_str_controlled(text, policy, &mut NativeDecodeControl::new(1 << 20, &mut callback)).unwrap();
    assert_eq!(actual, expected);
    let mut encode_callback = |_| true;
    let wire = semio_framework_pack_json::to_json_string_controlled(&actual, &mut NativeEncodeControl::new(1 << 20, &mut encode_callback)).unwrap();
    let decoded: T = semio_framework_pack_json::from_json_str_controlled(&wire, policy, &mut NativeDecodeControl::new(1 << 20, &mut callback)).unwrap();
    assert_eq!(decoded, expected);
    T::retire_decoded(actual);
    T::retire_decoded(decoded);
    T::retire_decoded(expected);
}

#[test]
fn controlled_nullable_diff_fields_match_ordinary_and_serde() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🫙️nullable/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let input = &case["value"];
        let text = input.to_string();
        match case["owner"].as_str().unwrap() {
            "layout" => roundtrip::<LayoutDiff>(&text),
            "page" => roundtrip::<PagePatch>(&text),
            "frame" => { roundtrip::<FramePatch>(&text); let oracle: FramePatch = serde_json::from_value(input.clone()).unwrap(); let actual: FramePatch = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(); assert_eq!(actual, oracle); }
            "link" => { roundtrip::<ImageLinkPatch>(&text); let oracle: ImageLinkPatch = serde_json::from_value(input.clone()).unwrap(); let actual: ImageLinkPatch = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(); assert_eq!(actual, oracle); }
            "character" => { roundtrip::<CharacterStylePatch>(&text); let oracle: CharacterStylePatch = serde_json::from_value(input.clone()).unwrap(); let actual: CharacterStylePatch = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(); assert_eq!(actual, oracle); }
            _ => panic!("unknown nullable owner"),
        }
    }
}

#[test]
fn controlled_nullable_construction_retains_admission_and_cancellation() {
    let mut callback = |_| false;
    assert!(semio_framework_pack_json::from_json_str_controlled::<LayoutDiff>("{}", semio_framework_pack_json::JsonMemberPolicy::Reject, &mut NativeDecodeControl::new(4096, &mut callback)).is_err());
    let mut callback = |_| true;
    assert!(semio_framework_pack_json::from_json_str_controlled::<PagePatch>(r#"{"parent_page_id":"long-parent-name"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject, &mut NativeDecodeControl::new(1, &mut callback)).is_err());
}
