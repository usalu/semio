//! 🧐️ GisMapAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::gismap::standards::v1::subsets::any::analyzer::GisMapAnalyzer as GisMapAnyAnalyzer;
pub use crate::artifacts::gismap::standards::v1::subsets::any::analyzer::GisMapParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.gismap", standard: StandardId("1"), subset: SubsetId("*") };

pub struct GisMapAnalyzer;

impl ArtifactAnalyzer for GisMapAnalyzer {
    type Parts = GisMapParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { GisMapAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { GisMapAnyAnalyzer::analyze(sources) }
}
