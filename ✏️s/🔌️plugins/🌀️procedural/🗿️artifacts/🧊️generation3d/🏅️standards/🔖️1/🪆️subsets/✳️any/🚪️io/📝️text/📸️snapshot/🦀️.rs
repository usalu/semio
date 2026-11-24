//! 📜️ Generation3d artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! See `generation2d`'s sibling `🗣️dsl/🦀️.rs` docstring for why the `*Dsl` mirror types below
//! are LOCAL structural twins rather than derives on the foreign `flow`/`playbook` types directly.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

//#region 🔖️Examples
pub const GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT: &str = include_str!("../../../📚️examples/🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_RECT_EXTRUDE_TEXT: &str = include_str!("../../../📚️examples/📦️rectangle-extrude-volume/🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_SPHERE_TORUS_TEXT: &str = include_str!("../../../📚️examples/🍩️sphere-cut-with-torus/🖼️assets/🍩️sphere-cut-with-torus/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_BOX_FILLET_TEXT: &str = include_str!("../../../📚️examples/📐️box-fillet-preview/🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT: &str = include_str!("../../../📚️examples/🧲️sphere-box-fuse/🖼️assets/🧲️sphere-box-fuse/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT: &str = include_str!("../../../📚️examples/🧹️face-sweep-extrude/🖼️assets/🧹️face-sweep-extrude/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT: &str = include_str!("../../../📚️examples/🪢️rectangle-wire-preview/🖼️assets/🪢️rectangle-wire-preview/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_BOX_SHELL_TEXT: &str = include_str!("../../../📚️examples/🐚️box-shell-preview/🖼️assets/🐚️box-shell-preview/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_MESH_WORKBENCH_TEXT: &str = include_str!("../../../📚️examples/🥽️mesh-workbench/🖼️assets/🥽️mesh-workbench/🗣️.dsl.semio");
//#endregion 🔖️Examples

//#region 🔖️DslMirror
pub use semio_s_artifact_procedural_3d_model::dsl_mirror::*;
//#endregion 🔖️DslMirror

/// 🔍️ Parses `.generation3d` DSL text into a `Generation3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    <Generation3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Generation3dSnapshot` back to `.generation3d` DSL text.
pub fn print_dsl(document: &Generation3dSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Generation3dSnapshotText = String;

pub use semio_s_artifact_procedural_3d_model::sqlite_codec::{decode_sqlite_native, encode_sqlite_native};
//#endregion 🚚️Carrier

mod source_examples {
use super::*;
use semio_s_artifact_procedural_3d_model::*;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use pack::snapshot::ArtifactDsl;
/// 📄️ The `procedural3d-play` "default" document — parsed from the bundled "hexagonal mushroom
/// column" example host_snapshot.
pub fn default_snapshot() -> Generation3dSnapshot {
    Generation3dSnapshot::parse_dsl(GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT).unwrap_or_default()
}
/// 🧾️ Builds the projection for a named bundled example; unknown ids return `None`.
pub fn example_snapshot(example_id: &str) -> Option<Generation3dSnapshot> {
    let dsl = match example_id {
        PROCEDURAL_EXAMPLE_HEX_COLUMN | "demo" => Some(GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT),
        PROCEDURAL_EXAMPLE_RECT_EXTRUDE => Some(GENERATION3D_EXAMPLE_RECT_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_TORUS => Some(GENERATION3D_EXAMPLE_SPHERE_TORUS_TEXT),
        PROCEDURAL_EXAMPLE_BOX_FILLET => Some(GENERATION3D_EXAMPLE_BOX_FILLET_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE => Some(GENERATION3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT),
        PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE => Some(GENERATION3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_RECTANGLE_WIRE => Some(GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT),
        PROCEDURAL_EXAMPLE_MESH_WORKBENCH => Some(GENERATION3D_EXAMPLE_MESH_WORKBENCH_TEXT),
        PROCEDURAL_EXAMPLE_BOX_SHELL => Some(GENERATION3D_EXAMPLE_BOX_SHELL_TEXT),
        _ => None,
    };
    dsl.and_then(|text| Generation3dSnapshot::parse_dsl(text).ok())
}
/// 📼️ Serializes an example's bare projection for registration via `App::example`.
pub fn example_document_json(example_id: &str) -> String {
    let snapshot = example_snapshot(example_id).unwrap_or_default();
    let json = semio_framework_pack_json::to_json_string(&snapshot);
    snapshot.retire_cold();
    json
}
}
pub use source_examples::*;

#[path = "🗂️catalogue/🦀️.rs"]
pub mod catalogue;


