//! 🧬️ Generation2d snapshot schema — artifact-lane fields only.

use flow::playbook::GenerationPlayRoot;
use flow::FlowFixture;
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Generation2dSnapshot
/// 🧬️ Generation2dSnapshot facet type.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.generation2d")]

pub struct Generation2dSnapshot {
    #[state(artifact)]
    pub fixture: FlowFixture,
    #[state(artifact)]
    pub generation: GenerationPlayRoot,
}
//#endregion 🔖️Generation2dSnapshot

impl Default for Generation2dSnapshot {
    fn default() -> Self {
        Self { fixture: FlowFixture::default(), generation: GenerationPlayRoot::default() }
    }
}
