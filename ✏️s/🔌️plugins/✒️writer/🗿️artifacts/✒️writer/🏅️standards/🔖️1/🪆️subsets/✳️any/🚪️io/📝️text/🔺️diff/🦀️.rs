//! 📝️ Physical text diff representation.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::WriterArtifact;
use crate::{document_child_handle_with_text, WriterSnapshot};
use protocol::MutationDiff;
use crate::schema::diff::*;

impl protocol::DiffText for WriterDiff {
fn print_diff(&self) -> String {
    semio_framework_pack_json::to_json_string(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(error.to_string()).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
}
}
}

pub use diff_codec::*;
