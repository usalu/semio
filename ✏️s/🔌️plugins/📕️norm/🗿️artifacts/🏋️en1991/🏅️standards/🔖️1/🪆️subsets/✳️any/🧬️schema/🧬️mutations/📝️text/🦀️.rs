//! ⚡️ En1991 mutations — OpText as the aggregate's JSON wire, OpBinary as the protocol-tagged payload frame (design-load subject).

pub use crate::artifact_schema::mutations::En1991Mutation;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

impl protocol::OpText for En1991Mutation {
    fn print_op(&self) -> String {
        pack::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        pack::json::from_json_str(line).map_err(|e| store::TextError::new(e.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for En1991Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        semio_s_artifact_norm_contract::payload_op_binary::encode::<crate::En1991Snapshot, _>(include_str!("../💾️binary/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        semio_s_artifact_norm_contract::payload_op_binary::decode::<crate::En1991Snapshot, _>(include_str!("../💾️binary/📡️.protocol.semio"), bytes)
    }
}


#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1991Mutation> {
    use crate::artifact_schema::mutations::*;
    vec![
        En1991Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::De }),
        En1991Mutation::ChangeAltitude(change_altitude::ChangeAltitude { new_altitude: 1.0 }),
        En1991Mutation::ChangeAssumedBridgeTandem(change_assumed_bridge_tandem::ChangeAssumedBridgeTandem { new_assumed_bridge_tandem: 1.0 }),
        En1991Mutation::ChangeAssumedBridgeUdl(change_assumed_bridge_udl::ChangeAssumedBridgeUdl { new_assumed_bridge_udl: 1.0 }),
        En1991Mutation::ChangeAssumedCraneHorizontal(change_assumed_crane_horizontal::ChangeAssumedCraneHorizontal { new_assumed_crane_horizontal: 1.0 }),
        En1991Mutation::ChangeAssumedSiloPatch(change_assumed_silo_patch::ChangeAssumedSiloPatch { new_assumed_silo_patch: 1.0 }),
        En1991Mutation::ChangeStructureKind(change_structure_kind::ChangeStructureKind { new_structure_kind: crate::StructureKind::Bridge }),
    ]
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
