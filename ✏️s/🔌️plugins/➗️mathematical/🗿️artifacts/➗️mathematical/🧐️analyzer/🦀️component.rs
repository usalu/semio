//! 🧐️ MathematicalAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::mathematical::standards::v1::analyzer::MathematicalAnalyzer as MathematicalRawAnalyzer;
pub use crate::artifacts::mathematical::standards::v1::analyzer::MathematicalParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.mathematical", standard: StandardId("1"), subset: SubsetId("*") };

pub struct MathematicalAnalyzer;

impl ArtifactAnalyzer for MathematicalAnalyzer {
    type Parts = MathematicalParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { MathematicalRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { MathematicalRawAnalyzer::analyze(sources) }
}
