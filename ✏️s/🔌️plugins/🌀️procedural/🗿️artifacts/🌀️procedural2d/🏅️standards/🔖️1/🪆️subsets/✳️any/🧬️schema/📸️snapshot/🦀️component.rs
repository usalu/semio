//! 🧬️ Procedural2d snapshot schema — persistent fields only.

use flow::FlowFixture;
use flow::playbook::GenerationPlayState;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Procedural2dSnapshot
/// 🧬️ Procedural2dSnapshot facet type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.procedural2d")]

pub struct Procedural2dSnapshot {
    #[state(artifact)] pub fixture: FlowFixture,
    #[state(artifact)] pub generation: GenerationPlayState}
//#endregion 🔖️Procedural2dSnapshot

impl Default for Procedural2dSnapshot {
    fn default() -> Self {
        Self {
            fixture: FlowFixture::default(),
            generation: GenerationPlayState::default()}
    }
}
