//! 🧐️ WiresAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::wires::standards::v1::analyzer::WiresAnalyzer as WiresRawAnalyzer;
pub use crate::artifacts::wires::standards::v1::analyzer::WiresParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.wires", standard: StandardId("1"), subset: SubsetId("*") };

pub struct WiresAnalyzer;

impl ArtifactAnalyzer for WiresAnalyzer {
    type Parts = WiresParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { WiresRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { WiresRawAnalyzer::analyze(sources) }
}
