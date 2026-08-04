//! 🎒️ Architect app binary document surface (constitutional: pack).

use architect::Program;

//#region 🔖️DocumentPack
/// 🎒️ Encodes an Architect document into its binary pack representation.
pub fn encode(document: &Program) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes an Architect document from its binary pack representation.
pub fn decode(bytes: &[u8]) -> Result<Program, store::PackError> {
    <Program as store::DocumentPack>::decode_pack(bytes)
}
//#endregion 🔖️DocumentPack
