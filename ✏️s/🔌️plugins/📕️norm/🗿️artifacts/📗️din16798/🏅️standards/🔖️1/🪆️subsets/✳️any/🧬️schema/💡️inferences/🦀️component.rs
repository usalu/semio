//! 💡️ Din16798 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::din16798::Din16798Snapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::Din16798Outline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a din16798 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din16798.inference")]
pub struct Din16798Inference {
    #[state(inferred)]
    pub outline: Din16798Outline,
}

impl protocol::Inference<Din16798Snapshot> for Din16798Inference {
    fn infer(snapshot: &Din16798Snapshot) -> Self {
        Self { outline: Din16798Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<Din16798Snapshot> for Din16798Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.din16798.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.din16798.inference.outline", reads: &[] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::din16798::standards::v1::subsets::any::schema::Din16798Builder {
    type Snapshot = Din16798Snapshot;
    type Inference = Din16798Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.din16798.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `din16798_artifact_schema_descriptor`'s registration.
pub fn din16798_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.norm.din16798.inference",
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
        let snapshot = Din16798Snapshot::default();
        assert_eq!(Din16798Inference::infer(&snapshot), Din16798Inference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Din16798Inference::infer(&Din16798Snapshot::default()), Din16798Inference::default());
    }
}
//#endregion 🧪️Tests
