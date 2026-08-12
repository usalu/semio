//! 💡️ Vdi3805 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::vdi3805::Vdi3805Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::Vdi3805Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a vdi3805 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.vdi3805.inference")]
pub struct Vdi3805Inference {
    #[state(inferred)]
    pub outline: Vdi3805Outline,
}

impl protocol::Inference<Vdi3805Snapshot> for Vdi3805Inference {
    fn infer(snapshot: &Vdi3805Snapshot) -> Self {
        Self { outline: Vdi3805Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<Vdi3805Snapshot> for Vdi3805Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.vdi3805.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.vdi3805.inference.outline", reads: &["edition_profile", "geometry", "curves"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::vdi3805::standards::v1::subsets::any::schema::Vdi3805Builder {
    type Snapshot = Vdi3805Snapshot;
    type Inference = Vdi3805Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.vdi3805.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `vdi3805_artifact_schema_descriptor`'s registration.
pub fn vdi3805_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.vdi3805.inference",
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
        let snapshot = Vdi3805Snapshot::default();
        assert_eq!(Vdi3805Inference::infer(&snapshot), Vdi3805Inference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Vdi3805Inference::infer(&Vdi3805Snapshot::default()), Vdi3805Inference::default());
    }
}
//#endregion 🧪️Tests
