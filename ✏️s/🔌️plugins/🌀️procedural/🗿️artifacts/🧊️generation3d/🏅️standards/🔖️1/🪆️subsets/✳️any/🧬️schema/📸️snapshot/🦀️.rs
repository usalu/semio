//! 🧬️ Generation3d snapshot schema — artifact-lane fields only.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::FlowFixture;
pub use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
use semio_framework_artifact_playbook_playbook::GenerationPlayState;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Generation3dSnapshot
/// 🧬️ Generation3dSnapshot facet type.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.generation3d")]

pub struct Generation3dSnapshot {
    #[state(artifact)]
    pub fixture: FlowFixture,
    #[state(artifact)]
    pub generation: GenerationPlayRoot,
}
//#endregion 🔖️Generation3dSnapshot

impl Default for Generation3dSnapshot {
    fn default() -> Self {
        Self { fixture: FlowFixture::default(), generation: GenerationPlayState::default().into() }
    }
}
