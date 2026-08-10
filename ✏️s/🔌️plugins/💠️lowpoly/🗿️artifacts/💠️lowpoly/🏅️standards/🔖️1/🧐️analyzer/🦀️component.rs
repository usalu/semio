//! 🧐️ LowpolyAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::lowpoly::standards::v1::subsets::any::analyzer::LowpolyAnalyzer as LowpolyAnyAnalyzer;
pub use crate::artifacts::lowpoly::standards::v1::subsets::any::analyzer::LowpolyParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.lowpoly", standard: StandardId("1"), subset: SubsetId("*") };

pub struct LowpolyAnalyzer;

impl ArtifactAnalyzer for LowpolyAnalyzer {
    type Parts = LowpolyParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { LowpolyAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { LowpolyAnyAnalyzer::analyze(sources) }
}
