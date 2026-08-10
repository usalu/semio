//! 🧬️ Mathematical snapshot schema — persistent fields only.

use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalGraph};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted mathematical document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.mathematical.mathematical")]
pub struct MathematicalSnapshot {
    #[state(persistent)]
    pub graph: MathematicalGraph,
    #[state(persistent)]
    pub geometry: MathematicalGeometry,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack via the DSL mirror in `crate::artifacts::mathematical::dsl`.
impl store::ArtifactDsl for MathematicalSnapshot {
    const EXTENSION: &'static str = "mathematical";
    fn envelope_id() -> &'static str {
        "mathematical.mathematical"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let dsl_snapshot = <crate::artifacts::mathematical::dsl::MathematicalSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?;
        crate::artifacts::mathematical::dsl::mathematical_snapshot_from_dsl(dsl_snapshot).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        <crate::artifacts::mathematical::dsl::MathematicalSnapshotDsl as store::ArtifactDsl>::print_dsl(&crate::artifacts::mathematical::dsl::mathematical_snapshot_to_dsl(self))
    }
}

impl store::ArtifactPack for MathematicalSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        crate::artifacts::mathematical::dsl::mathematical_snapshot_to_dsl(self).encode_pack_with(options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let dsl_snapshot = crate::artifacts::mathematical::dsl::MathematicalSnapshotDsl::decode_pack_with(bytes, options)?;
        crate::artifacts::mathematical::dsl::mathematical_snapshot_from_dsl(dsl_snapshot).map_err(|message| store::text_error_to_pack_error(store::TextError::new(message, store::TextSpan::at(1, 1))))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for MathematicalSnapshot {
    fn default() -> Self {
        Self {
            graph: MathematicalGraph::default(),
            geometry: MathematicalGeometry::default(),
        }
    }
}
//#endregion 🔖️Snapshot
