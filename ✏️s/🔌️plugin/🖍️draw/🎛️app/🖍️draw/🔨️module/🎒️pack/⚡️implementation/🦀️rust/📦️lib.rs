//! 📦️ Draw app — binary document surface + laws (constitutional: pack).

use draw::DrawDocument;
use store::PackError;

/// 📦️ Encodes a `DrawDocument` to its binary pack form.
pub fn encode(document: &DrawDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `DrawDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<DrawDocument, PackError> {
    <DrawDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use draw::{
        DrawArtboard, DrawCircle, DrawEllipse, DrawGroupBody, DrawImageAsset, DrawLayerNode, DrawLine, DrawPolygon, DrawShapeBody, DrawTextBody,
        FillStyle, GradientStop, PathSegment, StrokeStyle, DRAW_DOCUMENT_SCHEMA,
    };
    use draw_engine::{create_draw_boolean_layer, create_draw_image_layer, create_draw_path_layer, create_draw_shape_layer_rect, create_draw_trace_layer, default_draw_document, default_layer_base, layer_id};

    fn representative_draw_document() -> DrawDocument {
        let mut assets = std::collections::BTreeMap::new();
        assets.insert("src-1".to_string(), DrawImageAsset { mime: "image/png".into(), data: "aGVsbG8=".into(), width: Some(8), height: Some(8) });

        let mut rect_shape = create_draw_shape_layer_rect("Rect");
        if let DrawLayerNode::Shape(shape) = &mut rect_shape {
            shape.base.attributes.fill = Some(FillStyle::LinearGradient {
                x1: 0.0,
                y1: 0.0,
                x2: 10.0,
                y2: 10.0,
                stops: vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 1.0, 1.0] }],
            });
            shape.base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.5, cap: "round".into(), join: "round".into(), dash: Some(vec![2.0, 4.0]) });
        }
        let rect_id = layer_id(&rect_shape).to_string();

        let line_shape = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawLine { x1: 0.0, y1: 0.0, x2: 5.0, y2: 5.0 }), polygon: None });
        let line_id = layer_id(&line_shape).to_string();

        let polygon_shape = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Polygon"), shape_kind: "polygon".into(), rect: None, ellipse: None, circle: None, line: None, polygon: Some(DrawPolygon { points: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]] }) });

        let mut radial_circle = DrawShapeBody { base: default_layer_base("RadialCircle"), shape_kind: "circle".into(), rect: None, ellipse: None, circle: Some(DrawCircle { cx: 1.0, cy: 2.0, r: 3.0 }), line: None, polygon: None };
        radial_circle.base.attributes.fill = Some(FillStyle::RadialGradient { cx: 1.0, cy: 2.0, r: 3.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 0.0, 0.0] }] });
        let radial_circle = DrawLayerNode::Shape(radial_circle);

        let path_layer = create_draw_path_layer(
            "Path",
            vec![
                PathSegment::Move { to: [0.0, 0.0] },
                PathSegment::Line { to: [1.0, 0.0] },
                PathSegment::Quad { ctrl: [1.0, 1.0], to: [2.0, 1.0] },
                PathSegment::Cubic { ctrl1: [2.0, 2.0], ctrl2: [3.0, 2.0], to: [3.0, 3.0] },
                PathSegment::Arc { rx: 2.0, ry: 2.0, rotation: 0.0, large_arc: false, sweep: true, to: [1.0, -1.0] },
                PathSegment::Close,
            ],
        );

        let text_layer = DrawLayerNode::Text(DrawTextBody { base: default_layer_base("Label"), x: 4.0, y: 5.0, content: "semio \"draw\"\ndsl".into(), size: 12.0 });
        let image_layer = create_draw_image_layer("Image", "src-1");
        let trace_layer = create_draw_trace_layer("Trace", "src-1");
        let boolean_layer = create_draw_boolean_layer("Boolean", "xor", vec![rect_id.clone(), line_id]);

        let ellipse_shape = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Ellipse"), shape_kind: "ellipse".into(), rect: None, ellipse: Some(DrawEllipse { cx: 1.0, cy: 2.0, rx: 3.0, ry: 4.0 }), circle: None, line: None, polygon: None });
        let group_layer = DrawLayerNode::Group(DrawGroupBody { base: default_layer_base("Group \"nested\""), children: vec![ellipse_shape, radial_circle] });

        DrawDocument {
            schema: DRAW_DOCUMENT_SCHEMA.into(),
            id: "dsl-fixture".into(),
            title: Some("DSL Fixture \"Quotes\" \\ backslash".into()),
            layers: vec![rect_shape, line_shape, polygon_shape, path_layer, text_layer, image_layer, trace_layer, boolean_layer, group_layer],
            assets: Some(assets),
            artboard: Some(DrawArtboard { width: 640.0, height: 480.0 }),
        }
    }

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = draw_dsl::parse_dsl(draw_dsl::SEMIO_DRAW_EXAMPLE_TEXT).expect("parse semio example");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn pack_round_trips_representative_document() {
        store::test_support::assert_dsl_pack_equivalence(&representative_draw_document());
    }

    #[test]
    fn pack_round_trips_document_without_assets_or_artboard() {
        let mut doc = default_draw_document("no-extras", None);
        doc.assets = None;
        doc.artboard = None;
        doc.title = None;
        store::test_support::assert_dsl_pack_equivalence(&doc);
    }
}
//#endregion 🧪️Tests
