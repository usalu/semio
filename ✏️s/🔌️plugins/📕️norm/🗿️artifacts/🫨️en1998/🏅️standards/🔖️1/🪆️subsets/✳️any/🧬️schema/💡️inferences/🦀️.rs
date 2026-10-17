//! 💡️ En1998 inference schema + full seismic evaluate with remedies.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_en1998_schema::{na_de, part_1, part_2, part_3, part_4, part_5, part_6, AnnexParams};
use framework_schema::ArtifactSchema;

//#region 🔖️Inference
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1998.inference")]
pub struct En1998Inference {
    #[derived]
    pub outline: En1998Outline,
}

impl Default for En1998Inference {
    fn default() -> Self {
        Self { outline: En1998Outline::compute(&En1998Snapshot::default()) }
    }
}

impl protocol::Inference<En1998Snapshot> for En1998Inference {
    fn infer(snapshot: &En1998Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { outline: En1998Outline::compute(snapshot) }

        })
    }
}

impl protocol::InferenceSpec<En1998Snapshot> for En1998Inference {
    fn inference_schema_id() -> &'static str { "s.norm.en1998.inference" }
    fn schema_version() -> u32 { 1 }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.norm.en1998.inference.outline",
            reads: &["annex", "site", "buildings", "bridges", "assessments", "silos", "tanks", "foundations", "retainingWalls", "towers"],
        }]
    }
}
//#endregion 🔖️Inference

pub fn en1998_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.norm.en1998.inference",
        inference: semio_framework_schema_registry::FacetLeaves {
            rust: concat!(include_str!("🦀️.rs"), include_str!("🧰️seismic-helpers/🦀️.rs"), include_str!("🌍️full-seismic-check/🦀️.rs"), include_str!("⚖️part-evaluators/🦀️.rs")),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
    }
}

#[path = "🧰️seismic-helpers/🦀️.rs"]
mod seismic_helpers;
use seismic_helpers::*;

#[path = "🌍️full-seismic-check/🦀️.rs"]
mod full_seismic_check;
pub use full_seismic_check::*;

#[path = "⚖️part-evaluators/🦀️.rs"]
mod part_evaluators;
pub use part_evaluators::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️compliance-report/🦀️.rs"]
mod compliance_report_tests;

pub use crate::outline::En1998Outline;

//#region 🪢️Mounts
#[path = "🧾outline/🦀️.rs"]
pub mod outline;
//#endregion 🪢️Mounts
