//! 🎪️ Entwerfen mit Bestand demonstrator — the six demonstrator panes (generator, koordinator,
//! aggregator, aussuchen, bearbeiten, verfolgen) bundled as ONE hot-swappable WASM plugin instead of
//! six separate ones, so they share one framework/kernel linkage and one plugin worker/module (see
//! `acquirePluginModule`'s lease pool in framework core `📦️index.ts`) instead of statically
//! duplicating the SDK six times over.
//!
//! 🕳️ Deviation from the usual per-app-plugin shape: this crate owns no `🗿️artifacts` and no
//! `🎛️apps` — every app it registers belongs to one of six source plugins
//! (procedural/cad/puzzle/sourcing/process/gis) it depends on, so there is no document schema, no
//! command enum and no DSL/pack/spr codec of its own, and the `semio_plugin!` macro (which assumes
//! exactly that) does not apply. What demonstrator genuinely owns is the six PANES: each pane's host
//! export wiring plus its bundle registration, one `🎪️panes/<variant>/🦀️component.rs` per playground
//! variant declared in `Cargo.toml`. The manual `Plugin` builder + `plugin_exports!` invocation
//! below is the same pattern `🪐️space`'s bundle uses.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]`
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust`, hence the `../../` climb
//! back out to the owner root's tree). The grouping module carries `#[path = "."]` so its own name is
//! not spliced into that base directory. Do not inline any component file back into this one: the
//! taxonomy validator and the `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

//#region 🎪️Panes
#[path = "."]
pub mod panes {
    #[path = "../../🎪️panes/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🎪️panes/🌱️generator/🦀️component.rs"]
    pub mod generator;
    #[path = "../../🎪️panes/📐️koordinator/🦀️component.rs"]
    pub mod koordinator;
    #[path = "../../🎪️panes/🧩️aggregator/🦀️component.rs"]
    pub mod aggregator;
    #[path = "../../🎪️panes/🗂️aussuchen/🦀️component.rs"]
    pub mod aussuchen;
    #[path = "../../🎪️panes/🏭️bearbeiten/🦀️component.rs"]
    pub mod bearbeiten;
    #[path = "../../🎪️panes/🗺️verfolgen/🦀️component.rs"]
    pub mod verfolgen;
}
//#endregion 🎪️Panes

//#region 🔖️Manifest
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);
//#endregion 🔖️Manifest
