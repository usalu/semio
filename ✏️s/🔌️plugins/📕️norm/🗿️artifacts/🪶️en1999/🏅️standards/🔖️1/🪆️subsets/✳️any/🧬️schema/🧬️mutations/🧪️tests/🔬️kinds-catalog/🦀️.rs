//! 🔬️ KINDS catalog parity for EN 1999 mutations.

use crate::mutations::KINDS;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <En1999Mutation as protocol::SemanticMutation<En1999Snapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared En1999Mutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)] declaration order");
    }
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must appear in oracle manifest");
    }
}
