use super::*;
fn drain<T: RetireOwned>(value: T) {
    semio_s_fem_receiving::testkit::retire(value)
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
