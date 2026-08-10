//! 🧐️ ProgramAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::program::standards::v1::analyzer::ProgramAnalyzer as ProgramRawAnalyzer;
pub use crate::artifacts::program::standards::v1::analyzer::ProgramParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.program", standard: StandardId("1"), subset: SubsetId("*") };

pub struct ProgramAnalyzer;

impl ArtifactAnalyzer for ProgramAnalyzer {
    type Parts = ProgramParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { ProgramRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { ProgramRawAnalyzer::analyze(sources) }
}
