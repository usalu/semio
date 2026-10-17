use crate::editor::flow::commands::run_extension_action::FLOW_AUTOMATIONS;
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
#[allow(unused_imports)]
use crate::editor::flow::host::with_host_from_snapshot;
#[allow(unused_imports)]
use crate::editor::flow::terminology::{flow_extension_action_title_label, flow_extension_label, FlowPlayLabels};
#[allow(unused_imports)]
use crate::editor::flow::base::{flow_action, ui_value_bool, ui_value_map, ui_value_text};
#[allow(unused_imports)]
use crate::FlowSnapshot;
#[allow(unused_imports)]
use flow::FlowEvalSession;
#[allow(unused_imports)]
use semio_framework_ui_contract::Label;
#[allow(unused_imports)]
use semio_framework_plugin::tree_item_with_action;
#[allow(unused_imports)]
use semio_framework_plugin::tree_item_with_action_draggable;
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
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;
#[allow(unused_imports)]
use serde_json::Value;
use crate::editor::flow::panels::catalogue::*;
use crate::editor::flow::unit_tests::context::{flow_app, render as render_body, render_with_view};

#[semio_framework_async_macros::async_test]
async fn flow_widget_drag_json_wraps_descriptor_under_drag_mime() {
    let descriptor = flow_widget_descriptor("neuron", Some("math.add"));
    let drag = flow_widget_drag_json(&descriptor);
    assert!(drag.get(FLOW_WIDGET_DRAG_MIME).is_some());
}

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_module_operators() {
    let mut app = flow_app().await;
    let json = render_body(&mut app, FLOW_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("flow-play-catalogue.math"), "expected math module section: {json}");
    assert!(json.contains("math.add"), "expected math.add operator: {json}");
}

#[semio_framework_async_macros::async_test]
async fn catalogue_items_export_flow_widget_drag_payload() {
    let mut app = flow_app().await;
    let json = render_body(&mut app, FLOW_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains(FLOW_WIDGET_DRAG_MIME), "missing drag mime: {json}");
    assert!(json.contains(r#""draggable":true"#) || json.contains(r#""draggable": true"#));
}

#[semio_framework_async_macros::async_test]
async fn every_built_in_extension_is_listed_in_the_installed_section() {
    let mut app = flow_app().await;
    let view = semio_framework_plugin::ViewModel {
        tree_windows: vec![semio_framework_plugin::TreeWindowRequest { body_key: FLOW_PLAY_BODY_CATALOGUE.into(), node_key: "flow-play-extensions.installed".into(), open: Some(true), offset: 0, rows: FLOW_AUTOMATIONS.len() as u32 }],
        ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let json = render_with_view(&mut app, FLOW_PLAY_BODY_CATALOGUE, &view).await;
    for (id, ..) in FLOW_AUTOMATIONS {
        assert!(json.contains(&format!("flow-play-extensions.{id}")), "extension {id} missing: {json}");
    }
}
