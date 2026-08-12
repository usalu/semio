//! 💡️ Iso16757 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::iso16757::Iso16757Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::Iso16757Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a iso16757 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.iso16757.inference")]
pub struct Iso16757Inference {
    #[state(inferred)]
    pub outline: Iso16757Outline,
}

impl protocol::Inference<Iso16757Snapshot> for Iso16757Inference {
    fn infer(snapshot: &Iso16757Snapshot) -> Self {
        Self { outline: Iso16757Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<Iso16757Snapshot> for Iso16757Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.iso16757.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.iso16757.inference.outline", reads: &["part_number_inputs"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::iso16757::standards::v1::subsets::any::schema::Iso16757Builder {
    type Snapshot = Iso16757Snapshot;
    type Inference = Iso16757Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.iso16757.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `iso16757_artifact_schema_descriptor`'s registration.
pub fn iso16757_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.iso16757.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    #[test]
    fn inference_determinism_law() {
        let snapshot = Iso16757Snapshot::default();
        assert_eq!(Iso16757Inference::infer(&snapshot), Iso16757Inference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Iso16757Inference::infer(&Iso16757Snapshot::default()), Iso16757Inference::default());
    }
}
//#endregion 🧪️Tests
