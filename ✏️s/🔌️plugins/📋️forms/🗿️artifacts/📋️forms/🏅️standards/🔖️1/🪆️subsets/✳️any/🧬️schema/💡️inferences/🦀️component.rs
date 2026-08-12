//! 💡️ Forms inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::forms::FormsSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_forms_topology, FormsTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a forms snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms.inference")]
pub struct FormsInference {
    #[state(inferred)]
    pub topology: FormsTopology,
}

impl Default for FormsInference {
    fn default() -> Self {
        <Self as protocol::Inference<FormsSnapshot>>::infer(&FormsSnapshot::default())
    }
}

impl protocol::Inference<FormsSnapshot> for FormsInference {
    fn infer(snapshot: &FormsSnapshot) -> Self {
        Self { topology: compute_forms_topology(&snapshot.steps) }
    }
}

impl protocol::InferenceSpec<FormsSnapshot> for FormsInference {
    fn inference_schema_id() -> &'static str {
        "s.forms.forms.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.forms.forms.inference.topology", reads: &["steps"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::forms::standards::v1::subsets::any::schema::FormsBuilder {
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
    use crate::artifacts::forms::FormStep;
    use protocol::Inference;

    //#region 🧸️Fixtures
    fn step_with_conditional_block() -> FormsSnapshot {
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
        serde_json::from_str::<FormsSnapshot>(json).expect("valid forms snapshot json")
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = step_with_conditional_block();
        assert_eq!(FormsInference::infer(&snapshot), FormsInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(FormsInference::infer(&FormsSnapshot::default()), FormsInference::default());
    }

    #[test]
    fn topology_orders_the_conditioned_block_after_its_dependency() {
        let snapshot = step_with_conditional_block();
        let inferred = FormsInference::infer(&snapshot);
        let size_index = inferred.topology.topo_order.iter().position(|id| id == "team-size").unwrap();
        let name_index = inferred.topology.topo_order.iter().position(|id| id == "team-name").unwrap();
        assert!(size_index < name_index, "team-name's condition reads team-size, so team-size must precede it");
        assert!(inferred.topology.cycle_free);
    }

    #[test]
    fn empty_steps_produce_empty_topology() {
        let snapshot = FormsSnapshot { steps: Vec::<FormStep>::new(), ..FormsSnapshot::default() };
        let inferred = FormsInference::infer(&snapshot);
        assert!(inferred.topology.topo_order.is_empty());
        assert_eq!(inferred.topology.node_count, 0);
        assert!(inferred.topology.cycle_free);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
