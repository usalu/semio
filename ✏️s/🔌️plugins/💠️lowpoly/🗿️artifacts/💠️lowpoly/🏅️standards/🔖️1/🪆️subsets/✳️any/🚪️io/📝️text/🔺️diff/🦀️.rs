//! 📝️ Physical text diff representation.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type LowpolyDiffText = String;

/// 🔺️ Physical sparse delta text encoding.
impl protocol::os_spr::DiffText for crate::schema::diff::LowpolyDiff {
    fn print_diff(&self) -> String { super::lowpoly_json_encode(self) }
    fn parse_diff(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> { super::lowpoly_json_decode(text).map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1))) }
}
