//! ☁️ las round trip — the committed unit cube out as a LAS point cloud, and the honest refusal back.
//!
//! LAS is the one deliberately asymmetric lane, so this case pins BOTH halves: the export really is
//! a LAS file holding the cube's 8 vertices inside the cube's bounds (so the vertices survive
//! quantization), and the import really is a TYPED ERROR naming why no document can be built — not
//! the empty document the old placeholder returned, which is the single behaviour that would let a
//! silent regression back in.

use crate::{assert_close, project, unit_cube_semio_mesh, TOLERANCE};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::las::v1_0::any as export;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::las::v1_0::any as import;

fn exported() -> Vec<u8> {
    export::serialize_mesh_bytes(&unit_cube_semio_mesh()).expect("the unit cube exports as las")
}

#[test]
fn las_export_writes_a_real_las_container_not_the_artifact_dsl() {
    let bytes = exported();
    assert!(bytes.starts_with(b"LASF"), "a las file opens with the `LASF` signature");
    assert!(bytes.len() > 227, "a las file is at least a full 1.x public header block");
}

#[test]
fn las_export_keeps_every_vertex_of_the_cube_within_quantization() {
    let bytes = exported();
    let cloud = import::mesh_from_bytes(&bytes).expect("our own las bytes decode back to a point cloud");
    let projection = project(&cloud);
    assert_eq!(projection.vertex_count, 8, "the cube's 8 vertices all survive as points");
    assert_eq!(projection.triangle_count, 0, "las carries no connectivity, and this codec invents none");
    for axis in 0..3 {
        assert_close(&format!("las: min[{axis}]"), projection.min[axis], 0.0);
        assert_close(&format!("las: max[{axis}]"), projection.max[axis], 1.0);
    }
    assert!(TOLERANCE > 0.0001, "the tolerance clears las's own 0.0001-unit coordinate scale");
}

#[test]
fn las_import_refuses_to_build_a_document_and_says_why() {
    let bytes = exported();
    let error = import::deserialize_bytes(&bytes).expect_err("las must not silently produce an empty document");
    let message = error.to_string();
    assert!(message.contains("point cloud"), "the refusal names the reason, got {message}");
    assert!(message.contains("Export to LAS is supported"), "the refusal tells the user what does work, got {message}");
}
