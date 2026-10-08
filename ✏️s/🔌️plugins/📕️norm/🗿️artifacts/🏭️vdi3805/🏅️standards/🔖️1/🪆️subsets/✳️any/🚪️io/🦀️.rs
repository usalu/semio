//! 🚪️ IO s.norm.vdi3805 (1/✳️any) — no stdio format bridges. W5a (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) deleted the five
//! degenerate leaves (csv/json/txt/xlsx/zip) that either fabricated a one-cell-CSV/raw-DSL-dump
//! shape or silently defaulted to `Vdi3805Snapshot::default()` on import (an honesty bug, not a
//! real codec). Vdi3805Snapshot is a compliance document (scalar fields plus a handful of nested
//! records), not a flat row/column table, so no honest whole-artifact CSV round-trip exists to
//! re-register in their place. Registration flows through 🎹️composer::register (called once from
//! ⚙️engine::register) for the native `s.norm.vdi3805` dialect only.
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &[]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &[]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::io::Vdi3805Analyzer;
    use crate::Vdi3805Snapshot;
    use {semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.vdi3805", standard: StandardId("1"), subset: SubsetId("*") };

    pub struct Vdi3805ComposerComposition;

    impl ArtifactComposition for Vdi3805ComposerComposition {
        type Snapshot = Vdi3805Snapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = Vdi3805Analyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "Vdi3805ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️JsonSerializers
use crate::document::NormError;
/// 🚪️ Whole-artifact JSON (de)serializers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
/// — relocated verbatim from the deleted `⚙️engine`; serialization is exactly what `🚪️io` is for.
use crate::{ManufacturerCatalog, Vdi3805Snapshot};

/// 📤️ JSON round-trip for manufacturer catalogues.
pub fn catalog_to_json(catalog: &ManufacturerCatalog) -> Result<String, NormError> {
    Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(catalog))))
}

pub fn catalog_from_json(json: &str) -> Result<ManufacturerCatalog, NormError> {
    semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| NormError::InvalidValue { field: "json".into(), reason: e.to_string() })
}

pub fn document_to_json(document: &Vdi3805Snapshot) -> Result<String, NormError> {
    Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(document))))
}

pub fn document_from_json(json: &str) -> Result<Vdi3805Snapshot, NormError> {
    semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| NormError::InvalidValue { field: "json".into(), reason: e.to_string() })
}

//#endregion 🚪️JsonSerializers

//#region 🚪️IoRegistry
/// 🚪️ Composer registry — relocated verbatim from the deleted `⚙️engine`; io is exactly where
/// composer dispatch belongs.
pub mod io_registry {
    use crate::standards::v1::subsets::any::io::Vdi3805Composer as Vdi3805AnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Vdi3805AnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️IoRegistry

//#region 🧪️JsonSerializersTests
#[cfg(test)]
#[path = "🧪️tests/🔬️json-serializers/🦀️.rs"]
mod json_serializers_tests;
//#endregion 🧪️JsonSerializersTests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Vdi3805BuilderConstruction {
        snapshot: Vdi3805Snapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Vdi3805BuilderConstruction {
        type Snapshot = Vdi3805Snapshot;
        type Mutation = Vdi3805Mutation;
        type Diff = Vdi3805Diff;
        fn empty() -> Self {
            Self { snapshot: Vdi3805Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
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
    use crate::Vdi3805Snapshot;
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Vdi3805Parts {
        pub snapshot: Option<Vdi3805Snapshot>,
    }

    pub struct Vdi3805AnalyzerAnalysis;

    impl ArtifactAnalysis for Vdi3805AnalyzerAnalysis {
        type Parts = Vdi3805Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.vdi3805", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Vdi3805Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
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

semio_framework_plugin::derive_artifact_facets!(
    pub spec Vdi3805BuilderFacets {
        construction: Vdi3805BuilderConstruction,
        analysis: Vdi3805AnalyzerAnalysis,
        composition: crate::standards::v1::subsets::any::io::derived_composition::Vdi3805ComposerComposition,
    }
    builder: Vdi3805Builder,
    analyzer: Vdi3805Analyzer,
    composer: Vdi3805Composer,
);
