//! ✍️ Fill and submit a form in a dedicated window layout.
use crate::editor::forms::modes::blueprint::windows::try_wizard::FORMS_PLAY_WINDOW_TRY;
use semio_framework_plugin::create_default_layout;
use semio_framework_plugin::create_named_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::NamedLayout;

pub const MODE: &str = "fill";
pub const LAYOUT: &str = "forms-fill";

pub fn definition() -> ModeDefinition {
    ModeDefinition { id: MODE.into(), label: LocalizedLabel::native("Fill Form", "Formular ausfüllen"), icon_id: "play".into(), tools: Vec::new(), layout_id: Some(LAYOUT.into()), commands: Vec::new() }
}

pub fn layout() -> NamedLayout {
    create_named_layout(LAYOUT, "Fill Form", create_default_layout(&[FORMS_PLAY_WINDOW_TRY.into()], "row", None, None), "builtin", Some("play".into()), None)
}
