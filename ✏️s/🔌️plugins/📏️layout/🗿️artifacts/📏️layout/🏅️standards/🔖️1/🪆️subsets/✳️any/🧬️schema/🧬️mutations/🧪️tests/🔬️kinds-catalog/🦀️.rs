use super::*;
use protocol::SemanticMutation;

/// 🏷️ `KINDS` must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every entry must also appear in the committed
/// catalog — the framework reads the manifest and never parses Rust, so this test is the only
/// thing that keeps the two in step. A plain `#[test]`: it suspends on nothing.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = LayoutMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared LayoutMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = &crate::test_fixture_text("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed layout-1-any catalog");
    }
}

/// 🧬️ Canonical schema branches are the portable vocabulary authority for parent mutations.
#[test]
fn kinds_match_the_canonical_schema() {
    let schema: serde_json::Value = serde_json::from_str(&crate::test_fixture_text("../../🔣️.json")).unwrap();
    let kinds: Vec<&str> = schema["oneOf"].as_array().unwrap().iter().map(|branch| {
        let variant = branch["required"][0].as_str().unwrap();
        let reference = branch["properties"][variant]["$ref"].as_str().unwrap();
        reference.strip_suffix("/schema.json").unwrap().rsplit('/').next().unwrap()
    }).collect();
    assert_eq!(KINDS, kinds.as_slice());
}
