//! 🔧️ Din18599 artifact — OpText/OpBinary codecs for `Din18599Mutation`.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use semio_s_artifact_norm_din18599_mutations::Din18599Mutation;
use semio_s_artifact_norm_din18599_mut_building::{change_attachment, change_automation_class, change_building_category, change_delta_u_wb, change_geg_qp_factor, change_heated_volume_m3, change_method, change_net_floor_area_m2, change_use_class};
use semio_s_artifact_norm_din18599_mut_zoning::{change_element_u, update_climate};
use semio_s_artifact_norm_din18599_mut_zone_rows::{insert_zone, remove_zone, change_zone_label_en, change_zone_label_de, change_zone_usage_profile, change_zone_occupants};
use semio_s_artifact_norm_din18599_mut_zone_fields::{change_zone_area_m2, change_zone_volume_m3, change_zone_theta_i_heat_c, change_zone_theta_i_cool_c, change_zone_internal_gains_w_m2, change_zone_lighting_power_w_m2, change_zone_thermal_mass};
use semio_s_artifact_norm_din18599_mut_element_rows::{insert_element, remove_element, change_element_label_en, change_element_label_de, change_element_kind, change_element_zone_id};
use semio_s_artifact_norm_din18599_mut_element_fields::{change_element_area_m2, change_element_orientation_deg, change_element_tilt_deg, change_element_g_value, change_element_fc, change_element_adjacency};
use semio_s_artifact_norm_din18599_mut_systems::{update_cooling, specify_dhw_system, specify_heating_system, update_lighting, update_renewables, update_ventilation};
use semio_s_artifact_norm_din18599_model::{Attachment, AutomationClass, BuildingCategory, CalculationMethod, MonthlyClimate, UseClass};
use protocol::OpText;

//#region 🔖️OpText
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum Din18599MutationDsl {
    ChangeBuildingCategory {
        new_building_category: BuildingCategory,
    },
    ChangeAttachment {
        new_attachment: Attachment,
    },
    ChangeUseClass {
        new_use_class: UseClass,
    },
    ChangeMethod {
        new_method: CalculationMethod,
    },
    ChangeNetFloorAreaM2 {
        new_net_floor_area_m2: f64,
    },
    ChangeHeatedVolumeM3 {
        new_heated_volume_m3: f64,
    },
    ChangeGegQpFactor {
        new_geg_qp_factor: f64,
    },
    ChangeDeltaUWb {
        new_delta_u_wb_w_m2k: f64,
    },
    ChangeAutomationClass {
        new_automation_class: AutomationClass,
    },
    SpecifyHeatingSystem {
        #[dsl(block)]
        new_heating: semio_s_artifact_norm_din18599_model::HeatingSystem,
    },
    SpecifyDhwSystem {
        #[dsl(block)]
        new_dhw: semio_s_artifact_norm_din18599_model::DhwSystem,
    },
    UpdateVentilation {
        #[dsl(block)]
        new_ventilation: semio_s_artifact_norm_din18599_model::VentilationSystem,
    },
    UpdateCooling {
        #[dsl(block)]
        new_cooling: semio_s_artifact_norm_din18599_model::CoolingSystem,
    },
    UpdateLighting {
        #[dsl(block)]
        new_lighting: semio_s_artifact_norm_din18599_model::LightingSystem,
    },
    UpdateRenewables {
        #[dsl(block)]
        new_renewables: semio_s_artifact_norm_din18599_model::Renewables,
    },
    InsertZone {
        index: usize,
        #[dsl(block)]
        zone: semio_s_artifact_norm_din18599_model::ThermalZone,
    },
    RemoveZone {
        id: String,
    },
    ChangeZoneLabelEn {
        zone_id: String,
        new_label_en: String,
    },
    ChangeZoneLabelDe {
        zone_id: String,
        new_label_de: String,
    },
    ChangeZoneUsageProfile {
        zone_id: String,
        new_usage_profile: semio_s_artifact_norm_din18599_model::UsageProfile,
    },
    ChangeZoneOccupants {
        zone_id: String,
        new_occupants: u32,
    },
    ChangeZoneAreaM2 {
        zone_id: String,
        new_area_m2: f64,
    },
    ChangeZoneVolumeM3 {
        zone_id: String,
        new_volume_m3: f64,
    },
    ChangeZoneThetaIHeatC {
        zone_id: String,
        new_theta_i_heat_c: f64,
    },
    ChangeZoneThetaICoolC {
        zone_id: String,
        new_theta_i_cool_c: f64,
    },
    ChangeZoneInternalGainsWM2 {
        zone_id: String,
        new_internal_gains_w_m2: f64,
    },
    ChangeZoneLightingPowerWM2 {
        zone_id: String,
        new_lighting_power_w_m2: f64,
    },
    ChangeZoneThermalMass {
        zone_id: String,
        new_thermal_mass_class: semio_s_artifact_norm_din18599_model::ThermalMassClass,
        new_c_wirk_wh_m2k: f64,
    },
    InsertElement {
        index: usize,
        #[dsl(block)]
        element: semio_s_artifact_norm_din18599_model::EnvelopeElement,
    },
    RemoveElement {
        id: String,
    },
    ChangeElementLabelEn {
        element_id: String,
        new_label_en: String,
    },
    ChangeElementLabelDe {
        element_id: String,
        new_label_de: String,
    },
    ChangeElementKind {
        element_id: String,
        new_kind: semio_s_artifact_norm_din18599_model::ElementKind,
    },
    ChangeElementZoneId {
        element_id: String,
        new_zone_id: String,
    },
    ChangeElementAreaM2 {
        element_id: String,
        new_area_m2: f64,
    },
    ChangeElementOrientationDeg {
        element_id: String,
        new_orientation_deg: f64,
    },
    ChangeElementTiltDeg {
        element_id: String,
        new_tilt_deg: f64,
    },
    ChangeElementGValue {
        element_id: String,
        new_g_value: f64,
    },
    ChangeElementFc {
        element_id: String,
        new_fc: f64,
    },
    ChangeElementAdjacency {
        element_id: String,
        new_adjacency: semio_s_artifact_norm_din18599_model::Adjacency,
    },
    ChangeElementU {
        element_id: String,
        new_u_value_w_m2k: f64,
    },
    UpdateClimate {
        #[dsl(block)]
        new_climate: MonthlyClimate,
    },
}

