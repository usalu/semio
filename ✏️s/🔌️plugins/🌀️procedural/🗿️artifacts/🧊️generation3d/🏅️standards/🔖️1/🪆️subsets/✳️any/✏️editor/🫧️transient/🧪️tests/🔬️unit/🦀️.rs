use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn preview_lifecycle_matches_language_neutral_third_party_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️unit/🔄️preview-lifecycle/🔣️.json")).expect("fixture");
    let mut typed = Generation3dTransient::default();
    let mut oracle = serde_json::Map::new();
    for step in fixture["steps"].as_array().expect("steps") {
        let preview_text = step["previewText"].as_str().map(str::to_string);
        let mutation = Generation3dTransientMutation::from(SetGenerationPreview { preview_text });
        typed = mutation.diff(&typed).diff().apply(&typed).expect("typed mutation applies");
        oracle.insert("generationPreviewText".into(), step["previewText"].clone());
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
        assert_eq!(Generation3dTransientMutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(Generation3dTransientMutation::decode_op(&mutation.encode_op().expect("encode")).expect("binary round trip"), mutation);
    }
    assert_eq!(serde_json::to_value(&typed).expect("typed json"), fixture["expected"]);
    assert_eq!(serde_json::Value::Object(oracle), fixture["expected"]);
    assert_eq!(Generation3dTransient::parse_dsl(&typed.print_dsl()).expect("text state round trip"), typed);
    let packed = typed.encode_pack_with(&Default::default()).expect("pack");
    assert_eq!(Generation3dTransient::decode_pack_with(&packed, &Default::default()).expect("unpack"), typed);
}
