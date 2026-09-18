//! 🚪️ IO s.wfc.wfc3d (1/✳️any) — native DSL/pack plus full-fidelity `s.stdio.txt@utf-8`.
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.txt"]
}

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::Wfc3dAnalyzer;
    use crate::Wfc3dSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.wfc.wfc3d", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Wfc3dComposerComposition;

    impl ArtifactComposition for Wfc3dComposerComposition {
        type Snapshot = Wfc3dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(text) => AnalyzeSource::Text(text),
                        AnalyzeSource::Binary(bytes) => AnalyzeSource::Binary(bytes),
                    };
                    let analysis = Wfc3dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(text) => text.as_bytes().to_vec(),
                        AnalyzeSource::Binary(binary) => binary.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::High, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "Wfc3dComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️IoRegistry
pub mod io_registry {
    use crate::standards::v1::subsets::any::schema::Wfc3dComposer as Wfc3dAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<Wfc3dAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️IoRegistry

//#region 🔖️IoDeclaration
/// 🚪️ The `io_mechanism` declaration this subset's `subset()` binds: the native DSL/pack codec plus
/// the five hand-authored `dsl::LanguageSpec`s (document/op/diff/pack/spr). Foreign-format hops go
/// through the `ComposerEntry` registry above, so `entries` stays empty here.
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::{Wfc3dMutation, Wfc3dSnapshot, WFC3D_DOCUMENT_SCHEMA};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};

    let langs = crate::wfc3d_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<Wfc3dSnapshot, Wfc3dMutation>(WFC3D_DOCUMENT_SCHEMA.to_string()),
        },
        entries: &[],
    }
}
//#endregion 🔖️IoDeclaration
