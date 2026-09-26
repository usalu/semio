//! 🧬️ Din18599 artifact — closed semantic mutation dispatch enum.

use crate::diff::Din18599Diff;
use crate::Din18599Snapshot;

//#region 🔖️Leaves
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
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
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

//#region 🔖️FromSnapshot
impl Din18599Mutation {
    /// 📤️ Diff `base` → `target` into the closed semantic mutation vocabulary.
    pub fn from_snapshot(base: &Din18599Snapshot, target: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        let mut mutations = Vec::new();
        fn q(a: f64, b: f64) -> bool {
            (a - b).abs() > f64::EPSILON
        }
        if base.building_category != target.building_category {
            mutations.push(Din18599Mutation::ChangeBuildingCategory(change_building_category::ChangeBuildingCategory { new_building_category: target.building_category }));
        }
        if base.attachment != target.attachment {
            mutations.push(Din18599Mutation::ChangeAttachment(change_attachment::ChangeAttachment { new_attachment: target.attachment }));
        }
        if base.use_class != target.use_class {
            mutations.push(Din18599Mutation::ChangeUseClass(change_use_class::ChangeUseClass { new_use_class: target.use_class }));
        }
        if base.method != target.method {
            mutations.push(Din18599Mutation::ChangeMethod(change_method::ChangeMethod { new_method: target.method }));
        }
        if q(base.net_floor_area_m2, target.net_floor_area_m2) {
            mutations.push(Din18599Mutation::ChangeNetFloorAreaM2(change_net_floor_area_m2::ChangeNetFloorAreaM2 { new_net_floor_area_m2: target.net_floor_area_m2 }));
        }
        if q(base.heated_volume_m3, target.heated_volume_m3) {
            mutations.push(Din18599Mutation::ChangeHeatedVolumeM3(change_heated_volume_m3::ChangeHeatedVolumeM3 { new_heated_volume_m3: target.heated_volume_m3 }));
        }
        if q(base.geg_qp_factor, target.geg_qp_factor) {
            mutations.push(Din18599Mutation::ChangeGegQpFactor(change_geg_qp_factor::ChangeGegQpFactor { new_geg_qp_factor: target.geg_qp_factor }));
        }
        if q(base.delta_u_wb_w_m2k, target.delta_u_wb_w_m2k) {
            mutations.push(Din18599Mutation::ChangeDeltaUWb(change_delta_u_wb::ChangeDeltaUWb { new_delta_u_wb_w_m2k: target.delta_u_wb_w_m2k }));
        }
        if base.automation_class != target.automation_class {
            mutations.push(Din18599Mutation::ChangeAutomationClass(change_automation_class::ChangeAutomationClass { new_automation_class: target.automation_class }));
        }
        if base.zones != target.zones {
            mutations.push(Din18599Mutation::ReplaceZones(replace_zones::ReplaceZones { new_zones: target.zones.clone() }));
        }
        if base.elements != target.elements {
            let mut only_u = base.elements.len() == target.elements.len();
            if only_u {
                for (b, tgt) in base.elements.iter().zip(target.elements.iter()) {
                    if b.id != tgt.id {
                        only_u = false;
                        break;
                    }
                    let b_rest = (b.label_en.as_str(), b.label_de.as_str(), b.kind, b.zone_id.as_str(), b.area_m2, b.orientation_deg, b.tilt_deg, b.g_value, b.fc, b.adjacency);
                    let t_rest = (tgt.label_en.as_str(), tgt.label_de.as_str(), tgt.kind, tgt.zone_id.as_str(), tgt.area_m2, tgt.orientation_deg, tgt.tilt_deg, tgt.g_value, tgt.fc, tgt.adjacency);
                    if b_rest != t_rest {
                        only_u = false;
                        break;
                    }
                }
            }
            if only_u {
                for (b, tgt) in base.elements.iter().zip(target.elements.iter()) {
                    if q(b.u_value_w_m2k, tgt.u_value_w_m2k) {
                        mutations.push(Din18599Mutation::ChangeElementU(change_element_u::ChangeElementU {
                            element_id: tgt.id.clone(),
                            new_u_value_w_m2k: tgt.u_value_w_m2k,
                        }));
                    }
                }
            } else {
                mutations.push(Din18599Mutation::ReplaceElements(replace_elements::ReplaceElements { new_elements: target.elements.clone() }));
            }
        }
        if base.heating != target.heating {
            mutations.push(Din18599Mutation::SpecifyHeatingSystem(specify_heating_system::SpecifyHeatingSystem { new_heating: target.heating.clone() }));
        }
        if base.dhw != target.dhw {
            mutations.push(Din18599Mutation::SpecifyDhwSystem(specify_dhw_system::SpecifyDhwSystem { new_dhw: target.dhw.clone() }));
        }
        if base.ventilation != target.ventilation {
            mutations.push(Din18599Mutation::UpdateVentilation(update_ventilation::UpdateVentilation { new_ventilation: target.ventilation.clone() }));
        }
        if base.cooling != target.cooling {
            mutations.push(Din18599Mutation::UpdateCooling(update_cooling::UpdateCooling { new_cooling: target.cooling.clone() }));
        }
        if base.lighting != target.lighting {
            mutations.push(Din18599Mutation::UpdateLighting(update_lighting::UpdateLighting { new_lighting: target.lighting.clone() }));
        }
        if base.renewables != target.renewables {
            mutations.push(Din18599Mutation::UpdateRenewables(update_renewables::UpdateRenewables { new_renewables: target.renewables.clone() }));
        }
        let base_climate = crate::din18599_climate(base);
        let target_climate = crate::din18599_climate(target);
        if base_climate != target_climate {
            mutations.push(Din18599Mutation::UpdateClimate(update_climate::UpdateClimate { new_climate: target_climate }));
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

//#region 🌉️ExternalCodecBridge
pub fn decode_din18599_mutation_json(text: &str) -> Result<Din18599Mutation, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

pub fn apply_din18599_mutation(base: &Din18599Snapshot, mutation: &Din18599Mutation) -> Result<(Din18599Snapshot, Vec<String>), String> {
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(mutation, base);
    let messages = raised.messages().iter().map(|message| format!("{:?}:{}", message.level, message.code.0)).collect();
    let applied = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(raised.diff(), base).map_err(|error| format!("{error:?}"))?;
    Ok((applied, messages))
}

pub fn inverse_din18599_mutation(mutation: &Din18599Mutation, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

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
