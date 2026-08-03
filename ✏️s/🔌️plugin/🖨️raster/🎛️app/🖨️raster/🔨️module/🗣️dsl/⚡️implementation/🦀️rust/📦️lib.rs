//! 📜️ Raster app — textual document grammar surface + laws (constitutional: dsl).

use raster::RasterProjection;

/// 📄️ The `semio` example document, handcrafted in the `.raster` DSL.
pub const SEMIO_RASTER_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/🖨️raster/📚️example/🖨️semio.raster");

/// 📖️ Parses `.raster` DSL text into a `RasterProjection`.
pub fn parse_dsl(text: &str) -> Result<RasterProjection, store::TextError> {
    <RasterProjection as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `RasterProjection` back to `.raster` DSL text.
pub fn print_dsl(document: &RasterProjection) -> String {
    store::DocumentDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use raster::{RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};
    use std::collections::BTreeMap;

    /// 📄️ Handcrafted document exercising every layer kind/field, shared with the `pack`/`op`
    /// constitutional crates' own copies (each crate keeps its own private copy — crate boundaries
    /// prevent reuse of a `#[cfg(test)]`-only fn across crates).
    fn representative_raster_document() -> RasterProjection {
        let mut assets = BTreeMap::new();
        assets.insert("asset-1".into(), RasterImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc==".into() });
        let mut params = BTreeMap::new();
        params.insert("brightness".into(), dsl::to_dsl_value(&serde_json::json!(0.06)).expect("dsl value"));
        params.insert("label".into(), dsl::to_dsl_value(&serde_json::json!("Warm \"Curve\"")).expect("dsl value"));
        params.insert("enabled".into(), dsl::to_dsl_value(&serde_json::json!(true)).expect("dsl value"));
        params.insert("fallback".into(), dsl::DslValue::Null);
        params.insert("curves".into(), dsl::to_dsl_value(&serde_json::json!([[0.0, 0.0], [0.25, 0.2], [1.0, 1.0]])).expect("dsl value"));
        params.insert("nested".into(), dsl::to_dsl_value(&serde_json::json!({ "inner": 1.5 })).expect("dsl value"));
        RasterProjection {
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
                        RasterLayerNode::Group { id: "group-2".into(), name: "Nested Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() },
                    ],
                },
                RasterLayerNode::Adjustment { id: "adjust-1".into(), name: "Curves & Co".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "curves".into(), params },
            ],
        }
    }

    #[test]
    fn semio_example_dsl_round_trips() {
        let document = parse_dsl(SEMIO_RASTER_EXAMPLE_TEXT).expect("parse semio example");
        store::test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn raster_dsl_round_trips_representative_document() {
        store::test_support::assert_dsl_round_trip(&representative_raster_document());
    }
}
//#endregion 🧪️Tests
