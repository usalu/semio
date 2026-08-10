//! GisterrainDecomposer
use semio_framework_plugin::{ArtifactDecomposer, Confidence, Decomposition, DecomposeSource};
use crate::artifacts::gisterrain::GisTerrainSnapshot;

#[derive(Clone, Debug, Default)]
pub struct GisterrainParts { pub snapshot: Option<GisTerrainSnapshot> }

pub struct GisterrainDecomposer;

impl ArtifactDecomposer for GisterrainDecomposer {
    type Snapshot = GisTerrainSnapshot;
    type Parts = GisterrainParts;
    fn decompose(sources: &[DecomposeSource<'_>]) -> Decomposition<Self::Parts> {
        let mut parts = GisterrainParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = Confidence::High;
        for source in sources {
            match source {
                DecomposeSource::Text(text) => match <GisTerrainSnapshot as store::DocumentDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("gisterrain.decompose.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                DecomposeSource::Binary(bytes) => match <GisTerrainSnapshot as store::DocumentPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = Confidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("gisterrain.decompose.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
            }
        }
        Decomposition { parts, confidence, diagnostics }
    }
}
