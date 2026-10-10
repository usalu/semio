//! 💡️ Generation3d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use super::geometry::{infer_geometry, GeometryInput};
use super::topology::compute_generation3d_topology;
use ::semio_framework_schema::ArtifactSchema;
//#region 🔖️Inference
/// 💡️ Everything inferable from a generation3d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.procedural.generation3d.inference")]
pub struct Generation3dInference {
    #[derived]
    pub topology: Generation3dTopology,
    #[derived]
    pub geometry: Generation3dGeometryRecord,
}

impl<'a> protocol::Inference<GeometryInput<'a>> for Generation3dInference {
    fn infer(snapshot: &GeometryInput<'_>) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { topology: compute_generation3d_topology(snapshot.snapshot), geometry: Generation3dGeometryRecord::of(&infer_geometry(snapshot)) }
    
        })
    }
}

impl<'a> protocol::InferenceSpec<GeometryInput<'a>> for Generation3dInference {
    fn inference_schema_id() -> &'static str {
        "s.procedural.generation3d.inference"
    }
    fn schema_version() -> u32 {
        3
    }
    /// 🗺️ `topology` reads the widget ids and the synapse endpoints only: a layout, camera, schema or generation edit
    /// never reaches it (`protocol::DiffRegions` of `Generation3dDiff` names those regions apart).
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.procedural.generation3d.inference.topology", reads: &["hostSnapshot/widgets", "hostSnapshot/synapses"] },
            protocol::InferenceFieldSpec { id: "s.procedural.generation3d.inference.geometry", reads: &["hostSnapshot/widgets", "hostSnapshot/synapses"] },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.procedural.generation3d.inference`'s facet leaves into the OS-wide inference
/// catalog — call once at plugin init, alongside `generation3d_artifact_schema_descriptor`'s
/// registration.
pub fn generation3d_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.procedural.generation3d.inference",
        inference: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::geometry::{Generation3dFaultRecord, Generation3dGeometryRecord, Generation3dOutputRecord, Generation3dWidgetRecord};
pub use super::topology::Generation3dTopology;
//#endregion 🔁️Re-exports
