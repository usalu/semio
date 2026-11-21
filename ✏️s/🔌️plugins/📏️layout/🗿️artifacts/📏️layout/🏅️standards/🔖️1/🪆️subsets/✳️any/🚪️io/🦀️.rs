//! 🚪️ IO s.layout (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from the artifact root's `declaration()`), not per-leaf register().

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as schema;
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.svg", "stdio.pdf", "stdio.step", "stdio.idml"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.png", "stdio.svg", "stdio.pdf", "stdio.step", "stdio.idml"]
}
pub fn layout_to_wire(from: &LayoutSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(from)
}
pub fn layout_from_wire(bytes: &[u8]) -> Result<LayoutSnapshot, store::PackError> {
    <LayoutSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::LayoutAnalyzer;
    use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
    use {semio_plugin::io::AnalyzeSource,semio_plugin::ArtifactComposition,semio_plugin::io::ComposeError,semio_plugin::io::ComposeSource,semio_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.layout.layout", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_DXF: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_SVG: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };

    const DEP_PDF: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DEP_STEP: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };

    const DEP_IDML: Dialect = Dialect { artifact_kind: "s.stdio.idml", standard: StandardId("8.0"), subset: SubsetId("*") };

    pub struct LayoutComposerComposition;

    impl ArtifactComposition for LayoutComposerComposition {
        type Snapshot = LayoutSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_DXF, DEP_JSON, DEP_SVG, DEP_PDF, DEP_STEP, DEP_IDML]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if [DEP_PDF,DEP_STEP,DEP_IDML,DEP_SVG,DEP_DXF,DEP_DWG].contains(&source.dialect){return Err(ComposeError{message:"layout.import.retained-preparation-required".into(),diagnostics:Vec::new()})}
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = LayoutAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "LayoutComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region ⚠️Errors


//#endregion ⚠️Errors

//#region 🔖️MediaImportExport
#[path = "📤️export/📐️vector-scene/🦀️.rs"]
mod vector_scene;

use semio_s_artifact_layout_layout_model::{Frame, GridSettings, Layer, Page, PageColumns, PageMargins, Spread, LAYOUT_DOCUMENT_SCHEMA};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
pub use semio_s_artifact_layout_layout_model::snapshot::page_projection::{layout_snapshot_to_scene, rect_path_segments};

/// 📤️ Writes the cold page projection through the standard drawing encoder.
pub fn layout_snapshot_to_format(doc: &LayoutSnapshot, format: &str) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_semio_drawing::io::SemioDrawingFormat;
    let format = match format {
        "svg" => SemioDrawingFormat::Svg,
        "png" => SemioDrawingFormat::Png,
        "dxf" => SemioDrawingFormat::Dxf,
        "dwg" => SemioDrawingFormat::Dwg,
        _ => return Err("layout.export.drawing-format".into()),
    };
    vector_scene::encode_vector_scene(layout_snapshot_to_scene(doc)?, format)
}

/// 🧭️ Cold external analysis preserves original encoded bytes before identity, native decoding and publication.
pub fn layout_import_sources_from_format(format:&str,bytes:&[u8],name:&str,control:&mut crate::interchange::InterchangeControl<'_>)->Result<crate::interchange::LayoutImportSourcePlan,String>{
    use crate::interchange::{LayoutExternalDrawingFormat,LayoutImportSourcePlan,InterchangeProgress,MAX_INTERCHANGE_BYTES};
    let format=match format{"svg"=>LayoutExternalDrawingFormat::Svg,"dxf"=>LayoutExternalDrawingFormat::Dxf,"dwg"=>LayoutExternalDrawingFormat::Dwg,_=>return Err("layout.import.drawing-format".into())};
    if bytes.len()>MAX_INTERCHANGE_BYTES{return Err("layout.interchange.byte-limit".into())}
    if !control(InterchangeProgress{completed:0,total:bytes.len()}){return Err("layout.interchange.cancelled".into())}
    let mut source=Vec::new();source.try_reserve_exact(bytes.len()).map_err(|_|"layout.import.source-allocation")?;
    for(at,chunk)in bytes.chunks(4096).enumerate(){if !control(InterchangeProgress{completed:at*4096,total:bytes.len()}){return Err("layout.interchange.cancelled".into())}source.extend_from_slice(chunk)}
    if !control(InterchangeProgress{completed:bytes.len(),total:bytes.len()}){return Err("layout.interchange.cancelled".into())}
    Ok(LayoutImportSourcePlan::External{format,source,name:name.into()})
}
//#endregion 🔖️MediaImportExport

