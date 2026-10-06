//! 🗣️ Architect program artifact — the textual document surface (constitutional: dsl).
//!
//! `ProgramSnapshot`'s `store::ArtifactDsl` impl is `#[derive(dsl::DslRecord)]`-generated on the document
//! type itself (see `🦀️.rs`); this node owns the named entry points every consumer calls and
//! the bundled `.architect` example the derive is validated against.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::ProgramSnapshot;

/// 📦️ The "Sample Clinic" default example, embedded at compile time as handcrafted
/// `.architect` DSL text — a static transcription of `sample_plugin()`, kept in sync with it by
/// `architect_example_text_parses_to_sample_plugin_and_round_trips`. The app manifest's
/// `.example("sample", ...)` still registers `sample_plugin()` serialized to JSON at runtime
/// (a separate, pre-existing concern) — this constant exists so a static `.architect` fixture
/// is available on disk for DSL-notation round-trip testing.
pub const ARCHITECT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 🗣️ Parses an Architect program from its textual DSL representation.
pub fn parse(text: &str) -> Result<ProgramSnapshot, semio_framework_diagnostic::TextError> {
    <ProgramSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints an Architect program in its canonical textual DSL representation.
pub fn print(document: &ProgramSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type ProgramSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::kernel::*;
use crate::registers::*;
use framework_schema::ArtifactSchema;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for ProgramSnapshot {
    const EXTENSION: &'static str = "architect";
    fn envelope_id() -> &'static str {
        "architect.program"
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

/// 📖️ Parses `.architect` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_program_dsl(text: &str) -> Result<ProgramSnapshot, String> {
    <ProgramSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`ProgramSnapshot`] back to `.architect` DSL text under a name an external caller can reach, paired
/// with [`parse_program_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_program_dsl(snapshot: &ProgramSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;


#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 📥️ Decodes a committed `📸️snapshot/{⬅️before,➡️after}/🔣️.json` vector.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_program_snapshot_json(text: &str) -> Result<ProgramSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ The snapshot as the same canonical JSON the committed vectors are written in — the
/// projection an external test host compares through.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn encode_program_snapshot_json(snapshot: &ProgramSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}
}
pub use mutations_codec::*;
