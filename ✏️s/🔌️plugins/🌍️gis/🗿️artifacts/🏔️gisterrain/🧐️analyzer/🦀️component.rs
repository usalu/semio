//! 🧐️ GisTerrainAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::gisterrain::standards::v1::analyzer::GisTerrainAnalyzer as GisTerrainRawAnalyzer;
pub use crate::artifacts::gisterrain::standards::v1::analyzer::GisTerrainParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.gisterrain", standard: StandardId("1"), subset: SubsetId("*") };

pub struct GisTerrainAnalyzer;

impl ArtifactAnalyzer for GisTerrainAnalyzer {
    type Parts = GisTerrainParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { GisTerrainRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { GisTerrainRawAnalyzer::analyze(sources) }
}
