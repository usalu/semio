//! 🧬️ En1994 artifact — document mutation dispatch for the composite structure subject.

use crate::{En1994Diff, En1994Snapshot};

//#region 🔖️Leaves
#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::{resolve_edit, EDIT_RULES};

use super::change_annex;
use super::change_structure_kind;
use super::change_steel_fy_pa;
use super::change_fire_rating;
use super::change_insulation_thickness_m;
use super::change_fatigue_detail;
use super::insert_beam;
use super::remove_beam;
use super::change_beam_action_q_area_pa;
use super::change_beam_stud_spacing_m;
use super::change_beam_span_m;
use super::change_beam_slab_thickness_m;
use super::change_beam_stud_diameter_m;
use super::change_beam_stud_count;
use super::change_beam_stud_fu_pa;
use super::change_beam_transverse_as;
use super::change_beam_construction;
use super::insert_column;
use super::remove_column;
use super::change_column_action_force_n;
use super::change_column_kind;
use super::insert_slab;
use super::remove_slab;
use super::change_slab_action_q_area_pa;
use super::change_slab_thickness_m;
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
pub use tests::demo_mutation_cases;


//#region 🧫️Vectors
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧫️Vectors

#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod middle_row;
