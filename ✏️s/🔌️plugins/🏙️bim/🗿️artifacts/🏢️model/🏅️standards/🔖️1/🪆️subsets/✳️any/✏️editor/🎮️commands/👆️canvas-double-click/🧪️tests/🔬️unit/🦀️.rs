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
use crate::editor::bim::commands::canvas_double_click::*;
use crate::editor::bim::fixture::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn a_double_click_is_a_gesture_command_and_needs_its_retained_route() {
    let payload = CanvasDoubleClick { x: 1.0, y: 2.0, width: 3.0, height: 4.0, ..Default::default() };
    assert_eq!((payload.raw().x, payload.raw().height), (1.0, 4.0));
    assert!(run(&demo(), |doc, cfg| handle(&payload, doc, cfg, &mut ctx(&[]))).err().is_some());
}
