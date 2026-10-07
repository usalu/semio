//! ⚖️ En1999 app — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::En1999Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &En1999Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<En1999Mutation, protocol::ProtocolError> {
    En1999Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
use crate::document::AnnexChoice;
use crate::artifact_schema::mutations::En1999Mutation;
use crate::artifact_schema::mutations::{
    change_annex, change_bolt_count, change_material_designation, change_member_buckling_length, change_member_m_y_ed, change_member_n_ed, change_plate_thickness,
    change_weld_throat, add_member, remove_member, change_cold_formed, change_connections, change_fatigue_details, change_fire_scenarios, change_materials, change_members,
    change_sections, change_shells,
};
use crate::snapshot::{AluminiumConnection, AluminiumMaterial, AluminiumMember, AluminiumSection, AluminiumShell, ColdFormedSheet, FatigueDetail, FireScenario};
use protocol::OpText;
use crate::standards::v1::subsets::any::io::text::mutations::{En1999MutationDsl,en1999_mutation_to_dsl,en1999_mutation_from_dsl};

impl protocol::OpBinary for En1999MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}

impl protocol::OpBinary for En1999Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        en1999_mutation_to_dsl(self).encode_op()
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(en1999_mutation_from_dsl(En1999MutationDsl::decode_op(bytes)?))
    }
}
}
