//! 💡️ Flow inference schema — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::FlowSnapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

use super::topology::{compute_flow_topology, FlowTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a flow snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow.inference")]
pub struct FlowInference {
    #[derived]
    pub topology: FlowTopology,
}

impl protocol::Inference<FlowSnapshot> for FlowInference {
    fn infer(snapshot: &FlowSnapshot) -> Self {
        let fixture = snapshot.to_fixture();
        Self { topology: compute_flow_topology(&fixture.widgets, &fixture.synapses) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&FlowSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — `FlowSnapshot`'s own `Default` bridges `semio_framework_artifact_flow_flow::FlowFixture::default()`,
/// which ships a non-empty three-widget starter graph, the same "match `infer` of the real
/// default, don't derive structurally" trick as `AddInference`'s hand-written `Default` in
/// `📡️spr/🎮️command/🦀️.rs`.
impl Default for FlowInference {
    fn default() -> Self {
        <Self as protocol::Inference<FlowSnapshot>>::infer(&FlowSnapshot::default())
    }
}

impl protocol::InferenceSpec<FlowSnapshot> for FlowInference {
    fn inference_schema_id() -> &'static str {
        "s.flow.flow.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.flow.flow.inference.topology", reads: &["content"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::FlowBuilder {
    type Snapshot = FlowSnapshot;
    type Inference = FlowInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.flow.flow.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `flow_artifact_schema_descriptor`'s registration.
pub fn flow_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.flow.flow.inference",
        inference: framework_schema::FacetLeaves {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
