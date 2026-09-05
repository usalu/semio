//! ✒️ Writer artifact — the document entity this plugin's app edits.

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

pub fn default_zoom() -> f64 {
    1.0
}

pub fn default_uri() -> String {
    "writer://empty".into()
}

pub fn default_camera() -> WriterCamera {
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
pub fn document_snapshot_from_text(text: &str, language_id: &str) -> SemioDocumentSnapshot {
    let language = (!language_id.is_empty() && language_id != "plaintext").then(|| language_id.to_string());
    SemioDocumentSnapshot { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: Vec::new(), images: Vec::new(), blocks: vec![DocBlock::Code { language, text: text.to_string() }] }
}

/// 🌉 Inverse of [`document_snapshot_from_text`] — concatenates every `Code` block's body (the
/// common, lossless case is exactly one); any non-`Code` block is honestly skipped rather than
/// fabricating prose from block content this plugin never authored.
pub fn text_from_document_snapshot(snapshot: &SemioDocumentSnapshot) -> String {
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
pub fn document_child_handle(id: &str, text: &str, language_id: &str) -> WriterDocumentChild {
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
/// 🌱 Ephemeral, artifact-instance working representation of the composed document child's live
/// text. The owner is the `WriterDocumentChild` embedded by one snapshot, so independent apps and
/// stale handles cannot observe or replace one another's text. Persistence and DSL codecs skip
/// the text; constructors and mutations materialize their immutable local owner directly.
pub struct WriterWorkingScene {
    pub text: String,
    pub language_id: String,
}

/// 📝 Materializes text on this exact artifact-child owner without publishing process state.
pub fn attach_writer_document_text(handle: &mut WriterDocumentChild, text: &str) {
    handle.set_local_text(Arc::<str>::from(text));
}

/// 🔎 Reads this child owner's live text, or empty text when the child is unresolved.
pub fn writer_text_for_handle(handle: &WriterDocumentChild) -> String {
    handle.local_text().unwrap_or_default().to_string()
}

/// 🔎 Reads the current document's live text off its `document` child handle — the single read
/// call site every render/inference/export path in this plugin uses instead of the old
/// `snapshot.text` field access.
pub fn writer_text(snapshot: &WriterSnapshot) -> String {
    writer_text_for_handle(&snapshot.document)
}

/// 🧵️ Retains the immutable child-text owner for bounded worker jobs without cloning its bytes.
pub fn writer_text_owner(snapshot: &WriterSnapshot) -> Arc<str> {
    snapshot.document.local_text_owner().unwrap_or_else(|| Arc::<str>::from(""))
}

/// 🏗️ Mints a content-addressed handle carrying its artifact-instance text owner.
pub fn document_child_handle_with_text(id: &str, text: &str, language_id: &str) -> WriterDocumentChild {
    document_child_handle(id, text, language_id).with_local_text(Arc::<str>::from(text))
}

/// 🏗️ Builds a full `WriterSnapshot` from literal text — the standard fixture/import constructor
/// replacing the old 5-field `WriterSnapshot { ..., text }` struct literal now that `document` is a
/// composed child handle, not a plain field.
pub fn writer_snapshot_with_text(schema: &str, id: &str, language_id: &str, uri: &str, text: &str) -> WriterSnapshot {
    WriterSnapshot { schema: schema.into(), id: id.into(), language_id: language_id.into(), uri: uri.into(), document: document_child_handle_with_text(id, text, language_id) }
}
//#endregion 🔖️WorkingScene

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
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

    #[semio_framework_async_macros::async_test]
    async fn child_local_text_fixture_proves_bounded_identity_isolation_aba_and_wire_omission() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/⚖️writer-child-local-text-law.json")).expect("language-neutral writer child fixture");
        let cases = fixture["cases"].as_array().expect("fixture cases");
        assert_eq!(fixture["schemaVersion"], 1);
        assert_eq!(cases.len(), fixture["maximumCases"].as_u64().expect("bounded maximum") as usize);
        assert_eq!(cases.len(), 4);

        for case in cases {
            let law = case["law"].as_str().expect("law");
            let first = case["first"].as_str().expect("first");
            let second = case["second"].as_str().expect("second");
            let expected = case["expected"].as_str().expect("expected");
            match law {
                "cloneIdentity" => {
                    let snapshot = writer_snapshot_with_text(WRITER_DOCUMENT_SCHEMA, "identity", "plaintext", "writer://identity", first);
                    let retained = writer_text_owner(&snapshot);
                    let cloned = snapshot.clone();
                    let cloned_owner = writer_text_owner(&cloned);
                    assert!(Arc::ptr_eq(&retained, &cloned_owner));
                    assert_eq!(&*cloned_owner, expected);
                    assert_eq!(Arc::strong_count(&retained), 4);
                }
                "instanceIsolation" => {
                    let mut left = document_child_handle("collision", "", "plaintext");
                    let mut right = left.clone();
                    attach_writer_document_text(&mut left, first);
                    attach_writer_document_text(&mut right, second);
                    assert_eq!(format!("{}|{}", writer_text_for_handle(&left), writer_text_for_handle(&right)), expected);
                }
                "abaIsolation" => {
                    let mut stale = document_child_handle("aba", "", "plaintext");
                    attach_writer_document_text(&mut stale, first);
                    let mut reused_identity = document_child_handle("aba", "", "plaintext");
                    assert_eq!(stale.child_id, reused_identity.child_id);
                    attach_writer_document_text(&mut reused_identity, second);
                    assert_eq!(format!("{}|{}", writer_text_for_handle(&stale), writer_text_for_handle(&reused_identity)), expected);
                }
                "wireOmission" => {
                    let handle = document_child_handle_with_text("wire", first, "plaintext");
                    let wire = serde_json::to_value(&handle).expect("third-party serde oracle serializes handle");
                    assert!(wire.get("localText").is_none());
                    let decoded: WriterDocumentChild = serde_json::from_value(wire).expect("third-party serde oracle decodes handle");
                    assert_eq!(writer_text_for_handle(&decoded), expected);
                    assert_eq!(writer_text_for_handle(&handle), first);
                }
                other => panic!("unexpected writer child law {other}"),
            }
        }
    }
}
//#endregion 🧪️Tests
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.writer.writer")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.writer.writer")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.writer.writer")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.writer.writer.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.writer.writer.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.writer.writer@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.writer.writer@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.composer.txt")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.txt@utf-8/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.txt@utf-8/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.composer.md")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.md@commonmark/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.md@commonmark/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"writer.document:writer")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "writer.document")?)?
                .claim(ArtifactIdentityClaim::codec_extension("writer.document", "writer")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Writer")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Writer")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.writer.writer.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Writer")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Writer")?)?)
}

/// 🗿️ Declaration-tree root (design.md §1/§2) — ONE standard (`1`), atomic cutover: the old
/// `.artifact(declaration())` + `.editor::<>()`/`.viewer::<>()` channel is deleted in the SAME pass
/// (plugin root `🦀️.rs`), never coexisting with this. `kind` uses `WRITER_DIALECT`'s own
/// `artifact_kind` ("s.writer.writer") — the documented canonical coordinate, not guessed.
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse(WRITER_DIALECT.artifact_kind).expect("canonical writer kind"), localization: &[], standards: vec![crate::artifacts::writer::standards::v1::standard()] }
}
//#endregion 🔖️Declaration
