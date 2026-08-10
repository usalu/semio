//! 🧐️ LayoutAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::layout::standards::v1::analyzer::LayoutAnalyzer as LayoutRawAnalyzer;
pub use crate::artifacts::layout::standards::v1::analyzer::LayoutParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.layout", standard: StandardId("1"), subset: SubsetId("*") };

pub struct LayoutAnalyzer;

impl ArtifactAnalyzer for LayoutAnalyzer {
    type Parts = LayoutParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { LayoutRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { LayoutRawAnalyzer::analyze(sources) }
}
