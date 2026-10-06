#[semio_framework_async_macros::async_test]
async fn lowpoly_pack_schema_identity_is_derived_and_keeps_the_mesh_child() {
    let mut snapshot = crate::schema::snapshot::snapshot_from_mesh_json("{\"vertices\":[]}", "cube", "Cube");
    snapshot.objects[0].paint_layers[0].pixels = vec![1, 2, 3, 255];
    store::os_store::test_support::assert_pack_schema_identity(&snapshot);
    let decoded: crate::LowpolySnapshot = store::ArtifactPack::decode_pack(&store::ArtifactPack::encode_pack(&snapshot)).expect("decode");
    assert_eq!(decoded.objects[0].mesh, snapshot.objects[0].mesh);
}
