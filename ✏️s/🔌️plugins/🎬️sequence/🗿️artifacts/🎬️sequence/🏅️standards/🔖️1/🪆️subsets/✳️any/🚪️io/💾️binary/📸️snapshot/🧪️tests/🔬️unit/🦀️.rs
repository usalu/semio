#[semio_framework_async_macros::async_test]
async fn sequence_pack_schema_identity_is_derived() {
    store::os_store::test_support::assert_pack_schema_identity(&crate::schema::snapshot::default_persisted_snapshot());
}
