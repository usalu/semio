//! 🧐️ PlaygroundAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::playground::standards::v1::analyzer::PlaygroundAnalyzer as PlaygroundRawAnalyzer;
pub use crate::artifacts::playground::standards::v1::analyzer::PlaygroundParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.playground", standard: StandardId("1"), subset: SubsetId("*") };

pub struct PlaygroundAnalyzer;

impl ArtifactAnalyzer for PlaygroundAnalyzer {
    type Parts = PlaygroundParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PlaygroundRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PlaygroundRawAnalyzer::analyze(sources) }
}
