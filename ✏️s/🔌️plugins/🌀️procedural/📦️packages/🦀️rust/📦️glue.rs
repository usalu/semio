extern crate flow_extension_draw as flow_core;
//! 🌀️ Procedural plugin — 2D and 3D flow apps bundled as one hot-swappable WASM component.
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
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<...Operation, ...ConfigOperation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod procedural2d {
        #[path = "../../🗿️artifacts/🌀️procedural2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🌀️procedural2d/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🌀️procedural2d/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🌀️procedural2d/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🌀️procedural2d/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🌀️procedural2d/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🌀️procedural2d/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod procedural3d {
        #[path = "../../🗿️artifacts/🧊️procedural3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🧊️procedural3d/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🧊️procedural3d/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🧊️procedural3d/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🧊️procedural3d/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🧊️procedural3d/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🧊️procedural3d/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod procedural2d {
        #[path = "../../🎛️apps/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/◻2d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/◻2d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/◻2d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/◻2d/🎮️commands/🕸️graph/🦀️component.rs"]
            pub mod graph;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧩️widget/🦀️component.rs"]
            pub mod widget;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧬️generation/🦀️component.rs"]
            pub mod generation;
            #[path = "../../🎛️apps/◻2d/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧮️eval/🦀️component.rs"]
            pub mod eval;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️component.rs"]
                    pub mod flow;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
                }
            }

            #[path = "."]
            pub mod generate {
                #[path = "../../🎛️apps/◻2d/🎭️modes/🧬️generate/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/◻2d/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️component.rs"]
                    pub mod generations;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️component.rs"]
                    pub mod form;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/◻2d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/◻2d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod procedural3d {
        #[path = "../../🎛️apps/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🧊️3d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🧊️3d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🧊️3d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎨️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🕸️graph/🦀️component.rs"]
            pub mod graph;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧩️widget/🦀️component.rs"]
            pub mod widget;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧭️gumball/🦀️component.rs"]
            pub mod gumball;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧬️generation/🦀️component.rs"]
            pub mod generation;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🌞️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧮️eval/🦀️component.rs"]
            pub mod eval;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️component.rs"]
                    pub mod flow;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
                }
            }

            #[path = "."]
            pub mod generate {
                #[path = "../../🎛️apps/🧊️3d/🎭️modes/🧬️generate/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️component.rs"]
                    pub mod generations;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️component.rs"]
                    pub mod form;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🧊️3d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
fn register_procedural_exports() {
    artifacts::procedural2d::engine::register();
    artifacts::procedural3d::engine::register();
}

semio_framework_plugin::semio_plugin! {
    id: "procedural",
    label: "Procedural",
    version: "0.1.0",
    setup: register_procedural_exports,
    apps: [
        apps::procedural2d::create_procedural2d_app => apps::procedural2d::Procedural2dPlayApp,
        apps::procedural3d::create_procedural3d_app => apps::procedural3d::Procedural3dPlayApp,
    ],
}
//#endregion 🔖️Plugin
