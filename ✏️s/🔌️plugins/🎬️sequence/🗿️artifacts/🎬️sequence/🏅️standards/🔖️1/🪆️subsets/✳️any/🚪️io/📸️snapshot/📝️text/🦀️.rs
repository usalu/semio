//! 📜️ Sequence artifact — native `.sequence` DSL text codec (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §1 CORRECTION: the native codec is
//! one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>/`, unsplit —
//! relocated here from `🧬️schema/📸️snapshot/📝️text` verbatim, `🧬️schema` keeps only the
//! `SequenceSnapshot` type). Carries the grammar doc-string, the real `store::ArtifactDsl for
//! SequenceSnapshot` impl, example text, and round-trip laws. Ticket
//! `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`) dropped the old
//! `SequenceEdgeDsl` unified-`dsl::Wire` mirror here — the snapshot no longer embeds `edges`
//! structurally in its own text grammar at all (only the opaque composed `content` handle), so a
//! per-edge DSL mirror has nothing left to mirror.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::{SequenceContentChild, SequenceSnapshot};

//#region 🔖️ArtifactDslCodec
impl store::ArtifactDsl for SequenceSnapshot {
    const EXTENSION: &'static str = "sequence";
    fn envelope_id() -> &'static str {
        "sequence.sequence"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️ArtifactDslCodec

//#region 🔖️Example
/// 📄️ The handcrafted `.sequence` DSL-text fixture (regenerated from `default_snapshot()`'s canonical
/// print form) — the permanent proof that the checked-in fixture still parses and round trips, not a
/// one-time migration script.
pub const SEQUENCE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.sequence` DSL text into a `SequenceSnapshot`.
pub fn parse_dsl(text: &str) -> Result<SequenceSnapshot, store::TextError> {
    <SequenceSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `SequenceSnapshot` back to `.sequence` DSL text.
pub fn print_dsl(snapshot: &SequenceSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️Example

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

