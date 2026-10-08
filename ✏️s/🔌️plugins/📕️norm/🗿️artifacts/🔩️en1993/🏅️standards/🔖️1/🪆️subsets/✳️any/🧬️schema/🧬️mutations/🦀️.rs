//! 🧬️ En1993 artifact — hierarchical subject mutation dispatch.

use crate::{En1993Diff, En1993Snapshot};

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::EDIT_RULES;
pub use crate::standards::v1::subsets::any::io::mutation_bridge::{apply_en1993_mutation, inverse_en1993_mutation};

//#region 🔖️Mutations
//#region 🔖️Leaves
use super::change_annex;
use super::update_bolt_inputs;
use super::update_bridge_inputs;
use super::update_cold_formed_inputs;
use super::update_crane_inputs;
use super::update_fatigue_inputs;
use super::update_fire_inputs;
use super::update_hss_inputs;
use super::update_member_properties;
use super::update_pile_inputs;
use super::update_plated_inputs;
use super::update_silo_shell_inputs;
use super::update_stainless_inputs;
use super::update_tension_component_inputs;
use super::update_through_thickness_inputs;
use super::update_tower_inputs;
use super::update_weld_inputs;
use super::insert_material;
use super::remove_material;
use super::insert_section;
use super::remove_section;
use super::insert_member;
use super::remove_member;
use super::insert_load_case;
use super::remove_load_case;
use super::insert_member_action;
use super::remove_member_action;
use super::insert_joint;
use super::remove_joint;
use super::insert_fatigue_detail;
use super::remove_fatigue_detail;
use super::insert_fire_exposure;
use super::remove_fire_exposure;
use super::insert_cold_formed_member;
use super::remove_cold_formed_member;
use super::insert_plated_panel;
use super::remove_plated_panel;
use super::insert_silo_shell;
use super::remove_silo_shell;
use super::insert_tension_component;
use super::remove_tension_component;
use super::insert_bridge_fatigue;
use super::remove_bridge_fatigue;
use super::insert_tower_leg;
use super::remove_tower_leg;
use super::insert_pile;
use super::remove_pile;
use super::insert_crane_runway;
use super::remove_crane_runway;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1993Snapshot, diff = En1993Diff, schema = "s.norm.en1993")]
pub enum En1993Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    UpdateMemberProperties(update_member_properties::UpdateMemberProperties),
    UpdateFireInputs(update_fire_inputs::UpdateFireInputs),
    UpdateColdFormedInputs(update_cold_formed_inputs::UpdateColdFormedInputs),
    UpdateStainlessInputs(update_stainless_inputs::UpdateStainlessInputs),
    UpdatePlatedInputs(update_plated_inputs::UpdatePlatedInputs),
    UpdateSiloShellInputs(update_silo_shell_inputs::UpdateSiloShellInputs),
    UpdateBoltInputs(update_bolt_inputs::UpdateBoltInputs),
    UpdateWeldInputs(update_weld_inputs::UpdateWeldInputs),
    UpdateFatigueInputs(update_fatigue_inputs::UpdateFatigueInputs),
    UpdateThroughThicknessInputs(update_through_thickness_inputs::UpdateThroughThicknessInputs),
    UpdateTensionComponentInputs(update_tension_component_inputs::UpdateTensionComponentInputs),
    UpdateHssInputs(update_hss_inputs::UpdateHssInputs),
    UpdateBridgeInputs(update_bridge_inputs::UpdateBridgeInputs),
    UpdateTowerInputs(update_tower_inputs::UpdateTowerInputs),
    UpdatePileInputs(update_pile_inputs::UpdatePileInputs),
    UpdateCraneInputs(update_crane_inputs::UpdateCraneInputs),
    InsertMaterial(insert_material::InsertMaterial),
    RemoveMaterial(remove_material::RemoveMaterial),
    InsertSection(insert_section::InsertSection),
    RemoveSection(remove_section::RemoveSection),
    InsertMember(insert_member::InsertMember),
    RemoveMember(remove_member::RemoveMember),
    InsertLoadCase(insert_load_case::InsertLoadCase),
    RemoveLoadCase(remove_load_case::RemoveLoadCase),
    InsertMemberAction(insert_member_action::InsertMemberAction),
    RemoveMemberAction(remove_member_action::RemoveMemberAction),
    InsertJoint(insert_joint::InsertJoint),
    RemoveJoint(remove_joint::RemoveJoint),
    InsertFatigueDetail(insert_fatigue_detail::InsertFatigueDetail),
    RemoveFatigueDetail(remove_fatigue_detail::RemoveFatigueDetail),
    InsertFireExposure(insert_fire_exposure::InsertFireExposure),
    RemoveFireExposure(remove_fire_exposure::RemoveFireExposure),
    InsertColdFormedMember(insert_cold_formed_member::InsertColdFormedMember),
    RemoveColdFormedMember(remove_cold_formed_member::RemoveColdFormedMember),
    InsertPlatedPanel(insert_plated_panel::InsertPlatedPanel),
    RemovePlatedPanel(remove_plated_panel::RemovePlatedPanel),
    InsertSiloShell(insert_silo_shell::InsertSiloShell),
    RemoveSiloShell(remove_silo_shell::RemoveSiloShell),
    InsertTensionComponent(insert_tension_component::InsertTensionComponent),
    RemoveTensionComponent(remove_tension_component::RemoveTensionComponent),
    InsertBridgeFatigue(insert_bridge_fatigue::InsertBridgeFatigue),
    RemoveBridgeFatigue(remove_bridge_fatigue::RemoveBridgeFatigue),
    InsertTowerLeg(insert_tower_leg::InsertTowerLeg),
    RemoveTowerLeg(remove_tower_leg::RemoveTowerLeg),
    InsertPile(insert_pile::InsertPile),
    RemovePile(remove_pile::RemovePile),
    InsertCraneRunway(insert_crane_runway::InsertCraneRunway),
    RemoveCraneRunway(remove_crane_runway::RemoveCraneRunway),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "update-member-properties",
    "update-fire-inputs",
    "update-cold-formed-inputs",
    "update-stainless-inputs",
    "update-plated-inputs",
    "update-silo-shell-inputs",
    "update-bolt-inputs",
    "update-weld-inputs",
    "update-fatigue-inputs",
    "update-through-thickness-inputs",
    "update-tension-component-inputs",
    "update-hss-inputs",
    "update-bridge-inputs",
    "update-tower-inputs",
    "update-pile-inputs",
    "update-crane-inputs",
    "insert-material",
    "remove-material",
    "insert-section",
    "remove-section",
    "insert-member",
    "remove-member",
    "insert-load-case",
    "remove-load-case",
    "insert-member-action",
    "remove-member-action",
    "insert-joint",
    "remove-joint",
    "insert-fatigue-detail",
    "remove-fatigue-detail",
    "insert-fire-exposure",
    "remove-fire-exposure",
    "insert-cold-formed-member",
    "remove-cold-formed-member",
    "insert-plated-panel",
    "remove-plated-panel",
    "insert-silo-shell",
    "remove-silo-shell",
    "insert-tension-component",
    "remove-tension-component",
    "insert-bridge-fatigue",
    "remove-bridge-fatigue",
    "insert-tower-leg",
    "remove-tower-leg",
    "insert-pile",
    "remove-pile",
    "insert-crane-runway",
    "remove-crane-runway",
];
//#endregion 🔖️Mutations


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog


//#region 🧫️Vectors
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧫️Vectors

#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod middle_row;
