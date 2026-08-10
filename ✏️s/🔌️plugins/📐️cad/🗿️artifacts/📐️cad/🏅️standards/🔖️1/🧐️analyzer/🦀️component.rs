//! 🧐️ CadAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::cad::standards::v1::subsets::any::analyzer::CadAnalyzer as CadAnyAnalyzer;
pub use crate::artifacts::cad::standards::v1::subsets::any::analyzer::CadParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.cad", standard: StandardId("1"), subset: SubsetId("*") };

pub struct CadAnalyzer;

impl ArtifactAnalyzer for CadAnalyzer {
    type Parts = CadParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { CadAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { CadAnyAnalyzer::analyze(sources) }
}
