//! 🧐️ CadAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::cad::standards::v1::analyzer::CadAnalyzer as CadRawAnalyzer;
pub use crate::artifacts::cad::standards::v1::analyzer::CadParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.cad", standard: StandardId("1"), subset: SubsetId("*") };

pub struct CadAnalyzer;

impl ArtifactAnalyzer for CadAnalyzer {
    type Parts = CadParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { CadRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { CadRawAnalyzer::analyze(sources) }
}
