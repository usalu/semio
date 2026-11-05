use crate::En1991Mutation;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use crate::KINDS;

#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <En1991Mutation as protocol::SemanticMutation<En1991Snapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared En1991Mutation variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    semio_s_mutation_vectors::assert_mutation_catalogue(KINDS, &semio_framework_test_fixtures::fixture_path!("../../../../🔮️oracles/🔣️.json"));
}
