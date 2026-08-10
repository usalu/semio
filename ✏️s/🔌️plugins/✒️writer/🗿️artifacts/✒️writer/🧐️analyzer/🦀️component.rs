//! 🧐️ WriterAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::writer::standards::v1::analyzer::WriterAnalyzer as WriterRawAnalyzer;
pub use crate::artifacts::writer::standards::v1::analyzer::WriterParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.writer", standard: StandardId("1"), subset: SubsetId("*") };

pub struct WriterAnalyzer;

impl ArtifactAnalyzer for WriterAnalyzer {
    type Parts = WriterParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { WriterRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { WriterRawAnalyzer::analyze(sources) }
}
