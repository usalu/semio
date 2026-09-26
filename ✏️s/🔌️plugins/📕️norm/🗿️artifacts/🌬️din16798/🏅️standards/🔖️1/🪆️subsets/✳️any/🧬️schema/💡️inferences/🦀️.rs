//! 💡️ Din16798 inference schema — outline + compliance evaluate.

use crate::Din16798Snapshot;
use crate::document::CheckReport;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

pub use super::outline::Din16798Outline;

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
    fn infer(snapshot: &Din16798Snapshot) -> Self {
        Self { outline: Din16798Outline::compute(snapshot) }
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

impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::Din16798Builder {
    type Snapshot = Din16798Snapshot;
    type Inference = Din16798Inference;
}
//#endregion 🔖️Inference

/// ✅️ NormFamily evaluate entry.
pub fn evaluate(document: &Din16798Snapshot) -> CheckReport {
    crate::artifact_schema::check_full_environment(document)
}

pub fn din16798_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.din16798.inference",
        inference: framework_schema::FacetLeaves {
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
