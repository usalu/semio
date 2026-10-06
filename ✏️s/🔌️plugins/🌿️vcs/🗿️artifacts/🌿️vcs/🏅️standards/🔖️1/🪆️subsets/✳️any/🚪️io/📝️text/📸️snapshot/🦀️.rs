//! 📜️ VCS artifact — native `.vcs` DSL text codec (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §1 CORRECTION: the native codec is
//! one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>/`, unsplit).
//! Relocated here from `🚪️io/📝️text/📸️snapshot` — the real `store::ArtifactDsl for VcsSnapshot`
//! impl moved with it; `🧬️schema` keeps only the `VcsSnapshot` type. `store::ArtifactPack`'s twin
//! impl sits in the sibling `💾️binary` facet.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::VcsSnapshot;

//#region 🔖️ArtifactDslCodec
impl store::ArtifactDsl for VcsSnapshot {
    const EXTENSION: &'static str = "vcs";
    fn envelope_id() -> &'static str {
        "vcs.vcs"
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
//#endregion 🔖️ArtifactDslCodec

//#region 🔖️Example
/// 📄️ The `demo` example checkpoint, handcrafted in the `.vcsdemo` DSL — a mid-review structural
/// change with a non-zero counter, freeform notes, an in-progress status, and a few tags.
pub const VCS_DEMO_DEFAULT_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.vcsdemo` DSL text into a `VcsSnapshot`.
pub fn parse_dsl(text: &str) -> Result<VcsSnapshot, semio_framework_diagnostic::TextError> {
    <VcsSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `VcsSnapshot` back to `.vcsdemo` DSL text.
pub fn print_dsl(projection: &VcsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(projection)
}
//#endregion 🔖️Example

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::VCS_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

/// 📤️ Renders a [`VcsSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `🌿️mutate-vcs-1`'s scenarios are measured through, and the same shape the committed
/// `../🧫️fixtures/🧬️mutations/<slug>/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in.
///
/// A thin `dsl::json` wrapper (this facet's own first-party `DslValue` JSON codec, used behind
/// this interface per CLAUDE.md's "external libraries behind an interface" rule).
pub fn encode_vcs_snapshot_json(snapshot: &VcsSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_vcs_snapshot_json`] — decodes those committed specification vectors
/// into real [`VcsSnapshot`] values, so `🌿️mutate-vcs-1`'s adapter reads the committed fixture rather
/// than re-declaring it as a Rust literal beside it. Reaching `serde_json` from that adapter is
/// impossible: the generated test host links only this crate and `semio-repo-test-host`.
pub fn decode_vcs_snapshot_json(text: &str) -> Result<VcsSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📝️ Parses `.vcs.dsl.semio` text into a [`VcsSnapshot`] — a named, non-async pass-through of this
/// type's own `store::ArtifactDsl` impl (`../../🚪️io/📝️text/📸️snapshot/🦀️.rs`), whose trait
/// and error type are both unnameable outside this crate, so `🌿️mutate-vcs-1`'s `identity-round-trip`
/// scenario reaches the real committed artifact (`../../🖼️assets/🎬️demo/🗣️.dsl.semio`)
/// through this instead.
pub fn parse_vcs_dsl(text: &str) -> Result<VcsSnapshot, String> {
    <VcsSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders a [`VcsSnapshot`] back as `.vcs.dsl.semio` text — the inverse of [`parse_vcs_dsl`],
/// preamble included, which is what makes a printed document comparable to the committed one
/// byte for byte.
pub fn print_vcs_dsl(snapshot: &VcsSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;
