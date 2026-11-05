use crate::*;

#[test]
fn kinds_match_the_enum() {
    let descriptors = <En1996Mutation as protocol::SemanticMutation<En1996Snapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len());
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind);
    }
    semio_s_mutation_vectors::assert_mutation_catalogue(KINDS, &semio_framework_test_fixtures::fixture_path!("../../../../🔮️oracles/🔣️.json"));

}
