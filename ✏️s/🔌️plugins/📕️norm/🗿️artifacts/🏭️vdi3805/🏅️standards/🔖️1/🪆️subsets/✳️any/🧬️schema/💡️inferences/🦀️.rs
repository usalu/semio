//! 💡️ Vdi3805 inference schema — the fourth schema family alongside snapshot/diff/mutations
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

use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
use ::framework_schema::ArtifactSchema;

//#region 🔖️Inference
/// 💡️ Everything inferable from a vdi3805 snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir — this document's
/// own field/section structure, since a norm compliance record IS the document it describes).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.vdi3805.inference")]
pub struct Vdi3805Inference {
    #[derived]
    pub outline: Vdi3805Outline,
}

impl protocol::Inference<Vdi3805Snapshot> for Vdi3805Inference {
    fn infer(snapshot: &Vdi3805Snapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { outline: Vdi3805Outline::compute(snapshot) }

        })
    }
}

impl protocol::InferenceSpec<Vdi3805Snapshot> for Vdi3805Inference {
    fn inference_schema_id() -> &'static str {
        "s.norm.vdi3805.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.norm.vdi3805.inference.outline", reads: &["catalog", "edition_profile", "correction_as_of", "strict_mode", "index", "geometry", "curves"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.norm.vdi3805.inference`'s facet leaves into the OS-wide inference catalog — call once at
/// plugin init, alongside `vdi3805_artifact_schema_descriptor`'s registration.
pub fn vdi3805_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.norm.vdi3805.inference",
        inference: ::semio_framework_schema_registry::FacetLeaves {
            rust: concat!(include_str!("🦀️.rs"), include_str!("🧰️check-evidence/🦀️.rs"), include_str!("🧱️sheet-checks/🦀️.rs"), include_str!("🧱️sheet-checks/🚰️valve-radiator-and-pump-checks/🦀️.rs"), include_str!("🧱️sheet-checks/🔥️heat-generator-and-operative-sheet-checks/🦀️.rs"), include_str!("📚️document-checks/🦀️.rs"), include_str!("📚️document-checks/🗃️catalog-integrity/🦀️.rs"), include_str!("📚️document-checks/🗂️file-header-and-historical-checks/🦀️.rs"), include_str!("📏️parts/🦀️.rs"), include_str!("🧾outline/🦀️.rs")),
            typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto")
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️ComplianceReport
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_vdi3805_checks::{clause, subject_dataset, subject_product, validate_structure, valve_min_kvs_m3_h, ANNEX, RADIATOR_N_MAX, RADIATOR_N_MIN, VALVE_DN_SERIES};
use semio_s_artifact_norm_vdi3805_model::attributes_from_records;
use semio_s_artifact_norm_vdi3805_model::*;
use std::collections::{BTreeMap, BTreeSet};

#[path = "🧰️check-evidence/🦀️.rs"]
mod check_evidence;
use check_evidence::*;

#[path = "🧱️sheet-checks/🦀️.rs"]
mod sheet_checks;
use sheet_checks::*;

#[path = "📚️document-checks/🦀️.rs"]
mod document_checks;
pub use document_checks::*;

#[path = "📏️parts/🦀️.rs"]
mod parts;
pub use parts::*;

//#endregion 🔖️ComplianceReport

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::outline::Vdi3805Outline;
//#endregion 🔁️Re-exports

//#region 🪢️Mounts
#[path = "🧾outline/🦀️.rs"]
pub mod outline;
//#endregion 🪢️Mounts
