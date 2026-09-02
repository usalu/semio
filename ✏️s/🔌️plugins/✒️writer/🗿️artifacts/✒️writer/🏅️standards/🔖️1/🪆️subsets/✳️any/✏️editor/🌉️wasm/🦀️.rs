//! 🌉️ Writer play app — editor-host aliases (was: the plugin-root `document_vcs` module +
//! `WriterHost`/`WriterSession` aliases in the old bundle crate's `🦀️.rs`; the wasm-bindgen
//! document VCS bridge that used to live here was never built by any `wasm32-unknown-unknown`
//! target — see `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` — so it was
//! deleted rather than kept as unreachable third-party-dependent code).

pub use framework_editor::*;

pub type WriterHost = EditorHost;
