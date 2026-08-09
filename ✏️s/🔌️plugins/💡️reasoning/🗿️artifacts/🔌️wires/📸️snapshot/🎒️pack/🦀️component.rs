//! 📦️ Wires artifact — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::wires::WiresSnapshot;
use store::PackError;

/// 📦️ Encodes a `WiresSnapshot` to its binary pack form.
pub fn encode(snapshot: &WiresSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(snapshot)
}

/// 📖️ Decodes a `WiresSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<WiresSnapshot, PackError> {
    <WiresSnapshot as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::dsl as wires_dsl;
    use store::os_store::test_support::assert_dsl_pack_equivalence;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl_metabolism() {
        let snapshot = crate::artifacts::wires::engine::metabolism_wires_example_snapshot();
        let bytes = encode(&snapshot);
        let decoded = decode(&bytes).expect("decode metabolism pack");
        assert_eq!(decoded.board_fixture.get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
    }

    #[test]
    fn pack_round_trips_empty_snapshot() {
        let snapshot = crate::artifacts::wires::empty_wires_snapshot();
        assert_dsl_pack_equivalence(&snapshot);
    }
}
//#endregion 🧪️Tests
