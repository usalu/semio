#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use crate::editor::flow::edit_rules::ContentEdit;
#[allow(unused_imports)]
use crate::editor::flow::host::{flow_composed_content, flow_content_leaves_emit, host_from_snapshot};
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
#[allow(unused_imports)]
use crate::{FlowMutation, FlowSnapshot};
#[allow(unused_imports)]
use flow::FlowEvalSession;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::flow::commands::reorganize::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app, settle};
use crate::editor::flow::FlowCommand;

#[semio_framework_async_macros::async_test]
async fn reorganize_keeps_every_widget() {
    let mut app = flow_app().await;
    let before_scene = app.snapshot().expect("snapshot").to_host_snapshot();
    let before = before_scene.widgets.len();
    before_scene.retire_cold();
    dispatch(&mut app, FlowCommand::Reorganize(Reorganize {})).await;
    settle(&mut app).await;
    let after_scene = app.snapshot().expect("snapshot").to_host_snapshot();
    assert_eq!(after_scene.widgets.len(), before);
    after_scene.retire_cold();
}
