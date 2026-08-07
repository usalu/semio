extern crate infinite_canvas as infinite_board_port_directed_dag;
//! 🎬️ Sequence plugin — declarative sequence play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that
//! is written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).
//!
//! `🎮️commands/🏃️run` is wired here as Rust module `playback` (not `run`) — its own payload submodules
//! are `run_command`/`stop_command`, so naming the owning module `run` too would trip clippy's
//! `module_inception`. The directory keeps its taxonomy name (🏃️run); only the Rust identifier differs.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<SequenceOperation, SequenceConfigOperation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split
// creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod sequence {
        #[path = "../../🗿️artifacts/🎬️sequence/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🎬️sequence/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🎬️sequence/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🎬️sequence/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🎬️sequence/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🎬️sequence/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🎬️sequence/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod sequence {
        #[path = "../../🎛️apps/🎬️sequence/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🎬️sequence/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🎬️sequence/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[cfg(target_arch = "wasm32")]
        #[path = "../../🎛️apps/🎬️sequence/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🎬️sequence/🎮️commands/🪜️step/🦀️component.rs"]
            pub mod step;
            #[path = "../../🎛️apps/🎬️sequence/🎮️commands/🔗️connection/🦀️component.rs"]
            pub mod connection;
            #[path = "../../🎛️apps/🎬️sequence/🎮️commands/🕸️node-graph/🦀️component.rs"]
            pub mod node_graph;
            #[path = "../../🎛️apps/🎬️sequence/🎮️commands/🔄️layout/🦀️component.rs"]
            pub mod layout;
            #[path = "../../🎛️apps/🎬️sequence/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🎬️sequence/🎮️commands/🏃️run/🦀️component.rs"]
            pub mod playback;
            #[path = "../../🎛️apps/🎬️sequence/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🎬️sequence/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🎬️sequence/🎭️modes/✏️edit/🪟️windows/📽️main/🦀️component.rs"]
                    pub mod main;
                    #[path = "../../🎛️apps/🎬️sequence/🎭️modes/✏️edit/🪟️windows/📜️script/🦀️component.rs"]
                    pub mod script;
                    #[path = "../../🎛️apps/🎬️sequence/🎭️modes/✏️edit/🪟️windows/🧬️compiled/🦀️component.rs"]
                    pub mod compiled;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🎬️sequence/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🎬️sequence/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🎬️sequence/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);
//#endregion 🔖️Plugin
