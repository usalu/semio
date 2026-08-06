//! 🧮️ Mathematical plugin — declarative mathematical play app (graph algorithms + computational
//! geometry) bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<MathOperation, MathConfigOperation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod mathematical {
        #[path = "../../🗿️artifacts/➗️mathematical/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/➗️mathematical/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/➗️mathematical/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/➗️mathematical/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/➗️mathematical/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/➗️mathematical/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/➗️mathematical/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod mathematical {
        #[path = "../../🎛️apps/➗️mathematical/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/➗️mathematical/🎚️config/🦀️component.rs"]
        pub mod config;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/➗️mathematical/🎮️commands/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/➗️mathematical/🎮️commands/🕸️graph/🦀️component.rs"]
            pub mod graph;
            #[path = "../../🎛️apps/➗️mathematical/🎮️commands/📐️geometry/🦀️component.rs"]
            pub mod geometry;
            #[path = "../../🎛️apps/➗️mathematical/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/➗️mathematical/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/➗️mathematical/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️component.rs"]
                    pub mod graph;
                    #[path = "../../🎛️apps/➗️mathematical/🎭️modes/✏️edit/🪟️windows/📐️geometry/🦀️component.rs"]
                    pub mod geometry;
                }
            }
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
semio_framework_plugin::semio_plugin! {
    id: "mathematical", label: "Mathematical", version: "0.1.0",
    setup: artifacts::mathematical::engine::register,
    apps: [ apps::mathematical::create_mathematical_app => apps::mathematical::MathematicalPlayApp ],
}
//#endregion 🔖️Plugin
