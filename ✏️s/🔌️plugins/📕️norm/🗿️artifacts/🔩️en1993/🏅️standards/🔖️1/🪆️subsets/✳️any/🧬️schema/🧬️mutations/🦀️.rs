//! 🧬️ En1993 artifact — hierarchical subject mutation dispatch.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1993_diff::En1993Diff;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use crate::edit_rules::EDIT_RULES;

//#region 🔖️Mutations
//#region 🔖️Leaves
use semio_s_artifact_norm_en1993_mut_general::change_annex;
use semio_s_artifact_norm_en1993_mut_connections::update_bolt_inputs;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::update_bridge_inputs;
use semio_s_artifact_norm_en1993_mut_thin_walled::update_cold_formed_inputs;
use semio_s_artifact_norm_en1993_mut_cranes::update_crane_inputs;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::update_fatigue_inputs;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::update_fire_inputs;
use semio_s_artifact_norm_en1993_mut_connections::update_hss_inputs;
use semio_s_artifact_norm_en1993_mut_general::update_member_properties;
use semio_s_artifact_norm_en1993_mut_towers::update_pile_inputs;
use semio_s_artifact_norm_en1993_mut_thin_walled::update_plated_inputs;
use semio_s_artifact_norm_en1993_mut_thin_walled::update_silo_shell_inputs;
use semio_s_artifact_norm_en1993_mut_general::update_stainless_inputs;
use semio_s_artifact_norm_en1993_mut_cranes::update_tension_component_inputs;
use semio_s_artifact_norm_en1993_mut_general::update_through_thickness_inputs;
use semio_s_artifact_norm_en1993_mut_towers::update_tower_inputs;
use semio_s_artifact_norm_en1993_mut_connections::update_weld_inputs;
use semio_s_artifact_norm_en1993_mut_general::insert_material;
use semio_s_artifact_norm_en1993_mut_general::remove_material;
use semio_s_artifact_norm_en1993_mut_general::insert_section;
use semio_s_artifact_norm_en1993_mut_general::remove_section;
use semio_s_artifact_norm_en1993_mut_general::insert_member;
use semio_s_artifact_norm_en1993_mut_general::remove_member;
use semio_s_artifact_norm_en1993_mut_connections::insert_load_case;
use semio_s_artifact_norm_en1993_mut_connections::remove_load_case;
use semio_s_artifact_norm_en1993_mut_connections::insert_member_action;
use semio_s_artifact_norm_en1993_mut_connections::remove_member_action;
use semio_s_artifact_norm_en1993_mut_connections::insert_joint;
use semio_s_artifact_norm_en1993_mut_connections::remove_joint;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::insert_fatigue_detail;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::remove_fatigue_detail;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::insert_fire_exposure;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::remove_fire_exposure;
use semio_s_artifact_norm_en1993_mut_thin_walled::insert_cold_formed_member;
use semio_s_artifact_norm_en1993_mut_thin_walled::remove_cold_formed_member;
use semio_s_artifact_norm_en1993_mut_thin_walled::insert_plated_panel;
use semio_s_artifact_norm_en1993_mut_thin_walled::remove_plated_panel;
use semio_s_artifact_norm_en1993_mut_thin_walled::insert_silo_shell;
use semio_s_artifact_norm_en1993_mut_thin_walled::remove_silo_shell;
use semio_s_artifact_norm_en1993_mut_cranes::insert_tension_component;
use semio_s_artifact_norm_en1993_mut_cranes::remove_tension_component;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::insert_bridge_fatigue;
use semio_s_artifact_norm_en1993_mut_fire_fatigue::remove_bridge_fatigue;
use semio_s_artifact_norm_en1993_mut_towers::insert_tower_leg;
use semio_s_artifact_norm_en1993_mut_towers::remove_tower_leg;
use semio_s_artifact_norm_en1993_mut_towers::insert_pile;
use semio_s_artifact_norm_en1993_mut_towers::remove_pile;
use semio_s_artifact_norm_en1993_mut_cranes::insert_crane_runway;
use semio_s_artifact_norm_en1993_mut_cranes::remove_crane_runway;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `mutations` schema facet.
pub const MUTATION_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

/// 🌉️ The production dispatch the independent oracle hosts and the vector tests drive: a mutation's diff and inverse through the central applier.
pub mod mutation_bridge {
/// 🌉️ Applies one mutation to `base` through the central applier, returning the next snapshot and the raised `<Level>:<code>` messages.
pub fn apply_en1993_mutation(base: &semio_s_artifact_norm_en1993_model::En1993Snapshot, mutation: &crate::En1993Mutation) -> Result<(semio_s_artifact_norm_en1993_model::En1993Snapshot, Vec<String>), String> {
    let raised = <crate::En1993Mutation as protocol::Mutation<semio_s_artifact_norm_en1993_model::En1993Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = protocol::apply_diff(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

/// ↩️ The mutation rows that undo `mutation` applied to `base`, in store order.
pub fn inverse_en1993_mutation(mutation: &crate::En1993Mutation, base: &semio_s_artifact_norm_en1993_model::En1993Snapshot) -> Result<Vec<crate::En1993Mutation>, semio_framework_value::ValueError> {
    <crate::En1993Mutation as protocol::Mutation<semio_s_artifact_norm_en1993_model::En1993Snapshot>>::inverse(mutation, base)
}
}
pub use crate::mutation_bridge::*;
