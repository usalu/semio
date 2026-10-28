use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
use super::*;

#[test]
fn portable_diff_fixtures_apply_invert_and_preserve_sparse_presence() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🧮️diff-algebra/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let base: LayoutSnapshot = semio_framework_pack_json::from_json_str(&case["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let expected: LayoutSnapshot = semio_framework_pack_json::from_json_str(&case["after"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let diff: LayoutDiff = semio_framework_pack_json::from_json_str(&case["diff"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let saved = base.clone();
        let after = protocol::apply_diff_cold(&diff, &base).unwrap();
        let mut unpublished = base.clone();
        protocol::apply_diff_cold_into(&diff, &mut unpublished).unwrap();
        assert_eq!(unpublished, expected, "{name}: current cold in-place contract");
        assert_eq!(after, expected, "{name}: independent RFC 6902 witness");
        assert_eq!(base, saved, "{name}: immutable input");
        assert_eq!(protocol::apply_diff_cold(&diff.inverse(&base), &after).unwrap(), base, "{name}: inverse");
        let wire = semio_framework_pack_json::to_json_string(&diff);
        let decoded: LayoutDiff = semio_framework_pack_json::from_json_str(&wire, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(decoded, diff, "{name}: sparse wire presence");
    }
}

#[test]
fn positional_event_duplicates_are_refused_atomically() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🧮️diff-algebra/refusals.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let base: LayoutSnapshot = semio_framework_pack_json::from_json_str(&case["before"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let diff: LayoutDiff = semio_framework_pack_json::from_json_str(&case["diff"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let saved = base.clone();
        assert!(protocol::apply_diff_cold(&diff, &base).is_err(), "{}", case["name"]);
        assert_eq!(base, saved);
        let mut unpublished = base.clone();
        assert!(protocol::apply_diff_cold_into(&diff, &mut unpublished).is_err());
        assert_eq!(unpublished, saved, "cold refusal must not publish partial writes");
    }
}
