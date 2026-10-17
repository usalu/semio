//! 🧬️ Iso16757 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff, read row by row from the base.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("mutation.apply.missing-target", format!("{what} does not exist"))
}

#[path = "🩹️list-row-patches/🦀️.rs"]
mod list_row_patches;
pub use list_row_patches::*;

#[path = "🗂️keyed-row-sections/🦀️.rs"]
mod keyed_row_sections;
pub use keyed_row_sections::*;

#[path = "🔁️apply-and-inverse/🦀️.rs"]
mod apply_and_inverse;
pub use apply_and_inverse::*;

/// 🔺️ Keyed sparse diff of the Iso16757 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Diff {
    #[state(artifact)]
    pub catalogue_name: Option<String>,
    #[state(artifact)]
    pub manufacturer_name: Option<String>,
    #[state(artifact)]
    pub product_groups: Option<Iso16757ProductGroupsRows>,
    #[state(artifact)]
    pub product_classes: Option<Iso16757ProductClassesRows>,
    #[state(artifact)]
    pub product_series: Option<Iso16757ProductSeriesRows>,
    #[state(artifact)]
    pub products: Option<Iso16757ProductsRows>,
    #[state(artifact)]
    pub product_indexes: Option<Iso16757ProductIndexesRows>,
    #[state(artifact)]
    pub property_definitions: Option<Iso16757PropertyDefinitionsRows>,
    #[state(artifact)]
    pub subjects: Option<Iso16757SubjectsRows>,
    #[state(artifact)]
    pub geometry_objects: Option<Iso16757GeometryObjectsRows>,
    #[state(artifact)]
    pub selection_class_id: Option<String>,
    #[state(artifact)]
    pub selection_series_id: Option<Iso16757SelectionSeriesIdValue>,
    #[state(artifact)]
    pub selection_constraints: Option<Iso16757SelectionConstraintsRows>,
    #[state(artifact)]
    pub part_number_rule: Option<semio_s_artifact_norm_iso16757_model::part_5::PartNumberRule>,
    #[state(artifact)]
    pub part_number_inputs: Option<Iso16757PartNumberInputsRows>,
    #[state(artifact)]
    pub script_limits: Option<Iso16757ScriptLimitsPatch>,
    #[state(artifact)]
    pub exchange_process: Option<semio_s_artifact_norm_iso16757_model::part_5::ExchangeProcess>,
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: concat!(include_str!("🦀️.rs"), include_str!("🩹️list-row-patches/🦀️.rs"), include_str!("🗂️keyed-row-sections/🦀️.rs"), include_str!("🔁️apply-and-inverse/🦀️.rs")), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves
