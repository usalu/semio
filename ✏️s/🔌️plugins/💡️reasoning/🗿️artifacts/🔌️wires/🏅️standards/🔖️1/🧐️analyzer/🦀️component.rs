//! 🧐️ WiresAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::wires::standards::v1::subsets::any::analyzer::WiresAnalyzer as WiresAnyAnalyzer;
pub use crate::artifacts::wires::standards::v1::subsets::any::analyzer::WiresParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.wires", standard: StandardId("1"), subset: SubsetId("*") };

pub struct WiresAnalyzer;

impl ArtifactAnalyzer for WiresAnalyzer {
    type Parts = WiresParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { WiresAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { WiresAnyAnalyzer::analyze(sources) }
}
