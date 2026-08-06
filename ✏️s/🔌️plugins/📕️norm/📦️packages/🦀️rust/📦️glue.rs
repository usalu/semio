//! 📏️ Norm plugin — fifteen compliance-standard document apps (DIN 4108, DIN EN 16798, DIN V 18599,
//! EN 1990–1999, ISO 16757, VDI 3805) in one hot-swappable WASM plugin, each backed by a headless
//! `NormHost` that recomputes its `CheckReport` from the document on every read.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).
//!
//! 🫀️ `core` is unusually large for a plugin kernel here, and deliberately so: the fifteen standards are
//! structurally identical apps over fifteen genuinely different document schemas, so the *domain* kernel
//! (quantities, clause identity, check results, national annexes, the `NormFamily`/`NormHost` contract,
//! the generic whole-document operation and its text/binary codecs) and the *app-surface* kernel (the one
//! shared config, the media ports, the render primitives, the manifest constructors) each exist exactly
//! once, while every per-standard fact — schema, ids, labels, compute — lives in that standard's own
//! artifact and app nodes.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<Operation, NormConfigOperation>, Fault>`, the exact signature `DocumentApp::handle` and
// `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it here
// would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself (only
// on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🫀️Core
/// 🤝️ The cross-artifact, cross-app kernel: the norm domain model plus everything all fifteen apps
/// share verbatim. Depends on no artifact and on no app.
#[path = "."]
pub mod core {
    #[path = "../../🫀️core/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🫀️core/🎚️config/🦀️component.rs"]
    mod config;
    pub use config::*;

    #[path = "../../🫀️core/🖥️app-surface/🦀️component.rs"]
    pub mod app;
}
//#endregion 🫀️Core

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod din4108 {
        #[path = "../../🗿️artifacts/📕️din4108/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📕️din4108/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📕️din4108/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📕️din4108/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📕️din4108/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📕️din4108/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📕️din4108/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod din16798 {
        #[path = "../../🗿️artifacts/📗️din16798/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📗️din16798/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📗️din16798/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📗️din16798/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📗️din16798/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📗️din16798/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📗️din16798/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod din18599 {
        #[path = "../../🗿️artifacts/📙️din18599/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📙️din18599/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📙️din18599/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📙️din18599/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📙️din18599/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📙️din18599/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📙️din18599/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1990 {
        #[path = "../../🗿️artifacts/📘️en1990/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1990/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📘️en1990/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1990/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1990/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📘️en1990/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📘️en1990/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1991 {
        #[path = "../../🗿️artifacts/📘️en1991/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1991/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📘️en1991/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1991/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1991/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📘️en1991/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📘️en1991/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1992 {
        #[path = "../../🗿️artifacts/📘️en1992/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1992/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📘️en1992/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1992/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1992/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📘️en1992/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📘️en1992/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1993 {
        #[path = "../../🗿️artifacts/📘️en1993/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1993/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📘️en1993/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1993/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1993/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📘️en1993/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📘️en1993/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1994 {
        #[path = "../../🗿️artifacts/📘️en1994/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1994/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📘️en1994/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1994/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1994/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📘️en1994/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📘️en1994/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1995 {
        #[path = "../../🗿️artifacts/📘️en1995/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1995/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📘️en1995/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1995/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1995/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📘️en1995/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📘️en1995/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1996 {
        #[path = "../../🗿️artifacts/📘️en1996/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1996/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📘️en1996/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1996/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1996/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📘️en1996/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📘️en1996/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1997 {
        #[path = "../../🗿️artifacts/📘️en1997/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1997/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📘️en1997/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1997/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1997/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📘️en1997/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📘️en1997/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1998 {
        #[path = "../../🗿️artifacts/📘️en1998/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1998/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📘️en1998/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1998/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1998/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📘️en1998/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📘️en1998/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1999 {
        #[path = "../../🗿️artifacts/📘️en1999/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1999/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📘️en1999/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1999/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1999/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📘️en1999/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📘️en1999/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod iso16757 {
        #[path = "../../🗿️artifacts/📓️iso16757/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📓️iso16757/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📓️iso16757/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📓️iso16757/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📓️iso16757/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📓️iso16757/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📓️iso16757/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod vdi3805 {
        #[path = "../../🗿️artifacts/📔️vdi3805/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📔️vdi3805/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📔️vdi3805/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📔️vdi3805/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📔️vdi3805/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📔️vdi3805/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📔️vdi3805/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod din4108 {
        #[path = "../../🎛️apps/📕️din4108/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📕️din4108/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📕️din4108/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📕️din4108/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📕️din4108/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📕️din4108/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📕️din4108/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📕️din4108/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📕️din4108/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📕️din4108/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod din16798 {
        #[path = "../../🎛️apps/📗️din16798/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📗️din16798/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📗️din16798/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📗️din16798/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📗️din16798/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📗️din16798/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📗️din16798/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📗️din16798/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📗️din16798/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📗️din16798/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod din18599 {
        #[path = "../../🎛️apps/📙️din18599/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📙️din18599/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📙️din18599/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📙️din18599/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📙️din18599/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📙️din18599/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📙️din18599/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📙️din18599/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📙️din18599/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📙️din18599/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1990 {
        #[path = "../../🎛️apps/📘️en1990/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1990/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📘️en1990/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1990/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1990/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1990/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1990/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1990/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1990/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1990/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1991 {
        #[path = "../../🎛️apps/📘️en1991/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1991/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📘️en1991/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1991/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1991/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1991/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1991/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1991/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1991/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1991/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1992 {
        #[path = "../../🎛️apps/📘️en1992/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1992/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📘️en1992/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1992/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1992/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1992/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1992/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1992/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1992/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1992/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1993 {
        #[path = "../../🎛️apps/📘️en1993/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1993/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📘️en1993/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1993/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1993/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1993/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1993/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1993/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1993/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1993/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1994 {
        #[path = "../../🎛️apps/📘️en1994/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1994/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📘️en1994/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1994/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1994/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1994/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1994/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1994/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1994/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1994/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1995 {
        #[path = "../../🎛️apps/📘️en1995/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1995/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📘️en1995/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1995/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1995/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1995/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1995/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1995/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1995/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1995/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1996 {
        #[path = "../../🎛️apps/📘️en1996/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1996/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📘️en1996/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1996/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1996/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1996/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1996/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1996/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1996/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1996/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1997 {
        #[path = "../../🎛️apps/📘️en1997/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1997/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📘️en1997/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1997/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1997/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1997/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1997/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1997/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1997/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1997/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1998 {
        #[path = "../../🎛️apps/📘️en1998/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1998/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📘️en1998/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1998/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1998/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1998/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1998/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1998/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1998/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1998/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod en1999 {
        #[path = "../../🎛️apps/📘️en1999/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📘️en1999/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📘️en1999/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📘️en1999/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📘️en1999/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📘️en1999/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📘️en1999/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📘️en1999/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📘️en1999/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📘️en1999/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod iso16757 {
        #[path = "../../🎛️apps/📓️iso16757/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📓️iso16757/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📓️iso16757/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📓️iso16757/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📓️iso16757/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📓️iso16757/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📓️iso16757/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📓️iso16757/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📓️iso16757/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📓️iso16757/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod vdi3805 {
        #[path = "../../🎛️apps/📔️vdi3805/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📔️vdi3805/🎮️commands/📤️set-document/🦀️component.rs"]
            pub mod set_document;
            #[path = "../../🎛️apps/📔️vdi3805/🎮️commands/🧮️evaluate/🦀️component.rs"]
            pub mod evaluate;
            #[path = "../../🎛️apps/📔️vdi3805/🎮️commands/☑️selected-check/🦀️component.rs"]
            pub mod selected_check;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📔️vdi3805/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📔️vdi3805/🎭️modes/✏️edit/🪟️windows/📥️inputs/🦀️component.rs"]
                    pub mod inputs;
                    #[path = "../../🎛️apps/📔️vdi3805/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
                    pub mod results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📔️vdi3805/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📔️vdi3805/📌️panels/📚️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📔️vdi3805/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

}
//#endregion 🎛️Apps

