//! 🚪️ IO s.forms (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"] }
pub fn forms_to_wire(from: &crate::artifacts::forms::FormsSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(from)
}
pub fn forms_from_wire(bytes: &[u8]) -> Result<crate::artifacts::forms::FormsSnapshot, store::PackError> {
    <crate::artifacts::forms::FormsSnapshot as store::ArtifactPack>::decode_pack(bytes)
}
pub fn pack_err_as_text(err: store::PackError) -> store::TextError {
    store::TextError::new(err.to_string(), dsl::TextSpan::at(1, 1))
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::forms::FormsSnapshot;
    use crate::artifacts::forms::standards::v1::subsets::any::schema::FormsAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.forms", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };


    pub struct FormsComposerComposition;

    impl ArtifactComposition for FormsComposerComposition {
        type Snapshot = FormsSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_JSON]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                    };
                    let analysis = FormsAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_JSON {
                    let text: Option<String> = match &source.payload {
                        AnalyzeSource::Text(t) => Some(t.to_string()),
                        AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                    };
                    if let Some(text) = text {
                        if let Ok(snapshot) = crate::artifacts::forms::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text) {
                            return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                        }
                    }
                }

            }
            Err(ComposeError { message: "FormsComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
