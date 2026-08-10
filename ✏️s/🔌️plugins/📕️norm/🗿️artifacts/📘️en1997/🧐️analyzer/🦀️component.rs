//! 🧐️ En1997Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1997::standards::v1::analyzer::En1997Analyzer as En1997RawAnalyzer;
pub use crate::artifacts::en1997::standards::v1::analyzer::En1997Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1997", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1997Analyzer;

impl ArtifactAnalyzer for En1997Analyzer {
    type Parts = En1997Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1997RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1997RawAnalyzer::analyze(sources) }
}