//#region 🔖️Plugin
/// 🗂️ Sole native setup hook for the whole plugin bundle — registers all fifteen family document kinds'
/// pack↔dsl codecs. Each app's document schema is the single source of truth for its own registration.
fn register_norm_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::din4108::Din4108PlayApp>(apps::din4108::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::din16798::Din16798PlayApp>(apps::din16798::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::din18599::Din18599PlayApp>(apps::din18599::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::en1990::En1990PlayApp>(apps::en1990::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::en1991::En1991PlayApp>(apps::en1991::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::en1992::En1992PlayApp>(apps::en1992::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::en1993::En1993PlayApp>(apps::en1993::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::en1994::En1994PlayApp>(apps::en1994::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::en1995::En1995PlayApp>(apps::en1995::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::en1996::En1996PlayApp>(apps::en1996::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::en1997::En1997PlayApp>(apps::en1997::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::en1998::En1998PlayApp>(apps::en1998::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::en1999::En1999PlayApp>(apps::en1999::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::iso16757::Iso16757PlayApp>(apps::iso16757::DOCUMENT_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::vdi3805::Vdi3805PlayApp>(apps::vdi3805::DOCUMENT_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "norm",
    label: "Norm",
    version: "0.1.0",
    setup: register_norm_exports,
    apps: [
        apps::din4108::create_din4108_app => apps::din4108::Din4108PlayApp,
        apps::din16798::create_din16798_app => apps::din16798::Din16798PlayApp,
        apps::din18599::create_din18599_app => apps::din18599::Din18599PlayApp,
        apps::en1990::create_en1990_app => apps::en1990::En1990PlayApp,
        apps::en1991::create_en1991_app => apps::en1991::En1991PlayApp,
        apps::en1992::create_en1992_app => apps::en1992::En1992PlayApp,
        apps::en1993::create_en1993_app => apps::en1993::En1993PlayApp,
        apps::en1994::create_en1994_app => apps::en1994::En1994PlayApp,
        apps::en1995::create_en1995_app => apps::en1995::En1995PlayApp,
        apps::en1996::create_en1996_app => apps::en1996::En1996PlayApp,
        apps::en1997::create_en1997_app => apps::en1997::En1997PlayApp,
        apps::en1998::create_en1998_app => apps::en1998::En1998PlayApp,
        apps::en1999::create_en1999_app => apps::en1999::En1999PlayApp,
        apps::iso16757::create_iso16757_app => apps::iso16757::Iso16757PlayApp,
        apps::vdi3805::create_vdi3805_app => apps::vdi3805::Vdi3805PlayApp,
    ],
}
//#endregion 🔖️Plugin
