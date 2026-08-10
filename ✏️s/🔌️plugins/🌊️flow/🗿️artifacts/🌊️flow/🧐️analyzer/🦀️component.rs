//! 🧐️ FlowAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::flow::standards::v1::analyzer::FlowAnalyzer as FlowRawAnalyzer;
pub use crate::artifacts::flow::standards::v1::analyzer::FlowParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.flow", standard: StandardId("1"), subset: SubsetId("*") };

pub struct FlowAnalyzer;

impl ArtifactAnalyzer for FlowAnalyzer {
    type Parts = FlowParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { FlowRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { FlowRawAnalyzer::analyze(sources) }
}
