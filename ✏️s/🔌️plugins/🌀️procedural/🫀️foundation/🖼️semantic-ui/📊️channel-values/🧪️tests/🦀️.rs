//! 🧪️ Exact and bounded procedural channel text projection.
use super::*;
use semio_framework_pack_json::{parse, JsonMemberPolicy};

#[test]
fn text_pages_preserve_every_utf8_byte() {
    let fixture = parse(include_str!("🔣️.json"), JsonMemberPolicy::Reject).unwrap();
    let limit = fixture["pageBytes"].as_u64().unwrap() as usize;
    for case in fixture["cases"].as_array().unwrap() {
        let text = case["text"].as_str().unwrap();
        let expected = case["pages"].as_array().unwrap();
        let pages = TextPages(text);
        assert_eq!(pages.len(), expected.len(), "{}", case["name"]);
        for (index, expected) in expected.iter().enumerate().rev() {
            let page = pages.get(index).unwrap();
            assert_eq!(page, expected.as_str().unwrap(), "{}", case["name"]);
            assert!(page.len() <= limit + 3 && !page.is_empty());
        }
        assert!(pages.get(expected.len()).is_none());
        assert!(pages.get(usize::MAX).is_none());
        let restored: String = (0..pages.len()).map(|index| pages.get(index).unwrap()).collect();
        assert_eq!(restored, text);
    }
}

#[test]
fn object_member_keys_retain_their_exact_identity() {
    let fixture = parse(include_str!("📛️keys.json"), JsonMemberPolicy::Reject).unwrap();
    let limit = fixture["inlineBytes"].as_u64().unwrap() as usize;
    for case in fixture["cases"].as_array().unwrap() {
        let key = case["key"].as_str().unwrap();
        assert_eq!(CHANNEL_SUMMARY_LIMIT, limit);
        assert_eq!(key_requires_detail(key), case["detail"].as_bool().unwrap(), "{}", case["name"]);
        let pages = TextPages(key);
        let restored: String = (0..pages.len()).map(|index| pages.get(index).unwrap()).collect();
        assert_eq!(restored, key);
    }
}

#[test]
fn previews_only_evaluated_flow_image_dictionaries() {
    let fixture = parse(include_str!("🖼️images.json"), JsonMemberPolicy::Reject).unwrap();
    let source = fixture["image"]["dataUrl"].as_str().unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let expected = case["image"].as_bool().unwrap().then_some(source);
        assert_eq!(channel_image_source(&case["value"]), expected, "{}", case["name"]);
    }
}
