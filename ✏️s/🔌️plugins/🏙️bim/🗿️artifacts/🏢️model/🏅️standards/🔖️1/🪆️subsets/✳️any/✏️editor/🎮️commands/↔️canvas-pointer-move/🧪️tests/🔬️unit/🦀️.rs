#[allow(unused_imports)]
use crate::editor::bim::gestures::session::ToolEvent;
#[allow(unused_imports)]
use crate::editor::bim::commands::canvas_pointer_down::canvas_pointer;
#[allow(unused_imports)]
use crate::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use crate::editor::bim::commands::canvas_pointer_move::*;
use crate::editor::bim::fixture::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_move_outside_a_retained_route_is_refused_and_the_payload_reads_pixels_and_modifiers() {
    let payload = CanvasPointerMove { x: 440.0, y: 260.0, width: 800.0, height: 600.0, shift: true, ..Default::default() };
    let raw = payload.raw();
    assert_eq!((raw.x, raw.y, raw.width, raw.height, raw.modifiers.shift, raw.modifiers.ctrl, raw.ground), (440.0, 260.0, 800.0, 600.0, true, false, None));
    let fault = run(&demo(), |doc, cfg| handle(&payload, doc, cfg, &mut ctx(&[]))).err().expect("no session owner");
    assert!(format!("{:?}", fault.code).contains(crate::editor::bim::gestures::RETAINED_ROUTE));
}
