//! 🧐️ Iso16757Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::iso16757::standards::v1::subsets::any::analyzer::Iso16757Analyzer as Iso16757AnyAnalyzer;
pub use crate::artifacts::iso16757::standards::v1::subsets::any::analyzer::Iso16757Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.iso16757", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Iso16757Analyzer;

impl ArtifactAnalyzer for Iso16757Analyzer {
    type Parts = Iso16757Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Iso16757AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Iso16757AnyAnalyzer::analyze(sources) }
}
