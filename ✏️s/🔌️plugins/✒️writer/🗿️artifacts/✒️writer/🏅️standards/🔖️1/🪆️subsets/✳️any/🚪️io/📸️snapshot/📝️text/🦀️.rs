//! 📜️ Writer artifact — textual document grammar surface + laws (constitutional: dsl). Owns the
//! REAL `store::ArtifactDsl` impl for `WriterSnapshot` (design.md §1 CORRECTION: the native codec
//! is one bidirectional thing, unsplit, so it lives here rather than mirrored under import/export).

use crate::{schema, WriterDocumentChild, WriterSnapshot};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ `ArtifactDsl` over the derived spec-driven text of `WriterSnapshot::__dsl_spec()`, the same record
/// the pack encodes — the committed `🗣️.dsl.semio` examples are authored in this format.
impl store::ArtifactDsl for WriterSnapshot {
    const EXTENSION: &'static str = "writer";
    fn envelope_id() -> &'static str {
        "writer.writer"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        let mut snapshot = Self::__dsl_from_record(&record)?;
        crate::attach_writer_document_text(&mut snapshot.document, &snapshot.text.clone());
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
//#endregion 🔖️HandcraftedArtifactDsl

/// 📄️ The `jack` example document, handcrafted in the `.writer` DSL (see `store::ArtifactDsl`) instead
/// of JSON — {@link jack_example_document}/{@link jack_example_json} are the only ways it should be
/// consumed.
pub const JACK_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
/// 📄️ The `dag.jack` example document, handcrafted in the `.writer` DSL — see {@link JACK_EXAMPLE_TEXT}.
pub const DAG_JACK_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🧪️dag-example/🗣️.dsl.semio");



/// 📖️ Parses `.writer` DSL text into a `WriterSnapshot`.
pub fn parse_dsl(text: &str) -> Result<WriterSnapshot, store::TextError> {
    <WriterSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `WriterSnapshot` back to `.writer` DSL text.
pub fn print_dsl(projection: &WriterSnapshot) -> String {
    store::ArtifactDsl::print_dsl(projection)
}

//#region 🔖️ExternalBridges
/// 📖️ [`parse_dsl`] with a plain-`String` error, reachable from OUTSIDE this crate — `store` is a
/// private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named by the
/// `✒️mutate-writer-1` test adapter that has to read the committed `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_writer_dsl(text: &str) -> Result<WriterSnapshot, String> {
    <WriterSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ [`print_dsl`] under a name an external caller can reach, paired with [`parse_writer_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_writer_dsl(snapshot: &WriterSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges

//#region 🔖️Examples
/// 📄️ The `jack` example, parsed once from {@link JACK_EXAMPLE_TEXT} — the source of truth for every
/// call site below (`setActiveExample`, `.example("jack", ...)`, tests, "file-text"); never re-embed the
/// raw text.
pub fn jack_example_document() -> WriterSnapshot {
    parse_dsl(JACK_EXAMPLE_TEXT).unwrap_or_else(|_| schema::empty_writer_snapshot())
}

/// 📄️ JSON re-serialization of {@link jack_example_document}, for the framework-generic call sites
/// (`.example(...)`, `render(...)`) that still take a document as a JSON string.
pub fn jack_example_json() -> String {
    dsl::os_pack::json::to_json_string(&jack_example_document())
}

/// 📄️ The `dag.jack` example, parsed once from {@link DAG_JACK_EXAMPLE_TEXT} — see {@link jack_example_document}.
pub fn dag_jack_example_document() -> WriterSnapshot {
    parse_dsl(DAG_JACK_EXAMPLE_TEXT).unwrap_or_else(|_| schema::empty_writer_snapshot())
}

/// 📄️ JSON re-serialization of {@link dag_jack_example_document} — see {@link jack_example_json}.
pub fn dag_jack_example_json() -> String {
    dsl::os_pack::json::to_json_string(&dag_jack_example_document())
}
//#endregion 🔖️Examples

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

