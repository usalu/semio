//! 🎹️ Fem2dComposer (1/✳️any) — analyzer + builder glued. Reads native `s.fem2d` sources
//! plus any of: stdio.csv, stdio.json, stdio.md, stdio.txt (IMPORT direction only — `stdio.obj`/
//! `stdio.stl` are EXPORT-only real geometry, see the standard-level composer's doc; `stdio.zip`/
//! `stdio.png` were deleted outright, no honest mapping). Writes one `s.fem2d` (1/✳️any) snapshot.

use semio_framework_plugin::{ArtifactComposer, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
use crate::artifacts::fem2d::Fem2dSnapshot;
use crate::artifacts::fem2d::standards::v1::subsets::any::analyzer::Fem2dAnalyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect { artifact_kind: "s.fem2d", standard: StandardId("1"), subset: SubsetId("*") };
const DEP_CSV: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };
const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
const DEP_MD: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };


pub struct Fem2dComposer;

impl ArtifactComposer for Fem2dComposer {
    type Snapshot = Fem2dSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] {
        &[DIALECT, DEP_CSV, DEP_JSON, DEP_MD, DEP_TXT]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        for source in sources {
            if source.dialect == DIALECT {
                let native = match &source.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                };
                let analysis = Fem2dAnalyzer::analyze(&[native]);
                if let Some(snapshot) = analysis.parts.snapshot {
                    return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                }
            }
            if source.dialect == DEP_CSV {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::fem2d::io::import::deserializers::artifacts::csv::v_rfc4180::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }
            if source.dialect == DEP_JSON {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::fem2d::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }
            if source.dialect == DEP_MD {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::fem2d::io::import::deserializers::artifacts::md::v_commonmark::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }
            if source.dialect == DEP_TXT {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::fem2d::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }

        }
        Err(ComposeError { message: "Fem2dComposer: no source in a known read dialect".into(), diagnostics: Vec::new() })
    }
}
