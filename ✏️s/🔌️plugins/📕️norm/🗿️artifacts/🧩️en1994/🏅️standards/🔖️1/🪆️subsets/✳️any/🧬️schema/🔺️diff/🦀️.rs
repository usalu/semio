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

fn missing_target(what: impl std::fmt::Display) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("mutation.apply.missing-target", format!("{what} does not exist"))
}

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

/// 🩹️ Sparse scalar patch of one keyed row: the struct, its commit, coalescing, inverse and emptiness.
macro_rules! scalar_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { $($f:ident: $t:ty),+ $(,)? }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
        #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        pub struct $name { $(pub $f: Option<$t>),+ }

        impl protocol::list_delta::RowPatch<$row> for $name {
            fn commit_into(&self, row: &mut $row, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                $(if let Some(value) = &self.$f { row.$f = value.clone(); })+
                Ok(())
            }
            fn absorb(&mut self, later: Self) { $(if later.$f.is_some() { self.$f = later.$f; })+ }
            fn inverse(&self, row: &$row) -> Self { Self { $($f: self.$f.as_ref().map(|_| row.$f.clone())),+ } }
            fn is_empty(&self) -> bool { true $(&& self.$f.is_none())+ }
        }
    };
}

/// 🩹️ Sparse patch of one keyed row with direct leaves (`set`), leaves of nested structs every row owns (`within`) and owned row lanes (`rows`): the struct, its commit, coalescing, inverse and emptiness.
macro_rules! nested_patch {
    ($(#[$doc:meta])* $name:ident for $row:ty { set { $($f:ident: $t:ty),* $(,)? } within { $($w:ident => $obj:ident . $inner:ident : $wt:ty),* $(,)? } rows { $($lane:ident : $rows:ty = $label:literal),* $(,)? } }) => {
        $(#[$doc])*
        #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
        #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
        #[value(rename_all = "camelCase", default)]
        pub struct $name { $(pub $f: Option<$t>,)* $(pub $w: Option<$wt>,)* $(pub $lane: Option<$rows>,)* }

        impl protocol::list_delta::RowPatch<$row> for $name {
            #[allow(unused_variables)]
            fn commit_into(&self, row: &mut $row, capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
                $(if let Some(value) = &self.$f { row.$f = value.clone(); })*
                $(if let Some(value) = &self.$w { row.$obj.$inner = value.clone(); })*
                $(if let Some(rows) = &self.$lane { row.$lane = rows.commit_onto(&row.$lane, capability).map_err(|error| error.under([$label]))?; })*
                Ok(())
            }
            fn absorb(&mut self, later: Self) {
                $(if later.$f.is_some() { self.$f = later.$f; })*
                $(if later.$w.is_some() { self.$w = later.$w; })*
                $(if let Some(theirs) = later.$lane {
                    match self.$lane.as_mut() {
                        Some(mine) => mine.absorb(theirs),
                        None => self.$lane = Some(theirs),
                    }
                    self.$lane = self.$lane.take().filter(|rows| !rows.is_empty());
                })*
            }
            fn inverse(&self, row: &$row) -> Self {
                Self {
                    $($f: self.$f.as_ref().map(|_| row.$f.clone()),)*
                    $($w: self.$w.as_ref().map(|_| row.$obj.$inner.clone()),)*
                    $($lane: self.$lane.as_ref().map(|rows| rows.inverse(&row.$lane)).filter(|rows| !rows.is_empty()),)*
                }
            }
            fn is_empty(&self) -> bool { true $(&& self.$f.is_none())* $(&& self.$w.is_none())* $(&& self.$lane.as_ref().map_or(true, |rows| rows.is_empty()))* }
        }
    };
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
#[artifact_schema(id = "s.norm.en1994")]
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
    pub cases: Option<semio_s_plugin_norm_compliance::design_case::CaseRowDelta>,
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: concat!(include_str!("🦀️.rs"), include_str!("🏗️beam-rows/🦀️.rs"), include_str!("🏛️column-rows/🦀️.rs"), include_str!("🧱️slab-rows/🦀️.rs"), include_str!("📍️placement-rows/🦀️.rs"), include_str!("📍️stud-rows/🦀️.rs"), include_str!("🔁️apply-and-inverse/🦀️.rs")), typescript: semio_s_artifact_norm_en1994_diff_typescript::LEAF, graphql: semio_s_artifact_norm_en1994_diff_graphql::LEAF, json_schema: semio_s_artifact_norm_en1994_diff_json_schema::LEAF, proto: semio_s_artifact_norm_en1994_diff_proto::LEAF };
//#endregion 🍃️FacetLeaves

#[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
pub mod text;

#[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
pub mod binary;
