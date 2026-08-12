//! 💡️ En1993 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::en1993::En1993Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::En1993Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1993 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1993.inference")]
pub struct En1993Inference {
    #[state(inferred)]
    pub outline: En1993Outline,
}

impl protocol::Inference<En1993Snapshot> for En1993Inference {
    fn infer(snapshot: &En1993Snapshot) -> Self {
        Self { outline: En1993Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1993Snapshot> for En1993Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1993.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1993.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::en1993::standards::v1::subsets::any::schema::En1993Builder {
    type Snapshot = En1993Snapshot;
    type Inference = En1993Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1993.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1993_artifact_schema_descriptor`'s registration.
pub fn en1993_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1993.inference",
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
        let snapshot = En1993Snapshot::default();
        assert_eq!(En1993Inference::infer(&snapshot), En1993Inference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(En1993Inference::infer(&En1993Snapshot::default()), En1993Inference::default());
    }
}
//#endregion 🧪️Tests
