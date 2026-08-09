//! 📦️ VDI 3805 app — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use store::PackError;
use crate::artifacts::vdi3805::Vdi3805Snapshot;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Vdi3805Snapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Vdi3805Snapshot, PackError> {
    <Vdi3805Snapshot as store::DocumentPack>::decode_pack(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 🪲️ Blocked on the confirmed upstream `pack` crate bug root-caused by the `draw` wave-2 family
    // (`.🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/wave2-draw.txt` §4):
    // `pack/value/rs/lib.rs`'s `decode_table_soa` fallback branch drops the column's `Shape` (passes
    // `None` where `encode_table`'s matching branch passes `Some(&field.shape)`), so a `#[dsl(table)]`
    fn document_dsl_pack_equivalence_the_reference_fixture() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&crate::artifacts::vdi3805::reference_fixture());
    }

    #[test]
    fn pack_round_trips_the_reference_fixture() {
        let document = crate::artifacts::vdi3805::reference_fixture();
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
