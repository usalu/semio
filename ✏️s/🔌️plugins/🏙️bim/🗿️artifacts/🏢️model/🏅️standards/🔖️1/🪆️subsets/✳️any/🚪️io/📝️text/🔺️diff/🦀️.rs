//! 📝️ Text representation of the model diff: one compact JSON line, the same wire value the pack carries.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

use crate::ModelDiff;

/// 🚚️ The carrier this facet's `parse`/`print` speak.
pub type ModelDiffText = String;

impl protocol::DiffText for ModelDiff {
    fn print_diff(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error.under("bim model diff line"), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
