//! 🧬️ Lowpoly snapshot schema — artifact-lane fields only.
//!
//! `ArtifactDsl` and `ArtifactPack` are the derived spec-driven text and pack of the one
//! `dsl::DslRecord` spec, which carries each object's composed `mesh` child handle.

use crate::{LowpolyObject, LOWPOLY_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;




//#region 🔖️Snapshot
/// 📸️ Persisted lowpoly document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "lowpoly")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolySnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub objects: Vec<LowpolyObject>,
}

impl Default for LowpolySnapshot {
    fn default() -> Self {
        Self { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️DocumentHelpers
//#endregion 🔖️DocumentHelpers
