#[allow(unused_imports)]
use crate::editor::flow::base::FLOW_PLAY_WINDOW_GENERATE_FORM;
#[allow(unused_imports)]
use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::main::transient::FlowWindowTransient;
#[allow(unused_imports)]
use crate::editor::flow::base::{flow_action, ui_value_map, ui_value_text};
#[allow(unused_imports)]
use crate::playbook::{default_value_for_block, is_block_visible, selected_generation, PlaybookBlock, PlaybookValues};
#[allow(unused_imports)]
use crate::FlowSnapshot;
#[allow(unused_imports)]
use flow::forms_bridge::flow_host_snapshot_to_form_spec;
#[allow(unused_imports)]
use semio_framework_ui_contract::Label;
#[allow(unused_imports)]
use semio_framework_plugin::ActionId;
#[allow(unused_imports)]
use semio_framework_plugin::Buildable;
#[allow(unused_imports)]
use semio_framework_plugin::BuiltNode;
#[allow(unused_imports)]
use semio_framework_plugin::HasBase;
#[allow(unused_imports)]
use semio_framework_plugin::HasChildren;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::PluginAssemblyError;
#[allow(unused_imports)]
use semio_framework_plugin::SurfaceKind;
#[allow(unused_imports)]
use semio_framework_plugin::Trigger;
#[allow(unused_imports)]
use semio_framework_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_framework_plugin::UiFixedList;
#[allow(unused_imports)]
use semio_framework_plugin::UiText;
#[allow(unused_imports)]
use semio_framework_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::WindowOptions;
#[allow(unused_imports)]
use semio_framework_ui_contract::InputKind;
use crate::editor::flow::modes::generate::windows::form::*;
use crate::editor::flow::unit_tests::context::{flow_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn without_a_generation_the_form_shows_the_placeholder_copy() {
    let mut app = flow_app().await;
    assert!(render_body(&mut app, FLOW_PLAY_BODY_GENERATE_FORM).await.contains("Add a generation"));
}
