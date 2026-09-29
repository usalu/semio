use super::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
use crate::standards::v1::subsets::drawing::io::export::serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg;
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn p(x: f64, y: f64) -> SemioPoint2 {
    SemioPoint2 { x, y }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn rgba(r: f32, g: f32, b: f32, a: f32) -> Option<SemioRgba> {
    Some(SemioRgba { r, g, b, a })
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn drawing(width: f64, height: f64, styles: Vec<DrawStyle>, children: Vec<DrawNode>) -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        canvas: DrawCanvas { width, height, background: None },
        styles,
        layers: vec![DrawLayer { id: "l".into(), name: "l".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
        ..SemioDrawingSnapshot::default()
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn square(x0: f64, y0: f64, x1: f64, y1: f64, style: Option<&str>) -> DrawNode {
    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: p(x0, y0) }, PathSegment::LineTo { to: p(x1, y0) }, PathSegment::LineTo { to: p(x1, y1) }, PathSegment::LineTo { to: p(x0, y1) }, PathSegment::Close], style: style.map(Into::into) }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn scene() -> SemioDrawingSnapshot {
    let styles = vec![
        DrawStyle { name: "red-fill".into(), fill: rgba(0.9, 0.1, 0.1, 1.0), stroke: None, stroke_width: None, opacity: None },
        DrawStyle { name: "blue-outline".into(), fill: None, stroke: rgba(0.1, 0.2, 0.9, 1.0), stroke_width: Some(6.0), opacity: None },
        DrawStyle { name: "green-both".into(), fill: rgba(0.1, 0.8, 0.2, 0.6), stroke: rgba(0.0, 0.0, 0.0, 1.0), stroke_width: Some(2.0), opacity: Some(0.8) },
    ];
    let rotated = SemioTransform { translation: SemioPoint3 { x: 150.0, y: 60.0, z: 0.0 }, rotation: SemioQuaternion { x: 0.0, y: 0.0, z: (0.3f64).sin(), w: (0.3f64).cos() }, scale: SemioPoint3 { x: 1.5, y: 0.75, z: 1.0 } };
    drawing(
        240.0,
        160.0,
        styles,
        vec![
            square(10.0, 10.0, 90.0, 70.0, Some("red-fill")),
            DrawNode::Path { segments: vec![PathSegment::MoveTo { to: p(20.0, 140.0) }, PathSegment::CubicTo { c1: p(60.0, 80.0), c2: p(120.0, 200.0), to: p(200.0, 120.0) }, PathSegment::LineTo { to: p(220.0, 150.0) }], style: Some("blue-outline".into()) },
            DrawNode::Path {
                segments: vec![PathSegment::MoveTo { to: p(100.0, 40.0) }, PathSegment::ArcTo { rx: 30.0, ry: 20.0, x_rotation: 15.0, large_arc: true, sweep: false, to: p(160.0, 40.0) }, PathSegment::QuadTo { c: p(130.0, 0.0), to: p(100.0, 40.0) }, PathSegment::Close],
                style: Some("green-both".into()),
            },
            DrawNode::Group { transform: rotated, children: vec![square(-20.0, -10.0, 20.0, 10.0, None)] },
        ],
    )
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn premultiplied(rgba8: &[u8]) -> Vec<f32> {
    rgba8.chunks(4).flat_map(|px| {
        let a = px[3] as f32 / 255.0;
        [px[0] as f32 / 255.0 * a, px[1] as f32 / 255.0 * a, px[2] as f32 / 255.0 * a, a]
    }).collect()
}

/// 🔮️ resvg (third-party, test-only) renders the svg leaf's output of the same drawing.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn resvg_premultiplied(drawing: &SemioDrawingSnapshot, width: u32, height: u32) -> Vec<f32> {
    let svg = semio_framework_plugin::resolve_ready(SemioDrawingToSvg::serialize(drawing)).expect("svg leaf");
    let text = String::from_utf8(svg.export_utf8().expect("svg bytes")).expect("utf-8 svg");
    let tree = resvg::usvg::Tree::from_str(&text, &resvg::usvg::Options::default()).expect("resvg parses the svg leaf output");
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height).expect("pixmap");
    resvg::render(&tree, resvg::tiny_skia::Transform::identity(), &mut pixmap.as_mut());
    pixmap.data().iter().map(|v| *v as f32 / 255.0).collect()
}

#[test]
fn a_pixel_aligned_square_covers_exactly_its_pixels() {
    let raster = rasterize_drawing(&drawing(10.0, 10.0, Vec::new(), vec![square(2.0, 2.0, 8.0, 8.0, None)])).expect("raster");
    let covered = raster.rgba8.chunks(4).filter(|px| px[3] == 255).count();
    let empty = raster.rgba8.chunks(4).filter(|px| px[3] == 0).count();
    assert_eq!((raster.width, raster.height, covered, empty), (10, 10, 36, 64));
    assert!(raster.rgba8.chunks(4).filter(|px| px[3] == 255).all(|px| px[..3] == [0, 0, 0]));
}

#[test]
fn a_half_pixel_edge_is_half_covered() {
    let raster = rasterize_drawing(&drawing(4.0, 1.0, Vec::new(), vec![square(0.0, 0.0, 1.5, 1.0, None)])).expect("raster");
    let alphas: Vec<u8> = raster.rgba8.chunks(4).map(|px| px[3]).collect();
    assert_eq!(alphas, vec![255, 128, 0, 0]);
}

#[test]
fn opposite_windings_cancel_under_the_non_zero_rule() {
    let hole = DrawNode::Path {
        segments: vec![
            PathSegment::MoveTo { to: p(0.0, 0.0) }, PathSegment::LineTo { to: p(10.0, 0.0) }, PathSegment::LineTo { to: p(10.0, 10.0) }, PathSegment::LineTo { to: p(0.0, 10.0) }, PathSegment::Close,
            PathSegment::MoveTo { to: p(3.0, 3.0) }, PathSegment::LineTo { to: p(3.0, 7.0) }, PathSegment::LineTo { to: p(7.0, 7.0) }, PathSegment::LineTo { to: p(7.0, 3.0) }, PathSegment::Close,
        ],
        style: None,
    };
    let raster = rasterize_drawing(&drawing(10.0, 10.0, Vec::new(), vec![hole])).expect("raster");
    assert_eq!(raster.rgba8.chunks(4).filter(|px| px[3] == 0).count(), 16);
}

#[test]
fn the_raster_matches_resvg_rendering_the_svg_export_of_the_same_drawing() {
    let scene = scene();
    let ours = rasterize_drawing(&scene).expect("raster");
    let oracle = resvg_premultiplied(&scene, ours.width, ours.height);
    let mine = premultiplied(&ours.rgba8);
    assert_eq!(mine.len(), oracle.len());
    let differences: Vec<f32> = mine.iter().zip(&oracle).map(|(a, b)| (a - b).abs()).collect();
    let mean = differences.iter().sum::<f32>() / differences.len() as f32;
    let far = differences.iter().filter(|d| **d > 0.25).count() as f32 / differences.len() as f32;
    let painted = oracle.chunks(4).filter(|px| px[3] > 0.5).count();
    assert!(painted > 4000, "the oracle painted {painted} pixels");
    assert!(mean < 0.006, "mean premultiplied channel difference {mean}");
    assert!(far < 0.004, "fraction of channels differing by more than a quarter {far}");
}

#[test]
fn png_bytes_decode_back_to_the_same_pixels() {
    let scene = scene();
    let png = semio_framework_plugin::resolve_ready(SemioDrawingToPng::serialize(&scene)).expect("png leaf");
    let bytes = semio_s_artifact_stdio_png::io::encode_png(&png).expect("encode");
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
    let decoded = semio_s_artifact_stdio_png::io::decode_png(&bytes).expect("decode");
    assert_eq!((decoded.width, decoded.height, decoded.pixels), (240, 160, rasterize_drawing(&scene).expect("raster").rgba8));
}

#[test]
fn a_canvas_without_size_is_refused() {
    assert!(rasterize_drawing(&drawing(0.0, 10.0, Vec::new(), Vec::new())).is_err());
}
