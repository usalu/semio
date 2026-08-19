//! 📦️ GIS terrain artifact — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::gisterrain::GisTerrainSnapshot;
use store::PackError;

/// 📦️ Encodes a `GisTerrainSnapshot` to its binary pack form.
pub async fn encode(document: &GisTerrainSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `GisTerrainSnapshot` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<GisTerrainSnapshot, PackError> {
    <GisTerrainSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gisterrain::dsl;

    #[test]
    async fn gis3d_terrain_document_pack_agrees_with_dsl_for_bundled_reuse_example() {
        let document = dsl::parse_dsl(dsl::REUSE_TERRAIN_EXAMPLE_TEXT).expect("parse reuse-terrain example");
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        assert_eq!(decode(&encode(&document)).expect("decode"), document);
    }

    #[test]
    async fn gis3d_terrain_document_pack_agrees_with_dsl_for_arbitrary_exaggeration() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&GisTerrainSnapshot { exaggeration: 2.75, imported_features_json: String::new(), ..Default::default() });
    }

    #[test]
    async fn gis3d_terrain_document_pack_agrees_with_dsl_for_imported_features_json() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&GisTerrainSnapshot { exaggeration: 1.0, imported_features_json: r#"{"positions":[{"id":"p1","lon":1.0,"lat":2.0}],"routes":[],"regions":[]}"#.into(), ..Default::default() });
    }
}
//#endregion 🧪️Tests
