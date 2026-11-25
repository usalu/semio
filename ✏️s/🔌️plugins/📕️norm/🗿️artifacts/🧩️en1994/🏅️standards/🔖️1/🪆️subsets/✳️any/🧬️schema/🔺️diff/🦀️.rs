//! 🧬️ En1994 keyed sparse diff — scalar setters, id/index/key keyed row diffs and per-field section patches; no whole-snapshot, whole-list or generic value patch.
//!
//! `apply` is the only snapshot writer and is reachable solely through `protocol::apply_diff`, which mints the `ApplyCapability`.
//! `absorb` coalesces same-key entries (patch∘patch, create∘delete, delete∘create) and `DiffAlgebra::inverse` returns the negative
//! diff, read row by row from the base.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_plugin_norm_compliance::{nested_patch, scalar_patch};

/// 🔑️ Key extractors of the foreign model rows (`id`), local markers for `list_delta!`.
macro_rules! row_key {
    ($($key:ident for $row:ident;)+) => {$(
        #[doc = concat!("🔑️ Key extractor of the foreign `", stringify!($row), "` rows (`id`).")]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $key;
        impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1994_model::$row> for $key {
            type Key = String;
            fn key_of(row: &semio_s_artifact_norm_en1994_model::$row) -> String { row.id.clone() }
        }
    )+};
}

row_key! {
    En1994BeamKey for CompositeBeam;
    En1994ColumnKey for CompositeColumn;
    En1994SlabKey for CompositeSlab;
    En1994PlacementKey for Placement;
    En1994StudKey for Stud;
    En1994BeamsActionKey for CharacteristicAction;
    En1994ColumnsActionKey for ColumnAction;
    En1994SlabsActionKey for SlabAction;
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

#[path = "📍️placement-rows/🦀️.rs"]
mod placement_rows;
pub use placement_rows::*;
#[path = "📍️stud-rows/🦀️.rs"]
mod stud_rows;
pub use stud_rows::*;

#[path = "🔁️apply-and-inverse/🦀️.rs"]
mod apply_and_inverse;

/// 🔺️ Keyed sparse diff of the En1994 artifact: scalar setters, keyed row diffs and per-field section patches.
#[derive(Clone, Debug, Default, PartialEq, framework_schema::ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm-eurocode-structural.en1994")]
pub struct En1994Diff {
    #[state(artifact)]
    pub annex: Option<semio_s_plugin_norm_compliance::AnnexChoice>,
    #[state(artifact)]
    pub structure_kind: Option<semio_s_artifact_norm_en1994_model::StructureKind>,
    #[state(artifact)]
    pub steel_f_y_pa: Option<f64>,
    #[state(artifact)]
    pub beams: Option<En1994BeamsRows>,
    #[state(artifact)]
    pub columns: Option<En1994ColumnsRows>,
    #[state(artifact)]
    pub slabs: Option<En1994SlabsRows>,
    #[state(artifact)]
    pub placements: Option<En1994PlacementsRows>,
    #[state(artifact)]
    pub studs: Option<En1994StudsRows>,
    #[state(artifact)]
    pub fire_rating: Option<semio_s_artifact_norm_en1994_model::FireRating>,
    #[state(artifact)]
    pub insulation_thickness_m: Option<f64>,
    #[state(artifact)]
    pub fatigue_detail: Option<semio_s_artifact_norm_en1994_model::FatigueDetail>,
    #[state(artifact)]
    pub reliability_class: Option<semio_s_plugin_norm_compliance::ReliabilityClass>,
    #[state(artifact)]
    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: concat!(include_str!("🦀️.rs"), include_str!("🏗️beam-rows/🦀️.rs"), include_str!("🏛️column-rows/🦀️.rs"), include_str!("🧱️slab-rows/🦀️.rs"), include_str!("📍️placement-rows/🦀️.rs"), include_str!("📍️stud-rows/🦀️.rs"), include_str!("🔁️apply-and-inverse/🦀️.rs")), typescript: semio_s_artifact_norm_en1994_diff_typescript::LEAF, graphql: semio_s_artifact_norm_en1994_diff_graphql::LEAF, json_schema: semio_s_artifact_norm_en1994_diff_json_schema::LEAF, proto: semio_s_artifact_norm_en1994_diff_proto::LEAF };
//#endregion 🍃️FacetLeaves

#[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
pub mod text;

#[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
pub mod binary;
