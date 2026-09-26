//! 🧪 Mutation fixture smoke — hierarchical subject uses leaf named tests.
#[semio_framework_async_macros::async_test]
async fn hierarchical_mutations_mounted() {
    assert!(!crate::artifact_schema::mutations::KINDS.is_empty());
}
