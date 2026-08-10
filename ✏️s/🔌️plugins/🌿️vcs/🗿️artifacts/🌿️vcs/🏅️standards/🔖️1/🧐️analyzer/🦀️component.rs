//! 🧐️ VcsAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::vcs::standards::v1::subsets::any::analyzer::VcsAnalyzer as VcsAnyAnalyzer;
pub use crate::artifacts::vcs::standards::v1::subsets::any::analyzer::VcsParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.vcs", standard: StandardId("1"), subset: SubsetId("*") };

pub struct VcsAnalyzer;

impl ArtifactAnalyzer for VcsAnalyzer {
    type Parts = VcsParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { VcsAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { VcsAnyAnalyzer::analyze(sources) }
}
