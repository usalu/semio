//! 📜️ Wires artifact — native `.wires` DSL text codec (ticket
//! `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM` design.md §1 CORRECTION: the native codec is
//! one bidirectional thing and sits directly under `🚪️io/<facet>/<representation>/`, unsplit —
//! relocated here verbatim from `🧬️schema/📸️snapshot/📝️text`, taking `impl store::ArtifactDsl for
//! WiresSnapshot` with it from `🧬️schema/📸️snapshot/🦀️.rs`'s former `🔖️HandcraftedArtifactCodecs`
//! region — `🧬️schema` now keeps only the `WiresSnapshot` type). `content` is a composed
//! `store::ArtifactChild<SemioGraphSnapshot>`, which has no `dsl::DslRecord` derive support, so this
//! hand-rolls the whole codec. `WiresMutation`'s own op-text grammar is unaffected
//! (`#[derive(dsl::DslEnum)]`, in `crate::op`).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::{wires_working_scene, WiresSnapshot};
use dsl::DslValue;

/// 📄️ The `metabolism` example, handcrafted in the `.wires` DSL — source of truth for every
/// "metabolism" example call site (`setActiveExample`, `.example` manifest registration, tests).
pub const REASONING_WIRES_EXAMPLE_METABOLISM_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ P6 handcrafted `ArtifactDsl` (derive no longer emits this trait once `content` drops to a
/// composed `ArtifactChild` — see this file's module doc).
impl store::ArtifactDsl for WiresSnapshot {
    const EXTENSION: &'static str = "wires";
    fn envelope_id() -> &'static str {
        "reasoning.wires"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        super::binary::parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = super::binary::print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

/// 📖️ Parses `.wires` DSL text into a `WiresSnapshot`.
pub fn parse_dsl(text: &str) -> Result<WiresSnapshot, store::TextError> {
    <WiresSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `WiresSnapshot` back to `.wires` DSL text.
pub fn print_dsl(document: &WiresSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
