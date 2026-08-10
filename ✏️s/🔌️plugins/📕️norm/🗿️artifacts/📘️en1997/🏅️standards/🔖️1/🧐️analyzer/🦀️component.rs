//! 🧐️ En1997Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1997::standards::v1::subsets::any::analyzer::En1997Analyzer as En1997AnyAnalyzer;
pub use crate::artifacts::en1997::standards::v1::subsets::any::analyzer::En1997Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1997", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1997Analyzer;

impl ArtifactAnalyzer for En1997Analyzer {
    type Parts = En1997Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1997AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1997AnyAnalyzer::analyze(sources) }
}
