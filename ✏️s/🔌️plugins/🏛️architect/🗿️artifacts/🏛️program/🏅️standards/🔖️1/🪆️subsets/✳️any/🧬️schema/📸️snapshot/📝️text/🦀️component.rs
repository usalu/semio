//! 🗣️ Architect program artifact — the textual document surface (constitutional: dsl).
//!
//! `ProgramSnapshot`'s `store::ArtifactDsl` impl is `#[derive(dsl::DslRecord)]`-generated on the document
//! type itself (see `🦀️component.rs`); this node owns the named entry points every consumer calls and
//! the bundled `.architect` example the derive is validated against.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::program::ProgramSnapshot;

/// @emoji 📦️ The "Sample Clinic" default example, embedded at compile time as handcrafted
/// `.architect` DSL text — a static transcription of `sample_plugin()`, kept in sync with it by
/// `architect_example_text_parses_to_sample_plugin_and_round_trips`. The app manifest's
/// `.example("sample", ...)` still registers `sample_plugin()` serialized to JSON at runtime
/// (a separate, pre-existing concern) — this constant exists so a static `.architect` fixture
/// is available on disk for DSL-notation round-trip testing.
pub const ARCHITECT_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 🗣️ Parses an Architect program from its textual DSL representation.
pub fn parse(text: &str) -> Result<ProgramSnapshot, store::TextError> {
    <ProgramSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints an Architect program in its canonical textual DSL representation.
pub fn print(document: &ProgramSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::{empty_plugin, sample_plugin};

    #[test]
    fn parse_and_print_round_trip_the_sample_program() {
        let program = sample_plugin();
        assert_eq!(parse(&print(&program)).expect("parse"), program);
    }

    #[test]
    fn the_bundled_example_text_parses() {
        let parsed = parse(ARCHITECT_EXAMPLE_TEXT).expect("parse bundled example");
        assert_eq!(parsed.meta.title, sample_plugin().meta.title);
    }

    #[test]
    fn an_empty_program_prints_and_reparses() {
        let program = empty_plugin();
        assert_eq!(parse(&print(&program)).expect("parse"), program);
    }
}
//#endregion 🧪️Tests
