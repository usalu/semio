//! 🧐️ PlaygroundAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::playground::standards::v1::subsets::any::analyzer::PlaygroundAnalyzer as PlaygroundAnyAnalyzer;
pub use crate::artifacts::playground::standards::v1::subsets::any::analyzer::PlaygroundParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.playground", standard: StandardId("1"), subset: SubsetId("*") };

pub struct PlaygroundAnalyzer;

impl ArtifactAnalyzer for PlaygroundAnalyzer {
    type Parts = PlaygroundParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PlaygroundAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PlaygroundAnyAnalyzer::analyze(sources) }
}
