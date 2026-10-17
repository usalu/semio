//! 🧬️ En1994 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
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

use semio_s_artifact_norm_en1994_model::En1994Snapshot;

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("mutation.apply.missing-target", format!("{what} does not exist"))
}

#[path = "🏗️beam-rows/🦀️.rs"]
mod beam_rows;
pub use beam_rows::*;

#[path = "🏛️column-rows/🦀️.rs"]
mod column_rows;
pub use column_rows::*;

#[path = "🧱️slab-rows/🦀️.rs"]
mod slab_rows;
pub use slab_rows::*;

#[path = "🔁️apply-and-inverse/🦀️.rs"]
mod apply_and_inverse;

/// 🔺️ Keyed sparse diff of the En1994 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Diff {
    #[state(artifact)]
    pub annex: Option<semio_s_plugin_norm_compliance::AnnexChoice>,
    #[state(artifact)]
    pub structure_kind: Option<String>,
    #[state(artifact)]
    pub steel_f_y_pa: Option<f64>,
    #[state(artifact)]
    pub beams: Option<En1994BeamsRows>,
    #[state(artifact)]
    pub columns: Option<En1994ColumnsRows>,
    #[state(artifact)]
    pub slabs: Option<En1994SlabsRows>,
    #[state(artifact)]
    pub fire_rating: Option<String>,
    #[state(artifact)]
    pub insulation_thickness_m: Option<f64>,
    #[state(artifact)]
    pub fatigue_detail: Option<String>,
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: concat!(include_str!("🦀️.rs"), include_str!("🏗️beam-rows/🦀️.rs"), include_str!("🏛️column-rows/🦀️.rs"), include_str!("🧱️slab-rows/🦀️.rs"), include_str!("🔁️apply-and-inverse/🦀️.rs")), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves
