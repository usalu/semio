//! 🔧️ Din18599 artifact — OpText/OpBinary codecs for `Din18599Mutation`.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use semio_s_artifact_norm_din18599_mutations::Din18599Mutation;
use semio_s_artifact_norm_din18599_mut_building::{change_attachment, change_automation_class, change_building_category, change_delta_u_wb, change_geg_qp_factor, change_heated_volume_m3, change_method, change_net_floor_area_m2, change_use_class};
use semio_s_artifact_norm_din18599_mut_zoning::{change_element_u, replace_elements, replace_zones, update_climate};
use semio_s_artifact_norm_din18599_mut_systems::{update_cooling, specify_dhw_system, specify_heating_system, update_lighting, update_renewables, update_ventilation};
use semio_s_artifact_norm_din18599_model::{Attachment, AutomationClass, BuildingCategory, CalculationMethod, MonthlyClimate, UseClass};
use protocol::OpText;

//#region 🔖️OpText
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub(crate) enum Din18599MutationDsl {
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
    ReplaceZones {
        #[dsl(block)]
        new_zones: Vec<semio_s_artifact_norm_din18599_model::ThermalZone>,
    },
    ReplaceElements {
        #[dsl(block)]
        new_elements: Vec<semio_s_artifact_norm_din18599_model::EnvelopeElement>,
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

pub(crate) fn din18599_mutation_to_dsl(mutation: &Din18599Mutation) -> Din18599MutationDsl {
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
        Din18599Mutation::ReplaceZones(payload) => Din18599MutationDsl::ReplaceZones { new_zones: payload.new_zones.clone() },
        Din18599Mutation::ReplaceElements(payload) => Din18599MutationDsl::ReplaceElements { new_elements: payload.new_elements.clone() },
        Din18599Mutation::ChangeElementU(payload) => Din18599MutationDsl::ChangeElementU { element_id: payload.element_id.clone(), new_u_value_w_m2k: payload.new_u_value_w_m2k },
        Din18599Mutation::UpdateClimate(payload) => Din18599MutationDsl::UpdateClimate { new_climate: payload.new_climate.clone() },
    }
}

pub(crate) fn din18599_mutation_from_dsl(dsl: Din18599MutationDsl) -> Din18599Mutation {
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
        Din18599MutationDsl::ReplaceZones { new_zones } => Din18599Mutation::ReplaceZones(replace_zones::ReplaceZones { new_zones }),
        Din18599MutationDsl::ReplaceElements { new_elements } => Din18599Mutation::ReplaceElements(replace_elements::ReplaceElements { new_elements }),
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
use crate::text::mutations::*;
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
use semio_s_artifact_norm_din18599_mut_zoning::replace_zones;
use semio_s_artifact_norm_din18599_mut_zoning::replace_elements;
use semio_s_artifact_norm_din18599_mut_zoning::change_element_u;
use semio_s_artifact_norm_din18599_mut_zoning::update_climate;

pub fn decode_din18599_mutation_json(text: &str) -> Result<Din18599Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use crate::text::mutations::mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
