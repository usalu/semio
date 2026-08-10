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

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod engine {
                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/⏱️rate/🦀️component.rs"]
                    pub mod rate;
                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎛️config/🦀️component.rs"]
                    pub mod config;
                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎞️animation/🦀️component.rs"]
                    pub mod animation;
                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎥️camera/🦀️component.rs"]
                    pub mod camera;
                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️component.rs"]
                    pub mod video;
                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎬️scene/🦀️component.rs"]
                    pub mod scene;
                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/📐️geometry/🦀️component.rs"]
                    pub mod geometry;
                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🔤️text/🦀️component.rs"]
                    pub mod text;
                    pub mod animate {
                pub mod animation {
                    pub use super::super::animation::animation::*;
                    pub use super::super::animation::animations_catalog::*;
                }
                pub mod rate { pub use super::super::rate::rate::*; }
                pub mod updater { pub use super::super::rate::updater::*; }
                pub mod scene { pub use super::super::scene::scene::*; }
                pub mod section { pub use super::super::scene::section::*; }
                pub mod sobject { pub use super::super::scene::sobject::*; }
                pub mod geometry {
                    pub use super::super::geometry::geometry::*;
                    pub use super::super::geometry::three_d::*;
                    pub use super::super::geometry::axes::*;
                }
                pub mod camera {
                    pub use super::super::camera::camera::*;
                    pub use super::super::camera::matrix::*;
                }
                pub mod color { pub use super::super::text::color::*; }
                pub mod text { pub use super::super::text::text::*; }
                pub mod config {
                    pub use super::super::config::config::*;
                    pub use super::super::config::hash::*;
                    pub use super::super::config::graph::*;
                }
                pub use super::config::config::{AnimateConfig, QualityPreset};
                pub use super::scene::section::{Section, SectionList};
                pub use super::camera::camera::Camera;
                pub use super::scene::sobject::{Sobject, VSobject};
                pub use super::text::color::Color;
                pub use super::scene::scene::{BasicStage, Scene, SceneFrame, preview_scene_loop};
                pub use super::animation::animation::{compile_animations, interpolate_at, Animation, Wait};
            }
                    pub use video as animate_video;
                }
                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod tiles {
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞tiles/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞tiles/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞tiles/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_tiles {
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋set-tiles/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋set-tiles/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋set-tiles/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_source {
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎set-source/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎set-source/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎set-source/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📸set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod pptx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod pptx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::present::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::present::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::present::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::present::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::present::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::present::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::present::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::present::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::present::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::present::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::present::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::present::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::present::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::present::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::present::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::present::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::present::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::present::standards::v1::subsets::any::schema::snapshot::binary::*; } }

        #[path = "../../🗿️artifacts/🎬️present/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🎬️present/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/🎬️present/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎬️present/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
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

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🎬️present/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🎬️present/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🎬️present/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🎬️present/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
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
#[path = "../../🦀️component.rs"]
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
