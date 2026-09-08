//! 👁️ Lowpoly play app — the always-visible "show edges" window-chrome toggle. Shared verbatim by both
//! windows (model + uv), since they expose an identical option set — see the master ticket's TEMPLATE.md
//! §12.2 "shared options across multiple windows" pattern, extended here to windows split across modes.

use crate::editor::lowpoly::config::LowpolyConfig;
use crate::editor::lowpoly::lowpoly_window_action;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use semio_framework_plugin::WindowMeasure;

/// 🎛️ The live chrome measure for this option.
pub fn measure(config: &LowpolyConfig, labels: &LowpolyLabels) -> WindowMeasure {
    WindowMeasure::Toggle { id: "lowpoly-measure-show-edges".into(), icon_id: "grid-3x3".into(), label: Some(labels.show_edges.into()), pressed: config.show_edges, text: None, on_change: lowpoly_window_action("toggleShowEdges", None) }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
