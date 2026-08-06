//! 📋️ Forms plugin — declarative forms play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<FormOperation, FormsConfigOperation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod forms {
        #[path = "../../🗿️artifacts/📋️forms/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📋️forms/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📋️forms/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📋️forms/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📋️forms/🎒️pack/🦀️component.rs"]
        pub mod pack;
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

        #[path = "../../🎛️apps/📋️forms/🎚️config/🦀️component.rs"]
        pub mod config;
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
semio_framework_plugin::semio_plugin! {
    id: "forms", label: "Forms", version: "0.1.0",
    setup: artifacts::forms::engine::register,
    apps: [ apps::forms::create_forms_app => apps::forms::FormsPlayApp ],
}
//#endregion 🔖️Plugin
