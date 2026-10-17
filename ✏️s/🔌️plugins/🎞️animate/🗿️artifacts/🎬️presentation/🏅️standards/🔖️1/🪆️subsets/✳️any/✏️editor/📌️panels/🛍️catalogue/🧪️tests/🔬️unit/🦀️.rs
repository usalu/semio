use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
#[allow(unused_imports)]
use crate::editor::animate::terminology::AnimatePresentationLabels;
#[allow(unused_imports)]
use crate::{editor::animate::support::animate_presentation_action, editor::animate::support::ui_capacity_error, editor::animate::support::ui_children, editor::animate::support::ui_label, editor::animate::support::ui_map, editor::animate::support::ui_node, editor::animate::support::ui_text};
#[allow(unused_imports)]
use crate::PresentationSnapshot;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::PanelGroup;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::PanelTabKind;
#[allow(unused_imports)]
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;
#[allow(unused_imports)]
use semio_framework_ui_contract::{button, column, field, input, section, text, BuiltNode, HasBase, InputKind, Trigger, UiValue};
use crate::editor::animate::panels::catalogue::*;
use crate::editor::animate::unit_tests::context::{presentation_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
    assert_eq!(definition.body_key.as_deref(), Some(PRESENTATION_PLAY_BODY_CATALOGUE));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_templates() {
    let mut app = presentation_app().await;
    assert!(render_body(&mut app, PRESENTATION_PLAY_BODY_CATALOGUE).await.contains("animate.presentation.play.catalogue.templates"));
}
