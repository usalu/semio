//! 🧐️ Process3dAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::process3d::standards::v1::subsets::any::analyzer::Process3dAnalyzer as Process3dAnyAnalyzer;
pub use crate::artifacts::process3d::standards::v1::subsets::any::analyzer::Process3dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.process3d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Process3dAnalyzer;

impl ArtifactAnalyzer for Process3dAnalyzer {
    type Parts = Process3dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Process3dAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Process3dAnyAnalyzer::analyze(sources) }
}
