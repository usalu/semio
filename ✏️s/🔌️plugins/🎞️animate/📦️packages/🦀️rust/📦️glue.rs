//! 🎞️ Animate plugin — present tile play app bundled as a hot-swappable WASM component.
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
// `Result<Emit<PresentOperation, PresentConfigOperation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split creates),
// so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod present {
        #[path = "../../🗿️artifacts/🎬️present/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🎬️present/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🎬️present/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🎬️present/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🎬️present/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🎬️present/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

                        #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🎞️animation/🦀️component.rs"]
            pub mod animation;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🎬️scene/🦀️component.rs"]
            pub mod scene;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/📐️geometry/🦀️component.rs"]
            pub mod geometry;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🔤️text/🦀️component.rs"]
            pub mod text;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/⏱️rate/🦀️component.rs"]
            pub mod rate;
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🎛️config/🦀️component.rs"]
            pub mod config;
            pub mod animate {
                pub use super::animation::*;
                pub use super::scene::*;
                pub use super::geometry::*;
                pub use super::camera::*;
                pub use super::text::*;
                pub use super::rate::*;
                pub use super::config::*;
            }
            #[path = "../../🗿️artifacts/🎬️present/⚙️engine/🎥️video/🦀️component.rs"]
            pub mod animate_video;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod present {
        #[path = "../../🎛️apps/🎬️present/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🎬️present/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🎬️present/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🎬️present/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🎬️present/🎮️commands/🀄️tile/🦀️component.rs"]
            pub mod tile;
            #[path = "../../🎛️apps/🎬️present/🎮️commands/🌐️grid/🦀️component.rs"]
            pub mod grid;
            #[path = "../../🎛️apps/🎬️present/🎮️commands/🖼️source/🦀️component.rs"]
            pub mod source;
            #[path = "../../🎛️apps/🎬️present/🎮️commands/⌨️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/🎬️present/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/🎬️present/🎮️commands/🐚️shell/🦀️component.rs"]
            pub mod shell;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod main {
                #[path = "../../🎛️apps/🎬️present/🎭️modes/🖊️main/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🎬️present/🎭️modes/🖊️main/🪟️windows/🖼️tile-editor/🦀️component.rs"]
                    pub mod tile_editor;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🎬️present/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🎬️present/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🎬️present/📌️panels/🔍️inspection/🦀️component.rs"]
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
