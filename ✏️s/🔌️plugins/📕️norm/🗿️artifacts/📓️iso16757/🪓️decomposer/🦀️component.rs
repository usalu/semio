//! Iso16757Decomposer
use semio_framework_plugin::{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource};
use crate::artifacts::iso16757::Iso16757Snapshot;

#[derive(Clone, Debug, Default)]
pub struct Iso16757Parts { pub snapshot: Option<Iso16757Snapshot> }

pub struct Iso16757Decomposer;

impl ArtifactDecomposer for Iso16757Decomposer {
    type Snapshot = Iso16757Snapshot;
    type Parts = Iso16757Parts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = Iso16757Parts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <Iso16757Snapshot as store::DocumentDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("iso16757.decompose.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                DecomposeSource::Binary(bytes) => match <Iso16757Snapshot as store::DocumentPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("iso16757.decompose.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
            }
        }
        Decomposition { parts, confidence, diagnostics }
    }
}
