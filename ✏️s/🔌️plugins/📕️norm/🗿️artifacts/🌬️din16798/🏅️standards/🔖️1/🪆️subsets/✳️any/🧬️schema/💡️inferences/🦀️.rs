//! 💡️ Din16798 inference schema — outline + compliance evaluate.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
use semio_s_plugin_norm_compliance::CheckReport;
use framework_schema::ArtifactSchema;

pub use crate::outline::Din16798Outline;

//#region 🔖️Inference
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din16798.inference")]
pub struct Din16798Inference {
    #[derived]
    pub outline: Din16798Outline,
}

impl protocol::Inference<Din16798Snapshot> for Din16798Inference {
    fn infer(snapshot: &Din16798Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { outline: Din16798Outline::compute(snapshot) }
    
        })
    }
}

impl protocol::InferenceSpec<Din16798Snapshot> for Din16798Inference {
    fn inference_schema_id() -> &'static str { "s.norm.din16798.inference" }
    fn schema_version() -> u32 { 1 }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.norm.din16798.inference.outline",
            reads: &[
                "annex", "thetaRmC", "outdoorCo2Ppm", "zones", "ventSystems",
                "envelopeN50HInv", "envelopeVolumeM3", "cellarAreaM2", "cellarVentilationM3H", "nightSetbackK",
            ],
        }]
    }
}
//#endregion 🔖️Inference

/// ✅️ NormFamily evaluate entry.
pub fn evaluate(document: &Din16798Snapshot) -> CheckReport {
    semio_s_artifact_norm_din16798_checks::check_full_environment(document)
}

pub fn din16798_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.norm.din16798.inference",
        inference: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;

//#region 🪢️Mounts
#[path = "🧾outline/🦀️.rs"]
pub mod outline;
//#endregion 🪢️Mounts
