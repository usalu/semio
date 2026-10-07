//! ⚖️ En1997 app — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::mutations::En1997Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &En1997Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<En1997Mutation, protocol::ProtocolError> {
    En1997Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
use crate::document::AnnexChoice;
use crate::artifact_schema::mutations::En1997Mutation;
use crate::artifact_schema::mutations::{
    change_annex, change_design_approach, change_design_situation, change_footing_embedment, change_footing_width, change_geotechnical_category, change_groundwater_level,
    change_investigation_depth, change_layer_oedometric_modulus, change_layer_phi_prime, change_pile_count, change_pile_length, change_slope_angle, change_wall_base_width,
    insert_footing, insert_layer, insert_pile, remove_footing, remove_layer, remove_pile,
};
use crate::{Pile, SoilLayer, SpreadFoundation};
use protocol::OpText;
use crate::standards::v1::subsets::any::io::text::mutations::{En1997MutationDsl,en1997_mutation_to_dsl,en1997_mutation_from_dsl};

impl protocol::OpBinary for En1997MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("📡️.protocol.semio"), bytes)
    }
}

impl protocol::OpBinary for En1997Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        en1997_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(en1997_mutation_from_dsl(En1997MutationDsl::decode_op(bytes)?))
    }
}
}
