use super::*;
use protocol::{Mutation, MutationDiff};
use store::{ArtifactDsl, ArtifactPack};

#[test]
fn preview_state_matches_language_neutral_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/👁️preview-partition/🔣️.json")).expect("fixture");
    let mut typed = Generation2dTransient::default();
    let mut oracle = serde_json::Map::new();
    for step in fixture["steps"].as_array().expect("steps") {
        let preview_text = serde_json::from_value::<Option<String>>(step["previewText"].clone()).expect("preview text");
        let mutation = Generation2dTransientMutation::from(SetGenerationPreview { preview_text });
        let outcome = mutation.diff(&typed);
        typed = outcome.diff().apply(&typed).expect("typed mutation");
        oracle.insert("generationPreviewText".into(), step["previewText"].clone());
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
    }
    assert_eq!(serde_json::to_value(&typed).expect("typed json"), fixture["expected"]);
    assert_eq!(serde_json::Value::Object(oracle), fixture["expected"]);
    let text = typed.print_dsl();
    assert_eq!(Generation2dTransient::parse_dsl(&text).expect("text round trip"), typed);
    let binary = typed.encode_pack();
    assert_eq!(Generation2dTransient::decode_pack(&binary).expect("binary round trip"), typed);
}
