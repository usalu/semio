//! 💡️ Generation3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::generation3d::Generation3dSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use semio_framework_value_derive::{FromValue, ToValue};
use super::topology::{compute_generation3d_topology, Generation3dTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a generation3d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.generation3d.inference")]
pub struct Generation3dInference {
    #[derived]
    pub topology: Generation3dTopology,
}

impl protocol::Inference<Generation3dSnapshot> for Generation3dInference {
    fn infer(snapshot: &Generation3dSnapshot) -> Self {
        Self { topology: compute_generation3d_topology(snapshot) }
    }
}

impl protocol::InferenceSpec<Generation3dSnapshot> for Generation3dInference {
    fn inference_schema_id() -> &'static str {
        "s.procedural.generation3d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.procedural.generation3d.inference.topology", reads: &["fixture"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ `topology` is a whole-snapshot scalar (see `🧭topology/🦀️.rs`), so the default
/// `ArtifactInferrer::infer_cached` passthrough (plain `infer`, no `InferenceCache`/`InferenceSession`
/// involvement) is exactly right — nothing here benefits from per-entity incremental caching.
impl ArtifactInferrer for crate::artifacts::generation3d::standards::v1::subsets::any::schema::Generation3dBuilder {
    type Snapshot = Generation3dSnapshot;
    type Inference = Generation3dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.procedural.generation3d.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `generation3d_artifact_schema_descriptor`'s
/// registration.
pub fn generation3d_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.procedural.generation3d.inference",
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
    use flow::{FlowFixture, SynapseSpec, Widget};
    use protocol::Inference;

    //#region 🧸️Fixtures
    fn sample_snapshot() -> Generation3dSnapshot {
        let mut snapshot = Generation3dSnapshot::default();
        snapshot.fixture = FlowFixture {
            schema: "flow.fixture".into(),
            camera: flow::CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            widgets: vec![
                Widget::InputSlider { id: "a".into(), label: "A".into(), value: 1.0, min: 0.0, max: 10.0, step: 1.0 },
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
    #[test]
    fn inference_determinism_law() {
        let snapshot = sample_snapshot();
        assert_eq!(Generation3dInference::infer(&snapshot), Generation3dInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Generation3dInference::infer(&Generation3dSnapshot::default()), Generation3dInference::default());
    }

    #[test]
    fn topology_matches_the_linear_chain() {
        let snapshot = sample_snapshot();
        let inferred = Generation3dInference::infer(&snapshot);
        assert_eq!(inferred.topology.node_count, 3);
        assert_eq!(inferred.topology.edge_count, 2);
        assert!(inferred.topology.cycle_free);
        assert_eq!(inferred.topology.depth, 2);
        assert_eq!(inferred.topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
