//! ⚙️ Architect program artifact engine — headless compute over the program projection.
//!
//! The engine is genuinely multi-topic (the ex-`🦴️spine` crate's ten compute domains), so each topic
//! keeps its own sibling `🦀️<topic>.rs` file and this node is the hub: the plugin-runtime `register()`
//! hook plus a flat re-export of every topic, so `crate::artifacts::program::engine::*` reaches all of
//! them without a caller needing to know which topic file owns a given function.

use crate::artifacts::program::ProgramSnapshot;
pub use crate::artifacts::program::engine::adjacency::*;
pub use crate::artifacts::program::engine::analyze::*;
pub use crate::artifacts::program::engine::exchange::*;
pub use crate::artifacts::program::engine::outputs::*;
pub use crate::artifacts::program::engine::report::*;
pub use crate::artifacts::program::engine::search::*;
pub use crate::artifacts::program::engine::status_summary::*;
pub use crate::artifacts::program::engine::template::*;
pub use crate::artifacts::program::engine::trace::*;
pub use crate::artifacts::program::engine::validate::*;

//#region 🔖️Register
/// 🗂️ Registers `ProgramSnapshot`'s pack↔dsl codec under `ARCHITECT_PROGRAM_SCHEMA`. Called from the plugin
/// root's `semio_plugin!{ setup: … }`.
pub fn register() {
    crate::artifacts::program::io_registry::register();

    register_pilot_languages();
    register_artifact_schema();
    register_artifact_inference();
    crate::apps::architect::config::schema::register_app_schema();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::architect::ArchitectPlayApp>(crate::artifacts::program::ARCHITECT_PROGRAM_SCHEMA);
}

/// 🗂️ Plugin setup entry — same as `register`, named for `Plugin::builder(...).setup(...)`.
pub fn register_architect_exports() {
    register();
}


/// 📎 Registers the program artifact schema descriptor into the process-local registry.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::program::schema::program_artifact_schema_descriptor());
}

/// 💡️ Registers the program artifact `💡️inference` descriptor into the OS-wide inference catalog
/// — sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inference() {
    ::schema::register_artifact_inference_descriptor(
        crate::artifacts::program::standards::v1::subsets::any::schema::inferences::program_artifact_inference_descriptor(),
    );
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "architect.program",
        extension: Some("architect"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::program::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::program::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::program::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("architect.program"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "architect.program.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::program::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::program::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("architect.program.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "architect.program.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::program::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::program::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("architect.program.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "program.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::program::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("program.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "program.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("program.spr"),
    });
}

//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::artifacts::program::sample_plugin;

    /// 🧭️ The hub's flat re-export reaches every topic module — one representative entry point per
    /// topic file, so a dropped `pub use` fails here rather than at some distant call site.
    #[test]
    fn the_hub_re_exports_every_engine_topic() {
        use super::*;
        let program = sample_plugin();
        let _ = undirected_edges(&program);
        let _ = run_analysis(&program, crate::artifacts::program::registers::AnalysisKind::Gap);
        let _ = export_registers_csv(&program);
        let _ = build_report(&program, crate::artifacts::program::registers::ReportKind::ExecutiveSummary);
        let _ = search_plugin(&program, &SearchQuery::default(), None, None);
        let _ = status_summary(&program);
        let _ = audit_trail(&program, None);
        let _ = validate_plugin(&program);
    }
}
//#endregion 🧪️Tests



//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent program artifact engine — owns full artifact + cached snapshot.
pub struct ProgramEngine {
    artifact: crate::artifacts::program::schema::ProgramArtifact,
    snapshot: crate::artifacts::program::ProgramSnapshot,
}

impl ProgramEngine {
    pub fn new(snapshot: crate::artifacts::program::ProgramSnapshot) -> Self {
        let artifact = crate::artifacts::program::schema::ProgramArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }
    pub fn into_snapshot(self) -> crate::artifacts::program::ProgramSnapshot {
        self.snapshot
    }
}
//#endregion 🔖️ArtifactEngine
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::program::standards::v1::subsets::any::schema::ProgramComposer as ProgramAnyComposer;
    use crate::artifacts::program::standards::v1::subsets::any::schema::ProgramBuilder as ProgramAnyBuilder;

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
    const PROGRAM_DIALECT: Dialect = Dialect { artifact_kind: "s.program", standard: StandardId("1"), subset: SubsetId("*") };
    const PROGRAM_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::program::ProgramSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == PROGRAM_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => ProgramAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => ProgramAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "ProgramComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == PROGRAM_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::program::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "ProgramComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_zip(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::program::io::export::serializers::artifacts::zip::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_ZIP_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };
    fn compose_export_csv(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::program::io::export::serializers::artifacts::csv::v_rfc4180::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_CSV_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    fn compose_export_xlsx(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::program::io::export::serializers::artifacts::xlsx::v_ecma_376::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_XLSX_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::program::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<ProgramAnyComposer>(),
            ComposerEntry { writes: EXPORT_ZIP_DIALECT, reads: &[PROGRAM_DIALECT], compose: compose_export_zip },
            ComposerEntry { writes: EXPORT_CSV_DIALECT, reads: &[PROGRAM_DIALECT], compose: compose_export_csv },
            ComposerEntry { writes: EXPORT_XLSX_DIALECT, reads: &[PROGRAM_DIALECT], compose: compose_export_xlsx },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[PROGRAM_DIALECT], compose: compose_export_json },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
