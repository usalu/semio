//! 🧬️ Din18599 artifact — closed semantic mutation dispatch enum.

use crate::diff::Din18599Diff;
use crate::Din18599Snapshot;

//#region 🔖️Leaves
#[path = "🧭️edit-rules/🦀️.rs"]
mod edit_rules;
pub use edit_rules::{resolve_edit, EDIT_RULES};

use super::change_building_category;
use super::change_attachment;
use super::change_use_class;
use super::change_method;
use super::change_net_floor_area_m2;
use super::change_heated_volume_m3;
use super::change_geg_qp_factor;
use super::change_delta_u_wb;
use super::change_automation_class;
use super::specify_heating_system;
use super::specify_dhw_system;
use super::update_ventilation;
use super::update_cooling;
use super::update_lighting;
use super::update_renewables;
use super::replace_zones;
use super::replace_elements;
use super::change_element_u;
use super::update_climate;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the din18599 building subject.
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "mutation", rename_all = "camelCase"))]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Din18599Snapshot, diff = Din18599Diff, schema = "norm.din18599")]
pub enum Din18599Mutation {
    ChangeBuildingCategory(change_building_category::ChangeBuildingCategory),
    ChangeAttachment(change_attachment::ChangeAttachment),
    ChangeUseClass(change_use_class::ChangeUseClass),
    ChangeMethod(change_method::ChangeMethod),
    ChangeNetFloorAreaM2(change_net_floor_area_m2::ChangeNetFloorAreaM2),
    ChangeHeatedVolumeM3(change_heated_volume_m3::ChangeHeatedVolumeM3),
    ChangeGegQpFactor(change_geg_qp_factor::ChangeGegQpFactor),
    ChangeDeltaUWb(change_delta_u_wb::ChangeDeltaUWb),
    ChangeAutomationClass(change_automation_class::ChangeAutomationClass),
    SpecifyHeatingSystem(specify_heating_system::SpecifyHeatingSystem),
    SpecifyDhwSystem(specify_dhw_system::SpecifyDhwSystem),
    UpdateVentilation(update_ventilation::UpdateVentilation),
    UpdateCooling(update_cooling::UpdateCooling),
    UpdateLighting(update_lighting::UpdateLighting),
    UpdateRenewables(update_renewables::UpdateRenewables),
    ReplaceZones(replace_zones::ReplaceZones),
    ReplaceElements(replace_elements::ReplaceElements),
    ChangeElementU(change_element_u::ChangeElementU),
    UpdateClimate(update_climate::UpdateClimate),
}

pub const KINDS: &[&str] = &[
    "change-building-category",
    "change-attachment",
    "change-use-class",
    "change-method",
    "change-net-floor-area-m2",
    "change-heated-volume-m3",
    "change-geg-qp-factor",
    "change-delta-u-wb",
    "change-automation-class",
    "specify-heating-system",
    "specify-dhw-system",
    "update-ventilation",
    "update-cooling",
    "update-lighting",
    "update-renewables",
    "replace-zones",
    "replace-elements",
    "change-element-u",
    "update-climate",
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

//#region 🧪️FixtureCorpus
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture;
//#endregion 🧪️FixtureCorpus

#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod middle_row;
