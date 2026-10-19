#[allow(unused_imports)]
use crate::editor::flow::editor_vocabulary::FLOW_PLAY_WINDOW_GENERATE_PREVIEW;
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::main::transient::FlowWindowTransient;
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
#[allow(unused_imports)] use semio_framework_ui_declaration::ui::TextEditorScene;
use crate::editor::flow::modes::generate::windows::preview::*;
use crate::editor::flow::unit_tests::context::{flow_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn the_preview_renders_a_text_editor_surface() {
    let mut app = flow_app().await;
    assert!(render_body(&mut app, FLOW_PLAY_BODY_GENERATE_PREVIEW).await.contains("text-editor"));
}
