//! 🚪️ IO s.vdi3805 (1/✳️any) — no stdio format bridges. W5a (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) deleted the five
//! degenerate leaves (csv/json/txt/xlsx/zip) that either fabricated a one-cell-CSV/raw-DSL-dump
//! shape or silently defaulted to `Vdi3805Snapshot::default()` on import (an honesty bug, not a
//! real codec). Vdi3805Snapshot is a compliance document (scalar fields plus a handful of nested
//! records), not a flat row/column table, so no honest whole-artifact CSV round-trip exists to
//! re-register in their place. Registration flows through 🎹️composer::register (called once from
//! ⚙️engine::register) for the native `s.vdi3805` dialect only.
pub fn import_stdio_kinds() -> &'static [&'static str] { &[] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &[] }
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::vdi3805::Vdi3805Snapshot;
    use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::Vdi3805Analyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.vdi3805", standard: StandardId("1"), subset: SubsetId("*") };

    pub struct Vdi3805ComposerComposition;

    impl ArtifactComposition for Vdi3805ComposerComposition {
        type Snapshot = Vdi3805Snapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = Vdi3805Analyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "Vdi3805ComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
