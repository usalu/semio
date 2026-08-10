//! 🧐️ LowpolyAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::lowpoly::standards::v1::analyzer::LowpolyAnalyzer as LowpolyRawAnalyzer;
pub use crate::artifacts::lowpoly::standards::v1::analyzer::LowpolyParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.lowpoly", standard: StandardId("1"), subset: SubsetId("*") };

pub struct LowpolyAnalyzer;

impl ArtifactAnalyzer for LowpolyAnalyzer {
    type Parts = LowpolyParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { LowpolyRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { LowpolyRawAnalyzer::analyze(sources) }
}
