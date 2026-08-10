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
    crate::artifacts::playbook::composer::register();

    register_artifact_schema();
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
//#endregion 🔖️SchemaRegistry

