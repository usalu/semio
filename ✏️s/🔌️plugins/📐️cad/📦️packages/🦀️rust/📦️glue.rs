//! 📐️ CAD plugin — the spatial-model play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that
//! is written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, Shape V2 — see ticket
//! `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST`), prefixed with `../../` to reach back up to the plugin
//! root. The grouping modules carry `#[path = "."]` so their own names are not spliced into that base
//! directory — without it, Rust resolves an inline module's children under
//! `<file dir>/<inline mod name>/…` and every leaf path dangles. Do not inline any component file back
//! into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint both fail on it (see
//! master ticket `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard
//! ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<CadOperation, CadConfigOperation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing
// it here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl
// itself (only on the free functions the taxonomy split creates), so this is a pure artefact of
// decomposition.
#![allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod cad {
        #[path = "../../🗿️artifacts/📐️cad/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs"]
        mod interaction_spec;
        pub use interaction_spec::*;

        #[path = "../../🗿️artifacts/📐️cad/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📐️cad/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📐️cad/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📐️cad/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📐️cad/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/📥️geometry-import/🦀️component.rs"]
            pub mod geometry_import;
            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🔄️transformation/🦀️component.rs"]
            pub mod transformation;
            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🕹️interaction/🦀️component.rs"]
            pub mod interaction;
            #[path = "../../🗿️artifacts/📐️cad/⚙️engine/🔍️construct/🦀️component.rs"]
            pub mod construct;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod cad {
        #[path = "../../🎛️apps/📐️cad/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/📐️cad/🧮️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/📐️cad/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/📐️cad/🕸️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🧱️object/🦀️component.rs"]
            pub mod object;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🕸️node/🦀️component.rs"]
            pub mod node;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🔄️transform/🦀️component.rs"]
            pub mod transform;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/📥️io/🦀️component.rs"]
            pub mod io;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🖼️reference/🦀️component.rs"]
            pub mod reference;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🤝️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🌞️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/📐️cad/🎮️commands/🗺️model-definition/🦀️component.rs"]
            pub mod model_definition;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📐️cad/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📐️cad/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📐️cad/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🎚️options/🎥️projection/🦀️component.rs"]
                    pub mod projection;
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🎚️options/🌞️sun/🦀️component.rs"]
                    pub mod sun;
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🎚️options/🕹️dislocate/🦀️component.rs"]
                    pub mod dislocate;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🪟️windows/📐️shape/🦀️component.rs"]
                    pub mod shape;
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🪟️windows/🏢️building/🦀️component.rs"]
                    pub mod building;
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🪟️windows/🔥️energy/🦀️component.rs"]
                    pub mod energy;
                    #[path = "../../🎛️apps/📐️cad/🎭️modes/✏️edit/🪟️windows/🏛️structure-classic/🦀️component.rs"]
                    pub mod structure_classic;
                }
            }
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
semio_framework_plugin::semio_plugin! {
    id: "cad", label: "CAD", version: "0.1.0",
    setup: artifacts::cad::engine::register,
    apps: [ apps::cad::create_cad_app => apps::cad::CadPlayApp ],
}
//#endregion 🔖️Plugin
