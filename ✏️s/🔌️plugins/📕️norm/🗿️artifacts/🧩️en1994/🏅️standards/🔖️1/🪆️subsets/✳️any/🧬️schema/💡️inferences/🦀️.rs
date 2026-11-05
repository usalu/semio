//! 💡️ En1994 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Inference
/// 💡️ Everything inferable from a en1994 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1994.inference")]
pub struct En1994Inference {
    #[derived]
    pub outline: En1994Outline,
}

impl protocol::Inference<En1994Snapshot> for En1994Inference {
    fn infer(snapshot: &En1994Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { outline: En1994Outline::compute(snapshot) }
    
        })
    }
}

impl protocol::InferenceSpec<En1994Snapshot> for En1994Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1994.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.en1994.inference.outline", reads: &["annex", "reliabilityClass", "structureKind", "steelFYPa", "beams", "columns", "slabs", "fireRating", "insulationThicknessM", "fatigueDetail", "beams.actions", "columns.actions", "slabs.actions"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.en1994.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `en1994_artifact_schema_descriptor`'s registration.
pub fn en1994_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.norm.en1994.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
#[path = "🧮️compliance/🦀️.rs"]
mod compliance;
pub use compliance::{evaluate, evaluate_scoped};

#[path = "🧮️units/🦀️.rs"]
mod units;
pub use units::*;

//#endregion 🔖️ComplianceReport

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::outline::En1994Outline;
//#endregion 🔁️Re-exports

//#region 🪢️Mounts
#[path = "🧾outline/🦀️.rs"]
pub mod outline;
//#endregion 🪢️Mounts

//#region 🛰️LiveEvaluation
semio_s_plugin_norm_compliance::live_report!(pub mod live(En1994Snapshot) = En1994Units);
//#endregion 🛰️LiveEvaluation
