//! 🧐️ FlowAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::flow::standards::v1::subsets::any::analyzer::FlowAnalyzer as FlowAnyAnalyzer;
pub use crate::artifacts::flow::standards::v1::subsets::any::analyzer::FlowParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.flow", standard: StandardId("1"), subset: SubsetId("*") };

pub struct FlowAnalyzer;

impl ArtifactAnalyzer for FlowAnalyzer {
    type Parts = FlowParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { FlowAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { FlowAnyAnalyzer::analyze(sources) }
}
