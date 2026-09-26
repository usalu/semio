//! 🔧️ Din18599 artifact — OpText/OpBinary codecs for `Din18599Mutation`.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifact_schema::mutations::Din18599Mutation;
use crate::artifact_schema::mutations::{change_attachment, change_automation_class, change_building_category, change_delta_u_wb, change_element_u, change_geg_qp_factor, change_heated_volume_m3, change_method, change_net_floor_area_m2, change_use_class, replace_elements, replace_zones, update_climate, update_cooling, specify_dhw_system, specify_heating_system, update_lighting, update_renewables, update_ventilation};
use crate::{
    Attachment, AutomationClass, BuildingCategory, CalculationMethod, CoolingSystem, DhwSystem, EnvelopeElement, HeatingSystem, LightingSystem, MonthlyClimate, Renewables, ThermalZone, UseClass, VentilationSystem,
};
use protocol::OpText;

//#region 🔖️OpText
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum Din18599MutationDsl {
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
        new_heating: crate::HeatingSystem,
    },
    SpecifyDhwSystem {
        #[dsl(block)]
        new_dhw: crate::DhwSystem,
    },
    UpdateVentilation {
        #[dsl(block)]
        new_ventilation: crate::VentilationSystem,
    },
    UpdateCooling {
        #[dsl(block)]
        new_cooling: crate::CoolingSystem,
    },
    UpdateLighting {
        #[dsl(block)]
        new_lighting: crate::LightingSystem,
    },
    UpdateRenewables {
        #[dsl(block)]
        new_renewables: crate::Renewables,
    },
    ReplaceZones {
        #[dsl(block)]
        new_zones: Vec<crate::ThermalZone>,
    },
    ReplaceElements {
        #[dsl(block)]
        new_elements: Vec<crate::EnvelopeElement>,
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
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Din18599MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../💾️binary/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../💾️binary/📡️.protocol.semio"), bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn din18599_mutation_to_dsl(mutation: &Din18599Mutation) -> Din18599MutationDsl {
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

fn din18599_mutation_from_dsl(dsl: Din18599MutationDsl) -> Din18599Mutation {
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
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(din18599_mutation_from_dsl(Din18599MutationDsl::parse_op(line)?))
    }
    fn print_op(&self) -> String {
        din18599_mutation_to_dsl(self).print_op()
    }
}

impl protocol::OpBinary for Din18599Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        din18599_mutation_to_dsl(self).encode_op()
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(din18599_mutation_from_dsl(Din18599MutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
