//! 🧐️ LayoutAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::layout::standards::v1::subsets::any::analyzer::LayoutAnalyzer as LayoutAnyAnalyzer;
pub use crate::artifacts::layout::standards::v1::subsets::any::analyzer::LayoutParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.layout", standard: StandardId("1"), subset: SubsetId("*") };

pub struct LayoutAnalyzer;

impl ArtifactAnalyzer for LayoutAnalyzer {
    type Parts = LayoutParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { LayoutAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { LayoutAnyAnalyzer::analyze(sources) }
}
