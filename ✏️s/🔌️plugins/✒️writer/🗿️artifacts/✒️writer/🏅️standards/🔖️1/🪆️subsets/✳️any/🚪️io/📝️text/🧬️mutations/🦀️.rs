//! 🔧 Writer artifact — OpText/OpBinary codecs + grammar for serializing `WriterMutation`.

use crate::schema::mutations::WriterMutation;

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
//#endregion 🧪️Tests




pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] =
    &[("rename-writer", rename_writer::TEXT_OPCODE), ("change-uri", change_uri::TEXT_OPCODE), ("change-language", change_language::TEXT_OPCODE), ("edit-text", edit_text::TEXT_OPCODE), ("splice-text", splice_text::TEXT_OPCODE)];

pub use semio_s_artifact_writer_writer_tests::standards::v1::subsets::any::io::text::mutations::change_language;

pub use semio_s_artifact_writer_writer_tests::standards::v1::subsets::any::io::text::mutations::edit_text;

pub use semio_s_artifact_writer_writer_tests::standards::v1::subsets::any::io::text::mutations::splice_text;

pub use semio_s_artifact_writer_writer_tests::standards::v1::subsets::any::io::text::mutations::change_uri;

pub use semio_s_artifact_writer_writer_tests::standards::v1::subsets::any::io::text::mutations::rename_writer;

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

use crate::WriterSnapshot;
use protocol::Mutation;

/// 📥️ Decodes the internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) projection the
/// committed `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` vectors carry.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_writer_mutation_json(text: &str) -> Result<WriterMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;
