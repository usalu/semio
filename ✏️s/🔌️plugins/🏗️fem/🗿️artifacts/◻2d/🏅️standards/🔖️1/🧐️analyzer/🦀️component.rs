//! 🧐️ Fem2dAnalyzer (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::fem2d::standards::v1::subsets::any::analyzer::Fem2dAnalyzer as Fem2dAnyAnalyzer;
pub use crate::artifacts::fem2d::standards::v1::subsets::any::analyzer::Fem2dParts;

const DIALECT: Dialect = Dialect { artifact_kind: "s.fem2d", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Fem2dAnalyzer;

impl ArtifactAnalyzer for Fem2dAnalyzer {
    type Parts = Fem2dParts;
    const DIALECT: Dialect = DIALECT;
    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence { Fem2dAnyAnalyzer::sniff(source) }
    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> { Fem2dAnyAnalyzer::analyze(sources) }
}
