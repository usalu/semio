//! 🚪️ IO s.norm.iso16757 (1/✳️any) — no stdio format bridges. W5a (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) deleted the five
//! degenerate leaves (csv/json/txt/xlsx/zip) that either fabricated a one-cell-CSV/raw-DSL-dump
//! shape or silently defaulted to `Iso16757Snapshot::default()` on import (an honesty bug, not a
//! real codec). Iso16757Snapshot is a compliance document (scalar fields plus a handful of nested
//! records), not a flat row/column table, so no honest whole-artifact CSV round-trip exists to
//! re-register in their place. Registration flows through 🎹️composer::register (called once from
//! ⚙️engine::register) for the native `s.norm.iso16757` dialect only.
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &[]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &[]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::io::Iso16757Analyzer;
    use crate::Iso16757Snapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.iso16757", standard: StandardId("1"), subset: SubsetId("*") };

    pub struct Iso16757ComposerComposition;

    impl ArtifactComposition for Iso16757ComposerComposition {
        type Snapshot = Iso16757Snapshot;
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
                    let analysis = Iso16757Analyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "Iso16757ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️JsonSerializers
/// 🚪️ Whole-artifact JSON serializers (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`; serialization is exactly what `🚪️io` is for.
use crate::document::NormError;

pub mod io {
    use super::*;

    pub fn catalogue_to_json(catalogue: &crate::part_1::Catalogue) -> Result<String, NormError> {
        Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(catalogue))))
    }

    pub fn catalogue_from_json(json: &str) -> Result<crate::part_1::Catalogue, NormError> {
        semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| NormError::InvalidValue { field: "catalogue".into(), reason: e.to_string() })
    }

    pub fn dictionary_to_json(dictionary: &crate::part_4::Dictionary) -> Result<String, NormError> {
        Ok(semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(dictionary))))
    }
}

//#endregion 🚪️JsonSerializers

//#region 🚪️IoRegistry
/// 🚪️ Composer registry — relocated verbatim from the deleted `⚙️engine`; io is exactly where
/// composer dispatch belongs.
pub mod io_registry {
    use crate::standards::v1::subsets::any::io::Iso16757Composer as Iso16757AnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Iso16757AnyComposer>()]).as_slice()
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
    use crate::{Iso16757Diff, Iso16757Mutation, Iso16757Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Iso16757BuilderConstruction {
        snapshot: Iso16757Snapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Iso16757BuilderConstruction {
        type Snapshot = Iso16757Snapshot;
        type Mutation = Iso16757Mutation;
        type Diff = Iso16757Diff;
        fn empty() -> Self {
            Self { snapshot: Iso16757Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Iso16757Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Iso16757Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <Iso16757Diff as protocol::MutationDiff<Iso16757Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::Iso16757Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Iso16757Parts {
        pub snapshot: Option<Iso16757Snapshot>,
    }

    pub struct Iso16757AnalyzerAnalysis;

    impl ArtifactAnalysis for Iso16757AnalyzerAnalysis {
        type Parts = Iso16757Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.iso16757", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Iso16757Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Iso16757Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Iso16757Snapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
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
    pub spec Iso16757BuilderFacets {
        construction: Iso16757BuilderConstruction,
        analysis: Iso16757AnalyzerAnalysis,
        composition: crate::standards::v1::subsets::any::io::derived_composition::Iso16757ComposerComposition,
    }
    builder: Iso16757Builder,
    analyzer: Iso16757Analyzer,
    composer: Iso16757Composer,
);
