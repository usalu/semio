//! 📜️ `s.wfc.grid3d` snapshot — textual document grammar surface + laws (constitutional: dsl).
//! Every record of this document is a local type that already derives `dsl::DslRecord`, so there is
//! no twin mirror here: the codecs live on `Grid3dSnapshot` itself (sibling `🦀️.rs`) and this leaf
//! carries the normative grammar plus the facet's parse/print entry points.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::schema::snapshot::Grid3dSnapshot;

//#region 🔖️Examples
/// 📄️ The two authored tile sets this subset ships, in their own `.wfcgrid3d` DSL.
pub const GRID3D_EXAMPLE_BLOCKS_TEXT: &str = include_str!("../../../📚️examples/🧱️blocks/🖼️assets/🧱️blocks/🗣️.dsl.semio");
pub const GRID3D_EXAMPLE_PIPES_TEXT: &str = include_str!("../../../📚️examples/🪠️pipes-3d/🖼️assets/🪠️pipes-3d/🗣️.dsl.semio");
//#endregion 🔖️Examples

/// 📖️ Parses `.wfcgrid3d` DSL text into a `Grid3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Grid3dSnapshot, semio_framework_diagnostic::TextError> {
    <Grid3dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Grid3dSnapshot` back to `.wfcgrid3d` DSL text.
pub fn print_dsl(document: &Grid3dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse_dsl`/`print_dsl` speak.
pub type Grid3dSnapshotTextCarrier = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

/// ✉️ Handcrafted `ArtifactDsl`/`ArtifactPack` — the derive stopped emitting these traits, so every
/// artifact states its own envelope discipline.
impl store::ArtifactDsl for Grid3dSnapshot {
    const EXTENSION: &'static str = "wfcgrid3d";
    fn envelope_id() -> &'static str {
        "wfc.grid3d"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;
