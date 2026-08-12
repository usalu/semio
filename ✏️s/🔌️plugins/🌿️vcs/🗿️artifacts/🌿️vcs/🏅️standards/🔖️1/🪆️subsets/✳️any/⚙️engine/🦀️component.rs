//! ⚙️ VCS artifact — headless compute (was: constitutional `engine`).

use crate::artifacts::vcs::{VcsSnapshot, VCS_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
pub fn empty_vcs_snapshot() -> VcsSnapshot {
    VcsSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers `VcsSnapshot`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse vcs-play
/// documents without depending on this crate's concrete snapshot/mutation types. Called by
/// `semio_plugin!`'s `setup:` hook — was the old bundle crate's `register_vcs_exports()`.
pub fn register() {
    crate::artifacts::vcs::io_registry::register();

    register_artifact_schema();
    register_artifact_inference();
    register_pilot_languages();
    crate::apps::vcs::config::schema::register_app_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::vcs::VcsPlayApp>(VCS_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.document",
        extension: Some("vcs"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::vcs::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::vcs::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::vcs::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::vcs::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::vcs::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::vcs::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("vcs.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️SchemaRegistry
/// 📌️ Registers the twenty handcrafted schema leaves for `s.vcs.vcs`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::vcs::schema::vcs_artifact_schema_descriptor());
}

/// 💡️ Registers `s.vcs.vcs.inference`'s facet leaves into the OS-wide inference catalog — sibling
/// to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inference() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::vcs::standards::v1::subsets::any::schema::inferences::vcs_artifact_inference_descriptor());
}
//#endregion 🔖️SchemaRegistry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_vcs_snapshot();
        assert_eq!(snapshot.schema, VCS_DOCUMENT_SCHEMA);
        assert_eq!(snapshot.status, "new");
    }
}
//#endregion 🧪️Tests

//#region 🔖️ArtifactEngine
pub struct VcsDemoEngine {
    artifact: crate::artifacts::vcs::schema::VcsArtifact,
    snapshot: VcsSnapshot,
}

impl VcsDemoEngine {
    pub fn new(snapshot: VcsSnapshot) -> Self {
        let artifact = crate::artifacts::vcs::schema::VcsArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }
}
//#endregion 🔖️ArtifactEngine
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::vcs::standards::v1::subsets::any::schema::VcsComposer as VcsAnyComposer;
    use crate::artifacts::vcs::standards::v1::subsets::any::schema::VcsBuilder as VcsAnyBuilder;

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
    const VCS_DIALECT: Dialect = Dialect { artifact_kind: "s.vcs", standard: StandardId("1"), subset: SubsetId("*") };
    const VCS_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::vcs::VcsSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == VCS_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => VcsAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => VcsAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "VcsComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == VCS_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let text = match &source.payload {
                IoPayload::Text(t) => t.clone(),
                IoPayload::Binary(b) => String::from_utf8_lossy(b).into_owned(),
            };
            return crate::artifacts::vcs::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "VcsComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let text = crate::artifacts::vcs::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<VcsAnyComposer>(),
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[VCS_DIALECT], compose: compose_export_json },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
