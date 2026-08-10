//! 🧐️ Vdi3805Analyzer (1/✳️any) — read-only analysis, successor to the pre-migration
//! Vdi3805Decomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::vdi3805::Vdi3805Snapshot;

#[derive(Clone, Debug, Default)]
pub struct Vdi3805Parts {
    pub snapshot: Option<Vdi3805Snapshot>,
}

pub struct Vdi3805Analyzer;

impl ArtifactAnalyzer for Vdi3805Analyzer {
    type Parts = Vdi3805Parts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.vdi3805", standard: StandardId("1"), subset: SubsetId("*") };

    fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
        IoConfidence::Medium
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = Vdi3805Parts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <Vdi3805Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <Vdi3805Snapshot as store::ArtifactPack>::decode_pack(bytes) {
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