//#region 🔖️HandcraftedOpCodecs
impl OpText for Din18599MutationDsl {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

//#endregion 🔖️HandcraftedOpCodecs

pub fn din18599_mutation_to_dsl(mutation: &Din18599Mutation) -> Din18599MutationDsl {
    match mutation {
        Din18599Mutation::ChangeBuildingCategory(payload) => Din18599MutationDsl::ChangeBuildingCategory { new_building_category: payload.new_building_category },
        Din18599Mutation::ChangeAttachment(payload) => Din18599MutationDsl::ChangeAttachment { new_attachment: payload.new_attachment },
        Din18599Mutation::ChangeUseClass(payload) => Din18599MutationDsl::ChangeUseClass { new_use_class: payload.new_use_class },
        Din18599Mutation::ChangeMethod(payload) => Din18599MutationDsl::ChangeMethod { new_method: payload.new_method },
        Din18599Mutation::ChangeNetFloorAreaM2(payload) => Din18599MutationDsl::ChangeNetFloorAreaM2 { new_net_floor_area_m2: payload.new_net_floor_area_m2 },
        Din18599Mutation::ChangeHeatedVolumeM3(payload) => Din18599MutationDsl::ChangeHeatedVolumeM3 { new_heated_volume_m3: payload.new_heated_volume_m3 },
        Din18599Mutation::ChangeGegQpFactor(payload) => Din18599MutationDsl::ChangeGegQpFactor { new_geg_qp_factor: payload.new_geg_qp_factor },
        Din18599Mutation::ChangeDeltaUWb(payload) => Din18599MutationDsl::ChangeDeltaUWb { new_delta_u_wb_w_m2k: payload.new_delta_u_wb_w_m2k },
        Din18599Mutation::ChangeAutomationClass(payload) => Din18599MutationDsl::ChangeAutomationClass { new_automation_class: payload.new_automation_class },
        Din18599Mutation::SpecifyHeatingSystem(payload) => Din18599MutationDsl::SpecifyHeatingSystem { new_heating: payload.new_heating.clone() },
        Din18599Mutation::SpecifyDhwSystem(payload) => Din18599MutationDsl::SpecifyDhwSystem { new_dhw: payload.new_dhw.clone() },
        Din18599Mutation::UpdateVentilation(payload) => Din18599MutationDsl::UpdateVentilation { new_ventilation: payload.new_ventilation.clone() },
        Din18599Mutation::UpdateCooling(payload) => Din18599MutationDsl::UpdateCooling { new_cooling: payload.new_cooling.clone() },
        Din18599Mutation::UpdateLighting(payload) => Din18599MutationDsl::UpdateLighting { new_lighting: payload.new_lighting.clone() },
        Din18599Mutation::UpdateRenewables(payload) => Din18599MutationDsl::UpdateRenewables { new_renewables: payload.new_renewables.clone() },
        Din18599Mutation::InsertZone(payload) => Din18599MutationDsl::InsertZone { index: payload.index, zone: payload.zone.clone() },
        Din18599Mutation::RemoveZone(payload) => Din18599MutationDsl::RemoveZone { id: payload.id.clone() },
        Din18599Mutation::ChangeZoneLabelEn(payload) => Din18599MutationDsl::ChangeZoneLabelEn { zone_id: payload.zone_id.clone(), new_label_en: payload.new_label_en.clone() },
        Din18599Mutation::ChangeZoneLabelDe(payload) => Din18599MutationDsl::ChangeZoneLabelDe { zone_id: payload.zone_id.clone(), new_label_de: payload.new_label_de.clone() },
        Din18599Mutation::ChangeZoneUsageProfile(payload) => Din18599MutationDsl::ChangeZoneUsageProfile { zone_id: payload.zone_id.clone(), new_usage_profile: payload.new_usage_profile },
        Din18599Mutation::ChangeZoneOccupants(payload) => Din18599MutationDsl::ChangeZoneOccupants { zone_id: payload.zone_id.clone(), new_occupants: payload.new_occupants },
        Din18599Mutation::ChangeZoneAreaM2(payload) => Din18599MutationDsl::ChangeZoneAreaM2 { zone_id: payload.zone_id.clone(), new_area_m2: payload.new_area_m2 },
        Din18599Mutation::ChangeZoneVolumeM3(payload) => Din18599MutationDsl::ChangeZoneVolumeM3 { zone_id: payload.zone_id.clone(), new_volume_m3: payload.new_volume_m3 },
        Din18599Mutation::ChangeZoneThetaIHeatC(payload) => Din18599MutationDsl::ChangeZoneThetaIHeatC { zone_id: payload.zone_id.clone(), new_theta_i_heat_c: payload.new_theta_i_heat_c },
        Din18599Mutation::ChangeZoneThetaICoolC(payload) => Din18599MutationDsl::ChangeZoneThetaICoolC { zone_id: payload.zone_id.clone(), new_theta_i_cool_c: payload.new_theta_i_cool_c },
        Din18599Mutation::ChangeZoneInternalGainsWM2(payload) => Din18599MutationDsl::ChangeZoneInternalGainsWM2 { zone_id: payload.zone_id.clone(), new_internal_gains_w_m2: payload.new_internal_gains_w_m2 },
        Din18599Mutation::ChangeZoneThermalMass(payload) => Din18599MutationDsl::ChangeZoneThermalMass { zone_id: payload.zone_id.clone(), new_thermal_mass_class: payload.new_thermal_mass_class, new_c_wirk_wh_m2k: payload.new_c_wirk_wh_m2k },
        Din18599Mutation::ChangeZoneLightingPowerWM2(payload) => Din18599MutationDsl::ChangeZoneLightingPowerWM2 { zone_id: payload.zone_id.clone(), new_lighting_power_w_m2: payload.new_lighting_power_w_m2 },
        Din18599Mutation::InsertElement(payload) => Din18599MutationDsl::InsertElement { index: payload.index, element: payload.element.clone() },
        Din18599Mutation::RemoveElement(payload) => Din18599MutationDsl::RemoveElement { id: payload.id.clone() },
        Din18599Mutation::ChangeElementLabelEn(payload) => Din18599MutationDsl::ChangeElementLabelEn { element_id: payload.element_id.clone(), new_label_en: payload.new_label_en.clone() },
        Din18599Mutation::ChangeElementLabelDe(payload) => Din18599MutationDsl::ChangeElementLabelDe { element_id: payload.element_id.clone(), new_label_de: payload.new_label_de.clone() },
        Din18599Mutation::ChangeElementKind(payload) => Din18599MutationDsl::ChangeElementKind { element_id: payload.element_id.clone(), new_kind: payload.new_kind },
        Din18599Mutation::ChangeElementZoneId(payload) => Din18599MutationDsl::ChangeElementZoneId { element_id: payload.element_id.clone(), new_zone_id: payload.new_zone_id.clone() },
        Din18599Mutation::ChangeElementAreaM2(payload) => Din18599MutationDsl::ChangeElementAreaM2 { element_id: payload.element_id.clone(), new_area_m2: payload.new_area_m2 },
        Din18599Mutation::ChangeElementOrientationDeg(payload) => Din18599MutationDsl::ChangeElementOrientationDeg { element_id: payload.element_id.clone(), new_orientation_deg: payload.new_orientation_deg },
        Din18599Mutation::ChangeElementTiltDeg(payload) => Din18599MutationDsl::ChangeElementTiltDeg { element_id: payload.element_id.clone(), new_tilt_deg: payload.new_tilt_deg },
        Din18599Mutation::ChangeElementGValue(payload) => Din18599MutationDsl::ChangeElementGValue { element_id: payload.element_id.clone(), new_g_value: payload.new_g_value },
        Din18599Mutation::ChangeElementFc(payload) => Din18599MutationDsl::ChangeElementFc { element_id: payload.element_id.clone(), new_fc: payload.new_fc },
        Din18599Mutation::ChangeElementAdjacency(payload) => Din18599MutationDsl::ChangeElementAdjacency { element_id: payload.element_id.clone(), new_adjacency: payload.new_adjacency },
        Din18599Mutation::ChangeElementU(payload) => Din18599MutationDsl::ChangeElementU { element_id: payload.element_id.clone(), new_u_value_w_m2k: payload.new_u_value_w_m2k },
        Din18599Mutation::UpdateClimate(payload) => Din18599MutationDsl::UpdateClimate { new_climate: payload.new_climate.clone() },
    }
}

pub fn din18599_mutation_from_dsl(dsl: Din18599MutationDsl) -> Din18599Mutation {
    match dsl {
        Din18599MutationDsl::ChangeBuildingCategory { new_building_category } => Din18599Mutation::ChangeBuildingCategory(change_building_category::ChangeBuildingCategory { new_building_category }),
        Din18599MutationDsl::ChangeAttachment { new_attachment } => Din18599Mutation::ChangeAttachment(change_attachment::ChangeAttachment { new_attachment }),
        Din18599MutationDsl::ChangeUseClass { new_use_class } => Din18599Mutation::ChangeUseClass(change_use_class::ChangeUseClass { new_use_class }),
        Din18599MutationDsl::ChangeMethod { new_method } => Din18599Mutation::ChangeMethod(change_method::ChangeMethod { new_method }),
        Din18599MutationDsl::ChangeNetFloorAreaM2 { new_net_floor_area_m2 } => Din18599Mutation::ChangeNetFloorAreaM2(change_net_floor_area_m2::ChangeNetFloorAreaM2 { new_net_floor_area_m2 }),
        Din18599MutationDsl::ChangeHeatedVolumeM3 { new_heated_volume_m3 } => Din18599Mutation::ChangeHeatedVolumeM3(change_heated_volume_m3::ChangeHeatedVolumeM3 { new_heated_volume_m3 }),
        Din18599MutationDsl::ChangeGegQpFactor { new_geg_qp_factor } => Din18599Mutation::ChangeGegQpFactor(change_geg_qp_factor::ChangeGegQpFactor { new_geg_qp_factor }),
        Din18599MutationDsl::ChangeDeltaUWb { new_delta_u_wb_w_m2k } => Din18599Mutation::ChangeDeltaUWb(change_delta_u_wb::ChangeDeltaUWb { new_delta_u_wb_w_m2k }),
        Din18599MutationDsl::ChangeAutomationClass { new_automation_class } => Din18599Mutation::ChangeAutomationClass(change_automation_class::ChangeAutomationClass { new_automation_class }),
        Din18599MutationDsl::SpecifyHeatingSystem { new_heating } => Din18599Mutation::SpecifyHeatingSystem(specify_heating_system::SpecifyHeatingSystem { new_heating }),
        Din18599MutationDsl::SpecifyDhwSystem { new_dhw } => Din18599Mutation::SpecifyDhwSystem(specify_dhw_system::SpecifyDhwSystem { new_dhw }),
        Din18599MutationDsl::UpdateVentilation { new_ventilation } => Din18599Mutation::UpdateVentilation(update_ventilation::UpdateVentilation { new_ventilation }),
        Din18599MutationDsl::UpdateCooling { new_cooling } => Din18599Mutation::UpdateCooling(update_cooling::UpdateCooling { new_cooling }),
        Din18599MutationDsl::UpdateLighting { new_lighting } => Din18599Mutation::UpdateLighting(update_lighting::UpdateLighting { new_lighting }),
        Din18599MutationDsl::UpdateRenewables { new_renewables } => Din18599Mutation::UpdateRenewables(update_renewables::UpdateRenewables { new_renewables }),
        Din18599MutationDsl::InsertZone { index, zone } => Din18599Mutation::InsertZone(insert_zone::InsertZone { index, zone }),
        Din18599MutationDsl::RemoveZone { id } => Din18599Mutation::RemoveZone(remove_zone::RemoveZone { id }),
        Din18599MutationDsl::ChangeZoneLabelEn { zone_id, new_label_en } => Din18599Mutation::ChangeZoneLabelEn(change_zone_label_en::ChangeZoneLabelEn { zone_id, new_label_en }),
        Din18599MutationDsl::ChangeZoneLabelDe { zone_id, new_label_de } => Din18599Mutation::ChangeZoneLabelDe(change_zone_label_de::ChangeZoneLabelDe { zone_id, new_label_de }),
        Din18599MutationDsl::ChangeZoneUsageProfile { zone_id, new_usage_profile } => Din18599Mutation::ChangeZoneUsageProfile(change_zone_usage_profile::ChangeZoneUsageProfile { zone_id, new_usage_profile }),
        Din18599MutationDsl::ChangeZoneOccupants { zone_id, new_occupants } => Din18599Mutation::ChangeZoneOccupants(change_zone_occupants::ChangeZoneOccupants { zone_id, new_occupants }),
        Din18599MutationDsl::ChangeZoneAreaM2 { zone_id, new_area_m2 } => Din18599Mutation::ChangeZoneAreaM2(change_zone_area_m2::ChangeZoneAreaM2 { zone_id, new_area_m2 }),
        Din18599MutationDsl::ChangeZoneVolumeM3 { zone_id, new_volume_m3 } => Din18599Mutation::ChangeZoneVolumeM3(change_zone_volume_m3::ChangeZoneVolumeM3 { zone_id, new_volume_m3 }),
        Din18599MutationDsl::ChangeZoneThetaIHeatC { zone_id, new_theta_i_heat_c } => Din18599Mutation::ChangeZoneThetaIHeatC(change_zone_theta_i_heat_c::ChangeZoneThetaIHeatC { zone_id, new_theta_i_heat_c }),
        Din18599MutationDsl::ChangeZoneThetaICoolC { zone_id, new_theta_i_cool_c } => Din18599Mutation::ChangeZoneThetaICoolC(change_zone_theta_i_cool_c::ChangeZoneThetaICoolC { zone_id, new_theta_i_cool_c }),
        Din18599MutationDsl::ChangeZoneInternalGainsWM2 { zone_id, new_internal_gains_w_m2 } => Din18599Mutation::ChangeZoneInternalGainsWM2(change_zone_internal_gains_w_m2::ChangeZoneInternalGainsWM2 { zone_id, new_internal_gains_w_m2 }),
        Din18599MutationDsl::ChangeZoneThermalMass { zone_id, new_thermal_mass_class, new_c_wirk_wh_m2k } => Din18599Mutation::ChangeZoneThermalMass(change_zone_thermal_mass::ChangeZoneThermalMass { zone_id, new_thermal_mass_class, new_c_wirk_wh_m2k }),
        Din18599MutationDsl::ChangeZoneLightingPowerWM2 { zone_id, new_lighting_power_w_m2 } => Din18599Mutation::ChangeZoneLightingPowerWM2(change_zone_lighting_power_w_m2::ChangeZoneLightingPowerWM2 { zone_id, new_lighting_power_w_m2 }),
        Din18599MutationDsl::InsertElement { index, element } => Din18599Mutation::InsertElement(insert_element::InsertElement { index, element }),
        Din18599MutationDsl::RemoveElement { id } => Din18599Mutation::RemoveElement(remove_element::RemoveElement { id }),
        Din18599MutationDsl::ChangeElementLabelEn { element_id, new_label_en } => Din18599Mutation::ChangeElementLabelEn(change_element_label_en::ChangeElementLabelEn { element_id, new_label_en }),
        Din18599MutationDsl::ChangeElementLabelDe { element_id, new_label_de } => Din18599Mutation::ChangeElementLabelDe(change_element_label_de::ChangeElementLabelDe { element_id, new_label_de }),
        Din18599MutationDsl::ChangeElementKind { element_id, new_kind } => Din18599Mutation::ChangeElementKind(change_element_kind::ChangeElementKind { element_id, new_kind }),
        Din18599MutationDsl::ChangeElementZoneId { element_id, new_zone_id } => Din18599Mutation::ChangeElementZoneId(change_element_zone_id::ChangeElementZoneId { element_id, new_zone_id }),
        Din18599MutationDsl::ChangeElementAreaM2 { element_id, new_area_m2 } => Din18599Mutation::ChangeElementAreaM2(change_element_area_m2::ChangeElementAreaM2 { element_id, new_area_m2 }),
        Din18599MutationDsl::ChangeElementOrientationDeg { element_id, new_orientation_deg } => Din18599Mutation::ChangeElementOrientationDeg(change_element_orientation_deg::ChangeElementOrientationDeg { element_id, new_orientation_deg }),
        Din18599MutationDsl::ChangeElementTiltDeg { element_id, new_tilt_deg } => Din18599Mutation::ChangeElementTiltDeg(change_element_tilt_deg::ChangeElementTiltDeg { element_id, new_tilt_deg }),
        Din18599MutationDsl::ChangeElementGValue { element_id, new_g_value } => Din18599Mutation::ChangeElementGValue(change_element_g_value::ChangeElementGValue { element_id, new_g_value }),
        Din18599MutationDsl::ChangeElementFc { element_id, new_fc } => Din18599Mutation::ChangeElementFc(change_element_fc::ChangeElementFc { element_id, new_fc }),
        Din18599MutationDsl::ChangeElementAdjacency { element_id, new_adjacency } => Din18599Mutation::ChangeElementAdjacency(change_element_adjacency::ChangeElementAdjacency { element_id, new_adjacency }),
        Din18599MutationDsl::ChangeElementU { element_id, new_u_value_w_m2k } => Din18599Mutation::ChangeElementU(change_element_u::ChangeElementU { element_id, new_u_value_w_m2k }),
        Din18599MutationDsl::UpdateClimate { new_climate } => Din18599Mutation::UpdateClimate(update_climate::UpdateClimate { new_climate }),
    }
}

impl OpText for Din18599Mutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Ok(din18599_mutation_from_dsl(Din18599MutationDsl::parse_op(line)?))
    }
    fn print_op(&self) -> String {
        din18599_mutation_to_dsl(self).print_op()
    }
}

