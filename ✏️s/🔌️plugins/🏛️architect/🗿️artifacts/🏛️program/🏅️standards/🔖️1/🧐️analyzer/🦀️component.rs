//! 🧐️ ProgramAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::program::standards::v1::subsets::any::analyzer::ProgramAnalyzer as ProgramAnyAnalyzer;
pub use crate::artifacts::program::standards::v1::subsets::any::analyzer::ProgramParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.program", standard: StandardId("1"), subset: SubsetId("*") };

pub struct ProgramAnalyzer;

impl ArtifactAnalyzer for ProgramAnalyzer {
    type Parts = ProgramParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { ProgramAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { ProgramAnyAnalyzer::analyze(sources) }
}
