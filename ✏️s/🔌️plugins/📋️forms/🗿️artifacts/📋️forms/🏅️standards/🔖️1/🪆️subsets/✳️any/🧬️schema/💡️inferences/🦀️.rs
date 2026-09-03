//! 💡️ Forms inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::forms::{forms_steps, FormsSnapshot};
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
#[cfg(test)]
use serde::{Deserialize, Serialize};

use super::topology::{compute_forms_topology, FormsTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a forms snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[artifact_schema(id = "s.forms.forms.inference")]
pub struct FormsInference {
    #[derived]
    pub topology: FormsTopology,
}

impl Default for FormsInference {
    fn default() -> Self {
        <Self as protocol::Inference<FormsSnapshot>>::infer(&FormsSnapshot::default())
    }
}

impl protocol::Inference<FormsSnapshot> for FormsInference {
    async fn infer(snapshot: &FormsSnapshot) -> Self {
        Self { topology: compute_forms_topology(&forms_steps(snapshot)) }
    }
}

impl protocol::InferenceSpec<FormsSnapshot> for FormsInference {
    async fn inference_schema_id() -> &'static str {
        "s.forms.forms.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.forms.forms.inference.topology", reads: &["steps"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 🎯️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM: `ArtifactInferrer::infer` takes
/// `&Self::Snapshot` (never `&self`), so the impl target is a pure type-level anchor — a local
/// zero-sized marker, not the deleted `derive_artifact_facets!`-generated `FormsBuilder`
/// (retargeting onto `semio_framework_plugin::app::SnapshotBuilder<S, M>` is an orphan-rule
/// violation: it is a foreign, non-`#[fundamental]` generic struct — confirmed by the
/// `🎬️sequence` fan-out pass, `📓️w4-sequence-report.md` `## recipeGaps` #1).
pub struct FormsInferrer;
impl ArtifactInferrer for FormsInferrer {
    type Snapshot = FormsSnapshot;
    type Inference = FormsInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.forms.forms.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `forms_artifact_schema_descriptor`'s registration.
pub fn forms_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.forms.forms.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;

    //#region 🧸️Fixtures
    /// 🩹️ `FormsSnapshot` composes `structure`/`results` handles (ticket
    /// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM) so it no longer deserializes raw step/block JSON
    /// directly — `flow::playbook::PlaybookSpec` is the SAME `{schema,id,version,title,steps}`
    /// camelCase shape, so this fixture deserializes through it instead.
    async fn step_with_conditional_block() -> FormsSnapshot {
        let json = r#"{
            "schema": "forms.form",
            "id": "forms",
            "version": "1",
            "title": null,
            "steps": [
                {
                    "id": "s1",
                    "title": "Step 1",
                    "blocks": [
                        { "id": "team-size", "label": "Team size", "kind": "slider" },
                        {
                            "id": "team-name",
                            "label": "Team name",
                            "kind": "text",
                            "condition": { "kind": "truthy", "expr": { "kind": "var", "name": "team-size" } }
                        }
                    ]
                }
            ]
        }"#;
        let spec = dsl::os_pack::json::from_json_str::<flow::playbook::PlaybookSpec>(json).expect("valid playbook spec json");
        crate::artifacts::forms::forms_snapshot_with_state(spec.schema, spec.id, spec.version, spec.title, spec.steps)
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = step_with_conditional_block();
        assert_eq!(FormsInference::infer(&snapshot), FormsInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(FormsInference::infer(&FormsSnapshot::default()), FormsInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn topology_orders_the_conditioned_block_after_its_dependency() {
        let snapshot = step_with_conditional_block();
        let inferred = FormsInference::infer(&snapshot);
        let size_index = inferred.topology.topo_order.iter().position(|id| id == "team-size").unwrap();
        let name_index = inferred.topology.topo_order.iter().position(|id| id == "team-name").unwrap();
        assert!(size_index < name_index, "team-name's condition reads team-size, so team-size must precede it");
        assert!(inferred.topology.cycle_free);
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_steps_produce_empty_topology() {
        let snapshot = FormsSnapshot::default();
        let inferred = FormsInference::infer(&snapshot);
        assert!(inferred.topology.topo_order.is_empty());
        assert_eq!(inferred.topology.node_count, 0);
        assert!(inferred.topology.cycle_free);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
