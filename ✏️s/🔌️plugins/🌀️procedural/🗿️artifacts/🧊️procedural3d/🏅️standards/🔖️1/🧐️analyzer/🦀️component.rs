//! 🧐️ Procedural3dAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::procedural3d::standards::v1::subsets::any::analyzer::Procedural3dAnalyzer as Procedural3dAnyAnalyzer;
pub use crate::artifacts::procedural3d::standards::v1::subsets::any::analyzer::Procedural3dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural3d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Procedural3dAnalyzer;

impl ArtifactAnalyzer for Procedural3dAnalyzer {
    type Parts = Procedural3dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Procedural3dAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Procedural3dAnyAnalyzer::analyze(sources) }
}
