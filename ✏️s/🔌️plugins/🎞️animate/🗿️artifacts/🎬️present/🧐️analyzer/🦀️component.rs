//! 🧐️ PresentAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::present::standards::v1::analyzer::PresentAnalyzer as PresentRawAnalyzer;
pub use crate::artifacts::present::standards::v1::analyzer::PresentParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.present", standard: StandardId("1"), subset: SubsetId("*") };

pub struct PresentAnalyzer;

impl ArtifactAnalyzer for PresentAnalyzer {
    type Parts = PresentParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PresentRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PresentRawAnalyzer::analyze(sources) }
}
