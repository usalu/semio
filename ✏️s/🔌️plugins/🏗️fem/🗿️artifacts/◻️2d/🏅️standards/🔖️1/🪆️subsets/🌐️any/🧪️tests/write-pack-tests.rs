//! 🧪️ Actual persistent FEM Pack publication uses the shared original codec and final wire owner.
use super::*;
#[test]
fn actual_fem_native_pack_publication_matches_the_canonical_reference_document() {
    let source = Fem2dSnapshot::default();
    let expected = pack::snapshot::ArtifactPack::encode_pack(&source);
    let mut callback = |_| true;
    let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut native = semio_framework_value::NativeEncodeControl::new(128 * 1048576, &mut callback);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 2_000_000, maximum_copy_bytes: 128 * 1048576, maximum_capacity_bytes: 128 * 1048576, maximum_release_bytes: 128 * 1048576, maximum_depth: 128 };
    let mut original = store::NativeSnapshotEncodeOwner::new(&mut native, grant);
    let payload = source.encode_exchange_pack(&mut original).unwrap();
    let store::io_schema::IoPayload::Binary(wire) = &payload else {
        panic!("binary native Pack");
    };
    assert_eq!(wire, &expected);
    assert!(original.progress().copied_items > 0);
    drop(original);
    let mut close = semio_framework_value::retirement::controlled::ControlledRetirement::new(payload).ok().unwrap();
    while !close.terminal_is_empty() {
        close.step(grant).unwrap();
    }
}
