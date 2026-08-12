//! 💡️ Din4108 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::din4108::Din4108Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::Din4108Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a din4108 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din4108.inference")]
pub struct Din4108Inference {
    #[state(inferred)]
    pub outline: Din4108Outline,
}

impl protocol::Inference<Din4108Snapshot> for Din4108Inference {
    fn infer(snapshot: &Din4108Snapshot) -> Self {
        Self { outline: Din4108Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<Din4108Snapshot> for Din4108Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.din4108.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.din4108.inference.outline", reads: &["layers"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::din4108::standards::v1::subsets::any::schema::Din4108Builder {
    type Snapshot = Din4108Snapshot;
    type Inference = Din4108Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.din4108.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `din4108_artifact_schema_descriptor`'s registration.
pub fn din4108_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.din4108.inference",
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
        let snapshot = Din4108Snapshot::default();
        assert_eq!(Din4108Inference::infer(&snapshot), Din4108Inference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Din4108Inference::infer(&Din4108Snapshot::default()), Din4108Inference::default());
    }
}
//#endregion 🧪️Tests
