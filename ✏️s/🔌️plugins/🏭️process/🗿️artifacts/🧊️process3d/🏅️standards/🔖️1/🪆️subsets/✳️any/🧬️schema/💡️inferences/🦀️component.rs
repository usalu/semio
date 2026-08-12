//! 💡️ Process3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).
//!
//! `stock`/`steps` are a single workpiece solid plus an ordered process timeline, not a graph of
//! positioned objects — the honest whole-snapshot derivation is the stock's world-space bounding
//! box plus the step count, computed straight from `SolidSpec`/`Pose` (no kernel/tessellation
//! needed). Whole-snapshot scalars, so this uses the plain `protocol::Inference<P>` shape (no
//! `InferredField`/caching machinery — see `📦bounds/🦀️component.rs` for the derivation).

use crate::artifacts::process3d::Process3dSnapshot;
use protocol::Inference;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::bounds::{stock_bounding_box, BoundingBox};

//#region 🔖️Inference
/// 💡️ Everything inferable from a process3d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `stockBounds`/`stepCount`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.process.process3d.inference")]
pub struct Process3dInference {
    #[state(inferred)]
    pub stock_bounds: BoundingBox,
    #[state(inferred)]
    pub step_count: u64,
}

impl protocol::Inference<Process3dSnapshot> for Process3dInference {
    fn infer(snapshot: &Process3dSnapshot) -> Self {
        Self { stock_bounds: stock_bounding_box(&snapshot.stock), step_count: snapshot.steps.len() as u64 }
    }
}

/// 🌉️ Hand impl (not derived): a naive `#[derive(Default)]` would give `stock_bounds` an
/// all-zero box, which disagrees with `infer(&Process3dSnapshot::default())` (the default 1x1x1
/// `Stock` box has a real, non-zero bound) and would break `inference_default_law`. Defining
/// default as "infer the default snapshot" makes the two definitionally equal.
impl Default for Process3dInference {
    fn default() -> Self {
        Self::infer(&Process3dSnapshot::default())
    }
}

impl protocol::InferenceSpec<Process3dSnapshot> for Process3dInference {
    fn inference_schema_id() -> &'static str {
        "s.process.process3d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.process.process3d.inference.bounds.stockBounds", reads: &["stock"] },
            protocol::InferenceFieldSpec { id: "s.process.process3d.inference.bounds.stepCount", reads: &["steps"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::process3d::standards::v1::subsets::any::schema::Process3dBuilder {
    type Snapshot = Process3dSnapshot;
    type Inference = Process3dInference;

    /// 🎯️ Whole-snapshot scalars — nothing here is per-entity, so the cache/session are unused
    /// (same "plain `Inference`" shape the family doc calls out as correct for `dimensions`/
    /// `outline`/`bounds`-style facets).
    fn infer_cached(snapshot: &Self::Snapshot, cache: &mut store::InferenceCache, session: &mut store::InferenceSession) -> Self::Inference {
        let _ = (cache, session);
        <Process3dInference as protocol::Inference<Process3dSnapshot>>::infer(snapshot)
    }
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.process.process3d.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `process3d_artifact_schema_descriptor`'s registration.
pub fn process3d_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.process.process3d.inference",
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

    //#region 🧪️InferenceLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = Process3dSnapshot::default();
        assert_eq!(Process3dInference::infer(&snapshot), Process3dInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(Process3dInference::infer(&Process3dSnapshot::default()), Process3dInference::default());
    }

    #[test]
    fn step_count_matches_steps_len() {
        let mut snapshot = Process3dSnapshot::default();
        snapshot.steps.push(crate::artifacts::process3d::ProcessStep {
            id: "s1".into(),
            label: "Cut".into(),
            enabled: true,
            origin: None,
            measure: crate::artifacts::process3d::ProcessMeasure::Drill { radius: 0.1, depth: 0.2, pose: crate::artifacts::process3d::Pose::default() },
        });
        assert_eq!(Process3dInference::infer(&snapshot).step_count, 1);
    }
    //#endregion 🧪️InferenceLaws
}
//#endregion 🧪️Tests
