//! 🧠️ Reasoning plugin — declarative WIRES mindmap play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`), hence the `../../` prefix
//! back out to the plugin (owner) root. The grouping modules carry `#[path = "."]` so their own names are
//! not spliced into that base directory — without it, Rust resolves an inline module's children under
//! `<file dir>/<inline mod name>/…` and every leaf path dangles. Do not inline any component file back
//! into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint both fail on it (see
//! master ticket `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard
//! ruling, and its Shape V2 addendum for the `📦️packages`-relocated entry file).

extern crate infinite_canvas as infinite_board_port_directed;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<WiresMutation, WiresConfigMutation>, Fault>`, the exact signature
// `ArtifactApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split creates),
// so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
mod artifacts { pub use semio_s_artifact_reasoning_wires as wires; }
//#endregion 🗿️Artifacts

//#region ✏️Editor
mod editor { pub use semio_s_artifact_reasoning_wires::editor::*; }
//#endregion ✏️Editor

//#region 👁️Viewer
mod viewer { pub use semio_s_artifact_reasoning_wires::viewer::*; }
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::ReasoningApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_wires_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
