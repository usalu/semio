//! 📦️ Raster app — binary document surface + laws (constitutional: pack).

use raster::RasterProjection;
use store::PackError;

/// 📦️ Encodes a `RasterProjection` to its binary pack form.
pub fn encode(document: &RasterProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `RasterProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<RasterProjection, PackError> {
    <RasterProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use raster::{RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};
    use std::collections::BTreeMap;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = raster_dsl::parse_dsl(raster_dsl::SEMIO_RASTER_EXAMPLE_TEXT).expect("parse semio example");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn pack_round_trips_representative_document() {
        let mut assets = BTreeMap::new();
        assets.insert(
            "asset-1".into(),
            RasterImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc==".into() },
        );
        let mut params = serde_json::Map::new();
        params.insert("brightness".into(), serde_json::json!(0.06));
        params.insert("label".into(), serde_json::json!("Warm \"Curve\""));
        params.insert("enabled".into(), serde_json::json!(true));
        params.insert("fallback".into(), serde_json::Value::Null);
        params.insert("curves".into(), serde_json::json!([[0.0, 0.0], [0.25, 0.2], [1.0, 1.0]]));
        params.insert("nested".into(), serde_json::json!({ "inner": 1.5 }));
        let document = RasterProjection {
            schema: RASTER_DOCUMENT_SCHEMA.into(),
            id: "doc-1".into(),
            title: Some("Representative \"Doc\"".into()),
            assets,
            layers: vec![
                RasterLayerNode::Pixel {
                    id: "pixel-1".into(),
                    name: "Pixel One".into(),
                    visible: true,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    mask: Some(RasterLayerMask { enabled: true, linked: false, invert: true, width: Some(64), height: None }),
                    width: Some(256),
                    height: Some(256),
                    image_key: Some("asset-1".into()),
                },
                RasterLayerNode::Group {
                    id: "group-1".into(),
                    name: "Group / Nested".into(),
                    visible: false,
                    opacity: 0.5,
                    blend_mode: "screen".into(),
                    transform: RasterTransform { x: 1.0, y: -2.0, scale_x: 1.5, scale_y: 0.5, rotation: 12.0 },
                    mask: None,
                    children: vec![
                        RasterLayerNode::Pixel {
                            id: "pixel-2".into(),
                            name: "Child Pixel".into(),
                            visible: true,
                            opacity: 0.75,
                            blend_mode: "multiply".into(),
                            transform: RasterTransform::default(),
                            mask: None,
                            width: None,
                            height: None,
                            image_key: None,
                        },
                        RasterLayerNode::Group {
                            id: "group-2".into(),
                            name: "Nested Group".into(),
                            visible: true,
                            opacity: 1.0,
                            blend_mode: "normal".into(),
                            transform: RasterTransform::default(),
                            mask: None,
                            children: Vec::new(),
                        },
                    ],
                },
                RasterLayerNode::Adjustment {
                    id: "adjust-1".into(),
                    name: "Curves & Co".into(),
                    visible: true,
                    opacity: 1.0,
                    blend_mode: "normal".into(),
                    transform: RasterTransform::default(),
                    adjustment_kind: "curves".into(),
                    params,
                },
            ],
        };
        store::test_support::assert_dsl_pack_equivalence(&document);
    }
}
//#endregion 🧪️Tests
