//! 📜️ Writer artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::writer::WriterProjection;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


/// 📄️ The `jack` example document, handcrafted in the `.writer` DSL (see `store::DocumentDsl`) instead
/// of JSON — {@link crate::artifacts::writer::engine::jack_example_document}/
/// {@link crate::artifacts::writer::engine::jack_example_json} are the only ways it should be consumed.
pub const JACK_EXAMPLE_TEXT: &str = include_str!("../📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.writer.writer.jack-writer.dsl.semio");
/// 📄️ The `dag.jack` example document, handcrafted in the `.writer` DSL — see {@link JACK_EXAMPLE_TEXT}.
pub const DAG_JACK_EXAMPLE_TEXT: &str = include_str!("../📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.writer.writer.dag-jack-writer.dsl.semio");

/// 📖️ Parses `.writer` DSL text into a `WriterProjection`.
pub fn parse_dsl(text: &str) -> Result<WriterProjection, store::TextError> {
    <WriterProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `WriterProjection` back to `.writer` DSL text.
pub fn print_dsl(projection: &WriterProjection) -> String {
    store::DocumentDsl::print_dsl(projection)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::writer::engine;

    #[test]
    fn jack_example_dsl_round_trips() {
        let document = parse_dsl(JACK_EXAMPLE_TEXT).expect("parse jack example");
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn dag_jack_example_dsl_round_trips() {
        let document = parse_dsl(DAG_JACK_EXAMPLE_TEXT).expect("parse dag.jack example");
        store::test_support::assert_dsl_round_trip(&document);
    }

    /// ✍️ Hand-built representative document exercising the multiline/quoted-text path.
    fn jack_projection() -> WriterProjection {
        WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into() }
    }

    #[test]
    fn writer_dsl_round_trips_empty_and_jack_projections() {
        store::test_support::assert_dsl_round_trip(&engine::empty_writer_projection());
        store::test_support::assert_dsl_round_trip(&jack_projection());
    }

    #[test]
    fn writer_dsl_prints_readable_multiline_text() {
        let printed = print_dsl(&jack_projection());
        // Bare-ident-shaped scalars print unquoted (`is_bare_ident`); `writer://jack` contains `:`
        // and `/`, so it isn't bare and stays quoted.
        assert!(printed.contains("schema=writer.document"));
        assert!(printed.contains("id=jack"));
        assert!(printed.contains("language-id=jack"));
        assert!(printed.contains("uri=\"writer://jack\""));
        // `#[dsl(lang = "jack")]` prints `text` as a fenced ```jack verbatim block (`Shape::Embed`)
        // instead of an escaped-quoted string, so the embedded query keeps its raw newlines and its
        // own `"` needs no backslash-escaping.
        assert!(printed.contains("text=```jack\nMATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name\n```"));
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

