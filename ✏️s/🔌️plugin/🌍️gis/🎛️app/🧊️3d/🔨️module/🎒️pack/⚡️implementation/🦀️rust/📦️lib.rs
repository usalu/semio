//! 📦️ GIS 3D app — binary document surface + laws (constitutional: pack).

use gis3d::Gis3dTerrainDocument;
use store::PackError;

/// 📦️ Encodes a `Gis3dTerrainDocument` to its binary pack form.
pub fn encode(document: &Gis3dTerrainDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `Gis3dTerrainDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Gis3dTerrainDocument, PackError> {
    <Gis3dTerrainDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gis3d_terrain_document_pack_agrees_with_dsl_for_bundled_reuse_example() {
        let document = gis3d_dsl::parse_dsl(gis3d_dsl::REUSE_TERRAIN_EXAMPLE_TEXT).expect("parse reuse-terrain example");
        store::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[test]
    fn gis3d_terrain_document_pack_agrees_with_dsl_for_arbitrary_exaggeration() {
        store::test_support::assert_dsl_pack_equivalence(&Gis3dTerrainDocument { exaggeration: 2.75 });
    }
}
//#endregion 🧪️Tests
