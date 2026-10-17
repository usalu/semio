use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
#[allow(unused_imports)]
use crate::editor::flow::terminology::FlowPlayLabels;
#[allow(unused_imports)]
use semio_framework_ui_contract::Label;
#[allow(unused_imports)]
use semio_framework_plugin::BuiltNode;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::PanelGroup;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabKind;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTreeBuilder;
#[allow(unused_imports)]
use semio_framework_plugin::PluginAssemblyError;
#[allow(unused_imports)]
use semio_framework_plugin::TreeWindows;
#[allow(unused_imports)]
use semio_framework_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
use crate::editor::flow::panels::inspection::*;
use crate::editor::flow::unit_tests::context::{flow_app, render as render_body};
use crate::editor::flow::FLOW_PLAY_BODY_INSPECTOR as BODY_INSPECTOR;

#[semio_framework_async_macros::async_test]
async fn empty_inspector_no_longer_shows_canvas_settings() {
    let mut app = flow_app().await;
    let json = render_body(&mut app, BODY_INSPECTOR).await;
    assert!(!json.contains("flow-play-inspector.lod-mode"));
    assert!(json.contains("flow-play-inspector.empty"));
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert_eq!(definition.body_key.as_deref(), Some(FLOW_PLAY_BODY_INSPECTOR));
}
