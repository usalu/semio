//! 🧐️ Process3dAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::process3d::standards::v1::analyzer::Process3dAnalyzer as Process3dRawAnalyzer;
pub use crate::artifacts::process3d::standards::v1::analyzer::Process3dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.process3d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Process3dAnalyzer;

impl ArtifactAnalyzer for Process3dAnalyzer {
    type Parts = Process3dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Process3dRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Process3dRawAnalyzer::analyze(sources) }
}
