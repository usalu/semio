//! 🧐️ Procedural2dAnalyzer (1/✳️any) — read-only analysis, successor to the pre-migration
//! Procedural2dDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

#[derive(Clone, Debug, Default)]
pub struct Procedural2dParts {
    pub snapshot: Option<Procedural2dSnapshot>,
}

pub struct Procedural2dAnalyzer;

impl ArtifactAnalyzer for Procedural2dAnalyzer {
    type Parts = Procedural2dParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural2d", standard: StandardId("1"), subset: SubsetId("*") };

    fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
        IoConfidence::Medium
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = Procedural2dParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <Procedural2dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <Procedural2dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
