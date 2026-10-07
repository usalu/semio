//! 🔧 Writer artifact — OpText/OpBinary codecs + grammar for serializing `WriterMutation`.

use crate::schema::mutations::{apply_writer_mutation, change_language, change_uri, edit_text, inverse_writer_mutation, rename_writer, splice_text, ChangeLanguage, ChangeUri, EditText, RenameWriter, SpliceText, WriterMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for WriterMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}


//#endregion 🔖️HandcraftedOpCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;


pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] =
    &[("rename-writer", crate::standards::v1::subsets::any::io::text::mutations::rename_writer::TEXT_OPCODE), ("change-uri", crate::standards::v1::subsets::any::io::text::mutations::change_uri::TEXT_OPCODE), ("change-language", crate::standards::v1::subsets::any::io::text::mutations::change_language::TEXT_OPCODE), ("edit-text", crate::standards::v1::subsets::any::io::text::mutations::edit_text::TEXT_OPCODE), ("splice-text", crate::standards::v1::subsets::any::io::text::mutations::splice_text::TEXT_OPCODE)];

#[path = "🌐change-language/🦀️.rs"]
pub mod change_language;

#[path = "✏️edit-text/🦀️.rs"]
pub mod edit_text;

#[path = "✂️splice-text/🦀️.rs"]
pub mod splice_text;

#[path = "🔗change-uri/🦀️.rs"]
pub mod change_uri;

#[path = "🏷️rename-writer/🦀️.rs"]
pub mod rename_writer;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::operations::*;
use crate::schema::mutations::WriterMutation;
#[cfg(test)]
use crate::schema::mutations::{ChangeLanguage, ChangeUri, EditText, RenameWriter};
use crate::WriterDiff;
use crate::WriterSnapshot;
use protocol::{Mutation, MutationDiff};

/// 📥️ Decodes the internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) projection the
/// committed `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` vectors carry.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_writer_mutation_json(text: &str) -> Result<WriterMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;
