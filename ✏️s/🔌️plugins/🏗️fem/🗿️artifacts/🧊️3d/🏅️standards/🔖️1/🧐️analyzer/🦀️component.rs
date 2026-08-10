//! 🧐️ Fem3dAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::fem3d::standards::v1::subsets::any::analyzer::Fem3dAnalyzer as Fem3dAnyAnalyzer;
pub use crate::artifacts::fem3d::standards::v1::subsets::any::analyzer::Fem3dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.fem3d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Fem3dAnalyzer;

impl ArtifactAnalyzer for Fem3dAnalyzer {
    type Parts = Fem3dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Fem3dAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Fem3dAnyAnalyzer::analyze(sources) }
}
