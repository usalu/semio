//! 📜️ FEM 2D artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::Fem2dSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📦️ The `fem2d-play` "default" example, embedded at compile time as handcrafted `.fem2d` DSL text —
/// shared by the manifest's `.example(...)` registration, the `setActiveExample` handler, and every
/// test fixture.
pub const FEM2D_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.fem2d` DSL text into a `Fem2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Fem2dSnapshot, semio_framework_diagnostic::TextError> {
    <Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Fem2dSnapshot` back to `.fem2d` DSL text.
pub fn print_dsl(document: &Fem2dSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
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
pub type Fem2dSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::FemAnalysisSettings;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Fem2dSnapshot {
    const EXTENSION: &'static str = "fem2d";
    fn envelope_id() -> &'static str {
        "fem.fem2d"
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::FemAnalysisSettings;

/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `fem2d_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn fem2d_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <Fem2dSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <Fem2dSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <Fem2dSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <Fem2dSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
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

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use crate::FemAnalysisSettings;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::FemCombination;
use crate::FemElement;
use crate::FemLoadCase;
use crate::FemMaterial;
use crate::FemNode;
use crate::FemRegion;
use crate::FemSection;
use crate::FemSupport;

/// 🌱️ An empty `Fem2dSnapshot` — every test fixture's blank baseline and the fallback boot document.
pub fn empty_fem2d_snapshot() -> crate::Fem2dSnapshot {
    crate::Fem2dSnapshot::default()
}

/// 🌱️ The document every fresh fem2d surface boots on: the bundled `📚️examples/🎬️demo` DSL, so the
/// editor and the viewer both paint a real structure at first frame instead of an empty canvas. A
/// fixture that ever stops parsing degrades to `empty_fem2d_snapshot` rather than faulting the boot,
/// and says so on the console.
pub fn default_fem2d_snapshot() -> crate::Fem2dSnapshot {
    match <crate::Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT) {
        Ok(snapshot) => {
            eprintln!(
                "[TRACE] fem2d boot snapshot: loaded the bundled example — nodes={} elements={} regions={} materials={} sections={} supports={} loadCases={} combinations={}",
                snapshot.nodes.len(),
                snapshot.elements.len(),
                snapshot.regions.len(),
                snapshot.materials.len(),
                snapshot.sections.len(),
                snapshot.supports.len(),
                snapshot.load_cases.len(),
                snapshot.combinations.len()
            );
            snapshot
        }
        Err(error) => {
            eprintln!("[TRACE] fem2d boot snapshot: the bundled example failed to parse, falling back to the empty document — {error}");
            empty_fem2d_snapshot()
        }
    }
}
}
pub use snapshot_wire2_codec::*;
