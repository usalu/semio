//! 💡️ En1996 inference schema — masonry building outline + compliance evaluate.

use crate::En1996Snapshot;
use framework_schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;

//#region 🔖️Inference
/// 💡️ Everything inferable from an EN 1996 snapshot.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1996.inference")]
pub struct En1996Inference {
    #[derived]
    pub outline: En1996Outline,
}

impl protocol::Inference<En1996Snapshot> for En1996Inference {
    fn infer(snapshot: &En1996Snapshot) -> Self {
        Self { outline: En1996Outline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<En1996Snapshot> for En1996Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.en1996.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.norm.en1996.inference.outline",
            reads: &["annex", "masonryClass", "designSituation", "storeys", "walls"],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::standards::v1::subsets::any::schema::En1996Builder {
    type Snapshot = En1996Snapshot;
    type Inference = En1996Inference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
pub fn en1996_artifact_inference_descriptor() -> framework_schema::ArtifactInferenceDescriptor {
    framework_schema::ArtifactInferenceDescriptor {
        id: "s.norm.en1996.inference",
        inference: framework_schema::FacetLeaves {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🔖️ComplianceReport
/// 📋️ `En1996Snapshot -> CheckReport` — full masonry-building evaluation.
pub fn evaluate(document: &crate::En1996Snapshot) -> crate::document::CheckReport {
    crate::artifact_schema::evaluate_building(
        document.annex,
        document.masonry_class,
        document.design_situation,
        document.storeys,
        &document.walls,
    )
}
//#endregion 🔖️ComplianceReport

#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;

pub use super::outline::En1996Outline;
