//! 📨️ Review and export submissions independently of the form authoring layout.
use semio_framework_plugin::create_default_layout;
use semio_framework_plugin::create_named_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::NamedLayout;
#[path = "🪟️windows/📊️results/🦀️.rs"]
pub mod results;
pub const MODE: &str = "responses";
pub const LAYOUT: &str = "forms-responses";

pub fn definition() -> ModeDefinition {
    ModeDefinition { id: MODE.into(), label: LocalizedLabel::native("Responses", "Antworten"), icon_id: "list-checks".into(), tools: Vec::new(), layout_id: Some(LAYOUT.into()), commands: Vec::new() }
}

pub fn layout() -> NamedLayout {
    create_named_layout(LAYOUT, "Responses", create_default_layout(&[results::WINDOW.into()], "row", None, None), "builtin", Some("list-checks".into()), None)
}

