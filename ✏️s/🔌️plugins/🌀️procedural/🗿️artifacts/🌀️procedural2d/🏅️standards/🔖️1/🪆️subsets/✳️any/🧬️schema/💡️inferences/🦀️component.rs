//! 💡️ Procedural2d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::procedural2d::Procedural2dSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_procedural2d_topology, Procedural2dTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a procedural2d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.procedural2d.inference")]
pub struct Procedural2dInference {
    #[derived]
    pub topology: Procedural2dTopology,
}

impl protocol::Inference<Procedural2dSnapshot> for Procedural2dInference {
    async fn infer(snapshot: &Procedural2dSnapshot) -> Self {
        Self { topology: compute_procedural2d_topology(snapshot) }
    }
}

impl protocol::InferenceSpec<Procedural2dSnapshot> for Procedural2dInference {
    async fn inference_schema_id() -> &'static str {
        "s.procedural.procedural2d.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.procedural.procedural2d.inference.topology", reads: &["fixture"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ `topology` is a whole-snapshot scalar (see `🧭topology/🦀️component.rs`), so the default
/// `ArtifactInferrer::infer_cached` passthrough (plain `infer`, no `InferenceCache`/`InferenceSession`
/// involvement) is exactly right — nothing here benefits from per-entity incremental caching.
impl ArtifactInferrer for crate::artifacts::procedural2d::standards::v1::subsets::any::schema::Procedural2dBuilder {
    type Snapshot = Procedural2dSnapshot;
    type Inference = Procedural2dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.procedural.procedural2d.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `procedural2d_artifact_schema_descriptor`'s
/// registration.
pub async fn procedural2d_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.procedural.procedural2d.inference",
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
    use flow::{FlowFixture, SynapseSpec, Widget};
    use protocol::Inference;

    //#region 🧸️Fixtures
    async fn sample_snapshot() -> Procedural2dSnapshot {
        let mut snapshot = Procedural2dSnapshot::default();
        snapshot.fixture = FlowFixture {
            schema: "flow.fixture".into(),
            camera: flow::CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![
                Widget::InputSlider { id: "a".into(), value: 1.0, min: 0.0, max: 10.0, step: 1.0 },
                Widget::Neuron { id: "b".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: false },
                Widget::OutputPreview { id: "c".into(), preview: Default::default(), expanded: Default::default() },
            ],
            synapses: vec![
                SynapseSpec { id: "s1".into(), from: "a".into(), to: "b".into(), from_port: "value".into(), to_port: "a".into() },
                SynapseSpec { id: "s2".into(), from: "b".into(), to: "c".into(), from_port: "sum".into(), to_port: String::new() },
            ],
            layout: Default::default(),
        };
        snapshot
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = sample_snapshot();
        assert_eq!(Procedural2dInference::infer(&snapshot), Procedural2dInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(Procedural2dInference::infer(&Procedural2dSnapshot::default()), Procedural2dInference::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn topology_matches_the_linear_chain() {
        let snapshot = sample_snapshot();
        let inferred = Procedural2dInference::infer(&snapshot);
        assert_eq!(inferred.topology.node_count, 3);
        assert_eq!(inferred.topology.edge_count, 2);
        assert!(inferred.topology.cycle_free);
        assert_eq!(inferred.topology.depth, 2);
        assert_eq!(inferred.topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
