//! 📦️ VCS app — binary document surface + laws (constitutional: pack).

use store::PackError;
use vcs::VcsDemoProjection;

/// 📦️ Encodes a `VcsDemoProjection` to its binary pack form.
pub fn encode(projection: &VcsDemoProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(projection)
}

/// 📖️ Decodes a `VcsDemoProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<VcsDemoProjection, PackError> {
    <VcsDemoProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcs_demo_projection_dsl_pack_equivalence() {
        let projection = vcs_engine::empty_vcs_demo_projection();
        store::test_support::assert_dsl_pack_equivalence(&projection);
        let bytes = encode(&projection);
        assert_eq!(decode(&bytes).expect("decode"), projection);
    }
}
//#endregion 🧪️Tests
