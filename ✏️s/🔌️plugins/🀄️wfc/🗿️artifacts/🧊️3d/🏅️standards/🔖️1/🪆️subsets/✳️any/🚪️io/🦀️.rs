//! 🚪️ IO s.wfc.wfc3d (1/✳️any) — native DSL/pack plus full-fidelity `s.stdio.txt@utf-8`.
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt"]
}

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::io::Wfc3dAnalyzer;
    use crate::Wfc3dSnapshot;
    use {semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.wfc.wfc3d", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Wfc3dComposerComposition;

    impl ArtifactComposition for Wfc3dComposerComposition {
        type Snapshot = Wfc3dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(text) => AnalyzeSource::Text(text),
                        AnalyzeSource::Binary(bytes) => AnalyzeSource::Binary(bytes),
                    };
                    let analysis = Wfc3dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(text) => text.as_bytes().to_vec(),
                        AnalyzeSource::Binary(binary) => binary.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::High, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "Wfc3dComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️IoRegistry
pub mod io_registry {
    use crate::standards::v1::subsets::any::io::Wfc3dComposer as Wfc3dAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Wfc3dAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️IoRegistry

//#region 🔖️IoDeclaration
/// 🚪️ The `io_mechanism` declaration this subset's `subset()` binds: the native DSL/pack codec plus
/// the five hand-authored `dsl::LanguageSpec`s (document/op/diff/pack/spr). Foreign-format hops go
/// through the `ComposerEntry` registry above, so `entries` stays empty here.
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::{Wfc3dMutation, Wfc3dSnapshot, WFC3D_DOCUMENT_SCHEMA};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};

    let langs = crate::wfc3d_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::bare::<Wfc3dSnapshot, Wfc3dMutation>(WFC3D_DOCUMENT_SCHEMA.to_string()),
        },
        entries: &[],
    }
}
//#endregion 🔖️IoDeclaration

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{Wfc3dDiff, Wfc3dMutation, Wfc3dSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Wfc3dBuilderConstruction {
        snapshot: Wfc3dSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Wfc3dBuilderConstruction {
        type Snapshot = Wfc3dSnapshot;
        type Mutation = Wfc3dMutation;
        type Diff = Wfc3dDiff;
        fn empty() -> Self {
            Self { snapshot: Wfc3dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Wfc3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Wfc3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
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
    use crate::Wfc3dSnapshot;
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Wfc3dParts {
        pub snapshot: Option<Wfc3dSnapshot>,
    }

    pub struct Wfc3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Wfc3dAnalyzerAnalysis {
        type Parts = Wfc3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.wfc.wfc3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Wfc3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Wfc3dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Wfc3dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec Wfc3dBuilderFacets {
        construction: Wfc3dBuilderConstruction,
        analysis: Wfc3dAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::any::io::derived_composition::Wfc3dComposerComposition,
    }
    builder: Wfc3dBuilder,
    analyzer: Wfc3dAnalyzer,
    composer: Wfc3dComposer,
);
