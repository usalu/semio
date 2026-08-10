//! 🎹️ FormsComposer (1/✳️any) — analyzer + builder glued. Reads native `s.forms` sources
//! plus any of: stdio.csv, stdio.json, stdio.xlsx, stdio.zip. Writes one `s.forms` (1/✳️any) snapshot.

use semio_framework_plugin::{ArtifactComposer, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
use crate::artifacts::forms::FormsSnapshot;
use crate::artifacts::forms::standards::v1::subsets::any::analyzer::FormsAnalyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect { artifact_kind: "s.forms", standard: StandardId("1"), subset: SubsetId("*") };
const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };


pub struct FormsComposer;

impl ArtifactComposer for FormsComposer {
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
        Err(ComposeError { message: "FormsComposer: no source in a known read dialect".into(), diagnostics: Vec::new() })
    }
}
