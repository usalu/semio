use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
use super::*;
use protocol::{DiffAlgebra, Mutation, OpBinary, OpText};

#[test]
fn structure_fixtures_apply_inverse_and_transport() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🧬️structure/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let decode = |value: &serde_json::Value| semio_framework_pack_json::from_json_str::<LayoutSnapshot>(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let base = decode(&case["before"]);
        let expected = decode(&case["after"]);
        let mutation: LayoutMutation = semio_framework_pack_json::from_json_str(&case["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let saved = base.clone();
        let outcome = mutation.diff(&base);
        let after = protocol::apply_diff_cold(outcome.diff(), &base).unwrap();
        if name.starts_with("reject ") { assert!(!outcome.messages().is_empty(), "{name}: rejection must explain its refusal"); }
        assert_eq!(after, expected, "{name}: independent RFC 6902 fixture");
        assert_eq!(base, saved, "{name}: mutation must preserve its input");
        let restored = protocol::apply_diff_cold(&outcome.diff().inverse(&base), &after).unwrap();
        assert_eq!(restored, base, "{name}: diff inverse");
        let diff_text = semio_framework_pack_json::to_json_string(outcome.diff());
        let decoded_diff = semio_framework_pack_json::from_json_str::<LayoutDiff>(&diff_text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(decoded_diff, *outcome.diff(), "{name}: sparse diff presence must survive transport");
        if after != base {
            let mut restored = after.clone();
            for inverse in mutation.inverse(&base).unwrap().into_iter().rev() {
                restored = protocol::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap();
            }
            assert_eq!(restored, base, "{name}: semantic mutation inverse");
        }
        assert_eq!(LayoutMutation::parse_op(&mutation.print_op()).unwrap(), mutation, "{name}: text");
        assert_eq!(LayoutMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation, "{name}: binary");
    }
}
