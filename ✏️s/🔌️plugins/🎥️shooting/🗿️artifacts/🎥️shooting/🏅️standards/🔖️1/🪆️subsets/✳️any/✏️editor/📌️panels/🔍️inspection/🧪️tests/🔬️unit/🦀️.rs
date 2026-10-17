#[allow(unused_imports)]
use crate::editor::shooting::config::ShootingConfig;
#[allow(unused_imports)]
use crate::editor::shooting::terminology::ShootingLabels;
#[allow(unused_imports)]
use crate::{editor::shooting::ui_helpers_and_io::shooting_action, editor::shooting::ui_helpers_and_io::ui_capacity_error, editor::shooting::ui_helpers_and_io::ui_label, editor::shooting::ui_helpers_and_io::ui_text, editor::shooting::ui_helpers_and_io::ui_value_list, editor::shooting::ui_helpers_and_io::ui_value_map, editor::shooting::ui_helpers_and_io::ui_value_text};
#[allow(unused_imports)]
use semio_framework_plugin::ui_node_list;
#[allow(unused_imports)]
use crate::{ShootingShot, ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use semio_framework_plugin::{plugin_app_close_prelude::Buildable, plugin_app_close_prelude::HasBase, plugin_app_close_prelude::HasChildren, plugin_app_close_prelude::InputKind, plugin_app_close_prelude::Trigger};
#[allow(unused_imports)]
use semio_framework_plugin::tree_item;
#[allow(unused_imports)]
use semio_framework_plugin::tree_item_desc;
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
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
#[allow(unused_imports)]
use semio_framework_ui_contract::{input, BuiltNode};
use crate::editor::shooting::panels::inspection::*;
use crate::editor::shooting::unit_tests::context::{render as render_body, shooting_app};

#[semio_framework_async_macros::async_test]
async fn inspector_falls_back_to_the_active_shot() {
    let mut app = shooting_app().await;
    let json = render_body(&mut app, SHOOTING_PLAY_BODY_INSPECTION).await;
    assert!(json.contains("Shot"));
}
