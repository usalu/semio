//! ⚖️ DIN 4108 app — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::Din4108Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &Din4108Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<Din4108Mutation, protocol::ProtocolError> {
    Din4108Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
use crate::artifact_schema::mutations::Din4108Mutation;
use crate::artifact_schema::mutations::{
    change_climate_zone,
    change_usage,
    change_t_int_c,
    change_rh_int,
    change_airtightness_n50,
    change_has_mechanical_ventilation,
    change_bb2_details_conform,
    insert_zone,
    remove_zone,
    change_zone_floor_area,
    change_zone_heaviness,
    change_zone_night_ventilation,
    insert_zone_window,
    remove_zone_window,
    change_zone_window_area,
    change_zone_window_g_value,
    change_zone_window_shading_fc,
    insert_element,
    remove_element,
    change_element_area,
    change_element_adjacent,
    change_element_kind,
    insert_layer,
    remove_layer,
    reorder_layers,
    change_layer_thickness,
    change_layer_lambda,
    change_layer_mu,
    change_layer_material_id,
    insert_thermal_bridge,
    remove_thermal_bridge,
    change_thermal_bridge_psi,
    change_thermal_bridge_length,
    change_element_orientation_deg,
    change_element_inclination_deg,
    change_element_delta_u_g,
    change_element_delta_u_f,
    change_element_delta_u_r,
    change_thermal_bridge_bb2_type,
    change_zone_window_orientation,
    change_zone_window_inclination_deg,
    change_layer_application_type,
    change_layer_compressive_class,
};
use protocol::OpText;
use crate::standards::v1::subsets::any::io::text::mutations::{Din4108MutationDsl,din4108_mutation_to_dsl,din4108_mutation_from_dsl};

impl protocol::OpBinary for Din4108MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}

impl protocol::OpBinary for Din4108Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        din4108_mutation_to_dsl(self).encode_op()
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(din4108_mutation_from_dsl(Din4108MutationDsl::decode_op(bytes)?))
    }
}
}
