//! 🧐️ GisTerrainAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::gisterrain::standards::v1::subsets::any::analyzer::GisTerrainAnalyzer as GisTerrainAnyAnalyzer;
pub use crate::artifacts::gisterrain::standards::v1::subsets::any::analyzer::GisTerrainParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.gisterrain", standard: StandardId("1"), subset: SubsetId("*") };

pub struct GisTerrainAnalyzer;

impl ArtifactAnalyzer for GisTerrainAnalyzer {
    type Parts = GisTerrainParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { GisTerrainAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { GisTerrainAnyAnalyzer::analyze(sources) }
}
