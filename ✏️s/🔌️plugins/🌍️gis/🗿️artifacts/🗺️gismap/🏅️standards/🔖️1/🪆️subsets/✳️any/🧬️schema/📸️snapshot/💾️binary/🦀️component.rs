//! 📦️ GIS map artifact — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::gismap::GisMapSnapshot;
use store::PackError;

/// 📦️ Encodes a `GisMapSnapshot` to its binary pack form.
pub fn encode(document: &GisMapSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `GisMapSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<GisMapSnapshot, PackError> {
    <GisMapSnapshot as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gismap::dsl;
    use crate::artifacts::gismap::MapFeature;
    use serde_json::json;

    #[test]
    fn gis_map_document_pack_agrees_with_dsl_for_bundled_reuse_example() {
        let document = dsl::parse_dsl(dsl::REUSE_MAP_EXAMPLE_TEXT).expect("parse reuse-map example");
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        assert_eq!(decode(&encode(&document)).expect("decode"), document);
    }

    #[test]
    fn gis_map_document_pack_agrees_with_dsl_for_empty_document() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&GisMapSnapshot::default());
    }

    /// 🧬️ `MapFeature::data` is `dsl::DslValue` (deliberately untyped — see `crate::artifacts::gismap`'s
    /// doc comment) — this bridges a `serde_json::json!` literal into one for test-fixture ergonomics.
    #[test]
    fn gis_map_document_pack_agrees_with_dsl_for_synthetic_value_shapes() {
        let dsl_of = |value: serde_json::Value| ::dsl::to_dsl_value(&value).unwrap_or(::dsl::DslValue::Null);
        let document = GisMapSnapshot {
            positions: vec![MapFeature {
                id: "p1".into(),
                data: dsl_of(json!({
                    "id": "p1",
                    "lon": -0.1427,
                    "lat": 51.5142,
                    "flag": true,
                    "missing": null,
                    "tags": ["a", "b"],
                    "meta": { "nested": { "depth": 2.0 } },
                })),
            }],
            routes: vec![MapFeature { id: "r1".into(), data: dsl_of(json!({ "id": "r1", "points": [[1.0, 2.0], [3.0, 4.0]] })) }],
            regions: vec![MapFeature { id: "g1".into(), data: dsl_of(json!({ "id": "g1", "ring": [[0.0, 0.0], [1.0, 1.0], [1.0, 0.0]] })) }],
        };
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    }
}
//#endregion 🧪️Tests
