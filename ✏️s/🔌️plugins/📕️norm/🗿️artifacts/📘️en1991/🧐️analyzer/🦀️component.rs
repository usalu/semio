//! 🧐️ En1991Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1991::standards::v1::analyzer::En1991Analyzer as En1991RawAnalyzer;
pub use crate::artifacts::en1991::standards::v1::analyzer::En1991Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1991", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1991Analyzer;

impl ArtifactAnalyzer for En1991Analyzer {
    type Parts = En1991Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1991RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1991RawAnalyzer::analyze(sources) }
}
