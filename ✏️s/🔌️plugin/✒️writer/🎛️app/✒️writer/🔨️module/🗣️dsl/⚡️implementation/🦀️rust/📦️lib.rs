//! 📜️ Writer app — textual document grammar surface + laws (constitutional: dsl).

use writer::WriterProjection;

/// 📄️ The `jack` example document, handcrafted in the `.writer` DSL (see `store::DocumentDsl`) instead
/// of JSON — {@link writer_engine::jack_example_document}/{@link writer_engine::jack_example_json} are
/// the only ways it should be consumed.
pub const JACK_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/✒️writer/📚️example/✒️jack.writer");
/// 📄️ The `dag.jack` example document, handcrafted in the `.writer` DSL — see {@link JACK_EXAMPLE_TEXT}.
pub const DAG_JACK_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/✒️writer/📚️example/✒️dag.jack.writer");

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

    /// ✍️ Hand-built representative document exercising the multiline/quoted-text path
    /// (verbatim from the original file's `🔖️DslAndOpText` test region).
    fn jack_projection() -> WriterProjection {
        WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into() }
    }

    #[test]
    fn writer_dsl_round_trips_empty_and_jack_projections() {
        store::test_support::assert_dsl_round_trip(&writer_engine::empty_writer_projection());
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
