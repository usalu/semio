//! 📦 S Studio app — binary document surface + laws (constitutional: pack).
//!
//! 🕳️ Wraps `semio_framework_os::OsProjection` — see `space_op`'s doc comment for why this app owns no
//! document/operation type.

use semio_framework_os::OsProjection;
use store::PackError;

/// 📦 Encodes an `OsProjection` to its binary pack form.
pub fn encode(projection: &OsProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(projection)
}

/// 📖 Decodes an `OsProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<OsProjection, PackError> {
    <OsProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use space_shared::demo_space_projection;

    #[test]
    fn demo_document_dsl_pack_equivalence() {
        let projection = demo_space_projection();
        store::test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn pack_round_trips_demo_projection() {
        let projection = demo_space_projection();
        let bytes = encode(&projection);
        assert_eq!(decode(&bytes).expect("decode"), projection);
    }
}
//#endregion 🧪Tests
