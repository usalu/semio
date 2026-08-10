//! 🧐️ En1990Analyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::en1990::standards::v1::subsets::any::analyzer::En1990Analyzer as En1990AnyAnalyzer;
pub use crate::artifacts::en1990::standards::v1::subsets::any::analyzer::En1990Parts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.en1990", standard: StandardId("1"), subset: SubsetId("*") };

pub struct En1990Analyzer;

impl ArtifactAnalyzer for En1990Analyzer {
    type Parts = En1990Parts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { En1990AnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { En1990AnyAnalyzer::analyze(sources) }
}
