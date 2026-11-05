//! 🔬️ KINDS catalog parity for EN 1999 mutations.

use crate::KINDS;
use crate::En1999Mutation;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <En1999Mutation as protocol::SemanticMutation<En1999Snapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared En1999Mutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)] declaration order");
    }
    semio_s_mutation_vectors::assert_mutation_catalogue(KINDS, &semio_framework_test_fixtures::fixture_path!("../../../../🔮️oracles/🔣️.json"));
}
