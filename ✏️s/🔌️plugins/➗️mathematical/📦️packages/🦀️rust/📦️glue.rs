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
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod mathematical {
        #[path = "../../🗿️artifacts/➗️mathematical/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/➗️mathematical/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/➗️mathematical/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/➗️mathematical/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }
        #[path = "../../🗿️artifacts/➗️mathematical/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/➗️mathematical/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod set_graph {
                #[path = "../../🗿️artifacts/➗️mathematical/🧬️mutations/📊set-graph/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/➗️mathematical/🧬️mutations/📊set-graph/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_geometry {
                #[path = "../../🗿️artifacts/➗️mathematical/🧬️mutations/📐set-geometry/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/➗️mathematical/🧬️mutations/📐set-geometry/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/➗️mathematical/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/➗️mathematical/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/➗️mathematical/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/➗️mathematical/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "../../🗿️artifacts/➗️mathematical/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
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
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/➗️mathematical/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_mathematical_demo;
    #[path = "../../🎛️apps/➗️mathematical/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_mathematical_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
