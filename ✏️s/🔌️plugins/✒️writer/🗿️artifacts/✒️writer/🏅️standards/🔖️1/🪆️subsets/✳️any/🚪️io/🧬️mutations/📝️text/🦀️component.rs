//! 🔧 Writer artifact — OpText/OpBinary codecs + grammar for serializing `WriterMutation`.

pub use crate::artifacts::writer::schema::mutations::{
    apply_writer_mutation, inverse_writer_mutation, WriterMutation, RenameWriter, ChangeUri, ChangeLanguage, EditText,
    rename_writer, change_uri, change_language, edit_text,
};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for WriterMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for WriterMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// ✍️ Hand-built representative document — used across the artifact's own component tests.
    fn jack_snapshot() -> crate::artifacts::writer::WriterSnapshot {
        crate::artifacts::writer::writer_snapshot_with_text(
            "writer.document",
            "jack",
            "jack",
            "writer://jack",
            "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name",
        )
    }

    #[test]
    fn writer_op_text_round_trips_every_variant() {
        let jack = jack_snapshot();
        store::os_store::test_support::assert_op_line_round_trip(&WriterMutation::EditText(EditText { text: "line one\nline two".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&WriterMutation::RenameWriter(RenameWriter { new_id: jack.id.clone() }));
        store::os_store::test_support::assert_op_line_round_trip(&WriterMutation::ChangeUri(ChangeUri { new_uri: jack.uri.clone() }));
        store::os_store::test_support::assert_op_line_round_trip(&WriterMutation::ChangeLanguage(ChangeLanguage { new_language_id: jack.language_id.clone() }));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}
