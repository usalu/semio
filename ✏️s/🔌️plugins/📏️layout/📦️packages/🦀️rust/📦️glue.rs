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

        #[path = "../../🗿️artifacts/📏️layout/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📏️layout/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📏️layout/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }
        #[path = "../../🗿️artifacts/📏️layout/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod pages {
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/📄pages/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/📄pages/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/📄pages/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod stories {
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/📖stories/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/📖stories/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/📖stories/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod links {
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/🔗links/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/🔗links/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/🔗links/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod add_frame {
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/➕add-frame/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/➕add-frame/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/➕add-frame/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod remove_frame {
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/➖remove-frame/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/➖remove-frame/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/➖remove-frame/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod patch_frame {
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/🩹patch-frame/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/🩹patch-frame/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/🩹patch-frame/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod set_data_fields {
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/🧾set-data-fields/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/🧾set-data-fields/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📏️layout/🧬️mutations/🧾set-data-fields/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
        }

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📏️layout/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/📏️layout/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "../../🗿️artifacts/📏️layout/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📏️layout/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📏️layout/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/🖊️dwg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/🖊️dwg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/🖊️dxf/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/🖊️dxf/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/📄️pdf/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/📄️pdf/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/📷️png/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/📷️png/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/🎨️svg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📏️layout/🚪️io/🎨️svg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
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
