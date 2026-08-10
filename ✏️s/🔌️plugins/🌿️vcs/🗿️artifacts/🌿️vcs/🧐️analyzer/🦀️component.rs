//! 🧐️ VcsAnalyzer (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::vcs::standards::v1::analyzer::VcsAnalyzer as VcsRawAnalyzer;
pub use crate::artifacts::vcs::standards::v1::analyzer::VcsParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.vcs", standard: StandardId("1"), subset: SubsetId("*") };

pub struct VcsAnalyzer;

impl ArtifactAnalyzer for VcsAnalyzer {
    type Parts = VcsParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { VcsRawAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { VcsRawAnalyzer::analyze(sources) }
}
