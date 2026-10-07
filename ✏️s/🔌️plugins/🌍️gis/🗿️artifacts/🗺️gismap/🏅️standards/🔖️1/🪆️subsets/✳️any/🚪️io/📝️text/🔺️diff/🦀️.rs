//! 📝️ Native JSON transport for owned GIS feature edits.

use crate::GisMapDiff;

/// 📖️ Authored grammar for the physical edit representation.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub type GisMapDiffText = String;

impl protocol::DiffText for GisMapDiff {
    fn print_diff(&self) -> String { semio_framework_pack_json::to_json_string(self) }
    fn parse_diff(text: &str) -> Result<Self,semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))
    }
}
