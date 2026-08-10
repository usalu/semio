//! 🧐️ WriterAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::writer::standards::v1::subsets::any::analyzer::WriterAnalyzer as WriterAnyAnalyzer;
pub use crate::artifacts::writer::standards::v1::subsets::any::analyzer::WriterParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.writer", standard: StandardId("1"), subset: SubsetId("*") };

pub struct WriterAnalyzer;

impl ArtifactAnalyzer for WriterAnalyzer {
    type Parts = WriterParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { WriterAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { WriterAnyAnalyzer::analyze(sources) }
}
