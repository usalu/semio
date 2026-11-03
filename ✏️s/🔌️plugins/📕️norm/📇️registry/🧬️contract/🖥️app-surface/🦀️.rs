//! 🎛️ Norm plugin — the app-surface machinery every one of the fifteen compliance apps shares.
//!
//! 📌️ The fifteen norm apps are structurally identical by construction (one `edit` mode, an
//! inputs/results window pair, the framework document/catalogue/inspection panel trio, the same
//! `model:in`/`report:out` media ports, the same three commands) and differ only in their per-standard
//! `Document` type, ids and labels. Everything that does NOT vary lives here, ONCE; every taxonomy node
//! under each subset's `✏️editor/`/`👁️viewer/` states only what genuinely varies and calls into this module.
//!
//! 🧱️ This crate is the thin public facade of the split app-surface crates, bottom to top:
//! `foundation` (ids, value paths, field meta, report tree, store macros), `viewport3d` (scene lowering),
//! `visual-chrome` (2D diagrams, accessibility, themes, device profiles, manifest, interaction domain),
//! `live-core` (lanes, direct manipulation, edit rules, dispatch), `panels` (checks, properties, outliner,
//! selection mirror, catalogue, toolbar, lane overlays) and `live-editor` (retained commands, tool jobs,
//! editor wiring, oracles). The old `viewport3d` module path keeps its toolbar and lane overlays.
//!
//! Nothing here depends on any app or artifact module: every entry point is either a plain constructor
//! or generic over the artifact's `Document`/`NormFamily`, so this stays a leaf of the dependency graph.

#![allow(async_fn_in_trait, unused_imports)]

include!("📋️prelude/🦀️.rs");

pub use semio_s_plugin_norm_app_surface_foundation::*;
pub use semio_s_plugin_norm_app_surface_live_core::*;
pub use semio_s_plugin_norm_app_surface_visual_chrome::*;
pub use semio_s_plugin_norm_app_surface_panels::*;
pub use semio_s_plugin_norm_app_surface_live_editor::*;
pub use semio_s_plugin_norm_app_surface_visual_chrome::diagrams2d::{render_diagrams, DiagramsWindowSpec};

/// 🧊️ The 3D viewport module path: the viewport3d crate plus the panels crate's toolbar and lane overlays.
pub mod viewport3d {
    pub use semio_s_plugin_norm_app_surface_panels::{lane_overlays, toolbar};
    pub use semio_s_plugin_norm_app_surface_viewport3d::*;
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
