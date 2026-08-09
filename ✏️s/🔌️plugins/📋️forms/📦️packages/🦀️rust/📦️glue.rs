//! 📋️ Forms plugin — declarative forms play app bundled as a hot-swappable WASM component.
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
extern crate semio_framework_schema as schema;
extern crate flow;
pub use flow::playbook;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<FormMutation, FormsConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod forms {
        #[path = "../../🗿️artifacts/📋️forms/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📋️forms/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📋️forms/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/📋️forms/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📋️forms/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;
            #[path = "../../🗿️artifacts/📋️forms/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "../../🗿️artifacts/📋️forms/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod add_step {
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/➕add-step/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/➕add-step/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod remove_step {
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/➖remove-step/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/➖remove-step/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod move_step {
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/↔️move-step/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/↔️move-step/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod add_block {
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/➕add-block/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/➕add-block/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod remove_block {
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/➖remove-block/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/➖remove-block/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod move_block {
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/↔️move-block/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/↔️move-block/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod update_block {
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/🩹update-block/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/🩹update-block/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod update_step {
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/🩹update-step/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/🩹update-step/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod update_playbook {
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/📖update-playbook/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📋️forms/🧬️mutations/📖update-playbook/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📋️forms/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📋️forms/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📋️forms/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod forms {
        #[path = "../../🎛️apps/📋️forms/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/📋️forms/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📋️forms/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/📋️forms/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📋️forms/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/📋️forms/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📋️forms/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/📋️forms/🎮️commands/🧪️try/🦀️component.rs"]
            pub mod try_wizard;
            #[path = "../../🎛️apps/📋️forms/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/📋️forms/🎮️commands/🧩️contribution/🦀️component.rs"]
            pub mod contribution;
            #[path = "../../🎛️apps/📋️forms/🎮️commands/📃️step/🦀️component.rs"]
            pub mod step;
            #[path = "../../🎛️apps/📋️forms/🎮️commands/❓️question/🦀️component.rs"]
            pub mod question;
            #[path = "../../🎛️apps/📋️forms/🎮️commands/🔘️option/🦀️component.rs"]
            pub mod option;
            #[path = "../../🎛️apps/📋️forms/🎮️commands/📐️vector/🦀️component.rs"]
            pub mod vector;
            #[path = "../../🎛️apps/📋️forms/🎮️commands/📥️import/🦀️component.rs"]
            pub mod import;
            #[path = "../../🎛️apps/📋️forms/🎮️commands/📤️export/🦀️component.rs"]
            pub mod export;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod blueprint {
                #[path = "../../🎛️apps/📋️forms/🎭️modes/📝️blueprint/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📋️forms/🎭️modes/📝️blueprint/🪟️windows/🧱️builder/🦀️component.rs"]
                    pub mod builder;
                    #[path = "../../🎛️apps/📋️forms/🎭️modes/📝️blueprint/🪟️windows/▶️try/🦀️component.rs"]
                    pub mod try_wizard;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📋️forms/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📋️forms/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📋️forms/📌️panels/🔍️inspection/🦀️component.rs"]
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
    #[path = "../../🗿️artifacts/📋️forms/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_forms_demo;
    #[path = "../../🎛️apps/📋️forms/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_forms_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
