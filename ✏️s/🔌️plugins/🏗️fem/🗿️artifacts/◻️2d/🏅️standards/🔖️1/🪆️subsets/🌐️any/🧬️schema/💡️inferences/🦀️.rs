//! 💡️ Fem2d inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use super::bounds::compute_fem2d_bounds;
use crate::Fem2dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Inference
/// 💡️ Everything inferable from a fem2d snapshot. One field per named inference under
/// `💡️inferences/` (currently: `bounds`, backed by the `📦bounds/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.fem.fem2d.inference")]
pub struct Fem2dInference {
    #[derived]
    pub bounds: Fem2dBounds,
}

impl protocol::Inference<Fem2dSnapshot> for Fem2dInference {
    fn infer(snapshot: &Fem2dSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { bounds: compute_fem2d_bounds(snapshot) }
    
        })
    }
}

impl protocol::InferenceSpec<Fem2dSnapshot> for Fem2dInference {
    fn inference_schema_id() -> &'static str {
        "s.fem.fem2d.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.fem.fem2d.inference.bounds", reads: &["nodes", "elements"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.fem.fem2d.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `fem2d_artifact_schema_descriptor`'s registration.
pub fn fem2d_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.fem.fem2d.inference",
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
pub use super::bounds::Fem2dBounds;
//#endregion 🔁️Re-exports
