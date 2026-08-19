//! ✒️ Writer artifact — the document entity this plugin's app edits.

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

//#region 🔖️Constants
pub const WRITER_DOCUMENT_SCHEMA: &str = "writer.document";

/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1's canonical surface
/// coordinate for this artifact — lives at the ARTIFACT level (not under `editor`/`viewer`) so a
/// viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind = "s.writer.writer"` matches `definition()`'s own `"s.writer.schema.artifact"`
/// capability row (`descriptor(b"s.writer.writer")`/`claim(schema, "s.writer.writer")` below, not
/// guessed); `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location
/// — i.e. the canonical surface id is `s.writer.writer@1/*#editor` / `s.writer.writer@1/*#viewer`.
pub const WRITER_DIALECT: Dialect = Dialect { artifact_kind: "s.writer.writer", standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Constants

//#region 🔖️Types
/// 📷️ Editor viewport transform — session-only runtime state (flattened on the artifact for schema).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WriterCamera {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_zoom")]
    pub zoom: f64,
}

impl Default for WriterCamera {
    fn default() -> Self {
        default_camera()
    }
}

/// 📐️ Editor text selection range.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WriterEditorSelection {
    pub start: usize,
    pub end: usize,
}

/// ⚙️ Editor chrome settings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct WriterEditorSettings {
    pub show_line_numbers: bool,
    pub font_px: u32,
    pub line_height: u32,
    pub tab_size: u32,
}

impl Default for WriterEditorSettings {
    fn default() -> Self {
        Self { show_line_numbers: true, font_px: 14, line_height: 22, tab_size: 2 }
    }
}

pub async fn default_zoom() -> f64 {
    1.0
}

pub async fn default_uri() -> String {
    "writer://empty".into()
}

pub async fn default_camera() -> WriterCamera {
    WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 }
}

pub use crate::artifacts::writer::snapshot::schema::WriterSnapshot;
//#endregion 🔖️Types

//#region 🔖️DocumentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio.document` document — writer's
/// authored text now lives in this composed child's block tree rather than inline on
/// `WriterSnapshot` (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, `writer→C:document`).
pub type WriterDocumentChild = store::ArtifactChild<SemioDocumentSnapshot>;

/// 🌉 REAL bidirectional converter between writer's plain-text/language-id working representation
/// and the composed child's own `SemioDocumentSnapshot` block tree (the "ModelBridge" pattern from
/// `📓️wave3-reports/cad-report.md`) — writer's whole authored body becomes one `DocBlock::Code`
/// leaf (`language` = `language_id`, `text` = the raw buffer), which round-trips losslessly: `Code`
/// carries no run/formatting structure to lose, exactly matching what `text: String` used to carry
/// verbatim. `"plaintext"`/empty language ids map to `None` (no fenced-language hint).
pub async fn document_snapshot_from_text(text: &str, language_id: &str) -> SemioDocumentSnapshot {
    let language = (!language_id.is_empty() && language_id != "plaintext").then(|| language_id.to_string());
    SemioDocumentSnapshot { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: Vec::new(), images: Vec::new(), blocks: vec![DocBlock::Code { language, text: text.to_string() }] }
}

