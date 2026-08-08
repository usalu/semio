//! 🎒️ Architect program artifact — the binary document surface (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::program::ProgramSnapshot;
use store::PackError;

/// 🎒️ Encodes an Architect program into its binary pack representation.
pub fn encode(document: &ProgramSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes an Architect program from its binary pack representation.
pub fn decode(bytes: &[u8]) -> Result<ProgramSnapshot, PackError> {
    <ProgramSnapshot as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::{empty_plugin, sample_plugin};

    #[test]
    fn pack_round_trips_the_empty_program() {
        let document = empty_plugin();
        assert_eq!(decode(&encode(&document)).expect("decode"), document);
    }

    #[test]
    fn pack_round_trips_the_sample_program() {
        let document = sample_plugin();
        assert_eq!(decode(&encode(&document)).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
