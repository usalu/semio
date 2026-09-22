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
mod tests {
    use super::*;
    use crate::examples::blocks;
    use crate::Grid3dSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeSource, Dialect, StandardId, SubsetId};

    #[test]
    fn io_declares_the_stdio_txt_channel_and_no_foreign_entries() {
        let declaration = crate::standards::v1::subsets::any::io();
        assert!(declaration.entries.is_empty(), "grid3d ships no foreign hop of its own");
        assert_eq!(import_stdio_kinds(), &["stdio.txt"]);
        assert_eq!(export_stdio_kinds(), &["stdio.txt"]);
        assert!(declaration.native.snapshot.text.is_some());
        assert!(declaration.native.snapshot.binary.is_some());
    }

    #[test]
    fn the_derived_composition_rebuilds_blocks_from_the_native_dialect() {
        let document = blocks::snapshot();
        let pack = <Grid3dSnapshot as store::ArtifactPack>::encode_pack(&document);
        let dialect = Dialect { artifact_kind: "s.wfc.grid3d", standard: StandardId("1"), subset: SubsetId("*") };
        let sources = [ComposeSource { dialect, payload: AnalyzeSource::Binary(&pack) }];
        let composition = Grid3dComposerComposition::compose(&sources).expect("native dialect composes");
        assert_eq!(composition.snapshot.seed, document.seed);
        assert_eq!(composition.snapshot.width, document.width);
        assert_eq!(composition.snapshot.tiles.len(), document.tiles.len());
        assert_eq!(composition.snapshot.rules.len(), document.rules.len());
    }
}
//#endregion 🎪️Tests
