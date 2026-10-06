//! 📝️ Physical text diff representation.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Din18599DiffText = String;

semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::Din18599Diff);
