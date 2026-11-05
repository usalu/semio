//! 💡️ En1996 inference schema — masonry building outline + compliance evaluate.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Inference
/// 💡️ Everything inferable from an EN 1996 snapshot.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1996.inference")]
pub struct En1996Inference {
    #[derived]
    pub outline: En1996Outline,
}

impl protocol::Inference<En1996Snapshot> for En1996Inference {
    fn infer(snapshot: &En1996Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { outline: En1996Outline::compute(snapshot) }
    
        })
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
            reads: &["annex", "reliabilityClass", "masonryClass", "designSituation", "storeys", "walls"],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
pub fn en1996_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.norm.en1996.inference",
        inference: semio_framework_schema_registry::FacetLeaves {
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
pub fn evaluate(document: &En1996Snapshot) -> semio_s_plugin_norm_compliance::CheckReport {
    let mut report = semio_s_artifact_norm_en1996_schema::checks::masonry::evaluate_building(
        document.annex,
        document.masonry_class,
        document.design_situation,
        document.reliability_class,
        document.storeys,
        &document.walls,
    );
    report.extend(semio_s_artifact_norm_en1996_schema::checks::masonry::evaluate_design_cases(document.annex, &document.cases, semio_s_plugin_norm_compliance::incremental_evaluation::Scope::All).checks);
    report.extend(semio_s_artifact_norm_en1996_schema::checks::masonry::detailing_checks(document.annex, &document.walls, &document.chases, &document.movement_joints).checks);
    report
}
//#endregion 🔖️ComplianceReport

#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;

#[cfg(any(test, feature = "oracle-wiring"))]
#[path = "🧪️tests/🎯️oracle-wiring/🦀️.rs"]
pub mod oracle_wiring;

pub use crate::outline::En1996Outline;

//#region 🪢️Mounts
#[path = "🧾outline/🦀️.rs"]
pub mod outline;
//#endregion 🪢️Mounts

//#region 🛰️LiveEvaluation
semio_s_plugin_norm_compliance::live_report!(pub mod live(En1996Snapshot) = semio_s_artifact_norm_en1996_schema::checks::masonry::En1996Units);
//#endregion 🛰️LiveEvaluation
