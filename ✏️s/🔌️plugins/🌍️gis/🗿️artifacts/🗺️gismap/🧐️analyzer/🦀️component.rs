//! 🧐️ GisMapAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::gismap::standards::v1::analyzer::GisMapAnalyzer as GisMapRawAnalyzer;
pub use crate::artifacts::gismap::standards::v1::analyzer::GisMapParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.gismap", standard: StandardId("1"), subset: SubsetId("*") };

pub struct GisMapAnalyzer;

impl ArtifactAnalyzer for GisMapAnalyzer {
    type Parts = GisMapParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { GisMapRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { GisMapRawAnalyzer::analyze(sources) }
}
