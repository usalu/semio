//! 💡️ Iso16757 inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Inference
/// 💡️ Everything inferable from a iso16757 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.iso16757.inference")]
pub struct Iso16757Inference {
    #[derived]
    pub outline: Iso16757Outline,
}

impl protocol::Inference<Iso16757Snapshot> for Iso16757Inference {
    fn infer(snapshot: &Iso16757Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { outline: Iso16757Outline::compute(snapshot) }
    
        })
    }
}

impl protocol::InferenceSpec<Iso16757Snapshot> for Iso16757Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.iso16757.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.norm.iso16757.inference.outline",
            reads: &[
                "catalogue",
                "dictionary",
                "geometry",
                "selection",
                "part_number_rule",
                "part_number_inputs",
                "script_limits",
                "exchange_process",
            ],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.iso16757.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `iso16757_artifact_schema_descriptor`'s registration.
pub fn iso16757_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.norm.iso16757.inference",
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
/// 📋️ Full ISO 16757 compliance report — Parts 1/2/4/5 over the complete catalogue subject.

pub fn evaluate(document: &Iso16757Snapshot) -> semio_s_plugin_norm_compliance::CheckReport {
    semio_s_artifact_norm_iso16757_checks::evaluate_all(document)
}
//#endregion 🔖️ComplianceReport

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::outline::Iso16757Outline;
//#endregion 🔁️Re-exports

//#region 🪢️Mounts
#[path = "🧾outline/🦀️.rs"]
pub mod outline;
//#endregion 🪢️Mounts
