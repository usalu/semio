//! 🚪️ IO `s.wfc.grid3d` (1/✳️any) — native DSL/pack, declared through the subset root's
//! `io()`/`IoDeclaration`. The composition below is what lets a foreign source in a read dialect be
//! composed INTO this dialect; conformance runs inside the io mechanism's own entry, never here.

pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt"]
}

pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt"]
}

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::Grid3dAnalyzer;
    use crate::Grid3dSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.wfc.grid3d", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Grid3dComposerComposition;

    impl ArtifactComposition for Grid3dComposerComposition {
        type Snapshot = Grid3dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect != DIALECT && source.dialect != DEP_TXT {
                    continue;
                }
                let native = match &source.payload {
                    AnalyzeSource::Text(text) => AnalyzeSource::Text(text),
                    AnalyzeSource::Binary(bytes) => AnalyzeSource::Binary(bytes),
                };
                let analysis = Grid3dAnalyzer::analyze(&[native]);
                if let Some(snapshot) = analysis.parts.snapshot {
                    return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                }
            }
            Err(ComposeError { message: "Grid3dComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🎪️Tests
#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🎪️Tests
