//! 📐️ Writes a caller-owned neutral vector scene through the standard drawing serializers.
use semio_framework_io_ports::{
    VectorNode, VectorPoint2, VectorPoint3, VectorRgba, VectorScene, VectorSegment, VectorTransform,
};
use semio_s_artifact_stdio_semio_base::schema::geometry::{
    SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform,
};
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{
    DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot,
    STDIO_SEMIODRAWING_DOCUMENT_SCHEMA,
};
use semio_s_artifact_stdio_semio_drawing::io::SemioDrawingFormat;
use semio_s_artifact_stdio_semio_drawing_export::encode_drawing;

fn point(point: VectorPoint2) -> SemioPoint2 {
    SemioPoint2 { x: point.x, y: point.y }
}

fn point3(point: VectorPoint3) -> SemioPoint3 {
    SemioPoint3 { x: point.x, y: point.y, z: point.z }
}

fn color(rgba: VectorRgba) -> SemioRgba {
    SemioRgba { r: rgba.r, g: rgba.g, b: rgba.b, a: rgba.a }
}

fn transform(value: VectorTransform) -> SemioTransform {
    let rotation = value.rotation;
    SemioTransform {
        translation: point3(value.translation),
        rotation: SemioQuaternion { x: rotation.x, y: rotation.y, z: rotation.z, w: rotation.w },
        scale: point3(value.scale),
    }
}

fn segment(value: VectorSegment) -> PathSegment {
    match value {
        VectorSegment::MoveTo { to } => PathSegment::MoveTo { to: point(to) },
        VectorSegment::LineTo { to } => PathSegment::LineTo { to: point(to) },
        VectorSegment::CubicTo { c1, c2, to } => PathSegment::CubicTo { c1: point(c1), c2: point(c2), to: point(to) },
        VectorSegment::QuadTo { c, to } => PathSegment::QuadTo { c: point(c), to: point(to) },
        VectorSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to } =>
            PathSegment::ArcTo { rx, ry, x_rotation, large_arc, sweep, to: point(to) },
        VectorSegment::Close => PathSegment::Close,
    }
}

fn node(value: VectorNode) -> DrawNode {
    match value {
        VectorNode::Path { segments, style } =>
            DrawNode::Path { segments: segments.into_iter().map(segment).collect(), style },
        VectorNode::Text { value, at, style } => DrawNode::Text { value, at: point(at), style },
        VectorNode::Group { transform: placement, children } =>
            DrawNode::Group { transform: transform(placement), children: children.into_iter().map(node).collect() },
        VectorNode::Image { at, width, height, mime, bytes } =>
            DrawNode::Image { at: point(at), width, height, mime, bytes },
    }
}

/// 🖼️ Projects an owned vector scene into one standard file.
pub(super) fn encode_vector_scene(scene: VectorScene, format: SemioDrawingFormat) -> Result<Vec<u8>, String> {
    let drawing = SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas {
            width: scene.canvas.width,
            height: scene.canvas.height,
            background: scene.canvas.background.map(color),
        },
        styles: scene.styles.into_iter().map(|style| DrawStyle {
            name: style.name,
            fill: style.fill.map(color),
            stroke: style.stroke.map(color),
            stroke_width: style.stroke_width,
            opacity: style.opacity,
        }).collect(),
        layers: scene.layers.into_iter().map(|layer| DrawLayer {
            id: layer.id,
            name: layer.name,
            visible: layer.visible,
            root: node(layer.root),
        }).collect(),
    };
    encode_drawing(&drawing, format)
}

