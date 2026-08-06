//! 🏛️ Architect plugin — the architectural-programming document app, bundled as a hot-swappable WASM
//! plugin.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<ProgramOperation, ArchitectConfigOperation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned
// error type; boxing it here would diverge from the trait it must satisfy, and the lint does not fire on
// the trait impl itself (only on the free functions the taxonomy split creates), so this is a pure
// artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod program {
        #[path = "../../🗿️artifacts/🏛️program/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🏛️program/🧱️kernel/🦀️component.rs"]
        pub mod kernel;
        #[path = "../../🗿️artifacts/🏛️program/🗄️registers/🦀️component.rs"]
        pub mod registers;

        #[path = "../../🗿️artifacts/🏛️program/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🏛️program/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🏛️program/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🏛️program/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🏛️program/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/↔️adjacency/🦀️component.rs"]
            pub mod adjacency;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🔬️analyze/🦀️component.rs"]
            pub mod analyze;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📤️exchange/🦀️component.rs"]
            pub mod exchange;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🎁️outputs/🦀️component.rs"]
            pub mod outputs;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📄️report/🦀️component.rs"]
            pub mod report;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🔍️search/🦀️component.rs"]
            pub mod search;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📊️status-summary/🦀️component.rs"]
            pub mod status_summary;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/📐️template/🦀️component.rs"]
            pub mod template;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/🧭️trace/🦀️component.rs"]
            pub mod trace;
            #[path = "../../🗿️artifacts/🏛️program/⚙️engine/✅️validate/🦀️component.rs"]
            pub mod validate;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod architect {
        #[path = "../../🎛️apps/🏛️architect/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🏛️architect/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🏛️architect/🎨️chrome/🦀️component.rs"]
        pub mod chrome;
        #[path = "../../🎛️apps/🏛️architect/🗂️catalog/🦀️component.rs"]
        pub mod catalog;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/📋️register/🦀️component.rs"]
            pub mod register;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/↔️adjacency/🦀️component.rs"]
            pub mod adjacency;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/📐️template/🦀️component.rs"]
            pub mod template;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/📤️exchange/🦀️component.rs"]
            pub mod exchange;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/🏗️element/🦀️component.rs"]
            pub mod element;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/🔬️analysis/🦀️component.rs"]
            pub mod analysis;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/🕸️graph/🦀️component.rs"]
            pub mod graph;
            #[path = "../../🎛️apps/🏛️architect/🎮️commands/🔍️search/🦀️component.rs"]
            pub mod search;
        }

        #[path = "."]
        pub mod modes {
            #[path = "../../🎛️apps/🏛️architect/🎭️modes/🔍️review/🦀️component.rs"]
            pub mod review;
            #[path = "../../🎛️apps/🏛️architect/🎭️modes/📊️report/🦀️component.rs"]
            pub mod report;

            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🦀️component.rs"]
                    pub mod adjacency;
                    #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️component.rs"]
                    pub mod graph;
                    #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/📋️register/🦀️component.rs"]
                    pub mod register;
                    #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/📄️report/🦀️component.rs"]
                    pub mod report;
                    #[path = "../../🎛️apps/🏛️architect/🎭️modes/✏️edit/🪟️windows/🧭️trace/🦀️component.rs"]
                    pub mod trace;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🏛️architect/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🏛️architect/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🏛️architect/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
fn register_architect_exports() {
    artifacts::program::engine::register();
}

semio_framework_plugin::semio_plugin! {
    id: "architect",
    label: "Architect",
    version: "0.1.0",
    setup: register_architect_exports,
    apps: [ apps::architect::create_architect_app => apps::architect::ArchitectPlayApp ],
}
//#endregion 🔖️Plugin
