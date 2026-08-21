//! 💡️ Flow inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::flow::FlowSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_flow_topology, FlowTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a flow snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow.inference")]
pub struct FlowInference {
    #[derived]
    pub topology: FlowTopology,
}

impl protocol::Inference<FlowSnapshot> for FlowInference {
    async fn infer(snapshot: &FlowSnapshot) -> Self {
        let fixture = snapshot.to_fixture();
        Self { topology: compute_flow_topology(&fixture.widgets, &fixture.synapses) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&FlowSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — `FlowSnapshot`'s own `Default` bridges `flow::FlowFixture::default()`,
/// which ships a non-empty three-widget starter graph, the same "match `infer` of the real
/// default, don't derive structurally" trick as `AddInference`'s hand-written `Default` in
/// `📡️spr/🎮️command/🦀️component.rs`.
impl Default for FlowInference {
    fn default() -> Self {
        <Self as protocol::Inference<FlowSnapshot>>::infer(&FlowSnapshot::default())
    }
}

impl protocol::InferenceSpec<FlowSnapshot> for FlowInference {
    async fn inference_schema_id() -> &'static str {
        "s.flow.flow.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.flow.flow.inference.topology", reads: &["content"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::flow::standards::v1::subsets::any::schema::FlowBuilder {
    type Snapshot = FlowSnapshot;
    type Inference = FlowInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.flow.flow.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `flow_artifact_schema_descriptor`'s registration.
pub async fn flow_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.flow.flow.inference",
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
    use flow::Widget;
    use protocol::Inference;

    async fn chain_snapshot() -> FlowSnapshot {
        let mut fixture = FlowSnapshot::default().to_fixture();
        fixture.widgets = vec![Widget::InputSlider { id: "a".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 }, Widget::InputSlider { id: "b".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 }];
        fixture.synapses = vec![flow::SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: String::new(), to_port: String::new() }];
        FlowSnapshot::from_fixture(fixture)
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = chain_snapshot();
        assert_eq!(FlowInference::infer(&snapshot), FlowInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(FlowInference::infer(&FlowSnapshot::default()), FlowInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn topology_counts_every_widget_exactly_once() {
        let snapshot = chain_snapshot();
        let inferred = FlowInference::infer(&snapshot);
        let widget_count = snapshot.to_fixture().widgets.len();
        assert_eq!(inferred.topology.node_count as usize, widget_count);
        assert_eq!(inferred.topology.topo_order.len(), widget_count);
        assert!(inferred.topology.cycle_free);
    }
}
//#endregion 🧪️Tests