//#region 🚪️DerivedIoRegistry
/// 🚪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
/// The artifact root's own `io_registry` (a `&'static [&'static ComposerEntry]` view) wraps THIS
/// module's `entries()` (`&'static [ComposerEntry]`, owning storage) — deliberately different return
/// types; do not conflate them when qualifying paths.
pub mod io_registry {
    use crate::LayoutBuilder as LayoutAnyBuilder;
    use crate::LayoutComposer as LayoutAnyComposer;
    use {semio_plugin::composer_entry_of,semio_plugin::ArtifactBuilder,semio_plugin::io::ComposeError,semio_plugin::io::ComposedArtifact,semio_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_plugin::io::ErasedComposeSource,semio_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
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
    const LAYOUT_DIALECT: Dialect = Dialect { artifact_kind: "s.layout.layout", standard: StandardId("1"), subset: SubsetId("*") };
    const LAYOUT_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == LAYOUT_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => LayoutAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => LayoutAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "LayoutComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == LAYOUT_JSON_BRIDGE_DIALECT) {
            let text = match &source.payload {
                IoPayload::Text(t) => t.clone(),
                IoPayload::Binary(b) => String::from_utf8_lossy(b).into_owned(),
            };
            return crate::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "LayoutComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const EXPORT_DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId("*") };

    fn compose_page_projection<'a>(
        sources: &'a [ErasedComposeSource],
        format: &'static str,
        dialect: Dialect,
    ) -> semio_plugin::io::ComposeFuture<'a> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::layout_snapshot_to_format(&snapshot, format)
                .map_err(|message| ComposeError { message, diagnostics: Vec::new() })?;
            let payload = if format == "svg" {
                IoPayload::Text(String::from_utf8(bytes)
                    .map_err(|error| ComposeError { message: error.to_string(), diagnostics: Vec::new() })?)
            } else {
                IoPayload::Binary(bytes)
            };
            Ok(ComposedArtifact { dialect, payload, diagnostics: Vec::new(), confidence: semio_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_plugin::io::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let text = crate::export::serializers::artifacts::json::v_rfc8259::any::serialize_text(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Text(text), diagnostics: Vec::new(), confidence: semio_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const EXPORT_STEP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId("*") };
    const EXPORT_IDML_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.idml", standard: StandardId("8.0"), subset: SubsetId("*") };
    fn compose_binary_interchange<'a>(sources: &'a [ErasedComposeSource], format: &'static str, dialect: Dialect) -> semio_plugin::io::ComposeFuture<'a> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let result = crate::interchange::export_document(format, &snapshot, &mut |_| true).map_err(|message| ComposeError { message, diagnostics: Vec::new() })?;
            let diagnostics = result.diagnostics.into_iter().map(|diagnostic| semio_framework_diagnostic::Diagnostic { code: semio_framework_diagnostic::FaultCode::new(diagnostic.code), severity: semio_framework_diagnostic::Severity::Warning, span: semio_framework_diagnostic::TextSpan::at(1, 1), message: diagnostic.message, expected: None, scope: semio_framework_diagnostic::FaultScope::default() }).collect();
            Ok(ComposedArtifact { dialect, payload: IoPayload::Binary(result.value), diagnostics, confidence: semio_plugin::io::Confidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<LayoutAnyComposer>(),
                    ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[LAYOUT_DIALECT], compose: |sources| compose_binary_interchange(sources, "pdf", EXPORT_PDF_DIALECT) },
                    ComposerEntry { writes: EXPORT_STEP_DIALECT, reads: &[LAYOUT_DIALECT], compose: |sources| compose_binary_interchange(sources, "step", EXPORT_STEP_DIALECT) },
                    ComposerEntry { writes: EXPORT_IDML_DIALECT, reads: &[LAYOUT_DIALECT], compose: |sources| compose_binary_interchange(sources, "idml", EXPORT_IDML_DIALECT) },
                    ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[LAYOUT_DIALECT], compose: |sources| compose_page_projection(sources, "svg", EXPORT_SVG_DIALECT) },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[LAYOUT_DIALECT], compose: compose_export_json },
                    ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[LAYOUT_DIALECT], compose: |sources| compose_page_projection(sources, "png", EXPORT_PNG_DIALECT) },
                    ComposerEntry { writes: EXPORT_DXF_DIALECT, reads: &[LAYOUT_DIALECT], compose: |sources| compose_page_projection(sources, "dxf", EXPORT_DXF_DIALECT) },
                    ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[LAYOUT_DIALECT], compose: |sources| compose_page_projection(sources, "dwg", EXPORT_DWG_DIALECT) },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry


#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use semio_s_artifact_layout_layout_diff::LayoutDiff;
    use semio_s_artifact_layout_layout_mutations::LayoutMutation;
    use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
    use semio_plugin::ArtifactBuilder;

    #[derive(Clone, Debug)]
    pub struct LayoutBuilderConstruction {
        snapshot: LayoutSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for LayoutBuilderConstruction {
        type Snapshot = LayoutSnapshot;
        type Mutation = LayoutMutation;
        type Diff = LayoutDiff;
        fn empty() -> Self {
            Self { snapshot: semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<LayoutSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<LayoutSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::ColdMutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff_cold(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = protocol::apply_diff_cold(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
    use {semio_plugin::io::Analysis,semio_plugin::io::AnalyzeSource,semio_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct LayoutParts {
        pub snapshot: Option<LayoutSnapshot>,
    }

    pub struct LayoutAnalyzerAnalysis;

    impl ArtifactAnalysis for LayoutAnalyzerAnalysis {
        type Parts = LayoutParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.layout.layout", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_plugin::io::Confidence {
            semio_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = LayoutParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <LayoutSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <LayoutSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_plugin::derive_artifact_facets!(
    pub spec LayoutBuilderFacets {
        construction: LayoutBuilderConstruction,
        analysis: LayoutAnalyzerAnalysis,
        composition: LayoutComposerComposition,
    }
    builder: LayoutBuilder,
    analyzer: LayoutAnalyzer,
    composer: LayoutComposer,
);

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod export {
    #[path = "."]
    pub mod serializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod v_rfc8259 {
                    #[path = "📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
        }
    }
}
#[path = "."]
pub mod import {
    #[path = "."]
    pub mod deserializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod v_ac1018 {
                    #[path = "📥️import/🧩️deserializers/🗿️artifacts/🏗️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod v_r12 {
                    #[path = "📥️import/🧩️deserializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod v_rfc8259 {
                    #[path = "📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod v1_1 {
                    #[path = "📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                    pub mod any;
                }
            }
        }
    }
}
//#endregion 🪢️TaxonomyMounts

#[path = "🔀️interchange/🦀️.rs"]
pub mod interchange;
