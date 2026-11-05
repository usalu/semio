//! 📝️ Physical text diff representation.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

semio_os_kernel::diff_text!(crate::En1991Diff);
