//! 🧐️ SequenceAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::sequence::standards::v1::subsets::any::analyzer::SequenceAnalyzer as SequenceAnyAnalyzer;
pub use crate::artifacts::sequence::standards::v1::subsets::any::analyzer::SequenceParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.sequence", standard: StandardId("1"), subset: SubsetId("*") };

pub struct SequenceAnalyzer;

impl ArtifactAnalyzer for SequenceAnalyzer {
    type Parts = SequenceParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { SequenceAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { SequenceAnyAnalyzer::analyze(sources) }
}
