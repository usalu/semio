//! 📷️ png — the lane that is deliberately NOT this artifact's, pinned so it stays honest.
//!
//! generation2d owns the procedural plugin's png export claim, and a raster image cannot become a
//! BRep flow graph, so both directions refuse. What this case guards is the DIFFERENCE between the
//! two ways of not supporting something: before this ticket export handed back the artifact's own
//! DSL text under a `.png` name and import handed back an empty document — both of which read as
//! success. A refusal that names the reason is the behaviour under test.

use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::png::v1_2::any as export;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::png::v1_2::any as import;

#[test]
fn png_export_refuses_and_names_the_owning_artifact() {
    let error = export::serialize_bytes(&semio_s_artifact_procedural_generation3d::Generation3dSnapshot::default()).expect_err("png export is generation2d's claim, not this artifact's");
    assert!(error.to_string().contains("generation2d"), "the refusal names who owns the claim, got {error}");
}

#[test]
fn png_import_refuses_and_names_a_format_that_does_work() {
    let error = import::deserialize_bytes(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]).expect_err("png must not silently produce an empty document");
    let message = error.to_string();
    assert!(message.contains("raster image"), "the refusal names the reason, got {message}");
    assert!(message.contains("stl"), "the refusal points at a format that does work, got {message}");
}

#[test]
fn png_is_import_listed_but_not_export_listed() {
    let io = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export_stdio_kinds();
    assert!(!io.contains(&"stdio.png"), "generation2d owns the png export claim");
    let io = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import_stdio_kinds();
    assert!(io.contains(&"stdio.png"), "png stays import-listed so a dropped file reaches the refusal instead of vanishing");
}
