//! 🚪️ IO s.norm.en1990 (1/✳️any) — universal semio DSL/pack import+export for the native `s.norm.en1990`
//! dialect. Registration flows through 🎹️composer::register (called once from ⚙️engine::register).

use crate::En1990Snapshot;

pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["s.norm.en1990"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["s.norm.en1990"]
}

/// 📖️ Parses `.en1990` DSL bytes into a snapshot.
pub fn en1990_from_dsl_bytes(bytes: &[u8]) -> Result<En1990Snapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text)
}

/// 🖨️ Prints a snapshot to `.en1990` DSL bytes.
pub fn en1990_to_dsl_bytes(snapshot: &En1990Snapshot) -> Vec<u8> {
    crate::standards::v1::subsets::any::io::text::snapshot::print_dsl(snapshot).into_bytes()
}

/// 📦️ Decodes a semio pack into a snapshot.
pub fn en1990_from_pack(bytes: &[u8]) -> Result<En1990Snapshot, store::PackError> {
    <En1990Snapshot as store::ArtifactPack>::decode_pack(bytes)
}

/// 📦️ Encodes a snapshot as a semio pack.
pub fn en1990_to_pack(snapshot: &En1990Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::io::En1990Analyzer;
    use crate::En1990Snapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1990", standard: StandardId("1"), subset: SubsetId("*") };

    pub struct En1990ComposerComposition;

    impl ArtifactComposition for En1990ComposerComposition {
        type Snapshot = En1990Snapshot;
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
                    let analysis = En1990Analyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "En1990ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️IoRegistry
/// 🚪️ Composer registry (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — relocated
/// verbatim from the deleted `⚙️engine`; io is exactly where composer dispatch belongs.
pub mod io_registry {
    use crate::standards::v1::subsets::any::io::En1990Composer as En1990AnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<En1990AnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️IoRegistry

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
    use crate::{En1990Diff, En1990Mutation, En1990Snapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct En1990BuilderConstruction {
        snapshot: En1990Snapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for En1990BuilderConstruction {
        type Snapshot = En1990Snapshot;
        type Mutation = En1990Mutation;
        type Diff = En1990Diff;
        fn empty() -> Self {
            Self { snapshot: En1990Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<En1990Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<En1990Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation, &self.snapshot);
            match <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::En1990Snapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct En1990Parts {
        pub snapshot: Option<En1990Snapshot>,
    }

    pub struct En1990AnalyzerAnalysis;

    impl ArtifactAnalysis for En1990AnalyzerAnalysis {
        type Parts = En1990Parts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.norm.en1990", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = En1990Parts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <En1990Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <En1990Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec En1990BuilderFacets {
        construction: En1990BuilderConstruction,
        analysis: En1990AnalyzerAnalysis,
        composition: crate::standards::v1::subsets::any::io::derived_composition::En1990ComposerComposition,
    }
    builder: En1990Builder,
    analyzer: En1990Analyzer,
    composer: En1990Composer,
);
