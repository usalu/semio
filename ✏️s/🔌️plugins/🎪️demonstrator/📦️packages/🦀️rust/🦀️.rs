//! 🎪️ Entwerfen mit Bestand demonstrator — the six demonstrator panes (generator, koordinator,
//! aggregator, aussuchen, bearbeiten, verfolgen) bundled as ONE hot-swappable WASM plugin instead of
//! six separate ones, so they share one framework/kernel linkage and one plugin worker/module (see
//! `acquirePluginModule`'s lease pool in framework core `🟦️.ts`) instead of statically
//! duplicating the SDK six times over.
//!
//! This crate also owns the minimal `🎪️playground` artifact (schema/snapshot/diff/dsl/op/spr — no
//! engine, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) so the demonstrator taxonomy
//! slot is complete. Pane apps still come from the six source plugins.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]`
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust`, hence the `../../` climb
//! back out to the owner root's tree). The grouping module carries `#[path = "."]` so its own name is
//! not spliced into that base directory. Do not inline any component file back into this one: the
//! taxonomy validator and the `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

//#region 🗿️Artifacts
mod artifacts { pub use semio_s_artifact_demonstrator_playground as playground; }
//#endregion 🗿️Artifacts

//#region ✏️Editor
mod editor { pub use semio_s_artifact_demonstrator_playground::editor::*; }
//#endregion ✏️Editor

//#region 👁️Viewer
mod viewer { pub use semio_s_artifact_demonstrator_playground::viewer::*; }
//#endregion 👁️Viewer

//#region 🛂️Manifest
#[path = "../../🪪️manifest/🎪️demonstrator/🦀️.rs"]
mod manifest;
// 🎪️ Unconditional, unlike the six bundled panes' own `plugin-entry`-gated exports: this crate
// IS the single terminal wasm component (nothing depends on it as a lib), so it always owns the
// `semio_plugin_install_bundle` entry point — no other crate ever needs to disable it. This
// crate's `Cargo.toml` never declared a `plugin-entry` feature, so the gate this line used to
// carry was permanently false: the export never compiled in, on any build, ever.
semio_framework_plugin::plugin_exports!(manifest::plugin, manifest::DemonstratorApps);
//#endregion 🛂️Manifest

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_playground_demo;
}
//#endregion 📚️Examples
