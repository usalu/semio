//! 🌊️ Flow plugin — declarative flow play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<FlowOperation, FlowConfigOperation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod flow {
        #[path = "../../🗿️artifacts/🌊️flow/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🌊️flow/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🌊️flow/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🌊️flow/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🌊️flow/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🌊️flow/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🌊️flow/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod flow {
        #[path = "../../🎛️apps/🌊️flow/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🌊️flow/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🌊️flow/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/🧩️widget/🦀️component.rs"]
            pub mod widget;
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/🔗️synapse/🦀️component.rs"]
            pub mod synapse;
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/🕸️node-graph/🦀️component.rs"]
            pub mod node_graph;
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/🔭️lod/🦀️component.rs"]
            pub mod lod;
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/🌐️grid/🦀️component.rs"]
            pub mod grid;
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/🔄️layout/🦀️component.rs"]
            pub mod layout;
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/🧮️eval/🦀️component.rs"]
            pub mod eval;
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/🧩️extension/🦀️component.rs"]
            pub mod extension;
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🌊️flow/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🌊️flow/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🎛️apps/🌊️flow/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🌊️flow/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️options/🔭️lod/🦀️component.rs"]
                            pub mod lod;
                            #[path = "../../🎛️apps/🌊️flow/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️options/📏️proximity/🦀️component.rs"]
                            pub mod proximity;
                            #[path = "../../🎛️apps/🌊️flow/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️options/🌐️grid/🦀️component.rs"]
                            pub mod grid;
                        }
                    }

                    #[path = "../../🎛️apps/🌊️flow/🎭️modes/✏️edit/🪟️windows/🗣️compiled/🦀️component.rs"]
                    pub mod compiled;
                }
            }

            #[path = "."]
            pub mod generate {
                #[path = "../../🎛️apps/🌊️flow/🎭️modes/🧬️generate/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod commands {
                    #[path = "../../🎛️apps/🌊️flow/🎭️modes/🧬️generate/🎮️commands/🧬️generation/🦀️component.rs"]
                    pub mod generation;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🌊️flow/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️component.rs"]
                    pub mod generations;
                    #[path = "../../🎛️apps/🌊️flow/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️component.rs"]
                    pub mod form;
                    #[path = "../../🎛️apps/🌊️flow/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🌊️flow/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🌊️flow/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🌊️flow/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
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
    #[path = "../../🗿️artifacts/🌊️flow/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_flow_demo;
    #[path = "../../🎛️apps/🌊️flow/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_flow_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
