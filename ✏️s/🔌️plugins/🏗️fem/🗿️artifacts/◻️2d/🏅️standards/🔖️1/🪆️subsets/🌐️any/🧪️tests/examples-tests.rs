use super::*;
fn drain<T: RetireOwned>(value: T) {
    let mut owner = semio_framework_value::retirement::controlled::ControlledRetirement::new(value).ok().unwrap();
    let grant = RetainedCloneGrant { maximum_items: 10_000_000, maximum_copy_bytes: 256 * 1048576, maximum_capacity_bytes: 256 * 1048576, maximum_release_bytes: 256 * 1048576, maximum_depth: 128 };
    while !owner.terminal_is_empty() {
        owner.step(grant).unwrap();
    }
}
#[test]
fn bundled_examples_load_non_empty_through_the_editor_boot_path() {
    let boot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().unwrap();
    assert!(!boot.nodes.is_empty() && !boot.elements.is_empty(), "the demo boot document must not be empty");
    let packed = pack::snapshot::ArtifactPack::encode_pack(&boot);
    let again = <Fem2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&packed).unwrap();
    assert_eq!(again.nodes.len(), boot.nodes.len());
    assert_eq!(again.regions.len(), boot.regions.len());
    drain(boot);
    drain(again);
}
