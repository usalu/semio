//! 🎹️ Din18599Composer (1/✳️any) — analyzer glued. Reads native `s.din18599` sources only: W5a
//! (ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT) deleted the five stdio format-bridge read branches (csv/json/txt/xlsx/zip) —
//! see this subset's `🚪️io/🦀️component.rs` doc comment for why none of them were honest. Writes one
//! `s.din18599` (1/✳️any) snapshot.

use semio_framework_plugin::{ArtifactComposer, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
use crate::artifacts::din18599::Din18599Snapshot;
use crate::artifacts::din18599::standards::v1::subsets::any::analyzer::Din18599Analyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect { artifact_kind: "s.din18599", standard: StandardId("1"), subset: SubsetId("*") };

pub struct Din18599Composer;

impl ArtifactComposer for Din18599Composer {
    type Snapshot = Din18599Snapshot;
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
                let analysis = Din18599Analyzer::analyze(&[native]);
                if let Some(snapshot) = analysis.parts.snapshot {
                    return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                }
            }
        }
        Err(ComposeError { message: "Din18599Composer: no source in a known read dialect".into(), diagnostics: Vec::new() })
    }
}
