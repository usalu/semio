#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
#[allow(unused_imports)]
use crate::editor::flow::host::with_host_from_snapshot;
#[allow(unused_imports)]
use crate::FlowSnapshot;
#[allow(unused_imports)]
use flow::FlowEvalSession;
#[allow(unused_imports)]
use semio_framework_plugin::scene_surface;
#[allow(unused_imports)]
use semio_framework_plugin::BuiltNode;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::SurfaceKind;
#[allow(unused_imports)]
use semio_framework_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_framework_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowOptions;
#[allow(unused_imports)]
use semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind;
#[allow(unused_imports)]
use ui_wgpu::wgpu::TextEditorScene;
use crate::editor::flow::modes::edit::windows::compiled::*;
use crate::editor::flow::unit_tests::context::{flow_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_compiled_wire_editor() {
    let mut app = flow_app().await;
    assert!(render_body(&mut app, FLOW_PLAY_BODY_COMPILED).await.contains("text-editor"));
}
