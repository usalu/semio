use super::*;

/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every one of those spellings must also appear in the
/// committed `program-1-any` catalog. The framework reads the catalog and never the enum, so
/// this is the only thing standing between a renamed variant and a mutation catalog that
/// silently measures a vocabulary the code no longer has.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <ProgramMutation as protocol::SemanticMutation<ProgramSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared ProgramMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../⚖️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
