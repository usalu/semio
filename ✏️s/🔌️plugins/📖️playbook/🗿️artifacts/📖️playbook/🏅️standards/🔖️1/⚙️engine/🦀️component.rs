//! ⚙️ Playbook artifact — headless compute over the `PlaybookSnapshot` (constitutional: engine).
//!
//! Pure compute over the shared `playbook` kernel crate's step/block domain is re-exported here; this
//! component adds the media I/O declaration, the chapter-import payload shape, and the block-shell
//! builder — every doc-pure helper with more than one consumer across the app's taxonomy tree (a helper
//! with exactly one consumer lives in that consumer's component file). `PlaybookConfig` (view state, not
//! document state) does NOT live here — see `🎛️apps/📖️playbook/🎚️config/🦀️component.rs`.

use crate::artifacts::playbook::PLAYBOOK_DOCUMENT_SCHEMA;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
pub use crate::artifacts::playbook::{empty_playbook_snapshot, flatten_playbook_blocks, PlaybookBlock};
//#endregion 🔖️Types

//#region 🔖️Register
/// 🗂️ Registers `PlaybookSnapshot`'s pack↔dsl codec under `PLAYBOOK_DOCUMENT_SCHEMA` so `framework/sync`'s
/// `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse playbook documents without
/// depending on this crate's concrete `Projection`/`Mutation` types. Called from the plugin root's
/// `semio_plugin!{ setup: … }`.
pub fn register() {
    crate::artifacts::playbook::io_registry::register();

    register_artifact_schema();
    register_artifact_inference();
    crate::apps::playbook::config::schema::register_app_schema();
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::playbook::PlaybookPlayApp>(PLAYBOOK_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "playbook.playbook",
        extension: Some("playbook"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::playbook::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::playbook::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playbook.playbook"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "playbook.playbook.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::playbook::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::playbook::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playbook.playbook.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "playbook.playbook.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::playbook::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::playbook::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("playbook.playbook.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "playbook.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playbook.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "playbook.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playbook.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports plus one
/// extra input, `chapters:in` (Text×Document, kind `text.document`, `Many` — fans in from several
/// upstream `writer` nodes' `"text:out"`).
pub fn playbook_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Text, form: semio_framework_plugin::MediaForm::Document },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "chapters:in".into(),
            label: "Chapters".into(),
            direction: semio_framework_plugin::MediaPortDirection::In,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Text, form: semio_framework_plugin::MediaForm::Document },
            kind_id: Some("text.document".into()),
            required: false,
            multiplicity: semio_framework_plugin::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "text.playbook".into(), name: "Playbook".into(), dimension: "text".into(), component_kind: "playbook".into() },
    }
}

/// 📥️ Mirror of `writer_engine::WriterChapterPayload` — the JSON shape `"chapters:in"` decodes (a
/// writer document's text as one "chapter"). Kept structurally identical rather than shared: the two
/// apps are on opposite sides of the wire and this crate must not depend on the writer plugin.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybookChapterPayload {
    pub id: String,
    pub title: String,
    pub text: String,
    #[serde(default)]
    pub language_id: String,
}
//#endregion 🔖️Io

//#region 🔖️DocumentHelpers
/// 🧱️ A blank block of the requested kind — every optional field defaulted, ready to be edited.
pub fn default_block(id: String, kind: &str) -> PlaybookBlock {
    PlaybookBlock {
        id,
        label: kind.into(),
        kind: kind.into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playbook_config_dsl_placeholder_module_compiles() {
        // 🩹️ Sanity marker so this file always has at least one leaf test in isolation (the substantive
        // engine-region tests live in the app's config/io/command tests, which exercise these helpers
        // through real dispatch paths).
        assert_eq!(default_block("b1".into(), "text").kind, "text");
    }

    #[test]
    fn playbook_io_declares_the_extra_chapters_in_port() {
        let io = playbook_io();
        let ports = io.all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let chapters_in = ports.iter().find(|port| port.id == "chapters:in").expect("chapters:in port declared");
        assert_eq!(chapters_in.kind_id.as_deref(), Some("text.document"));
        assert_eq!(chapters_in.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
        assert_eq!(chapters_in.direction, semio_framework_plugin::MediaPortDirection::In);
    }
}
//#endregion 🧪️Tests

//#region 🔖️ArtifactEngine
/// 🧬️ UI-independent document engine — every transition is a `PlaybookMutation`.
pub struct PlaybookEngine {
    artifact: crate::artifacts::playbook::schema::PlaybookArtifact,
    snapshot: crate::artifacts::playbook::PlaybookSnapshot,
}

impl PlaybookEngine {
    pub fn new(snapshot: crate::artifacts::playbook::PlaybookSnapshot) -> Self {
        let artifact = crate::artifacts::playbook::schema::PlaybookArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> crate::artifacts::playbook::PlaybookSnapshot {
        self.snapshot
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🔖️SchemaRegistry
/// 📌️ Registers the twenty handcrafted schema leaves for `s.playbook.playbook`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::playbook::schema::playbook_artifact_schema_descriptor());
}

/// 💡️ Registers `s.playbook.playbook.inference`'s facet leaves — sibling registry to
/// `register_artifact_schema()` above (ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inference() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::playbook::standards::v1::subsets::any::schema::inferences::playbook_artifact_inference_descriptor());
}
//#endregion 🔖️SchemaRegistry
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::playbook::standards::v1::subsets::any::schema::PlaybookComposer as PlaybookAnyComposer;
    use crate::artifacts::playbook::standards::v1::subsets::any::schema::PlaybookBuilder as PlaybookAnyBuilder;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const PLAYBOOK_DIALECT: Dialect = Dialect { artifact_kind: "s.playbook", standard: StandardId("1"), subset: SubsetId("*") };
    const PLAYBOOK_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::playbook::PlaybookSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == PLAYBOOK_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => PlaybookAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => PlaybookAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "PlaybookComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == PLAYBOOK_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::playbook::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "PlaybookComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
    fn compose_export_txt(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::playbook::io::export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_TXT_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    fn compose_export_pdf(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::playbook::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_DOCX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    fn compose_export_docx(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::playbook::io::export::serializers::artifacts::docx::v_ecma_376::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_DOCX_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
    fn compose_export_md(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::playbook::io::export::serializers::artifacts::md::v_commonmark::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_MD_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::playbook::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<PlaybookAnyComposer>(),
            ComposerEntry { writes: EXPORT_TXT_DIALECT, reads: &[PLAYBOOK_DIALECT], compose: compose_export_txt },
            ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[PLAYBOOK_DIALECT], compose: compose_export_pdf },
            ComposerEntry { writes: EXPORT_DOCX_DIALECT, reads: &[PLAYBOOK_DIALECT], compose: compose_export_docx },
            ComposerEntry { writes: EXPORT_MD_DIALECT, reads: &[PLAYBOOK_DIALECT], compose: compose_export_md },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[PLAYBOOK_DIALECT], compose: compose_export_json },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
