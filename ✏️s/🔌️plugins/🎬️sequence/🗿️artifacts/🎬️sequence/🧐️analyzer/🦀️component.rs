//! 🧐️ SequenceAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::sequence::standards::v1::analyzer::SequenceAnalyzer as SequenceRawAnalyzer;
pub use crate::artifacts::sequence::standards::v1::analyzer::SequenceParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.sequence", standard: StandardId("1"), subset: SubsetId("*") };

pub struct SequenceAnalyzer;

impl ArtifactAnalyzer for SequenceAnalyzer {
    type Parts = SequenceParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { SequenceRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { SequenceRawAnalyzer::analyze(sources) }
}
