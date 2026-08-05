//! 📏️ Layout plugin — blueprint/preview document editor bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<LayoutOperation, LayoutConfigOperation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod layout {
        #[path = "🗿️artifacts/📏️layout/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "🗿️artifacts/📏️layout/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "🗿️artifacts/📏️layout/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "🗿️artifacts/📏️layout/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "🗿️artifacts/📏️layout/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "🗿️artifacts/📏️layout/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "🗿️artifacts/📏️layout/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "🗿️artifacts/📏️layout/⚙️engine/🦀️scene.rs"]
            pub mod scene;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod layout {
        #[path = "🎛️apps/📏️layout/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "🎛️apps/📏️layout/🦀️config.rs"]
        pub mod config;
        #[path = "🎛️apps/📏️layout/🦀️terminology.rs"]
        pub mod terminology;
        #[path = "🎛️apps/📏️layout/🦀️canvas.rs"]
        pub mod canvas;
        #[path = "🎛️apps/📏️layout/🦀️wasm.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "🎛️apps/📏️layout/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "🎛️apps/📏️layout/🎮️commands/🖱️pointer/🦀️component.rs"]
            pub mod pointer;
            #[path = "🎛️apps/📏️layout/🎮️commands/✏️author/🦀️component.rs"]
            pub mod author;
            #[path = "🎛️apps/📏️layout/🎮️commands/🐚️export/🦀️component.rs"]
            pub mod export;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🎛️apps/📏️layout/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🎛️apps/📏️layout/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🦀️component.rs"]
                    pub mod blueprint;
                    #[path = "🎛️apps/📏️layout/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🎛️apps/📏️layout/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "🎛️apps/📏️layout/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "🎛️apps/📏️layout/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
            #[path = "🎛️apps/📏️layout/📌️panels/🚦️preflight/🦀️component.rs"]
            pub mod preflight;
        }
    }
}
//#endregion 🎛️Apps

//#region 🕸️Wasm
#[cfg(target_arch = "wasm32")]
pub use apps::layout::wasm::LayoutSession;
//#endregion 🕸️Wasm

//#region 🔖️Plugin
semio_framework_plugin::semio_plugin! {
    id: "layout",
    label: "Layout",
    version: "0.1.0",
    setup: artifacts::layout::engine::register,
    apps: [ apps::layout::create_layout_app => apps::layout::LayoutPlayApp ],
}
//#endregion 🔖️Plugin
