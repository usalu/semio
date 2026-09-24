//! 🖼️ The page exports run on the BUNDLED example, evaluated live: the flow host with the packaged
//! `draw` flow extension its `neuron-kind`s resolve to linked in-process (the served guest receives it
//! as a host contribution instead). Its own test binary: a linked operator pack is process-wide, and
//! the lib tests pin the bare host, where the demo's operators are unknown.
use semio_s_artifact_procedural_generation2d::standards::v1::subsets::any::io::export::serializers::artifacts::{dxf::v_r12::any as dxf_out, pdf::v1_4::any as pdf_out, png::v1_2::any as png_out, svg::v1_1::any as svg_out};
use semio_s_artifact_procedural_generation2d::standards::v1::subsets::any::io::generation2d_drawing;
use semio_s_artifact_procedural_generation2d::standards::v1::subsets::any::schema::default_snapshot;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn installed() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        semio_framework_os_flow::register_linked_flow_extension_installer("draw", semio_s_plugin_flow_extension_draw::register);
        semio_framework_os_flow::install_flow_extension_manifest("flow-extension-draw", &semio_s_plugin_flow_extension_draw::extension_manifest_json()).expect("draw extension admission");
    });
}

/// 📐️ The example's slider (30) drives a `draw.shape.rect` of width 30 and default height 10 at the
/// origin, outlined by `draw.style.stroke` (black, width 1); every page export carries exactly that
/// rectangle on a page padded by 16 on each side.
#[test]
fn bundled_example_exports_its_evaluated_rectangle_to_every_page_format() {
    installed();
    let example = default_snapshot();
    let drawing = generation2d_drawing(&example).expect("the bundled example evaluates to a drawing");
    assert_eq!((drawing.canvas.width, drawing.canvas.height), (30.0 + 32.0, 10.0 + 32.0));
    let svg = String::from_utf8(svg_out::serialize_bytes(&example).expect("svg")).expect("utf-8");
    assert!(svg.starts_with("<svg") || svg.contains("<svg "), "{svg}");
    assert_eq!(svg.matches("<path").count(), 1, "{svg}");
    assert!(pdf_out::serialize_bytes(&example).expect("pdf").starts_with(b"%PDF-1.4"));
    let png = semio_s_artifact_stdio_png::io::decode_png(&png_out::serialize_bytes(&example).expect("png")).expect("decodes as png");
    assert_eq!((png.width, png.height), (62, 42));
    assert!(png.pixels.chunks(4).filter(|px| px[3] > 0).count() >= 2 * (30 + 10), "the rectangle outline is painted");
    let alpha = |x: u32, y: u32| png.pixels[((y * png.width + x) * 4 + 3) as usize];
    assert!(alpha(31, 16) > 0 && alpha(46, 21) > 0, "the outline sits inside the 16-unit page margin");
    assert_eq!((alpha(4, 4), alpha(31, 21), alpha(57, 37)), (0, 0, 0), "margin and interior stay empty");
    assert!(svg.contains("transform=\"matrix(1,0,") && svg.contains(",1,16,16)\""), "the page margin reaches the svg: {svg}");
    let dxf = String::from_utf8(dxf_out::serialize_bytes(&example).expect("dxf")).expect("dxf text");
    assert!(dxf.contains("POLYLINE") || dxf.contains("LWPOLYLINE"), "{dxf}");
    example.retire_cold();
}
