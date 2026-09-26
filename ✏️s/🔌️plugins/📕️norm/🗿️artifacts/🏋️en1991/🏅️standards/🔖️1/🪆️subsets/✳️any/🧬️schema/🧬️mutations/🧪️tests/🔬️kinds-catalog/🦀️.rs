use crate::{En1991Mutation, En1991Snapshot};
use super::KINDS;

#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <En1991Mutation as protocol::SemanticMutation<En1991Snapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared En1991Mutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        let needle = format!("\"{kind}\"");
        assert!(manifest.contains(&needle), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
