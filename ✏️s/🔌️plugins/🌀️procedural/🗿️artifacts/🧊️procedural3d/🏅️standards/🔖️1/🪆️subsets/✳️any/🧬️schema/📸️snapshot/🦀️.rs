//! 🧬️ Procedural3d snapshot schema — artifact-lane fields only.

use flow::playbook::GenerationPlayState;
use flow::FlowFixture;
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
pub use flow::playbook::GenerationPlayRoot;

//#region 🔖️Procedural3dSnapshot
/// 🧬️ Procedural3dSnapshot facet type.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.procedural3d")]

pub struct Procedural3dSnapshot {
    #[state(artifact)]
    pub fixture: FlowFixture,
    #[state(artifact)]
    pub generation: GenerationPlayRoot,
}
//#endregion 🔖️Procedural3dSnapshot

impl Default for Procedural3dSnapshot {
    fn default() -> Self {
        Self { fixture: FlowFixture::default(), generation: GenerationPlayState::default().into() }
    }
}
