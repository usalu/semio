use super::*;

/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every one of those spellings must also appear in the
/// committed `forms-1-any` catalog. The framework reads the catalog and never the enum, so
/// this is the only thing standing between a renamed variant and a mutation catalog that
/// silently measures a vocabulary the code no longer has.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <FormMutation as protocol::SemanticMutation<FormsSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared FormMutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest: serde_json::Value = serde_json::from_str(include_str!("../../../../🔮️oracles/🔣️.json")).unwrap();
    let catalog: std::collections::BTreeSet<&str> = manifest["mutationCatalogs"].as_array().unwrap().iter().flat_map(|catalog| catalog["kinds"].as_array().unwrap()).map(|kind| kind.as_str().unwrap()).collect();
    assert_eq!(catalog, KINDS.iter().copied().collect());
}
