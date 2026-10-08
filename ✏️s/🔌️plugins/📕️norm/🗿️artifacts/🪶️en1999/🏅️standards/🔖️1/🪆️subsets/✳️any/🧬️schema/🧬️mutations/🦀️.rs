//! 🧬️ EN 1999 mutations — hierarchical aluminium-structure vocabulary.

use crate::diff::En1999Diff;
use crate::En1999Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::{resolve_edit, EDIT_RULES};

use super::change_annex;
use super::change_materials;
use super::change_sections;
use super::change_members;
use super::change_connections;
use super::change_fire_scenarios;
use super::change_fatigue_details;
use super::change_cold_formed;
use super::change_shells;
use super::add_member;
use super::remove_member;
use super::change_member_n_ed;
use super::change_member_m_y_ed;
use super::change_member_buckling_length;
use super::change_material_designation;
use super::change_plate_thickness;
use super::change_weld_throat;
use super::change_bolt_count;

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = En1999Snapshot, diff = En1999Diff, schema = "norm.en1999")]
pub enum En1999Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeMaterials(change_materials::ChangeMaterials),
    ChangeSections(change_sections::ChangeSections),
    ChangeMembers(change_members::ChangeMembers),
    ChangeConnections(change_connections::ChangeConnections),
    ChangeFireScenarios(change_fire_scenarios::ChangeFireScenarios),
    ChangeFatigueDetails(change_fatigue_details::ChangeFatigueDetails),
    ChangeColdFormed(change_cold_formed::ChangeColdFormed),
    ChangeShells(change_shells::ChangeShells),
    AddMember(add_member::AddMember),
    RemoveMember(remove_member::RemoveMember),
    ChangeMemberNEd(change_member_n_ed::ChangeMemberNEd),
    ChangeMemberMYEd(change_member_m_y_ed::ChangeMemberMYEd),
    ChangeMemberBucklingLength(change_member_buckling_length::ChangeMemberBucklingLength),
    ChangeMaterialDesignation(change_material_designation::ChangeMaterialDesignation),
    ChangePlateThickness(change_plate_thickness::ChangePlateThickness),
    ChangeWeldThroat(change_weld_throat::ChangeWeldThroat),
    ChangeBoltCount(change_bolt_count::ChangeBoltCount),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-materials",
    "change-sections",
    "change-members",
    "change-connections",
    "change-fire-scenarios",
    "change-fatigue-details",
    "change-cold-formed",
    "change-shells",
    "add-member",
    "remove-member",
    "change-member-n-ed",
    "change-member-my-ed",
    "change-member-buckling-length",
    "change-material-designation",
    "change-plate-thickness",
    "change-weld-throat",
    "change-bolt-count",
];
//#endregion 🔖️Mutations


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;



#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;


//#region 🧫️Vectors
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧫️Vectors

#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod middle_row;
