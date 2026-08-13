//! 📜️ Writer artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::artifacts::writer::WriterSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


/// 📄️ The `jack` example document, handcrafted in the `.writer` DSL (see `store::ArtifactDsl`) instead
/// of JSON — {@link jack_example_document}/{@link jack_example_json} are the only ways it should be
/// consumed.
pub const JACK_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
/// 📄️ The `dag.jack` example document, handcrafted in the `.writer` DSL — see {@link JACK_EXAMPLE_TEXT}.
pub const DAG_JACK_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️dag-example.dsl.semio");

/// ✍️ The `jack`/`dag.jack` examples' real query text. Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`:
/// `WriterSnapshot::document` is a composed `s.stdio.semio.document` CHILD HANDLE now, so
/// `JACK_EXAMPLE_TEXT`/`DAG_JACK_EXAMPLE_TEXT` themselves only carry the opaque
/// `document=[childId,target]` pair (content-addressed, matching every other composed-child DSL
/// fixture in this ticket — cad's `shapeModel=`/lowpoly's `mesh=` lines are equally opaque). These
/// constants are the honest source of the actual text those handles were minted from — the working-
/// scene cache (`writer_text`) has no way to recover it from the handle alone otherwise, exactly the
/// documented `WriterWorkingScene` gap (`🗿️artifacts/✒️writer/🦀️component.rs`'s module doc comment).
const JACK_QUERY_TEXT: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name";
const DAG_JACK_QUERY_TEXT: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a, b";

/// 📖️ Parses `.writer` DSL text into a `WriterSnapshot`.
pub fn parse_dsl(text: &str) -> Result<WriterSnapshot, store::TextError> {
    <WriterSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `WriterSnapshot` back to `.writer` DSL text.
pub fn print_dsl(projection: &WriterSnapshot) -> String {
    store::ArtifactDsl::print_dsl(projection)
}

//#region 🔖️Examples
/// 📄️ The `jack` example, parsed once from {@link JACK_EXAMPLE_TEXT} — the source of truth for every
/// call site below (`setActiveExample`, `.example("jack", ...)`, tests, "file-text"); never re-embed the
/// raw text.
pub fn jack_example_document() -> WriterSnapshot {
    let document = parse_dsl(JACK_EXAMPLE_TEXT).unwrap_or_else(|_| crate::artifacts::writer::schema::empty_writer_snapshot());
    crate::artifacts::writer::cache_writer_document_text(&document.document.child_id, JACK_QUERY_TEXT);
    document
}

/// 📄️ JSON re-serialization of {@link jack_example_document}, for the framework-generic call sites
/// (`.example(...)`, `render(...)`) that still take a document as a JSON string.
pub fn jack_example_json() -> String {
    serde_json::to_string(&jack_example_document()).expect("serialize jack example document")
}

/// 📄️ The `dag.jack` example, parsed once from {@link DAG_JACK_EXAMPLE_TEXT} — see {@link jack_example_document}.
pub fn dag_jack_example_document() -> WriterSnapshot {
    let document = parse_dsl(DAG_JACK_EXAMPLE_TEXT).unwrap_or_else(|_| crate::artifacts::writer::schema::empty_writer_snapshot());
    crate::artifacts::writer::cache_writer_document_text(&document.document.child_id, DAG_JACK_QUERY_TEXT);
    document
}

/// 📄️ JSON re-serialization of {@link dag_jack_example_document} — see {@link jack_example_json}.
pub fn dag_jack_example_json() -> String {
    serde_json::to_string(&dag_jack_example_document()).expect("serialize dag.jack example document")
}
//#endregion 🔖️Examples

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::writer::schema;

    #[test]
    fn jack_example_dsl_round_trips() {
        let document = parse_dsl(JACK_EXAMPLE_TEXT).expect("parse jack example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn dag_jack_example_dsl_round_trips() {
        let document = parse_dsl(DAG_JACK_EXAMPLE_TEXT).expect("parse dag.jack example");
        store::os_store::test_support::assert_dsl_round_trip(&document);
    }

    /// ✍️ Hand-built representative document exercising the multiline/quoted-text path.
    fn jack_snapshot() -> WriterSnapshot {
        crate::artifacts::writer::writer_snapshot_with_text(
            "writer.document",
            "jack",
            "jack",
            "writer://jack",
            "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name",
        )
    }

    #[test]
    fn writer_dsl_round_trips_empty_and_jack_snapshots() {
        store::os_store::test_support::assert_dsl_round_trip(&schema::empty_writer_snapshot());
        store::os_store::test_support::assert_dsl_round_trip(&jack_snapshot());
    }

    /// 📄️ The hand-rolled `document` codec (`📸️snapshot/🦀️component.rs`'s
    /// `print_writer_snapshot_body`) prints one hex-encoded `key=value` line per persistent field —
    /// `document` is now a two-string CHILD HANDLE, not the embedded text, so this law only checks
    /// the scalar fields print readably; the actual text content is proven separately by
    /// `writer_dsl_round_trips_empty_and_jack_snapshots` (round trip) and `writer_text` reads.
    #[test]
    fn writer_dsl_prints_readable_scalar_fields() {
        let printed = print_dsl(&jack_snapshot());
        assert!(printed.contains(&format!("schema={}", hex_encode_for_test("writer.document"))));
        assert!(printed.contains(&format!("id={}", hex_encode_for_test("jack"))));
        assert!(printed.contains(&format!("languageId={}", hex_encode_for_test("jack"))));
        assert!(printed.contains(&format!("uri={}", hex_encode_for_test("writer://jack"))));
        assert!(printed.contains("document=["));
    }

    fn hex_encode_for_test(s: &str) -> String {
        s.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
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

