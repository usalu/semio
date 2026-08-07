//! 🎥️ Shooting plugin — icon-studio play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<ShootingOperation, ShootingConfigOperation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split
// creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod shooting {
        #[path = "../../🗿️artifacts/🎥️shooting/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🎥️shooting/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🎥️shooting/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🎥️shooting/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🎥️shooting/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🎥️shooting/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🎥️shooting/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod shooting {
        #[path = "../../🎛️apps/🎥️shooting/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🎥️shooting/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🎥️shooting/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🎥️shooting/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🗃️fixture/🦀️component.rs"]
            pub mod fixture;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/📷️shot/🦀️component.rs"]
            pub mod shot;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/📦️asset/🦀️component.rs"]
            pub mod asset;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/☀️scene/🦀️component.rs"]
            pub mod scene;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🧭️gumball/🦀️component.rs"]
            pub mod gumball;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/🎥️shooting/🎮️commands/🖨️export/🦀️component.rs"]
            pub mod export;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod scene {
                        #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🎯️center-model/🦀️component.rs"]
                            pub mod center_model;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/☀️sun-enabled/🦀️component.rs"]
                            pub mod sun_enabled;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🧭️sun-azimuth/🦀️component.rs"]
                            pub mod sun_azimuth;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/📐️sun-elevation/🦀️component.rs"]
                            pub mod sun_elevation;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/💡️sun-intensity/🦀️component.rs"]
                            pub mod sun_intensity;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🌫️ambient/🦀️component.rs"]
                            pub mod ambient;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/🌑️shadow/🦀️component.rs"]
                            pub mod shadow;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🎥️scene/🎚️options/✨️roughness/🦀️component.rs"]
                            pub mod roughness;
                        }
                    }

                    #[path = "."]
                    pub mod icon {
                        #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🖼️icon/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🖼️icon/🎚️options/📷️shot/🦀️component.rs"]
                            pub mod shot;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🖼️icon/🎚️options/🗂️format/🦀️component.rs"]
                            pub mod format;
                            #[path = "../../🎛️apps/🎥️shooting/🎭️modes/✏️edit/🪟️windows/🖼️icon/🎚️options/🔷️shape/🦀️component.rs"]
                            pub mod shape;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🎥️shooting/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🎥️shooting/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🎥️shooting/📌️panels/🔍️inspection/🦀️component.rs"]
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
    #[path = "../../🗿️artifacts/🎥️shooting/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_shooting_demo;
    #[path = "../../🎛️apps/🎥️shooting/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_shooting_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
