//! 📝️ Physical text diff representation.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

pub type En1999DiffText = String;

semio_framework_os_kernel::diff_text!(semio_s_artifact_norm_en1999_diff::En1999Diff);
