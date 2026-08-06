//! 🌐️ GIS plugin — 2D map + 3D terrain apps bundled as one hot-swappable WASM component.
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
    pub mod gismap {
        #[path = "../../🗿️artifacts/🗺️gismap/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🗺️gismap/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🗺️gismap/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🗺️gismap/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🗺️gismap/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🗺️gismap/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🗺️gismap/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod gisterrain {
        #[path = "../../🗿️artifacts/🏔️gisterrain/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🏔️gisterrain/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🏔️gisterrain/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🏔️gisterrain/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🏔️gisterrain/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🏔️gisterrain/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🏔️gisterrain/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod gis2d {
        #[path = "../../🎛️apps/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/◻2d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/◻2d/🗺️maphost/🦀️component.rs"]
        pub mod maphost;
        #[path = "../../🎛️apps/◻2d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/◻2d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗺️features/🦀️component.rs"]
            pub mod features;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/◻2d/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🌐️shell/🦀️component.rs"]
            pub mod shell;
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
                    #[path = "."]
                    pub mod map {
                        #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🖼️render-mode/🦀️component.rs"]
                            pub mod render_mode;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🎨️vector-style/🦀️component.rs"]
                            pub mod vector_style;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🔽️lod-mode/🦀️component.rs"]
                            pub mod lod_mode;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/🖱️selection-method/🦀️component.rs"]
                            pub mod selection_method;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/👁️layers/🦀️component.rs"]
                            pub mod layers;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️options/📏️layer-weights/🦀️component.rs"]
                            pub mod layer_weights;
                        }
                    }
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
    pub mod gis3d {
        #[path = "../../🎛️apps/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🧊️3d/🎚️config/🦀️component.rs"]
        pub mod config;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🏔️exaggeration/🦀️component.rs"]
            pub mod exaggeration;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🎛️apps/🧊️3d/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/👁️view/🪟️windows/🏔️terrain/🦀️component.rs"]
                    pub mod terrain;
                }
            }
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
fn register_gis_exports() {
    artifacts::gismap::engine::register();
    artifacts::gisterrain::engine::register();
}

semio_framework_plugin::semio_plugin! {
    id: "gis",
    label: "GIS",
    version: "0.1.0",
    setup: register_gis_exports,
    apps: [
        apps::gis2d::create_gis2d_app => apps::gis2d::Gis2dPlayApp,
        apps::gis3d::create_gis3d_app => apps::gis3d::Gis3dPlayApp,
    ],
}
//#endregion 🔖️Plugin
