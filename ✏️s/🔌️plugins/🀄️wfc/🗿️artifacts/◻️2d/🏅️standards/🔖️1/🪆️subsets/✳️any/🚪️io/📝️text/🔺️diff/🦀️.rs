//! 🔺️ WFC 2D diff — grammar spec asset only. `Wfc2dDiff` has no `impl store::ArtifactDsl` anywhere
//! in this plugin: this facet exists purely to register `wfc.wfc2d.diff`'s handcrafted text grammar
//! for LSP/verification tooling through `io()`'s `LanguageSpec`. The real apply/absorb algebra lives
//! in `🧬️schema/🔺️diff/🦀️.rs` (pure snapshot transforms).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

impl semio_framework_os_kernel::DiffText for crate::standards::v1::subsets::any::schema::diff::Wfc2dDiff {
    fn print_diff(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_diff(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
