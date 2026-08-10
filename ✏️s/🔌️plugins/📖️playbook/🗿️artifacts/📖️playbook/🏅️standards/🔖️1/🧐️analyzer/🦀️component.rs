//! 🧐️ PlaybookAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::playbook::standards::v1::subsets::any::analyzer::PlaybookAnalyzer as PlaybookAnyAnalyzer;
pub use crate::artifacts::playbook::standards::v1::subsets::any::analyzer::PlaybookParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.playbook", standard: StandardId("1"), subset: SubsetId("*") };

pub struct PlaybookAnalyzer;

impl ArtifactAnalyzer for PlaybookAnalyzer {
    type Parts = PlaybookParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { PlaybookAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { PlaybookAnyAnalyzer::analyze(sources) }
}
