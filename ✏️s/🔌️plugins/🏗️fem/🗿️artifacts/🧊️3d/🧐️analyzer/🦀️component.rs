//! 🧐️ Fem3dAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::fem3d::standards::v1::analyzer::Fem3dAnalyzer as Fem3dRawAnalyzer;
pub use crate::artifacts::fem3d::standards::v1::analyzer::Fem3dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.fem3d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Fem3dAnalyzer;

impl ArtifactAnalyzer for Fem3dAnalyzer {
    type Parts = Fem3dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Fem3dRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Fem3dRawAnalyzer::analyze(sources) }
}
