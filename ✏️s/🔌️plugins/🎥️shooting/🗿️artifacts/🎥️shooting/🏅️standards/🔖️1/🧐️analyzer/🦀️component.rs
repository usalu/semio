//! 🧐️ ShootingAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::shooting::standards::v1::subsets::any::analyzer::ShootingAnalyzer as ShootingAnyAnalyzer;
pub use crate::artifacts::shooting::standards::v1::subsets::any::analyzer::ShootingParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.shooting", standard: StandardId("1"), subset: SubsetId("*") };

pub struct ShootingAnalyzer;

impl ArtifactAnalyzer for ShootingAnalyzer {
    type Parts = ShootingParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { ShootingAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { ShootingAnyAnalyzer::analyze(sources) }
}
