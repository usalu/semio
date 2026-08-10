//! 🧐️ PresentAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::present::standards::v1::subsets::any::analyzer::PresentAnalyzer as PresentAnyAnalyzer;
pub use crate::artifacts::present::standards::v1::subsets::any::analyzer::PresentParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.present", standard: StandardId("1"), subset: SubsetId("*") };

pub struct PresentAnalyzer;

impl ArtifactAnalyzer for PresentAnalyzer {
    type Parts = PresentParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PresentAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PresentAnyAnalyzer::analyze(sources) }
}
