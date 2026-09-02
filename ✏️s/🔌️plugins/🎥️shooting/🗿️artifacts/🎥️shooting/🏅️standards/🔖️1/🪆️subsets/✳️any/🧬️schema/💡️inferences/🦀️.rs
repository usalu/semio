//! 💡️ Shooting inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::artifacts::shooting::ShootingSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::topology::{compute_shooting_topology, ShootingTopology};

//#region 🔖️Inference
/// 💡️ Everything inferable from a shooting snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir) — the ONLY
/// cross-entity reference this snapshot carries is `ShootingShot.camera_id` into `savedCameras`,
/// so "topology" here is that reference graph recast as a trivial two-level DAG.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.shooting.shooting.inference")]
pub struct ShootingInference {
    #[derived]
    pub topology: ShootingTopology,
}

impl protocol::Inference<ShootingSnapshot> for ShootingInference {
    async fn infer(snapshot: &ShootingSnapshot) -> Self {
        Self { topology: compute_shooting_topology(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) so this stays correct regardless of what
/// `ShootingSnapshot::default()` happens to contain.
impl Default for ShootingInference {
    fn default() -> Self {
        <Self as protocol::Inference<ShootingSnapshot>>::infer(&ShootingSnapshot::default())
    }
}

impl protocol::InferenceSpec<ShootingSnapshot> for ShootingInference {
    async fn inference_schema_id() -> &'static str {
        "s.shooting.shooting.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.shooting.shooting.inference.topology", reads: &["shots", "saved_cameras"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::shooting::standards::v1::subsets::any::schema::ShootingBuilderFacets {
    type Snapshot = ShootingSnapshot;
    type Inference = ShootingInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.shooting.shooting.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `shooting_artifact_schema_descriptor`'s registration.
pub async fn shooting_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.shooting.shooting.inference",
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
    use crate::artifacts::shooting::{ShootingCamera, ShootingSavedCamera, ShootingShot};
    use protocol::Inference;

    //#region 🧸️Fixtures
    async fn sample_snapshot() -> ShootingSnapshot {
        ShootingSnapshot {
            saved_cameras: vec![ShootingSavedCamera { id: "cam-1".into(), label: "Front".into(), camera: ShootingCamera::default() }],
            shots: vec![ShootingShot { id: "shot-1".into(), label: "Shot 1".into(), width: 1024, height: 768, format: "png".into(), shape: "rectangle".into(), background: None, camera_id: Some("cam-1".into()) }],
            ..ShootingSnapshot::default()
        }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️InferenceLaws
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = sample_snapshot();
        assert_eq!(ShootingInference::infer(&snapshot), ShootingInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(ShootingInference::infer(&ShootingSnapshot::default()), ShootingInference::default());
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
