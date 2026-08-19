//! 📦️ Procedural3d artifact — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::procedural3d::Procedural3dSnapshot;
use store::PackError;

/// 📦️ Encodes a `Procedural3dSnapshot` to its binary pack form.
pub async fn encode(document: &Procedural3dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Procedural3dSnapshot` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<Procedural3dSnapshot, PackError> {
    <Procedural3dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural3d::dsl;
    use semio_framework_os_kernel::os_store::test_support;

    #[semio_framework_async_macros::async_test]
    async fn dsl_pack_equivalence_empty_projection() {
        test_support::assert_dsl_pack_equivalence(&Procedural3dSnapshot::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trips_the_hex_column_example() {
        let projection = dsl::parse_dsl(dsl::PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT).expect("parse fixture");
        let bytes = encode(&projection);
        assert_eq!(decode(&bytes).expect("decode"), projection);
    }
}
//#endregion 🧪️Tests
