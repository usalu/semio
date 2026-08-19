//! ✏️ Architect play app — the `edit` mode: the app's default mode, and the only one that carries a
//! window layout.
//!
//! Architect's five window kinds are declared at APP level by the manifest (`AppBuilder::window_kind_def`
//! binds a window kind to the app, never to a mode) and no mode declares its own layout — so all five
//! live under this mode, the shallowest common ancestor and the sole owner of a layout referencing them
//! (the `🔍️review` and `📊️report` modes reuse the same window kinds; see TEMPLATE §12.4a).

use crate::editor::architect::modes::edit::windows::{adjacency, graph, register, report};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const ARCHITECT_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: ARCHITECT_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub async fn layout() -> WindowLayout {
    create_default_layout(
        &[adjacency::ARCHITECT_WINDOW_ADJACENCY.into(), graph::ARCHITECT_WINDOW_GRAPH.into(), register::ARCHITECT_WINDOW_REGISTER.into(), report::ARCHITECT_WINDOW_REPORT.into()],
        "row",
        Some(&[30.0, 30.0, 20.0, 20.0]),
        Some(&["Adjacency".into(), "Graph".into(), "Register".into(), "Report".into()]),
    )
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn the_default_layout_lists_the_four_edit_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        for window in [adjacency::ARCHITECT_WINDOW_ADJACENCY, graph::ARCHITECT_WINDOW_GRAPH, register::ARCHITECT_WINDOW_REGISTER, report::ARCHITECT_WINDOW_REPORT] {
            assert!(json.contains(window), "layout must reference {window}: {json}");
        }
    }

    #[test]
    async fn the_mode_is_the_pencil_edit_mode() {
        let definition = definition();
        assert_eq!(definition.id, ARCHITECT_MODE_EDIT);
        assert_eq!(definition.icon_id.as_str(), "pencil");
    }
}
//#endregion 🧪️Tests
