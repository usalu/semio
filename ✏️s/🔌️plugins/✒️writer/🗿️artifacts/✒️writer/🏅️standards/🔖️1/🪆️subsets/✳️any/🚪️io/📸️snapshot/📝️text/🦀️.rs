//! 📜️ Writer artifact — textual document grammar surface + laws (constitutional: dsl). Owns the
//! REAL `store::ArtifactDsl` impl for `WriterSnapshot` (design.md §1 CORRECTION: the native codec
//! is one bidirectional thing, unsplit, so it lives here rather than mirrored under import/export).

use crate::{schema, WriterDocumentChild, WriterSnapshot};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `📐️cad`'s own `enc_child`/`dec_child`, the
/// working reference for a composite subset's handle codec) — a handle is exactly two strings
/// (`child_id`, the target's `ArtifactRef` flattened via `to_uri()`), never the child's own content.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}

fn enc_child(c: &WriterDocumentChild) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_child(s: &str) -> Result<WriterDocumentChild, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
fn print_writer_snapshot_body(s: &WriterSnapshot) -> String {
    format!("schema={}\nid={}\nlanguageId={}\nuri={}\ndocument={}", enc_str(&s.schema), enc_str(&s.id), enc_str(&s.language_id), enc_str(&s.uri), enc_child(&s.document))
}
fn parse_writer_snapshot_body(body: &str) -> Result<WriterSnapshot, String> {
    let mut snapshot = WriterSnapshot::default();
    let mut saw_schema = false;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            snapshot.schema = dec_str(rest)?;
            saw_schema = true;
        } else if let Some(rest) = line.strip_prefix("id=") {
            snapshot.id = dec_str(rest)?;
        } else if let Some(rest) = line.strip_prefix("languageId=") {
            snapshot.language_id = dec_str(rest)?;
        } else if let Some(rest) = line.strip_prefix("uri=") {
            snapshot.uri = dec_str(rest)?;
        } else if let Some(rest) = line.strip_prefix("document=") {
            snapshot.document = dec_child(rest)?;
        } else {
            return Err(format!("writer snapshot: unknown line {line:?}"));
        }
    }
    if !saw_schema {
        return Err("writer snapshot: missing schema line".to_string());
    }
    Ok(snapshot)
}
//#endregion 🔖️TextPrimitives

//#region 🔖️HandcraftedArtifactDsl
/// ✉️ Handcrafted `ArtifactDsl`, real hex/bracket text primitives — same upgrade `📐️cad`/`💠️lowpoly`
/// made when they gained a real `ArtifactChild<S>` slot (the old `dsl::DslRecord`-derive-driven
/// `Self::__dsl_spec()` path cannot express a composed child slot, which has no `dsl::DslField` impl
/// reachable from this crate).
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
        parse_writer_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_writer_snapshot_body(self);
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

/// ✍️ The `jack`/`dag.jack` examples' real query text. Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`:
/// `WriterSnapshot::document` is a composed `s.stdio.semio`/`document` CHILD HANDLE now, so
/// `JACK_EXAMPLE_TEXT`/`DAG_JACK_EXAMPLE_TEXT` themselves only carry the opaque
/// `document=[childId,target]` pair (content-addressed, matching every other composed-child DSL
/// fixture in this ticket — cad's `shapeModel=`/lowpoly's `mesh=` lines are equally opaque). These
/// constants are the honest source of the actual text those handles were minted from — the working-
/// scene cache (`writer_text`) has no way to recover it from the handle alone otherwise, exactly the
/// documented `WriterWorkingScene` gap (`🗿️artifacts/✒️writer/🦀️.rs`'s module doc comment).
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
    let mut document = parse_dsl(JACK_EXAMPLE_TEXT).unwrap_or_else(|_| schema::empty_writer_snapshot());
    crate::attach_writer_document_text(&mut document.document, JACK_QUERY_TEXT);
    document
}

/// 📄️ JSON re-serialization of {@link jack_example_document}, for the framework-generic call sites
/// (`.example(...)`, `render(...)`) that still take a document as a JSON string.
pub fn jack_example_json() -> String {
    dsl::os_pack::json::to_json_string(&jack_example_document())
}

/// 📄️ The `dag.jack` example, parsed once from {@link DAG_JACK_EXAMPLE_TEXT} — see {@link jack_example_document}.
pub fn dag_jack_example_document() -> WriterSnapshot {
    let mut document = parse_dsl(DAG_JACK_EXAMPLE_TEXT).unwrap_or_else(|_| schema::empty_writer_snapshot());
    crate::attach_writer_document_text(&mut document.document, DAG_JACK_QUERY_TEXT);
    document
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
