use super::*;

#[semio_framework_async_macros::async_test]
async fn vcs_demo_presence_noop_has_exact_mutation_metadata() {
    let descriptor = VcsDemoPresenceMutation::Noop.descriptor();
    assert_eq!(<VcsDemoPresenceMutation as Mutation<VcsDemoPresence>>::DESCRIPTORS, &[descriptor.clone()]);
    assert_eq!((descriptor.semantic_kind, descriptor.aggregate_variant), ("noop", "Noop"));
    assert_eq!(VcsDemoPresenceMutation::Noop.diff(&VcsDemoPresence::default()).diff(), &VcsDemoPresence::default());
}
