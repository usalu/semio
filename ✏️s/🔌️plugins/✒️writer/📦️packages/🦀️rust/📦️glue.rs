//! ✒️ Writer plugin — declarative writer play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]`
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, per SHAPE V2 —
//! `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST` — so every leaf path carries a `../../` prefix to reach
//! back out to the owner-root tree). The grouping modules carry `#[path = "."]` so their own names
//! are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl_lsp;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<WriterMutation, WriterConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod writer {
        #[path = "../../🗿️artifacts/✒️writer/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/✒️writer/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/✒️writer/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/✒️writer/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_text {
                #[path = "../../🗿️artifacts/✒️writer/🧬️mutations/✍️set-text/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/✒️writer/🧬️mutations/✍️set-text/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/✒️writer/🧬️mutations/✍️set-text/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_document {
                #[path = "../../🗿️artifacts/✒️writer/🧬️mutations/📄set-document/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/✒️writer/🧬️mutations/📄set-document/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/✒️writer/🧬️mutations/📄set-document/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/✒️writer/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/✒️writer/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/✒️writer/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/✒️writer/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod writer {
        #[path = "../../🎛️apps/✒️writer/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/✒️writer/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/✒️writer/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/✒️writer/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/✒️writer/🎮️commands/✍️text/🦀️component.rs"]
            pub mod text;
            #[path = "../../🎛️apps/✒️writer/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/✒️writer/🎮️commands/🔍️inspect/🦀️component.rs"]
            pub mod inspect;
            #[path = "../../🎛️apps/✒️writer/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/✒️writer/🎮️commands/⚙️editor-settings/🦀️component.rs"]
            pub mod editor_settings;
            #[path = "../../🎛️apps/✒️writer/🎮️commands/💬️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/✒️writer/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/✒️writer/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🎛️apps/✒️writer/🎭️modes/✏️edit/🪟️windows/✒️main/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/✒️writer/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️options/🔤️font-size/🦀️component.rs"]
                            pub mod font_size;
                            #[path = "../../🎛️apps/✒️writer/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️options/📏️line-height/🦀️component.rs"]
                            pub mod line_height;
                            #[path = "../../🎛️apps/✒️writer/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️options/⇥️tab-size/🦀️component.rs"]
                            pub mod tab_size;
                            #[path = "../../🎛️apps/✒️writer/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️options/🔢️line-numbers/🦀️component.rs"]
                            pub mod line_numbers;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/✒️writer/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/✒️writer/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/✒️writer/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🌉️WasmBridge
/// 🌉️ `WriterHost` (all targets) plus `WriterSession`/`WriterDocumentVcs` (wasm32 only) — see
/// `apps::writer::wasm` for the individual `cfg` gates; this re-export just surfaces them at the crate
/// root, matching the old bundle crate's `📦️glue.rs` surface.
pub use apps::writer::wasm::*;
//#endregion 🌉️WasmBridge

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/✒️writer/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_writer_demo;
    #[path = "../../🎛️apps/✒️writer/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_writer_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
