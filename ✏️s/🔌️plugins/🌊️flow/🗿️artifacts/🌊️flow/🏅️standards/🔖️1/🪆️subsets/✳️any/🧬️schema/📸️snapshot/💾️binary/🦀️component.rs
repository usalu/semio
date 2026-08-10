//! 📦️ Flow artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::flow::FlowSnapshot;
use store::PackError;

/// 📦️ Encodes a `FlowSnapshot` to its binary pack form.
pub fn encode(snapshot: &FlowSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

/// 📖️ Decodes a `FlowSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<FlowSnapshot, PackError> {
    <FlowSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::flow::dsl;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let snapshot = dsl::parse_dsl(dsl::FLOW_EXAMPLE_TEXT).expect("parse default snapshot");
        store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
        let bytes = encode(&snapshot);
        assert_eq!(decode(&bytes).expect("decode"), snapshot);
    }

    #[test]
    fn pack_protocol_names_snapshot_segment() {
        assert!(COMPONENT_PROTOCOL_SEMIO.contains("segment payload"));
    }
}
//#endregion 🧪️Tests
