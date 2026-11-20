use super::*;
fn drain<T: RetireOwned>(value: T) {
    semio_s_fem_receiving::testkit::retire(value)
}
#[test]
fn bundled_examples_load_non_empty_through_the_editor_boot_path() {
    let boot = semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::fem3d_boot_snapshot().unwrap();
    assert!(!boot.nodes.is_empty() && !boot.elements.is_empty(), "the concrete-forest boot document must not be empty");
    drain(boot);
    for (id, text) in [
        (semio_s_artifact_fem_3d_io::examples::concrete_forest::ID, semio_s_artifact_fem_3d_io::examples::concrete_forest::PRIMARY_TEXT),
        (semio_s_artifact_fem_3d_io::examples::demo::ID, semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::FEM3D_EXAMPLE_TEXT),
        (semio_s_artifact_fem_3d_io::examples::house::ID, semio_s_artifact_fem_3d_io::examples::house::PRIMARY_TEXT),
    ] {
        let document = <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).unwrap_or_else(|error| panic!("example {id} must load: {error}"));
        assert!(!document.nodes.is_empty() && (!document.elements.is_empty() || !document.solids.is_empty()), "example {id} must load non-empty");
        let packed = pack::snapshot::ArtifactPack::encode_pack(&document);
        let again = <Fem3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&packed).unwrap_or_else(|error| panic!("example {id} pack must decode: {error}"));
        assert_eq!(again.nodes.len(), document.nodes.len());
        assert_eq!(again.elements.len(), document.elements.len());
        drain(document);
        drain(again);
    }
}
