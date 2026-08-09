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
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<PresentMutation, PresentConfigMutation>, Fault>`, the exact signature
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

        #[path = "../../🗿️artifacts/🎬️present/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🎬️present/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/🎬️present/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🎬️present/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/🎬️present/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "../../🗿️artifacts/🎬️present/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod tiles {
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/🎞tiles/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/🎞tiles/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/🎞tiles/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod set_source {
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/📎set-source/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/📎set-source/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/📎set-source/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod set_tiles {
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/📋set-tiles/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/📋set-tiles/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/📋set-tiles/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/📸set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/📸set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/🎬️present/🧬️mutations/📸set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
        }

        #[path = "../../🗿️artifacts/🎬️present/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
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
                pub mod animation {
                    pub use super::super::animation::animation::*;
                }
                pub mod animations_catalog {
                    pub use super::super::animation::animations_catalog::*;
                }
                pub mod scene {
                    pub use super::super::scene::scene::*;
                }
                pub mod section {
                    pub use super::super::scene::section::*;
                }
                pub mod sobject {
                    pub use super::super::scene::sobject::*;
                }
                pub mod geometry {
                    pub use super::super::geometry::geometry::*;
                }
                pub mod three_d {
                    pub use super::super::geometry::three_d::*;
                }
                pub mod axes {
                    pub use super::super::geometry::axes::*;
                }
                pub mod camera {
                    pub use super::super::camera::camera::*;
                }
                pub mod matrix {
                    pub use super::super::camera::matrix::*;
                }
                pub mod text {
                    pub use super::super::text::text::*;
                }
                pub mod color {
                    pub use super::super::text::color::*;
                }
                pub mod rate {
                    pub use super::super::rate::rate::*;
                }
                pub mod updater {
                    pub use super::super::rate::updater::*;
                }
                pub mod config {
                    pub use super::super::config::config::*;
                }
                pub mod hash {
                    pub use super::super::config::hash::*;
                }
                pub mod graph {
                    pub use super::super::config::graph::*;
                }
                pub use super::config::config::{AnimateConfig, QualityPreset};
                pub use super::scene::section::Section;
                pub use super::scene::scene::{Scene, SceneFrame, preview_scene_loop};
                pub use super::scene::section::SectionList;
                pub use super::animation::animation::{compile_animations, interpolate_at, Animation, Wait};
                pub use super::scene::sobject::{Sobject, VSobject};
                pub use super::camera::camera::Camera;
                pub use super::text::color::Color;
                pub use super::scene::scene::BasicStage;
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

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🎬️present/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_present_demo;
    #[path = "../../🎛️apps/🎬️present/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_present_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
