//! 📖️ Playbook plugin — declarative Blockly-like builder play app bundled as a hot-swappable WASM
//! component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to the owner root (this file itself now lives two levels deeper, in
//! `📦️packages/🦀️rust/`, so every path carries a `../../` prefix back out to the owner root). The
//! grouping modules carry `#[path = "."]` so their own names are not spliced into that base
//! directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).
//!
//! Playbook's document/operation domain (steps, blocks, generation forms, and their `dsl`/`pack`/
//! `spr` derive impls) is owned by the FRAMEWORK kernel crate `playbook`
//! (`semio-framework-os-kernel-playbook`, untouched by this migration) — the same domain `📋️forms`
//! builds on. This plugin owns only its own document schema id, `ArtifactKindSpec`, `PlaybookConfig`/
//! `PlaybookConfigMutation`/`PlaybookCommand`, and the `PlaybookPlayApp` `DocumentApp` impl.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework as semio_framework;
extern crate flow;
pub use flow::playbook;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned
// error type; boxing it here would diverge from the trait it must satisfy, and the lint does not fire on
// the trait impl itself (only on the free functions the taxonomy split creates), so this is a pure
// artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod playbook {
        #[path = "../../🗿️artifacts/📖️playbook/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📖️playbook/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📖️playbook/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/📖️playbook/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📖️playbook/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/📖️playbook/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }
        #[path = "../../🗿️artifacts/📖️playbook/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod add_step {
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➕add-step/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➕add-step/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➕add-step/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod remove_step {
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➖remove-step/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➖remove-step/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➖remove-step/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod move_step {
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/↔️move-step/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/↔️move-step/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/↔️move-step/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod add_block {
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➕add-block/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➕add-block/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➕add-block/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod remove_block {
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➖remove-block/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➖remove-block/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/➖remove-block/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod move_block {
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/↔️move-block/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/↔️move-block/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/↔️move-block/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod update_block {
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/🩹update-block/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/🩹update-block/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/🩹update-block/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod update_step {
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/🩹update-step/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/🩹update-step/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/🩹update-step/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod update_playbook {
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/📖update-playbook/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/📖update-playbook/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📖️playbook/🧬️mutations/📖update-playbook/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
        }

        #[path = "../../🗿️artifacts/📖️playbook/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📖️playbook/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📖️playbook/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod docx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📖️playbook/🚪️io/📜️docx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📖️playbook/🚪️io/📜️docx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📖️playbook/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📖️playbook/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod md {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📖️playbook/🚪️io/📝️md/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📖️playbook/🚪️io/📝️md/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📖️playbook/🚪️io/📄️pdf/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📖️playbook/🚪️io/📄️pdf/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod txt {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📖️playbook/🚪️io/📄txt/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📖️playbook/🚪️io/📄txt/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📖️playbook/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod playbook {
        #[path = "../../🎛️apps/📖️playbook/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/📖️playbook/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📖️playbook/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/📖️playbook/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📖️playbook/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/📖️playbook/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📖️playbook/🎮️commands/🪜️step/🦀️component.rs"]
            pub mod step;
            #[path = "../../🎛️apps/📖️playbook/🎮️commands/🧱️block/🦀️component.rs"]
            pub mod block;
            #[path = "../../🎛️apps/📖️playbook/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/📖️playbook/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/📖️playbook/🎮️commands/🧩️contribution/🦀️component.rs"]
            pub mod contribution;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod builder {
                #[path = "../../🎛️apps/📖️playbook/🎭️modes/🏗️builder/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📖️playbook/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️component.rs"]
                    pub mod builder;
                }
            }
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
    #[path = "../../🗿️artifacts/📖️playbook/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_playbook_demo;
    #[path = "../../🎛️apps/📖️playbook/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_playbook_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
