//! 🧐️ ShootingAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::shooting::standards::v1::analyzer::ShootingAnalyzer as ShootingRawAnalyzer;
pub use crate::artifacts::shooting::standards::v1::analyzer::ShootingParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.shooting", standard: StandardId("1"), subset: SubsetId("*") };

pub struct ShootingAnalyzer;

impl ArtifactAnalyzer for ShootingAnalyzer {
    type Parts = ShootingParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { ShootingRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { ShootingRawAnalyzer::analyze(sources) }
}
