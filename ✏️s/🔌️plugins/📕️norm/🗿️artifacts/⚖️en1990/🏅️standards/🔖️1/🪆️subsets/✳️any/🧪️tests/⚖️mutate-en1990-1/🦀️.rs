//! Language-agnostic mutate scenario for EN 1990 hierarchical subject.

#[semio_framework_async_macros::async_test]
async fn mutate_raises_consequence_class() {
    let base = crate::En1990Snapshot::default();
    let mut target = base.clone();
    target.consequence_class = 3;
    let mutations = crate::En1990Mutation::from_snapshot(&base, &target);
    assert_eq!(mutations.len(), 1);
}
