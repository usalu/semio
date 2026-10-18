#[allow(unused_imports)]
use crate::editor::bim::gestures::{keep_entry, run_keyed};
#[allow(unused_imports)]
use crate::editor::bim::gestures::session::ToolEvent;
#[allow(unused_imports)]
use crate::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use crate::editor::bim::commands::canvas_escape::*;
use crate::editor::bim::fixture::support::{ctx, demo, run};

#[semio_framework_async_macros::async_test]
async fn escape_cancels_only_through_the_retained_route() {
    let fault = run(&demo(), |doc, cfg| handle(&CanvasEscape {}, doc, cfg, &mut ctx(&[]))).err().expect("no session owner");
    assert!(format!("{:?}", fault.code).contains(crate::editor::bim::gestures::RETAINED_ROUTE));
}
