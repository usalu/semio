//! 🎯️ Keyboard focus of the grips of the open artifact: the walk rules and the per-domain registry are domain-neutral and live in the framework (`semio_framework::walk_grips`, pinned by the language-agnostic fixture `🐚️Shell/🧪️tests/🧫️fixtures/🎯️grip-focus/🔣️.json` that the React shell answers too); this module is where the Norm shell hooks reach them from.
//!
//! 🔗️ Paint: `lane_overlays::grips`; keys: `NormShellHooks::handle_key`; see `visual-editor-architecture.md` §8.2.

pub use semio_framework::{focused_grip, note_grips, noted_grips, set_focused_grip, walk_focus, walk_grips, GripRef, GripStep, GripWalk};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
