//! 🧐️ Procedural2dAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::procedural2d::standards::v1::subsets::any::analyzer::Procedural2dAnalyzer as Procedural2dAnyAnalyzer;
pub use crate::artifacts::procedural2d::standards::v1::subsets::any::analyzer::Procedural2dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural2d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Procedural2dAnalyzer;

impl ArtifactAnalyzer for Procedural2dAnalyzer {
    type Parts = Procedural2dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Procedural2dAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Procedural2dAnyAnalyzer::analyze(sources) }
}