//#endregion 🔖️OpText

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod mutations_codec {
use crate::mutations::*;
use semio_s_artifact_norm_din18599_mutations::*;
use semio_s_artifact_norm_din18599_diff::Din18599Diff;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_artifact_norm_din18599_mut_building::change_building_category;
use semio_s_artifact_norm_din18599_mut_building::change_attachment;
use semio_s_artifact_norm_din18599_mut_building::change_use_class;
use semio_s_artifact_norm_din18599_mut_building::change_method;
use semio_s_artifact_norm_din18599_mut_building::change_net_floor_area_m2;
use semio_s_artifact_norm_din18599_mut_building::change_heated_volume_m3;
use semio_s_artifact_norm_din18599_mut_building::change_geg_qp_factor;
use semio_s_artifact_norm_din18599_mut_building::change_delta_u_wb;
use semio_s_artifact_norm_din18599_mut_building::change_automation_class;
use semio_s_artifact_norm_din18599_mut_systems::specify_heating_system;
use semio_s_artifact_norm_din18599_mut_systems::specify_dhw_system;
use semio_s_artifact_norm_din18599_mut_systems::update_ventilation;
use semio_s_artifact_norm_din18599_mut_systems::update_cooling;
use semio_s_artifact_norm_din18599_mut_systems::update_lighting;
use semio_s_artifact_norm_din18599_mut_systems::update_renewables;
use semio_s_artifact_norm_din18599_mut_zoning::change_element_u;
use semio_s_artifact_norm_din18599_mut_zoning::update_climate;

pub fn decode_din18599_mutation_json(text: &str) -> Result<Din18599Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use crate::mutations::mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
