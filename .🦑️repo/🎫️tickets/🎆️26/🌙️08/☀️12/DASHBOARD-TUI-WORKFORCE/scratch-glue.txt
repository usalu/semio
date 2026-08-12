//! 🖱️ Handcrafted semio UI. One crate, one target per feature: `tui` renders to terminal cells,
//! `wgpu` renders declarative components (and, with `wgpu-engine`, a retained-mode GPU engine).
//! The two share `ui_styling` tokens and the co-located `🧱️elements/<Element>/` widget sources but
//! nothing else — hence features rather than sibling crates, so a wasm32-wasip2 program component
//! pulling in the component types never links winit/parley/swash.

#[cfg(feature = "tui")]
#[path = "🎯️targets/⌨️tui/📦️glue.rs"]
pub mod tui;

#[cfg(feature = "wgpu")]
#[path = "🎯️targets/🧊️wgpu/📦️glue.rs"]
pub mod wgpu;
