//! 🔬️ The page exports run on the BUNDLED example, evaluated live: the flow host, with the two
//! packaged flow extensions its `neuron-kind`s resolve to linked in-process (the served guest receives
//! them as host contributions instead).
use super::*;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::{dxf::v_r12::any as dxf_out, pdf::v1_4::any as pdf_out, png::v1_2::any as png_out, svg::v1_1::any as svg_out};
use crate::standards::v1::subsets::any::schema::default_snapshot;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn installed() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        semio_framework_os_flow::register_linked_flow_extension_installer("math", semio_s_plugin_flow_extension_math::register);
        semio_framework_os_flow::register_linked_flow_extension_installer("draw", semio_s_plugin_flow_extension_draw::register);
        semio_framework_os_flow::install_flow_extension_manifest("flow-extension-math", &semio_s_plugin_flow_extension_math::extension_manifest_json()).expect("math extension admission");
        semio_framework_os_flow::install_flow_extension_manifest("flow-extension-draw", &semio_s_plugin_flow_extension_draw::extension_manifest_json()).expect("draw extension admission");
    });
}

#[test]
fn bundled_example_page_exports() {
    installed();
    let example = default_snapshot();
    let eval = crate::standards::v1::subsets::any::schema::with_host(&example.host_snapshot, |host| host.evaluate().unwrap_or_default());
    eprintln!("[DEBUG] bundled example evaluation: {eval}");
    eprintln!("[DEBUG] drawing: {:?}", generation2d_drawing(&example).map(|drawing| drawing.canvas));
    for (format, result) in [("svg", svg_out::serialize_bytes(&example)), ("pdf", pdf_out::serialize_bytes(&example)), ("png", png_out::serialize_bytes(&example)), ("dxf", dxf_out::serialize_bytes(&example))] {
        eprintln!("[DEBUG] {format}: {:?}", result.map(|bytes| bytes.len()));
    }
    example.retire_cold();
    let text = crate::standards::v1::subsets::any::schema::snapshot::text::GENERATION2D_EXAMPLE_TEXT.replace("neuron-kind=math.add", "neuron-kind=draw.shape.rect").replace("slider@number->rect@a", "slider@number->rect@width");
    let variant = <crate::Generation2dSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("variant parses");
    let eval = crate::standards::v1::subsets::any::schema::with_host(&variant.host_snapshot, |host| host.evaluate().unwrap_or_default());
    eprintln!("[DEBUG] variant evaluation: {eval}");
    variant.retire_cold();
}
