//! 🧐️ LowpolyAnalyzer (1/✳️any) — read-only analysis, successor to the pre-migration
//! LowpolyDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::lowpoly::LowpolySnapshot;

#[derive(Clone, Debug, Default)]
pub struct LowpolyParts {
    pub snapshot: Option<LowpolySnapshot>,
}

pub struct LowpolyAnalyzer;

impl ArtifactAnalyzer for LowpolyAnalyzer {
    type Parts = LowpolyParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.lowpoly", standard: StandardId("1"), subset: SubsetId("*") };

    fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
        IoConfidence::Medium
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = LowpolyParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <LowpolySnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <LowpolySnapshot as store::ArtifactPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
            }
        }
        Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
    }
}
