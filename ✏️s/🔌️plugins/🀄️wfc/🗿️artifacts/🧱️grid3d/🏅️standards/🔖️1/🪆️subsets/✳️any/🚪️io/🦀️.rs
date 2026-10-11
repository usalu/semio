//! 🚪️ IO `s.wfc.grid3d` (1/✳️any) — native DSL/pack, declared through the subset root's
//! `io()`/`IoDeclaration`. The composition below is what lets a foreign source in a read dialect be
//! composed INTO this dialect; conformance runs inside the io mechanism's own entry, never here.

/// 📸️ Native document capability shared by the exact subset declaration and owned I/O laws.
pub fn native_snapshot_codec()->store::ArtifactCodec{store::ArtifactCodec::bare::<crate::Grid3dSnapshot,crate::Grid3dMutation>(crate::WFC_GRID3D_DOCUMENT_SCHEMA.to_string())}

pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt"]
}

pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt"]
}

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::io::Grid3dAnalyzer;
    use crate::Grid3dSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.wfc.grid3d", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Grid3dComposerComposition;

    impl ArtifactComposition for Grid3dComposerComposition {
        type Snapshot = Grid3dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect != DIALECT && source.dialect != DEP_TXT {
                    continue;
                }
                let native = match &source.payload {
                    AnalyzeSource::Text(text) => AnalyzeSource::Text(text),
                    AnalyzeSource::Binary(bytes) => AnalyzeSource::Binary(bytes),
                };
                let analysis = Grid3dAnalyzer::analyze(&[native]);
                if let Some(snapshot) = analysis.parts.snapshot {
                    return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                }
            }
            Err(ComposeError { message: "Grid3dComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🎪️Tests
#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🎪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{Grid3dDiff, Grid3dMutation, Grid3dSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Grid3dBuilderConstruction {
        snapshot: Grid3dSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Grid3dBuilderConstruction {
        type Snapshot = Grid3dSnapshot;
        type Mutation = Grid3dMutation;
        type Diff = Grid3dDiff;
        fn empty() -> Self {
            Self { snapshot: Grid3dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Grid3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Grid3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
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
    use crate::Grid3dSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Grid3dParts {
        pub snapshot: Option<Grid3dSnapshot>,
    }

    pub struct Grid3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Grid3dAnalyzerAnalysis {
        type Parts = Grid3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.wfc.grid3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Grid3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Grid3dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(error) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Grid3dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(error) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string()));
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
    pub spec Grid3dBuilderFacets {
        construction: Grid3dBuilderConstruction,
        analysis: Grid3dAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::any::io::derived_composition::Grid3dComposerComposition,
    }
    builder: Grid3dBuilder,
    analyzer: Grid3dAnalyzer,
    composer: Grid3dComposer,
);
