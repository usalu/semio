//! 🚪️ IO s.program (1/✳️any) — the artifact declaration owns this composer table.
use dsl::ToValue as _;

pub async fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.csv", "stdio.json", "stdio.txt", "stdio.xlsx", "stdio.zip"]
}
pub async fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.csv", "stdio.json", "stdio.txt", "stdio.xlsx", "stdio.zip"]
}

//#region 📊️ExportTables
/// 📊️ One format-neutral program table consumed by structured multi-table exporters. A row is an
/// object's own entries in field-declaration order (`DslValue::Object`'s backing shape) — never a
/// sorted map, matching `dsl::ToValue`'s insertion-order contract.
pub(crate) struct ProgramExportTable {
    pub name: &'static str,
    pub rows: Vec<Vec<(String, dsl::DslValue)>>,
}

struct ProgramIdentity<'a> {
    schema: &'a str,
    knowledge: &'a crate::artifacts::program::ProgramKnowledgeChild,
    benchmarks: &'a crate::artifacts::program::ProgramBenchmarksChild,
}

/// 🖐️ Hand-written rather than `#[derive(ToValue)]`: every field here is a reference
/// (`&'a str`/`&'a ArtifactChild<_>`), and the derive's generated code calls the fully-qualified
/// `ToValue::to_value(&self.field)` form, which would need `&&'a str`/`&&'a ArtifactChild<_>` impls
/// that don't exist. Plain method-call syntax (`self.field.to_value()`) auto-derefs through the
/// reference to the real impl, so a hand-written `impl` sidesteps the gap entirely.
impl dsl::ToValue for ProgramIdentity<'_> {
    fn to_value(&self) -> dsl::DslValue {
        dsl::DslValue::object([("schema".to_string(), self.schema.to_value()), ("knowledge".to_string(), self.knowledge.to_value()), ("benchmarks".to_string(), self.benchmarks.to_value())])
    }
}

fn export_rows<T: dsl::ToValue>(name: &str, records: &[T]) -> Result<Vec<Vec<(String, dsl::DslValue)>>, String> {
    records
        .iter()
        .enumerate()
        .map(|(index, record)| match dsl::ToValue::to_value(record) {
            dsl::DslValue::Object(fields) => Ok(fields),
            _ => Err(format!("program export table {name} row {index} is not an object")),
        })
        .collect()
}

impl ProgramExportTable {
    fn records<T: dsl::ToValue>(name: &'static str, records: &[T]) -> Result<Self, String> {
        Ok(Self { name, rows: export_rows(name, records)? })
    }

    fn singleton<T: dsl::ToValue>(name: &'static str, value: &T) -> Result<Self, String> {
        Ok(Self { name, rows: export_rows(name, std::slice::from_ref(value))? })
    }
}

