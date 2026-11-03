//! 🕹️ Direct manipulation of the visual norm editors: handle drags bound to field paths, snapping, keyboard equivalents and pick targets.
//! Gestures only ever write the transient lane; release raises one `setFields`, abort leaves no trace.

#[path = "🧲️snap/🦀️.rs"]
mod snap_rules;
pub use snap_rules::*;

#[path = "🎯️pick-targets/🦀️.rs"]
mod pick_targets;
pub use pick_targets::*;

#[path = "🛠️handle-drag-tool/🦀️.rs"]
mod handle_drag_tool;
pub use handle_drag_tool::*;

#[path = "➕insert-tool/🦀️.rs"]
mod insert_tool;
pub use insert_tool::*;

#[path = "📍️placement-drag/🦀️.rs"]
mod placement_drag;
pub use placement_drag::*;

#[path = "⌨️keyboard/🦀️.rs"]
mod keyboard;
pub use keyboard::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
