//! 🗣️ CAD artifact — the textual `.cad` document grammar surface: `parse_dsl`/`print_dsl` over the
//! derive-generated `store::ArtifactDsl`, plus the handcrafted `default` example the app registers.

use crate::CadSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📄️ The `default` example scene, handcrafted in the `.cad` DSL — a small structural column with
/// a two-vertex/one-edge/one-wire/one-face/one-shell/one-solid brep, a site-photo reference, and
/// objects across the shape/building/structure-classic panes.
pub const CAD_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

/// 📖️ Parses `.cad` DSL text into a `CadSnapshot`.
pub fn parse_dsl(text: &str) -> Result<CadSnapshot, semio_framework_diagnostic::TextError> {
    <CadSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `CadSnapshot` back to `.cad` DSL text.
pub fn print_dsl(document: &CadSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type CadSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{empty_cad_snapshot, CadDrawingChild, CadModelChild, CadNode, CadReferenceList};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::CadReferenceIndex;

pub(crate) fn exact_child(target: &semio_framework_artifact_reference::ArtifactRef, subset: &str) -> Result<(), String> {
    if target.dialect.artifact_kind != "s.stdio.semio" || target.dialect.standard != "v1" || target.dialect.subset != subset {
        return Err(format!("cad child must target s.stdio.semio@v1/{subset}"));
    }
    Ok(())
}

/// 🛡️ Every literal child handle retains its independent local identity and exact declared domain.
pub(crate) fn require_exact_children(s: &CadSnapshot) -> Result<(), String> {
    for child in [&s.shape_model, &s.building_model, &s.energy_model, &s.structure_classic_model].into_iter().flatten() {
        exact_child(&child.target, "model")?;
    }
    for child in &s.drawings {
        exact_child(&child.target, "drawing")?;
    }
    Ok(())
}

/// ✉️ `ArtifactDsl` and `ArtifactPack` are the derived spec-driven text and pack of the one
/// `dsl::DslRecord` spec; both re-check every composed child's exact identity on decode.
impl store::ArtifactDsl for CadSnapshot {
    const EXTENSION: &'static str = "cad";
    fn envelope_id() -> &'static str {
        "cad.cad"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        let snapshot = Self::__dsl_from_record(&record)?;
        require_exact_children(&snapshot).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;
