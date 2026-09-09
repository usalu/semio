use super::*;

/// 🧷️ LAW: the single `Fem2dPresenceMutation` variant owns exactly one `MutationLeafDescriptor`.
#[test]
fn the_presence_mutation_variant_has_its_own_descriptor() {
    assert_eq!(<Fem2dPresenceMutation as Mutation<Fem2dPresence>>::DESCRIPTORS.len(), 1);
    let descriptor = Fem2dPresenceMutation::Noop.descriptor();
    assert_eq!(descriptor.aggregate_variant, "Noop");
    assert_eq!(descriptor.schema_version, 1);
}
