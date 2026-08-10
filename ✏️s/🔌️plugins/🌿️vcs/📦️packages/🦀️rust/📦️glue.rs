//! 🌿️ VCS plugin — declarative version-control play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as vcs_kernel;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned
// error type; boxing it here would diverge from the trait it must satisfy, and the lint does not fire on
// the trait impl itself (only on the free functions the taxonomy split creates), so this is a pure
// artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod vcs {
        #[path = "../../🗿️artifacts/🌿️vcs/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::vcs::schema::snapshot::VcsSnapshot;
        pub use crate::artifacts::vcs::schema::mutations::VcsDemoMutation;
        pub use crate::artifacts::vcs::schema::diff::VcsDiff;

        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                pub use text::*;
                #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod add_tag {
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/🏷️add-tag/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/🏷️add-tag/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_title {
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/📛set-title/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/📛set-title/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_notes {
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/📝set-notes/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/📝set-notes/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_counter {
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/🔢set-counter/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/🔢set-counter/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/🔢set-counter/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_tag {
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/🗑️remove-tag/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/🗑️remove-tag/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_status {
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/🚦set-status/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🌿️vcs/🧬️schema/🧬️mutations/🚦set-status/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }

        pub mod op { pub use crate::artifacts::vcs::schema::mutations::text::*; pub use crate::artifacts::vcs::schema::mutations::VcsDemoMutation; }
        pub mod dsl { pub use crate::artifacts::vcs::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::vcs::schema::mutations::binary::*; }
        pub mod pack { pub use crate::artifacts::vcs::schema::snapshot::binary::*; }
        pub mod diff { pub use crate::artifacts::vcs::schema::diff::*; pub mod schema { pub use crate::artifacts::vcs::schema::diff::*; } pub mod text { pub use crate::artifacts::vcs::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::vcs::schema::mutations::*; }
        #[path = "."]
        pub mod snapshot {
            pub mod schema { pub use crate::artifacts::vcs::schema::snapshot::*; }
            pub mod pack { pub use crate::artifacts::vcs::schema::snapshot::binary::*; }
        }
        #[path = "../../🗿️artifacts/🌿️vcs/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🌿️vcs/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🌿️vcs/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🌿️vcs/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🌿️vcs/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod xlsx {
                            #[path = "../../🗿️artifacts/🌿️vcs/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🌿️vcs/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
                            mod component;
                            pub use component::*;
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
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🌿️vcs/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🌿️vcs/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod xlsx {
                            #[path = "../../🗿️artifacts/🌿️vcs/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🌿️vcs/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::vcs::io::export::serializers::artifacts::csv::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::vcs::io::import::deserializers::artifacts::csv::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::vcs::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::vcs::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::vcs::io::export::serializers::artifacts::xlsx::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::vcs::io::import::deserializers::artifacts::xlsx::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::vcs::io::export::serializers::artifacts::zip::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::vcs::io::import::deserializers::artifacts::zip::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/🌿️vcs/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}

//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod vcs {
        #[path = "../../🎛️apps/🌿️vcs/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🌿️vcs/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🌿️vcs/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🌿️vcs/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🌿️vcs/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/🌿️vcs/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🌿️vcs/🎮️commands/📈️counter/🦀️component.rs"]
            pub mod counter;
            #[path = "../../🎛️apps/🌿️vcs/🎮️commands/🩹️patch/🦀️component.rs"]
            pub mod patch;
            #[path = "../../🎛️apps/🌿️vcs/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🌿️vcs/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/🌿️vcs/🎮️commands/🖱️canvas/🦀️component.rs"]
            pub mod canvas;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🌿️vcs/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🌿️vcs/🎭️modes/✏️edit/🪟️windows/📝️editor/🦀️component.rs"]
                    pub mod editor;
                    #[path = "../../🎛️apps/🌿️vcs/🎭️modes/✏️edit/🪟️windows/📜️history/🦀️component.rs"]
                    pub mod history;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🌿️vcs/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🌿️vcs/📌️panels/🔍️inspection/🦀️component.rs"]
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
    #[path = "../../🗿️artifacts/🌿️vcs/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_vcs_demo;
    #[path = "../../🎛️apps/🌿️vcs/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_vcs_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
