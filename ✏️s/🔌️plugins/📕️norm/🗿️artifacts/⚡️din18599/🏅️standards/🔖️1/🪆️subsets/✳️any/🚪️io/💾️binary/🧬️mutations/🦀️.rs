//! ⚖️ Din18599 app — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::Din18599Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &Din18599Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<Din18599Mutation, protocol::ProtocolError> {
    Din18599Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
use crate::artifact_schema::mutations::Din18599Mutation;
use crate::artifact_schema::mutations::{change_attachment, change_automation_class, change_building_category, change_delta_u_wb, change_element_u, change_geg_qp_factor, change_heated_volume_m3, change_method, change_net_floor_area_m2, change_use_class, replace_elements, replace_zones, update_climate, update_cooling, specify_dhw_system, specify_heating_system, update_lighting, update_renewables, update_ventilation};
use crate::{
    Attachment, AutomationClass, BuildingCategory, CalculationMethod, CoolingSystem, DhwSystem, EnvelopeElement, HeatingSystem, LightingSystem, MonthlyClimate, Renewables, ThermalZone, UseClass, VentilationSystem,
};
use protocol::OpText;
use crate::standards::v1::subsets::any::io::text::mutations::{Din18599MutationDsl,din18599_mutation_to_dsl,din18599_mutation_from_dsl};

impl protocol::OpBinary for Din18599MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
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
}
