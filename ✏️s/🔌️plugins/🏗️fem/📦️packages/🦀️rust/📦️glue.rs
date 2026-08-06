//! 🏗️ FEM plugin — finite-element structural analysis, bundled as a hot-swappable WASM component.
//! Two independent artifacts (`fem2d`, `fem3d`) share one cross-artifact compute kernel (`core`).
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that
//! is written in full, relative to THIS file's directory — `📦️packages/🦀️rust/`, two levels below the
//! owner root the taxonomy tree hangs off, hence every LEAF path's `../../` prefix. The grouping
//! modules carry a bare `#[path = "."]` so their own names are not spliced into that base directory —
//! without it, Rust resolves an inline module's children under `<file dir>/<inline mod name>/…` and
//! every leaf path dangles. A `"."` reset composes against its parent's already-resolved base, never
//! against the raw file directory, so it must NOT carry the `../../` prefix. Do not inline any
//! component file back into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint
//! both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<FemXOperation, FemXConfigOperation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing
// it here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl
// itself (only on the free functions the taxonomy split creates), so this is a pure artefact of
// decomposition.
#![allow(clippy::result_large_err)]

//#region 🫀️Core
#[path = "."]
pub mod core {
    #[path = "../../🫀️core/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🫀️core/🧮️analyses/🦀️component.rs"]
    pub mod analyses;
    #[path = "../../🫀️core/📏️elements2d/🦀️component.rs"]
    pub mod elements2d;
    #[path = "../../🫀️core/🧊️elements3d/🦀️component.rs"]
    pub mod elements3d;
    #[path = "../../🫀️core/➗️formulation/🦀️component.rs"]
    pub mod formulation;
    #[path = "../../🫀️core/🕸️mesh/🦀️component.rs"]
    pub mod mesh;
    #[path = "../../🫀️core/🔢️sparse/🦀️component.rs"]
    pub mod sparse;
    #[path = "../../🫀️core/🤝️shared/🦀️component.rs"]
    pub mod shared;

    /// 🗂️ Registers both artifacts' engines with the host — the cross-artifact combinator the old
    /// bundle crate's `register_fem_exports()` used to be; lives here (not in the plugin root) because
    /// lib.rs is wiring-only.
    pub fn register_all_engines() {
        crate::artifacts::fem2d::engine::register();
        crate::artifacts::fem3d::engine::register();
    }
}
//#endregion 🫀️Core

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod fem2d {
        #[path = "../../🗿️artifacts/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/◻2d/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/◻2d/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/◻2d/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/◻2d/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/◻2d/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🕸️meshing/🦀️component.rs"]
            pub mod meshing;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🎵️modal-buckling/🦀️component.rs"]
            pub mod modal_buckling;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🗺️mesh-preview/🦀️component.rs"]
            pub mod mesh_preview;
        }
    }

    #[path = "."]
    pub mod fem3d {
        #[path = "../../🗿️artifacts/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🧊️3d/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🧊️3d/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🧊️3d/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🧊️3d/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🧊️3d/📡️spr/🦀️component.rs"]
        pub mod spr;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🕸️meshing/🦀️component.rs"]
            pub mod meshing;
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🎵️modal-buckling/🦀️component.rs"]
            pub mod modal_buckling;
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🗺️mesh-preview/🦀️component.rs"]
            pub mod mesh_preview;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod fem2d {
        #[path = "../../🎛️apps/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/◻2d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/◻2d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧱️model/🦀️component.rs"]
            pub mod model;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🏋️loads/🦀️component.rs"]
            pub mod loads;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧮️analysis/🦀️component.rs"]
            pub mod analysis;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/◻2d/🎮️commands/📚️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/◻2d/🎮️commands/👁️results/🦀️component.rs"]
            pub mod results;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️component.rs"]
                    pub mod model;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }
    }

    #[path = "."]
    pub mod fem3d {
        #[path = "../../🎛️apps/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🧊️3d/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🧊️3d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧱️model/🦀️component.rs"]
            pub mod model;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🏋️loads/🦀️component.rs"]
            pub mod loads;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧮️analysis/🦀️component.rs"]
            pub mod analysis;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/📚️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/👁️results/🦀️component.rs"]
            pub mod results;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️component.rs"]
                    pub mod model;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
semio_framework_plugin::semio_plugin! {
    id: "fem", label: "FEM", version: "0.1.0",
    setup: core::register_all_engines,
    apps: [ apps::fem2d::create_fem2d_app => apps::fem2d::Fem2dPlayApp, apps::fem3d::create_fem3d_app => apps::fem3d::Fem3dPlayApp ],
}
//#endregion 🔖️Plugin