/// 🧭️ Explicitly projects every persisted program field into a stable named table.
pub(crate) async fn program_export_tables(snapshot: &crate::artifacts::program::ProgramSnapshot) -> Result<Vec<ProgramExportTable>, String> {
    let knowledge = crate::artifacts::program::program_knowledge(snapshot).await;
    let benchmarks = crate::artifacts::program::program_benchmarks(snapshot).await;
    let mut tables = vec![
        ProgramExportTable::singleton("program", &ProgramIdentity { schema: &snapshot.schema, knowledge: &snapshot.knowledge, benchmarks: &snapshot.benchmarks })?,
        ProgramExportTable::singleton("meta", &snapshot.meta)?,
        ProgramExportTable::singleton("project", &snapshot.project)?,
    ];
    macro_rules! registers {
        ($($field:ident),+ $(,)?) => {
            $(tables.push(ProgramExportTable::records(stringify!($field), &snapshot.$field)?);)+
        };
    }
    registers!(
        stakeholders,
        users,
        activities,
        functions,
        elements,
        quantities,
        relationships,
        adjacencies,
        processes,
        flows,
        access_rules,
        operations,
        equipment,
        resources,
        storage,
        environmental,
        human_factors,
        accessibility,
        privacy,
        safety,
        security,
        regulatory,
        site_context,
        organizational,
        services,
        infrastructure,
        information,
        communication,
        wayfinding,
        schedules,
        flexibility,
        growth,
        sustainability,
        resilience,
        costs,
        delivery,
        risks,
        conflicts,
        requirements,
        priorities,
        scenarios,
        options,
        decisions,
        validations,
        performance,
        quality,
        artifacts,
        assumptions,
        constraints,
        compliance_records,
        approvals,
        meetings,
        changes,
        collaboration,
        analyses,
        reports,
        search_filters,
        status_records,
        workshops,
        surveys,
        issues,
        audit_events,
        templates,
    );
    tables.push(ProgramExportTable::records("knowledge", &knowledge)?);
    tables.push(ProgramExportTable::records("benchmarks", &benchmarks)?);
    tables.push(ProgramExportTable::records("traces", &snapshot.traces)?);
    tables.push(ProgramExportTable::singleton("governance", &snapshot.governance)?);
    Ok(tables)
}
//#endregion 📊️ExportTables

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::program::standards::v1::subsets::any::schema::ProgramAnalyzer;
    use crate::artifacts::program::ProgramSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.architect.program", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_CSV: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
    const DEP_XLSX: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    pub struct ProgramComposerComposition;

    impl ArtifactComposition for ProgramComposerComposition {
        type Snapshot = ProgramSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_CSV, DEP_JSON, DEP_TXT, DEP_XLSX, DEP_ZIP]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = ProgramAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_CSV {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::program::io::import::deserializers::artifacts::csv::v_rfc4180::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::program::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::program::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_XLSX {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::program::io::import::deserializers::artifacts::xlsx::v_ecma_376::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_ZIP {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::program::io::import::deserializers::artifacts::zip::v2_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "ProgramComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🗄️ Dissolved out of the former `⚙️engine` root (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — the real `io_registry` module (see the
/// `🧱️block/◻️2d` exemplar's own `🚪️io/🦀️.rs`). The artifact root's OWN `io_registry`
/// shim (`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🦀️.rs`) wraps this one behind a
/// DIFFERENT type (`&'static [&'static ComposerEntry]` vs this module's `&'static [ComposerEntry]`)
/// — any bare `io_registry::entries()` reachable from artifact-root code silently rebinds to that
/// shim instead of this real registry, so every call site fully qualifies this module's path.
pub mod io_registry {
    use crate::artifacts::program::standards::v1::subsets::any::schema::ProgramBuilder as ProgramAnyBuilder;
    use crate::artifacts::program::standards::v1::subsets::any::schema::ProgramComposer as ProgramAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ArtifactBuilder, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource, IoConfidence, IoPayload, StandardId, SubsetId};
    use std::sync::OnceLock;

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
    const PROGRAM_DIALECT: Dialect = Dialect { artifact_kind: "s.architect.program", standard: StandardId("1"), subset: SubsetId("*") };
    const PROGRAM_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    async fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::program::ProgramSnapshot, ComposeError> {
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
    async fn compose_export_zip(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::program::io::export::serializers::artifacts::zip::v2_0::any::serialize_bytes(&snapshot).await.map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_ZIP_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };
    async fn compose_export_csv(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::program::io::export::serializers::artifacts::csv::v_rfc4180::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_CSV_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    async fn compose_export_xlsx(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::program::io::export::serializers::artifacts::xlsx::v_ecma_376::any::serialize_bytes(&snapshot).await.map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_XLSX_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    async fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::program::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub async fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<ProgramAnyComposer>(),
                    ComposerEntry { writes: EXPORT_ZIP_DIALECT, reads: &[PROGRAM_DIALECT], compose: compose_export_zip },
                    ComposerEntry { writes: EXPORT_CSV_DIALECT, reads: &[PROGRAM_DIALECT], compose: compose_export_csv },
                    ComposerEntry { writes: EXPORT_XLSX_DIALECT, reads: &[PROGRAM_DIALECT], compose: compose_export_xlsx },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[PROGRAM_DIALECT], compose: compose_export_json },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
