//! 🌿️ VCS plugin — declarative version-control play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<VcsDemoOperation, VcsDemoConfigOperation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned
// error type; boxing it here would diverge from the trait it must satisfy, and the lint does not fire on
// the trait impl itself (only on the free functions the taxonomy split creates), so this is a pure
// artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod vcs {
        #[path = "🗿️artifacts/🌿️vcs/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "🗿️artifacts/🌿️vcs/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "🗿️artifacts/🌿️vcs/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "🗿️artifacts/🌿️vcs/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "🗿️artifacts/🌿️vcs/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "🗿️artifacts/🌿️vcs/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "🗿️artifacts/🌿️vcs/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod vcs {
        #[path = "🎛️apps/🌿️vcs/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "🎛️apps/🌿️vcs/🦀️config.rs"]
        pub mod config;
        #[path = "🎛️apps/🌿️vcs/🦀️terminology.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "🎛️apps/🌿️vcs/🎮️commands/📈️counter/🦀️component.rs"]
            pub mod counter;
            #[path = "🎛️apps/🌿️vcs/🎮️commands/🩹️patch/🦀️component.rs"]
            pub mod patch;
            #[path = "🎛️apps/🌿️vcs/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "🎛️apps/🌿️vcs/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "🎛️apps/🌿️vcs/🎮️commands/🖱️canvas/🦀️component.rs"]
            pub mod canvas;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🎛️apps/🌿️vcs/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🎛️apps/🌿️vcs/🎭️modes/✏️edit/🪟️windows/📝️editor/🦀️component.rs"]
                    pub mod editor;
                    #[path = "🎛️apps/🌿️vcs/🎭️modes/✏️edit/🪟️windows/📜️history/🦀️component.rs"]
                    pub mod history;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🎛️apps/🌿️vcs/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "🎛️apps/🌿️vcs/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
semio_framework_plugin::semio_plugin! {
    id: "vcs", label: "VCS", version: "0.1.0",
    setup: artifacts::vcs::engine::register,
    apps: [ apps::vcs::create_vcs_app => apps::vcs::VcsPlayApp ],
}
//#endregion 🔖️Plugin