/// 🌉 Inverse of [`document_snapshot_from_text`] — concatenates every `Code` block's body (the
/// common, lossless case is exactly one); any non-`Code` block is honestly skipped rather than
/// fabricating prose from block content this plugin never authored.
pub async fn text_from_document_snapshot(snapshot: &SemioDocumentSnapshot) -> String {
    snapshot
        .blocks
        .iter()
        .filter_map(|block| match block {
            DocBlock::Code { text, .. } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// 🕸️ Deterministic content-addressed CHILD handle for the document — same `(child_id, target)`
/// for identical `(text, language_id)`, a different pair once the content actually changes; the
/// handle alone is the change signal the parent's diff/mutation machinery reads without ever
/// comparing embedded content, mirroring lowpoly's `mesh_child_handle`/cad's `cad_model_child_handle`.
pub async fn document_child_handle(id: &str, text: &str, language_id: &str) -> WriterDocumentChild {
    use std::hash::{Hash, Hasher};
    let snapshot = document_snapshot_from_text(text, language_id);
    let content_json = serde_json::to_string(&snapshot).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("document-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "document".into() };
    let target = store::os_io::ArtifactRef { artifact_id: format!("{id}-document"), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️DocumentBridge

//#region 🔖️WorkingScene
/// 🌱 Ephemeral, session-side working representation of the composed document child's live text —
/// NEVER persisted, NEVER a durable field on `WriterSnapshot` itself (matches the `EngineRep`
/// contract §`design-full-plan.md` corrigendum: wholly derived, droppable at any instant, rebuilt
/// from base). Exists because writer edits at keystroke granularity and no `LinkResolver`/child-
/// dispatch seam is wired into `ArtifactApp::handle` yet (checked: `🔌️plugin/🦀️component.rs` has
/// no such plumbing — same standing gap cad/lowpoly's reports both document); until one exists, the
/// only way a persisted content-addressed HANDLE can round-trip to real text within one process is
/// this cache, keyed by `WriterDocumentChild::child_id` — mirrors lowpoly's
/// `LowpolyScratch.mesh_workspace: HashMap<String, String>` (`📓️wave3-reports/lowpoly-report.md`).
///
/// ⚠️ Same documented gap as lowpoly's `StaleMeshWorkspace`: store-level undo/redo bypasses
/// `ArtifactApp::handle` entirely, so a handle can in principle go uncached (a fresh process, or an
/// undo past this session's history) — `writer_text`/`writer_text_for_handle` fail soft (empty
/// string) rather than panicking. A real fix needs child-document resolution, which no WASM-guest
/// plugin in this repo has yet (repeatedly flagged in this ticket already).
pub struct WriterWorkingScene {
    pub text: String,
    pub language_id: String,
}

thread_local! {
    static WRITER_SCRATCH: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// 📝 Seeds the scratch cache for a handle — call whenever new text content is about to become a
/// document's `document` field (every mutation-diff/fixture builder in this plugin does, via
/// [`document_child_handle_and_cache`]).
pub async fn cache_writer_document_text(child_id: &str, text: &str) {
    WRITER_SCRATCH.with(|cache| cache.borrow_mut().insert(child_id.to_string(), text.to_string()));
}

/// 🔎 Reads the cached live text for a document child handle — empty string (never a panic) when
/// nothing has cached it yet (see this region's module doc comment for why that can happen).
pub async fn writer_text_for_handle(handle: &WriterDocumentChild) -> String {
    WRITER_SCRATCH.with(|cache| cache.borrow().get(&handle.child_id).cloned().unwrap_or_default())
}

/// 🔎 Reads the current document's live text off its `document` child handle — the single read
/// call site every render/inference/export path in this plugin uses instead of the old
/// `snapshot.text` field access.
pub async fn writer_text(snapshot: &WriterSnapshot) -> String {
    writer_text_for_handle(&snapshot.document)
}

/// 🏗️ Mints a new content-addressed handle AND seeds the scratch cache with its text in one call —
/// the standard way every mutation-diff/fixture builder in this plugin creates a `document` field
/// value; never construct a handle without also caching, or [`writer_text`] will read back empty.
pub async fn document_child_handle_and_cache(id: &str, text: &str, language_id: &str) -> WriterDocumentChild {
    let handle = document_child_handle(id, text, language_id);
    cache_writer_document_text(&handle.child_id, text);
    handle
}

/// 🏗️ Builds a full `WriterSnapshot` from literal text — the standard fixture/import constructor
/// replacing the old 5-field `WriterSnapshot { ..., text }` struct literal now that `document` is a
/// composed child handle, not a plain field.
pub async fn writer_snapshot_with_text(schema: &str, id: &str, language_id: &str, uri: &str, text: &str) -> WriterSnapshot {
    WriterSnapshot { schema: schema.into(), id: id.into(), language_id: language_id.into(), uri: uri.into(), document: document_child_handle_and_cache(id, text, language_id) }
}
//#endregion 🔖️WorkingScene

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "text.document".into(),
        name: "Text Document".into(),
        source_format: WRITER_DOCUMENT_SCHEMA.into(),
        component_kind: "writer".into(),
        dimension: "text".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: WRITER_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_keeps_the_media_schema_matching_the_store_schema() {
        assert_eq!(artifact_kind().schema, WRITER_DOCUMENT_SCHEMA);
        assert_eq!(WRITER_DOCUMENT_SCHEMA, "writer.document");
    }

    #[semio_framework_async_macros::async_test]
    async fn default_camera_is_centered_and_unzoomed() {
        assert_eq!(WriterCamera::default(), WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 });
    }
}
//#endregion 🧪️Tests
//#region 🔖️Declaration
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.writer")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.writer.writer")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.writer.writer")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.writer.writer.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.writer.writer.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.writer@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.writer@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.composer.txt")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.txt@utf-8/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.txt@utf-8/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.composer.md")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.md@commonmark/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.md@commonmark/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"writer.document:writer")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "writer.document")?)?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "writer")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.writer.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Writer")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Writer")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.writer.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Writer")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Writer")?)?)
}

/// 🗿️ Declaration-tree root (design.md §1/§2) — ONE standard (`1`), atomic cutover: the old
/// `.artifact(declaration())` + `.editor::<>()`/`.viewer::<>()` channel is deleted in the SAME pass
/// (plugin root `🦀️component.rs`), never coexisting with this. `kind` uses `WRITER_DIALECT`'s own
/// `artifact_kind` ("s.writer.writer") — the documented canonical coordinate, not guessed.
pub async fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse(WRITER_DIALECT.artifact_kind).expect("canonical writer kind"), localization: &[], standards: vec![crate::artifacts::writer::standards::v1::standard()] }
}
//#endregion 🔖️Declaration
