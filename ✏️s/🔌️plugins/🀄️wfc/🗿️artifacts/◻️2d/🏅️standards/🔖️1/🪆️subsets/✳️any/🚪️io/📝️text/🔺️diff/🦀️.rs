//! 🔺️ WFC 2D diff — grammar spec asset only. `Wfc2dDiff` has no `impl store::ArtifactDsl` anywhere
//! in this plugin: this facet exists purely to register `wfc.wfc2d.diff`'s handcrafted text grammar
//! for LSP/verification tooling through `io()`'s `LanguageSpec`. The real apply/absorb algebra lives
//! in `🧬️schema/🔺️diff/🦀️.rs` (pure snapshot transforms).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar
