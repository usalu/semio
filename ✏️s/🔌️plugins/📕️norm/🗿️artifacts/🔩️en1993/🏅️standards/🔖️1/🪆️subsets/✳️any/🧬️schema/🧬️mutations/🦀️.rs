//! 🧬️ En1993 artifact — hierarchical subject mutation dispatch.

use crate::{En1993Diff, En1993Snapshot};

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

//#region 🔖️FromSnapshot
impl En1993Mutation {
    /// 📤️ Emits semantic mutations that carry `base` to `target` (list replace + annex).
    pub fn from_snapshot(base: &En1993Snapshot, target: &En1993Snapshot) -> Vec<En1993Mutation> {
        let mut mutations = Vec::new();
        if base.annex != target.annex {
            mutations.push(En1993Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: target.annex }));
        }
        if base.materials != target.materials {
            for index in (0..base.materials.len()).rev() {
                mutations.push(En1993Mutation::RemoveMaterial(remove_material::RemoveMaterial { index }));
            }
            for (index, item) in target.materials.iter().enumerate() {
                mutations.push(En1993Mutation::InsertMaterial(insert_material::InsertMaterial { index, material: item.clone() }));
            }
        }
        if base.sections != target.sections {
            for index in (0..base.sections.len()).rev() {
                mutations.push(En1993Mutation::RemoveSection(remove_section::RemoveSection { index }));
            }
            for (index, item) in target.sections.iter().enumerate() {
                mutations.push(En1993Mutation::InsertSection(insert_section::InsertSection { index, section: item.clone() }));
            }
        }
        if base.members != target.members {
            for index in (0..base.members.len()).rev() {
                mutations.push(En1993Mutation::RemoveMember(remove_member::RemoveMember { index }));
            }
            for (index, item) in target.members.iter().enumerate() {
                mutations.push(En1993Mutation::InsertMember(insert_member::InsertMember { index, member: item.clone() }));
            }
        }
        if base.load_cases != target.load_cases {
            for index in (0..base.load_cases.len()).rev() {
                mutations.push(En1993Mutation::RemoveLoadCase(remove_load_case::RemoveLoadCase { index  }));
            }
            for (index, item) in target.load_cases.iter().enumerate() {
                mutations.push(En1993Mutation::InsertLoadCase(insert_load_case::InsertLoadCase { index, load_case: item.clone()  }));
            }
        }
        if base.member_actions != target.member_actions {
            for index in (0..base.member_actions.len()).rev() {
                mutations.push(En1993Mutation::RemoveMemberAction(remove_member_action::RemoveMemberAction { index }));
            }
            for (index, item) in target.member_actions.iter().enumerate() {
                mutations.push(En1993Mutation::InsertMemberAction(insert_member_action::InsertMemberAction { index, member_action: item.clone() }));
            }
        }
        if base.joints != target.joints {
            for index in (0..base.joints.len()).rev() {
                mutations.push(En1993Mutation::RemoveJoint(remove_joint::RemoveJoint { index }));
            }
            for (index, item) in target.joints.iter().enumerate() {
                mutations.push(En1993Mutation::InsertJoint(insert_joint::InsertJoint { index, joint: item.clone() }));
            }
        }
        if base.fatigue_details != target.fatigue_details {
            for index in (0..base.fatigue_details.len()).rev() {
                mutations.push(En1993Mutation::RemoveFatigueDetail(remove_fatigue_detail::RemoveFatigueDetail { index }));
            }
            for (index, item) in target.fatigue_details.iter().enumerate() {
                mutations.push(En1993Mutation::InsertFatigueDetail(insert_fatigue_detail::InsertFatigueDetail { index, fatigue_detail: item.clone() }));
            }
        }
        if base.fire_exposures != target.fire_exposures {
            for index in (0..base.fire_exposures.len()).rev() {
                mutations.push(En1993Mutation::RemoveFireExposure(remove_fire_exposure::RemoveFireExposure { index }));
            }
            for (index, item) in target.fire_exposures.iter().enumerate() {
                mutations.push(En1993Mutation::InsertFireExposure(insert_fire_exposure::InsertFireExposure { index, fire_exposure: item.clone() }));
            }
        }
        if base.cold_formed_members != target.cold_formed_members {
            for index in (0..base.cold_formed_members.len()).rev() {
                mutations.push(En1993Mutation::RemoveColdFormedMember(remove_cold_formed_member::RemoveColdFormedMember { index }));
            }
            for (index, item) in target.cold_formed_members.iter().enumerate() {
                mutations.push(En1993Mutation::InsertColdFormedMember(insert_cold_formed_member::InsertColdFormedMember { index, cold_formed_member: item.clone() }));
            }
        }
        if base.plated_panels != target.plated_panels {
            for index in (0..base.plated_panels.len()).rev() {
                mutations.push(En1993Mutation::RemovePlatedPanel(remove_plated_panel::RemovePlatedPanel { index }));
            }
            for (index, item) in target.plated_panels.iter().enumerate() {
                mutations.push(En1993Mutation::InsertPlatedPanel(insert_plated_panel::InsertPlatedPanel { index, plated_panel: item.clone() }));
            }
        }
        if base.silo_shells != target.silo_shells {
            for index in (0..base.silo_shells.len()).rev() {
                mutations.push(En1993Mutation::RemoveSiloShell(remove_silo_shell::RemoveSiloShell { index }));
            }
            for (index, item) in target.silo_shells.iter().enumerate() {
                mutations.push(En1993Mutation::InsertSiloShell(insert_silo_shell::InsertSiloShell { index, silo_shell: item.clone() }));
            }
        }
        if base.tension_components != target.tension_components {
            for index in (0..base.tension_components.len()).rev() {
                mutations.push(En1993Mutation::RemoveTensionComponent(remove_tension_component::RemoveTensionComponent { index }));
            }
            for (index, item) in target.tension_components.iter().enumerate() {
                mutations.push(En1993Mutation::InsertTensionComponent(insert_tension_component::InsertTensionComponent { index, tension_component: item.clone() }));
            }
        }
        if base.bridge_fatigue != target.bridge_fatigue {
            for index in (0..base.bridge_fatigue.len()).rev() {
                mutations.push(En1993Mutation::RemoveBridgeFatigue(remove_bridge_fatigue::RemoveBridgeFatigue { index }));
            }
            for (index, item) in target.bridge_fatigue.iter().enumerate() {
                mutations.push(En1993Mutation::InsertBridgeFatigue(insert_bridge_fatigue::InsertBridgeFatigue { index, bridge_fatigue_item: item.clone() }));
            }
        }
        if base.tower_legs != target.tower_legs {
            for index in (0..base.tower_legs.len()).rev() {
                mutations.push(En1993Mutation::RemoveTowerLeg(remove_tower_leg::RemoveTowerLeg { index }));
            }
            for (index, item) in target.tower_legs.iter().enumerate() {
                mutations.push(En1993Mutation::InsertTowerLeg(insert_tower_leg::InsertTowerLeg { index, tower_leg: item.clone() }));
            }
        }
        if base.piles != target.piles {
            for index in (0..base.piles.len()).rev() {
                mutations.push(En1993Mutation::RemovePile(remove_pile::RemovePile { index }));
            }
            for (index, item) in target.piles.iter().enumerate() {
                mutations.push(En1993Mutation::InsertPile(insert_pile::InsertPile { index, pile: item.clone() }));
            }
        }
        if base.crane_runways != target.crane_runways {
            for index in (0..base.crane_runways.len()).rev() {
                mutations.push(En1993Mutation::RemoveCraneRunway(remove_crane_runway::RemoveCraneRunway { index }));
            }
            for (index, item) in target.crane_runways.iter().enumerate() {
                mutations.push(En1993Mutation::InsertCraneRunway(insert_crane_runway::InsertCraneRunway { index, crane_runway: item.clone() }));
            }
        }
        mutations
    }
}
//#endregion 🔖️FromSnapshot

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests

//#region 🌉️ExternalCodecBridge
pub fn decode_en1993_mutation_json(text: &str) -> Result<En1993Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}
pub fn apply_en1993_mutation(base: &En1993Snapshot, mutation: &En1993Mutation) -> Result<(En1993Snapshot, Vec<String>), String> {
    let raised = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}
pub fn inverse_en1993_mutation(mutation: &En1993Mutation, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    <En1993Mutation as protocol::Mutation<En1993Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️KindsCatalog
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️KindsCatalog
