//! 🧬️ Procedural3d snapshot schema — persistent fields only.

use flow::FlowFixture;
use flow::playbook::GenerationPlayState;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Procedural3dSnapshot
/// 🧬️ Procedural3dSnapshot facet type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.procedural3d")]

pub struct Procedural3dSnapshot {
    #[state(artifact)] pub fixture: FlowFixture,
    #[state(artifact)] pub generation: GenerationPlayState}
//#endregion 🔖️Procedural3dSnapshot

impl Default for Procedural3dSnapshot {
    fn default() -> Self {
        Self {
            fixture: FlowFixture::default(),
            generation: GenerationPlayState::default()}
    }
}
