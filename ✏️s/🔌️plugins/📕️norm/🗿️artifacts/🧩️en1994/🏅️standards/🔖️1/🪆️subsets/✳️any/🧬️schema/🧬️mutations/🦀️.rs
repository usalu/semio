//! 🧬️ En1994 artifact — document mutation dispatch for the composite structure subject.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1994_diff::En1994Diff;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

//#region 🔖️Leaves
#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::{resolve_edit, EDIT_RULES};

use semio_s_artifact_norm_en1994_mut_project::change_annex;
use semio_s_artifact_norm_en1994_mut_project::change_structure_kind;
use semio_s_artifact_norm_en1994_mut_project::change_steel_fy_pa;
use semio_s_artifact_norm_en1994_mut_project::change_fire_rating;
use semio_s_artifact_norm_en1994_mut_project::change_insulation_thickness_m;
use semio_s_artifact_norm_en1994_mut_project::change_fatigue_detail;
use semio_s_artifact_norm_en1994_mut_beams::insert_beam;
use semio_s_artifact_norm_en1994_mut_beams::remove_beam;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_action_q_area_pa;
use semio_s_artifact_norm_en1994_mut_studs::change_beam_stud_spacing_m;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_span_m;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_slab_thickness_m;
use semio_s_artifact_norm_en1994_mut_studs::change_beam_stud_diameter_m;
use semio_s_artifact_norm_en1994_mut_studs::change_beam_stud_count;
use semio_s_artifact_norm_en1994_mut_studs::change_beam_stud_fu_pa;
use semio_s_artifact_norm_en1994_mut_studs::change_beam_transverse_as;
use semio_s_artifact_norm_en1994_mut_beams::change_beam_construction;
use semio_s_artifact_norm_en1994_mut_columns_slabs::insert_column;
use semio_s_artifact_norm_en1994_mut_columns_slabs::remove_column;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_column_action_force_n;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_column_kind;
use semio_s_artifact_norm_en1994_mut_columns_slabs::insert_slab;
use semio_s_artifact_norm_en1994_mut_columns_slabs::remove_slab;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_slab_action_q_area_pa;
use semio_s_artifact_norm_en1994_mut_columns_slabs::change_slab_thickness_m;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1994Snapshot, diff = En1994Diff, schema = "s.norm.en1994")]
pub enum En1994Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeStructureKind(change_structure_kind::ChangeStructureKind),
    ChangeSteelFYPa(change_steel_fy_pa::ChangeSteelFYPa),
    ChangeFireRating(change_fire_rating::ChangeFireRating),
    ChangeInsulationThicknessM(change_insulation_thickness_m::ChangeInsulationThicknessM),
    ChangeFatigueDetail(change_fatigue_detail::ChangeFatigueDetail),
    InsertBeam(insert_beam::InsertBeam),
    RemoveBeam(remove_beam::RemoveBeam),
    ChangeBeamActionQAreaPa(change_beam_action_q_area_pa::ChangeBeamActionQAreaPa),
    ChangeBeamStudSpacingM(change_beam_stud_spacing_m::ChangeBeamStudSpacingM),
    ChangeBeamSpanM(change_beam_span_m::ChangeBeamSpanM),
    ChangeBeamSlabThicknessM(change_beam_slab_thickness_m::ChangeBeamSlabThicknessM),
    ChangeBeamStudDiameterM(change_beam_stud_diameter_m::ChangeBeamStudDiameterM),
    ChangeBeamStudCount(change_beam_stud_count::ChangeBeamStudCount),
    ChangeBeamStudFUPa(change_beam_stud_fu_pa::ChangeBeamStudFUPa),
    ChangeBeamTransverseAs(change_beam_transverse_as::ChangeBeamTransverseAs),
    ChangeBeamConstruction(change_beam_construction::ChangeBeamConstruction),
    InsertColumn(insert_column::InsertColumn),
    RemoveColumn(remove_column::RemoveColumn),
    ChangeColumnActionForceN(change_column_action_force_n::ChangeColumnActionForceN),
    ChangeColumnKind(change_column_kind::ChangeColumnKind),
    InsertSlab(insert_slab::InsertSlab),
    RemoveSlab(remove_slab::RemoveSlab),
    ChangeSlabActionQAreaPa(change_slab_action_q_area_pa::ChangeSlabActionQAreaPa),
    ChangeSlabThicknessM(change_slab_thickness_m::ChangeSlabThicknessM),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-structure-kind",
    "change-steel-fy-pa",
    "change-fire-rating",
    "change-insulation-thickness-m",
    "change-fatigue-detail",
    "insert-beam",
    "remove-beam",
    "change-beam-action-q-area-pa",
    "change-beam-stud-spacing-m",
    "change-beam-span-m",
    "change-beam-slab-thickness-m",
    "change-beam-stud-diameter-m",
    "change-beam-stud-count",
    "change-beam-stud-fu-pa",
    "change-beam-transverse-as",
    "change-beam-construction",
    "insert-column",
    "remove-column",
    "change-column-action-force-n",
    "change-column-kind",
    "insert-slab",
    "remove-slab",
    "change-slab-action-q-area-pa",
    "change-slab-thickness-m",
];
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️Tests

#[cfg(test)]
pub use crate::tests::demo_mutation_cases;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff, applied through the central applier.
pub mod mutation_bridge {
    use crate::En1994Mutation;
    use semio_s_artifact_norm_en1994_model::En1994Snapshot;

    /// 🎯️ Applies one mutation to `base` through production dispatch, returning the next snapshot and every raised diagnostic as `level:code`.
    pub fn apply_en1994_mutation(base: &En1994Snapshot, mutation: &En1994Mutation) -> Result<(En1994Snapshot, Vec<String>), String> {
        let raised = <En1994Mutation as protocol::Mutation<En1994Snapshot>>::diff(mutation, base);
        let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
        let applied = protocol::apply_diff(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
        Ok((applied, messages))
    }

    /// ↩️ The inverse steps production dispatch computes for `mutation` against `base`.
    pub fn inverse_en1994_mutation(mutation: &En1994Mutation, base: &En1994Snapshot) -> Result<Vec<En1994Mutation>, semio_framework_value::ValueError> {
        <En1994Mutation as protocol::Mutation<En1994Snapshot>>::inverse(mutation, base)
    }
}
pub use crate::mutation_bridge::*;
