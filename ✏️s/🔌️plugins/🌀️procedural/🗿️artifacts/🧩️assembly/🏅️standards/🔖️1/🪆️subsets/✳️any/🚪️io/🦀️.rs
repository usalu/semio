//! 🚪️ IO s.assembly (1/✳️any) — native DSL/pack plus full-fidelity `s.stdio.txt@utf-8`.
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt"]
}

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::AssemblyAnalyzer;
    use crate::AssemblySnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.assembly", standard: StandardId("1"), subset: SubsetId("*") };
    const OWNER: Dialect = Dialect { artifact_kind: "s.procedural.assembly", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct AssemblyComposerComposition;

    impl ArtifactComposition for AssemblyComposerComposition {
        type Snapshot = AssemblySnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, OWNER, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT || source.dialect == OWNER {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = AssemblyAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::High, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "AssemblyComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️IoRegistry
pub mod io_registry {
    use crate::standards::v1::subsets::any::schema::AssemblyComposer as AssemblyAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<AssemblyAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️IoRegistry
