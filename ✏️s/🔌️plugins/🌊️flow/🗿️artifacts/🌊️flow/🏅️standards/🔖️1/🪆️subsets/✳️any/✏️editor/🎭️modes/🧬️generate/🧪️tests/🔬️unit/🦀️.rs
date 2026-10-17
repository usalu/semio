use crate::editor::flow::base::{FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW, FLOW_PLAY_WINDOW_GENERATIONS};
#[allow(unused_imports)]
use crate::editor::flow::modes::generate::windows::{form, generations, preview};
#[allow(unused_imports)]
use semio_framework_plugin::create_default_layout;
#[allow(unused_imports)]
use semio_framework_plugin::create_named_layout;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::CommandDefinition;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::ModeDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::NamedLayout;
use crate::editor::flow::modes::generate::*;

#[semio_framework_async_macros::async_test]
async fn the_named_layout_lists_all_three_generate_windows() {
    let json = serde_json::to_string(&layout()).expect("layout json");
    for window in [FLOW_PLAY_WINDOW_GENERATIONS, FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW] {
        assert!(json.contains(window), "layout must reference {window}: {json}");
    }
}

#[semio_framework_async_macros::async_test]
async fn generation_commands_are_owned_by_the_generate_mode() {
    let mode = definition();
    assert_eq!(mode.commands.iter().map(|command| command.id.as_str()).collect::<Vec<_>>(), ["addGeneration", "removeGeneration", "selectGeneration", "renameGeneration", "updateGenerationValues"]);
    assert!(mode.commands.iter().all(|command| !command.in_palette));
}
