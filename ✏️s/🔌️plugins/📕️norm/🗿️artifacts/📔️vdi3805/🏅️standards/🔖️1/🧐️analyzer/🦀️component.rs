//! 🧐️ Vdi3805Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::vdi3805::standards::v1::subsets::any::analyzer::Vdi3805Analyzer as Vdi3805AnyAnalyzer;
pub use crate::artifacts::vdi3805::standards::v1::subsets::any::analyzer::Vdi3805Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.vdi3805", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Vdi3805Analyzer;

impl ArtifactAnalyzer for Vdi3805Analyzer {
    type Parts = Vdi3805Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Vdi3805AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Vdi3805AnyAnalyzer::analyze(sources) }
}
