//! 🧐️ En1999Analyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1999::standards::v1::analyzer::En1999Analyzer as En1999RawAnalyzer;
pub use crate::artifacts::en1999::standards::v1::analyzer::En1999Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1999", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1999Analyzer;

impl ArtifactAnalyzer for En1999Analyzer {
    type Parts = En1999Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1999RawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1999RawAnalyzer::analyze(sources) }
}
