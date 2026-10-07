//! ⚖️ EN 1990 basis of structural design — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::En1990Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &En1990Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<En1990Mutation, protocol::ProtocolError> {
    En1990Mutation::decode_op(bytes)
}

mod native_codec {
use super::*;
use crate::artifact_schema::mutations::En1990Mutation;
use crate::artifact_schema::mutations::{
    change_accidentals, change_annex, change_beta_computed, change_consequence_class, change_design_working_life_category, change_design_working_life_years, change_effects,
    change_inspection_level, change_members, change_bridge_sls, change_permanents, change_project_id, change_altitude_m, change_reference_period_years, change_reliability_class, change_seismics, change_supervision_level,
    change_variables, insert_accidental, insert_effect, insert_member, insert_permanent, insert_seismic, insert_variable, remove_accidental, remove_effect, remove_member, remove_permanent,
    remove_seismic, remove_variable,
};

impl protocol::OpBinary for En1990Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        semio_s_artifact_norm_contract::payload_op_binary::encode::<crate::En1990Snapshot, _>(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        semio_s_artifact_norm_contract::payload_op_binary::decode::<crate::En1990Snapshot, _>(include_str!("📡️.protocol.semio"), bytes)
    }
}
}
