use super::*;
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, ValueRefusalKind};
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../../../🌱️value/⚠️refusal/🧫️fixtures/🔣️.json")).unwrap() }
#[test]
fn controlled_json_refusal_preserves_real_kind_and_nested_path() {
    for row in fixture()["jsonCases"].as_array().unwrap() {
        let text = row["text"].as_str().unwrap(); let mut accept_decode = |_| true; let mut accept_encode = |_| true;
        let mut cancel_decode = |_| false; let mut cancel_encode = |_| false;
        let error = match row["operation"].as_str().unwrap() {
            "decodeMalformed" | "decodeDuplicate" | "decodeDepth" => from_json_str_controlled::<DslValue>(text, JsonMemberPolicy::Reject, &mut NativeDecodeControl::new(65536, &mut accept_decode)).unwrap_err(),
            "decodeCancel" => from_json_str_controlled::<DslValue>(text, JsonMemberPolicy::Reject, &mut NativeDecodeControl::new(65536, &mut cancel_decode)).unwrap_err(),
            "decodeOwnership" => from_json_str_controlled::<DslValue>(text, JsonMemberPolicy::Reject, &mut NativeDecodeControl::new(0, &mut accept_decode)).unwrap_err(),
            "decodeNestedInvalid" => from_json_str_controlled::<Vec<Vec<u32>>>(text, JsonMemberPolicy::Reject, &mut NativeDecodeControl::new(65536, &mut accept_decode)).unwrap_err(),
            "decodeNestedCancel" => { let mut cancel = |p: semio_framework_value::native_decoding::NativeDecodeProgress| p.completed != 1 || p.total != 1; from_json_str_controlled::<Vec<Vec<u32>>>(text, JsonMemberPolicy::Reject, &mut NativeDecodeControl::new(65536, &mut cancel)).unwrap_err() },
            "encodeCancel" => to_json_string_controlled(&text, &mut NativeEncodeControl::new(65536, &mut cancel_encode)).unwrap_err(),
            "encodeOwnership" => to_json_string_controlled(&text, &mut NativeEncodeControl::new(0, &mut accept_encode)).unwrap_err(),
            _ => panic!("closed actual JSON refusal operation"),
        };
        assert_eq!(error.kind.as_str(), row["expectedKind"].as_str().unwrap(), "{}", row["id"]);
        if let Some(display) = row["expectedDisplay"].as_str() { assert_eq!(error.to_string(), display, "{}", row["id"]); }
    }
    assert_eq!(JsonError::TrailingData(4).kind(), ValueRefusalKind::InvalidValue);
    assert_eq!(JsonError::MaxDepthExceeded(128).kind(), ValueRefusalKind::DepthLimit);
    println!("[DEBUG] controlled JSON preserves malformed input, cancellation, ownership refusal and nested authority paths");
}
