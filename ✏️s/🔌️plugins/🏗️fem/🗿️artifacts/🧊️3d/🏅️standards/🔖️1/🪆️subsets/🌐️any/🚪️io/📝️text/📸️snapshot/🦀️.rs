//! 📜️ FEM 3D artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📦️ The `fem3d-play` "default" example, embedded at compile time as handcrafted `.fem3d` DSL text —
/// shared by the manifest's `.example(...)` registration, the `setActiveExample` handler, and every
/// test fixture.
pub const FEM3D_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.fem3d` DSL text into a `Fem3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Fem3dSnapshot, semio_framework_diagnostic::TextError> {
    <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Fem3dSnapshot` back to `.fem3d` DSL text.
pub fn print_dsl(document: &Fem3dSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(document)
}


/// 🎬️ The bundled `demo` fixture — the only built-in document that carries nodes, elements, solids,
/// supports and both load cases at once, so it is the document every law about concrete entity ids
/// (`n20_l1`, `sol1`, `steel`, `hea200`, …) is stated over. Distinct from the io boot document (`fem3d_boot_snapshot`)
/// since the boot document became the `concrete-forest` example (frames only, no solids).
pub fn fem3d_demo_snapshot() -> Result<Fem3dSnapshot, semio_framework_diagnostic::TextError> {
    parse_dsl(FEM3D_EXAMPLE_TEXT)
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Fem3dSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use crate::standards::v1::subsets::any::schema::structure::FemCombination;
use crate::standards::v1::subsets::any::schema::structure::FemElement;
use crate::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_fem_material::volumetric::FemMaterialValue;
use semio_s_fem_node::volumetric::FemNodeValue;
use crate::standards::v1::subsets::any::schema::structure::FemSection;
use crate::standards::v1::subsets::any::schema::structure::FemSolid;
use crate::standards::v1::subsets::any::schema::structure::FemSupport;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl pack::snapshot::ArtifactDsl for Fem3dSnapshot {
    const EXTENSION: &'static str = "fem3d";
    fn envelope_id() -> &'static str {
        "fem.fem3d"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
#[cfg(test)] pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use crate::standards::v1::subsets::any::schema::structure::FemCombination;
use crate::standards::v1::subsets::any::schema::structure::FemElement;
use crate::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_fem_material::volumetric::FemMaterialValue;
use semio_s_fem_node::volumetric::FemNodeValue;
use crate::standards::v1::subsets::any::schema::structure::FemSection;
use crate::standards::v1::subsets::any::schema::structure::FemSolid;
use crate::standards::v1::subsets::any::schema::structure::FemSupport;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `fem3d_mutation_report_json`:
/// `pack::snapshot::ArtifactDsl`/`pack::snapshot::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`pack::snapshot::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn fem3d_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <Fem3dSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <Fem3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
    let report = semio_framework_value::DslValue::object([
        ("parsed".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&parsed))),
        ("reparsed".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&reparsed))),
        ("packDecoded".to_string(), semio_framework_value::ToValue::to_value(&semio_framework_value::ToValue::to_value(&unpacked))),
        ("canonicalText".to_string(), semio_framework_value::ToValue::to_value(&canonical)),
        ("canonicalTextAgain".to_string(), semio_framework_value::ToValue::to_value(&canonical_again)),
    ]);
    Ok(semio_framework_pack_json::to_json_string(&report))
}
}
pub use snapshot_wire_codec::*;
