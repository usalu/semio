//! 🧐️ MathematicalAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::mathematical::standards::v1::subsets::any::analyzer::MathematicalAnalyzer as MathematicalAnyAnalyzer;
pub use crate::artifacts::mathematical::standards::v1::subsets::any::analyzer::MathematicalParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.mathematical", standard: StandardId("1"), subset: SubsetId("*") };

pub struct MathematicalAnalyzer;

impl ArtifactAnalyzer for MathematicalAnalyzer {
    type Parts = MathematicalParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { MathematicalAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { MathematicalAnyAnalyzer::analyze(sources) }
}
