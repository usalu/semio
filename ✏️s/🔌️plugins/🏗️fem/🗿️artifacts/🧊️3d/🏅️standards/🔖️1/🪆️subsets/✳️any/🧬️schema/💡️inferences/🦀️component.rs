//! 💡️ Fem3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::artifacts::fem3d::Fem3dSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{compute_fem3d_bounds, Fem3dBounds};

//#region 🔖️Inference
/// 💡️ Everything inferable from a fem3d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.fem.fem3d.inference")]
pub struct Fem3dInference {
    #[derived]
    pub bounds: Fem3dBounds,
}

impl protocol::Inference<Fem3dSnapshot> for Fem3dInference {
    fn infer(snapshot: &Fem3dSnapshot) -> Self {
        Self { bounds: compute_fem3d_bounds(snapshot) }
    }
}

impl protocol::InferenceSpec<Fem3dSnapshot> for Fem3dInference {
    fn inference_schema_id() -> &'static str {
        "s.fem.fem3d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.fem.fem3d.inference.bounds", reads: &["nodes", "elements"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ `bounds` is a whole-snapshot scalar (see `📦bounds/🦀️component.rs`), so the default
/// `ArtifactInferrer::infer_cached` passthrough (plain `infer`, no `InferenceCache`/`InferenceSession`
/// involvement) is exactly right — nothing here benefits from per-entity incremental caching.
impl ArtifactInferrer for crate::artifacts::fem3d::standards::v1::subsets::any::schema::Fem3dBuilder {
    type Snapshot = Fem3dSnapshot;
    type Inference = Fem3dInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.fem.fem3d.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `fem3d_artifact_schema_descriptor`'s registration.
pub fn fem3d_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.fem.fem3d.inference",
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
    use crate::artifacts::fem3d::FemNode;
    use protocol::Inference;

    //#region 🧸️Fixtures
    fn sample_snapshot() -> Fem3dSnapshot {
        Fem3dSnapshot {
            nodes: vec![
                FemNode { id: "n1".into(), x: 0.0, y: 0.0, z: 0.0 },
                FemNode { id: "n2".into(), x: 4.0, y: 0.0, z: 0.0 },
                FemNode { id: "n3".into(), x: 4.0, y: 3.0, z: 2.0 },
            ],
            ..Default::default()
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = sample_snapshot();
        assert_eq!(Fem3dInference::infer(&snapshot), Fem3dInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Fem3dInference::infer(&Fem3dSnapshot::default()), Fem3dInference::default());
    }

    #[test]
    fn bounds_matches_node_extent() {
        let snapshot = sample_snapshot();
        let inferred = Fem3dInference::infer(&snapshot);
        assert_eq!(inferred.bounds.node_count, 3);
        assert_eq!(inferred.bounds.bounding_box.min, [0.0, 0.0, 0.0]);
        assert_eq!(inferred.bounds.bounding_box.max, [4.0, 3.0, 2.0]);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
