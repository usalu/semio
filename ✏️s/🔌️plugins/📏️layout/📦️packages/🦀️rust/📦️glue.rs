//! 📏️ Layout plugin — blueprint/preview document editor bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, two levels deeper than the
//! owner root under Shape V2 — hence every leaf path below is prefixed `../../` to reach back out). The
//! grouping modules carry `#[path = "."]` so their own names are not spliced into that base directory —
//! without it, Rust resolves an inline module's children under `<file dir>/<inline mod name>/…` and every
//! leaf path dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).
extern crate semio_framework_schema as schema;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod layout {
        #[path = "../../🗿️artifacts/📏️layout/🦀️component.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/📏️layout/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/📏️layout/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/📏️layout/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod add_frame {
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/➕add-frame/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/➕add-frame/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/➕add-frame/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_frame {
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/➖remove-frame/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/➖remove-frame/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/➖remove-frame/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod pages {
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/📄pages/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/📄pages/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/📄pages/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod stories {
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/📖stories/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/📖stories/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/📖stories/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod links {
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/🔗links/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/🔗links/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/🔗links/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_data_fields {
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/🧾set-data-fields/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/🧾set-data-fields/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/🧾set-data-fields/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod patch_frame {
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/🩹patch-frame/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/🩹patch-frame/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🧬️mutations/🩹patch-frame/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::layout::schema::mutations::text::*; pub use crate::artifacts::layout::schema::mutations::{apply_layout_mutation, LayoutMutation}; }
        pub mod dsl { pub use crate::artifacts::layout::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::layout::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::layout::schema::diff::text::*; pub use crate::artifacts::layout::schema::diff::*; pub mod schema { pub use crate::artifacts::layout::schema::diff::*; } pub mod text { pub use crate::artifacts::layout::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::layout::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::layout::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::layout::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/📏️layout/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📏️layout/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📏️layout/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod dwg {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod dxf {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dxf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod pdf {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
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
                        pub mod dwg {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod dxf {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod pdf {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/📏️layout/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::layout::io::export::serializers::artifacts::dwg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::layout::io::import::deserializers::artifacts::dwg::*;
                }
            }
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::layout::io::export::serializers::artifacts::dxf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::layout::io::import::deserializers::artifacts::dxf::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::layout::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::layout::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::layout::io::export::serializers::artifacts::pdf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::layout::io::import::deserializers::artifacts::pdf::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::layout::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::layout::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::layout::io::export::serializers::artifacts::svg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::layout::io::import::deserializers::artifacts::svg::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/📏️layout/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/📏️layout/⚙️engine/🎬️scene/🦀️component.rs"]
            pub mod scene;
        }
    }
}

//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod layout {
        #[path = "../../🎛️apps/📏️layout/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/📏️layout/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📏️layout/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/📏️layout/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📏️layout/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/📏️layout/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/📏️layout/🖼️canvas/🦀️component.rs"]
        pub mod canvas;
        #[path = "../../🎛️apps/📏️layout/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📏️layout/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/📏️layout/🎮️commands/🖱️pointer/🦀️component.rs"]
            pub mod pointer;
            #[path = "../../🎛️apps/📏️layout/🎮️commands/✏️author/🦀️component.rs"]
            pub mod author;
            #[path = "../../🎛️apps/📏️layout/🎮️commands/🐚️export/🦀️component.rs"]
            pub mod export;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📏️layout/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📏️layout/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🦀️component.rs"]
                    pub mod blueprint;
                    #[path = "../../🎛️apps/📏️layout/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs"]
                    pub mod preview;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📏️layout/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📏️layout/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📏️layout/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
            #[path = "../../🎛️apps/📏️layout/📌️panels/🚦️preflight/🦀️component.rs"]
            pub mod preflight;
        }
    }
}
//#endregion 🎛️Apps

//#region 🕸️Wasm
#[cfg(target_arch = "wasm32")]
pub use apps::layout::wasm::LayoutSession;
//#endregion 🕸️Wasm

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/📏️layout/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_layout_demo;
    #[path = "../../🎛️apps/📏️layout/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_layout_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
