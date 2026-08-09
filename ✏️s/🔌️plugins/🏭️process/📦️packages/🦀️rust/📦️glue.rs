//! 🏭️ Process plugin — subtractive/additive processing simulation bundled as a hot-swappable WASM
//! component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_schema as schema;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split creates),
// so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod process3d {
        #[path = "../../🗿️artifacts/🧊️process3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🧊️process3d/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🧊️process3d/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🧊️process3d/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }
        #[path = "../../🗿️artifacts/🧊️process3d/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod steps {
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/📋steps/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/📋steps/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/📋steps/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod machines {
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/🛠️machines/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/🛠️machines/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/🛠️machines/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_stock {
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/🧱set-stock/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/🧱set-stock/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/🧱set-stock/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_cursor {
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/⏱️set-cursor/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/⏱️set-cursor/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/⏱️set-cursor/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🧊️process3d/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

        }

        #[path = "../../🗿️artifacts/🧊️process3d/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🧊️process3d/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/🧊️process3d/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }
        #[path = "../../🗿️artifacts/🧊️process3d/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🧊️process3d/⚙️engine/🪵️wood/🦀️component.rs"]
            pub mod wood;
            #[path = "../../🗿️artifacts/🧊️process3d/⚙️engine/🧱️concrete/🦀️component.rs"]
            pub mod concrete;
            #[path = "../../🗿️artifacts/🧊️process3d/⚙️engine/🔩️metal/🦀️component.rs"]
            pub mod metal;
            #[path = "../../🗿️artifacts/🧊️process3d/⚙️engine/🤖️robotic/🦀️component.rs"]
            pub mod robotic;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod process3d {
        #[path = "../../🎛️apps/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🧊️3d/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🧊️3d/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🧊️3d/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/🧊️3d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🧊️3d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🪜️step/🦀️component.rs"]
            pub mod step;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🛠️workshop/🦀️component.rs"]
            pub mod workshop;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🪵️stock/🦀️component.rs"]
            pub mod stock;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🔎️inspector/🦀️component.rs"]
            pub mod inspector;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/⏱️cursor/🦀️component.rs"]
            pub mod cursor;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎛️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🌍️world/🦀️component.rs"]
            pub mod world;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/☀️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧩️contribution/🦀️component.rs"]
            pub mod contribution;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/📤️media/🦀️component.rs"]
            pub mod media;
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
                    #[path = "."]
                    pub mod workpiece {
                        #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🎚️options/☀️sun/🦀️component.rs"]
                            pub mod sun;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🧊️3d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🛠️workshop/🦀️component.rs"]
            pub mod workshop;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🔍️inspection/🦀️component.rs"]
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
    #[path = "../../🗿️artifacts/🧊️process3d/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_process3d_demo;
    #[path = "../../🎛️apps/🧊️3d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_3d_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
