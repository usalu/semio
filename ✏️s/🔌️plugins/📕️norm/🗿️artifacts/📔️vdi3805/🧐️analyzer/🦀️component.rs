//! 🧐️ Vdi3805Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::vdi3805::standards::v1::analyzer::Vdi3805Analyzer as Vdi3805RawAnalyzer;
pub use crate::artifacts::vdi3805::standards::v1::analyzer::Vdi3805Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.vdi3805", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Vdi3805Analyzer;

impl ArtifactAnalyzer for Vdi3805Analyzer {
    type Parts = Vdi3805Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Vdi3805RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Vdi3805RawAnalyzer::analyze(sources) }
}
