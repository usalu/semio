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
//! the generic whole-document mutation and its text/binary codecs) and the *app-surface* kernel (the one
//! shared config, the media ports, the render primitives, the manifest constructors) each exist exactly
//! once, while every per-standard fact — schema, ids, labels, compute — lives in that standard's own
//! artifact and app nodes.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit< Mutation, NormConfigMutation>, Fault>`, the exact signature `DocumentApp::handle` and
// `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it here
// would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself (only
// on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 📄️Document kernel
#[path = "../../📄️document/🦀️component.rs"]
pub mod document;
#[path = "."]
pub mod config {
    #[path = "../../🎚️config/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🎚️config/🧬️schema/🦀️component.rs"]
    pub mod schema;
}
#[path = "."]
pub mod presence {
    #[path = "../../👥️presence/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "../../👥️presence/🧬️schema/🦀️component.rs"]
    pub mod schema;
}
#[path = "../../🖥️app-surface/🦀️component.rs"]
pub mod app_surface;
//#endregion 📄️Document kernel

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod din4108 {
        #[path = "../../🗿️artifacts/📕️din4108/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📕️din4108/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📕️din4108/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📕️din4108/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📕️din4108/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📕️din4108/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📕️din4108/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📕️din4108/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📕️din4108/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📕️din4108/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📕️din4108/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📕️din4108/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📕️din4108/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📕️din4108/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📕️din4108/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📕️din4108/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📕️din4108/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📕️din4108/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📕️din4108/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📕️din4108/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📕️din4108/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📕️din4108/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📕️din4108/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod din16798 {
        #[path = "../../🗿️artifacts/📗️din16798/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📗️din16798/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📗️din16798/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📗️din16798/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📗️din16798/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📗️din16798/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📗️din16798/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📗️din16798/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📗️din16798/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📗️din16798/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📗️din16798/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📗️din16798/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📗️din16798/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📗️din16798/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📗️din16798/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📗️din16798/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📗️din16798/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📗️din16798/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📗️din16798/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📗️din16798/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📗️din16798/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📗️din16798/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📗️din16798/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod din18599 {
        #[path = "../../🗿️artifacts/📙️din18599/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📙️din18599/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📙️din18599/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📙️din18599/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📙️din18599/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📙️din18599/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📙️din18599/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📙️din18599/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📙️din18599/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📙️din18599/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📙️din18599/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📙️din18599/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📙️din18599/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📙️din18599/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📙️din18599/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📙️din18599/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📙️din18599/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📙️din18599/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📙️din18599/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📙️din18599/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📙️din18599/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📙️din18599/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📙️din18599/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1990 {
        #[path = "../../🗿️artifacts/📘️en1990/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1990/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📘️en1990/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📘️en1990/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📘️en1990/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📘️en1990/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📘️en1990/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📘️en1990/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📘️en1990/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📘️en1990/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📘️en1990/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1990/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1990/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📘️en1990/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1990/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1990/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1990/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1990/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1990/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1990/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1990/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1990/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📘️en1990/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1991 {
        #[path = "../../🗿️artifacts/📘️en1991/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1991/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📘️en1991/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📘️en1991/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📘️en1991/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📘️en1991/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📘️en1991/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📘️en1991/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📘️en1991/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📘️en1991/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📘️en1991/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1991/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1991/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📘️en1991/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1991/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1991/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1991/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1991/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1991/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1991/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1991/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1991/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📘️en1991/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1992 {
        #[path = "../../🗿️artifacts/📘️en1992/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1992/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📘️en1992/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📘️en1992/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📘️en1992/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📘️en1992/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📘️en1992/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📘️en1992/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📘️en1992/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📘️en1992/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📘️en1992/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1992/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1992/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📘️en1992/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1992/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1992/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1992/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1992/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1992/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1992/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1992/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1992/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📘️en1992/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1993 {
        #[path = "../../🗿️artifacts/📘️en1993/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1993/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📘️en1993/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📘️en1993/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📘️en1993/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📘️en1993/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📘️en1993/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📘️en1993/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📘️en1993/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📘️en1993/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📘️en1993/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1993/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1993/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📘️en1993/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1993/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1993/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1993/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1993/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1993/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1993/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1993/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1993/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📘️en1993/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1994 {
        #[path = "../../🗿️artifacts/📘️en1994/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1994/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📘️en1994/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📘️en1994/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📘️en1994/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📘️en1994/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📘️en1994/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📘️en1994/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📘️en1994/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📘️en1994/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📘️en1994/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1994/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1994/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📘️en1994/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1994/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1994/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1994/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1994/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1994/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1994/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1994/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1994/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📘️en1994/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1995 {
        #[path = "../../🗿️artifacts/📘️en1995/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1995/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📘️en1995/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📘️en1995/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📘️en1995/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📘️en1995/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📘️en1995/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📘️en1995/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📘️en1995/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📘️en1995/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📘️en1995/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1995/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1995/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📘️en1995/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1995/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1995/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1995/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1995/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1995/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1995/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1995/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1995/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📘️en1995/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1996 {
        #[path = "../../🗿️artifacts/📘️en1996/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1996/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📘️en1996/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📘️en1996/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📘️en1996/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📘️en1996/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📘️en1996/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📘️en1996/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📘️en1996/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📘️en1996/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📘️en1996/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1996/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1996/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📘️en1996/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1996/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1996/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1996/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1996/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1996/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1996/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1996/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1996/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📘️en1996/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1997 {
        #[path = "../../🗿️artifacts/📘️en1997/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1997/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📘️en1997/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📘️en1997/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📘️en1997/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📘️en1997/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📘️en1997/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📘️en1997/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📘️en1997/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📘️en1997/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📘️en1997/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1997/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1997/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📘️en1997/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1997/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1997/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1997/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1997/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1997/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1997/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1997/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1997/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📘️en1997/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1998 {
        #[path = "../../🗿️artifacts/📘️en1998/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1998/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📘️en1998/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📘️en1998/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📘️en1998/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📘️en1998/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📘️en1998/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📘️en1998/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📘️en1998/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📘️en1998/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📘️en1998/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1998/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1998/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📘️en1998/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1998/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1998/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1998/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1998/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1998/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1998/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1998/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1998/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📘️en1998/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod en1999 {
        #[path = "../../🗿️artifacts/📘️en1999/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📘️en1999/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📘️en1999/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📘️en1999/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📘️en1999/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📘️en1999/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📘️en1999/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📘️en1999/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📘️en1999/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📘️en1999/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📘️en1999/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📘️en1999/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📘️en1999/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📘️en1999/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1999/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1999/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1999/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1999/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1999/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1999/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📘️en1999/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📘️en1999/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📘️en1999/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod iso16757 {
        #[path = "../../🗿️artifacts/📓️iso16757/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📓️iso16757/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📓️iso16757/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📓️iso16757/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📓️iso16757/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📓️iso16757/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📓️iso16757/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📓️iso16757/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📓️iso16757/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📓️iso16757/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📓️iso16757/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📓️iso16757/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📓️iso16757/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📓️iso16757/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📓️iso16757/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📓️iso16757/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📓️iso16757/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📓️iso16757/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📓️iso16757/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📓️iso16757/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📓️iso16757/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📓️iso16757/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
        #[path = "../../🗿️artifacts/📓️iso16757/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }

    #[path = "."]
    pub mod vdi3805 {
        #[path = "../../🗿️artifacts/📔️vdi3805/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📔️vdi3805/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📔️vdi3805/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📔️vdi3805/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📔️vdi3805/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📔️vdi3805/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📔️vdi3805/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/📔️vdi3805/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📔️vdi3805/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📔️vdi3805/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/📔️vdi3805/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📔️vdi3805/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📔️vdi3805/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📔️vdi3805/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📔️vdi3805/🚪️io/📊️csv/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📔️vdi3805/🚪️io/📊️csv/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📔️vdi3805/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📔️vdi3805/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod xlsx {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📔️vdi3805/🚪️io/📕️xlsx/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📔️vdi3805/🚪️io/📕️xlsx/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📔️vdi3805/🚪️io/🎒️zip/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📔️vdi3805/🚪️io/🎒️zip/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
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
            #[path = "../../🎛️apps/📕️din4108/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📗️din16798/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📙️din18599/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📘️en1990/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📘️en1991/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📘️en1992/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📘️en1993/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📘️en1994/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📘️en1995/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📘️en1996/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📘️en1997/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📘️en1998/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📘️en1999/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📓️iso16757/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
            #[path = "../../🎛️apps/📔️vdi3805/🎮️commands/📤️set-snapshot/🦀️component.rs"]
            pub mod set_snapshot;
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
#[path = "../../🔌️plugin/🔧️setup/🦀️component.rs"]
mod setup;
pub use setup::register_norm_exports;

#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/📓️iso16757/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_iso16757_demo;
    #[path = "../../🗿️artifacts/📔️vdi3805/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_vdi3805_demo;
    #[path = "../../🗿️artifacts/📕️din4108/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_din4108_demo;
    #[path = "../../🗿️artifacts/📗️din16798/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_din16798_demo;
    #[path = "../../🗿️artifacts/📘️en1990/📚️examples/📕️high-consequence-office/🦀️component.rs"]
    pub mod art_en1990_high_consequence_office;
    #[path = "../../🗿️artifacts/📘️en1991/📚️examples/📕️retail-hydrocarbon-fire/🦀️component.rs"]
    pub mod art_en1991_retail_hydrocarbon_fire;
    #[path = "../../🗿️artifacts/📘️en1992/📚️examples/📕️liquid-retaining-fem-anchor/🦀️component.rs"]
    pub mod art_en1992_liquid_retaining_fem_anchor;
    #[path = "../../🗿️artifacts/📘️en1993/📚️examples/📕️high-strength-connection/🦀️component.rs"]
    pub mod art_en1993_high_strength_connection;
    #[path = "../../🗿️artifacts/📘️en1994/📚️examples/📕️composite-bridge-girder/🦀️component.rs"]
    pub mod art_en1994_composite_bridge_girder;
    #[path = "../../🗿️artifacts/📘️en1995/📚️examples/📕️glulam-footbridge/🦀️component.rs"]
    pub mod art_en1995_glulam_footbridge;
    #[path = "../../🗿️artifacts/📘️en1996/📚️examples/📕️loadbearing-wall/🦀️component.rs"]
    pub mod art_en1996_loadbearing_wall;
    #[path = "../../🗿️artifacts/📘️en1997/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_en1997_demo;
    #[path = "../../🗿️artifacts/📘️en1998/📚️examples/📕️seismic-rc-frame/🦀️component.rs"]
    pub mod art_en1998_seismic_rc_frame;
    #[path = "../../🗿️artifacts/📘️en1999/📚️examples/📕️aluminium-roof-purlin/🦀️component.rs"]
    pub mod art_en1999_aluminium_roof_purlin;
    #[path = "../../🗿️artifacts/📙️din18599/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_din18599_demo;
    #[path = "../../🎛️apps/📓️iso16757/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_iso16757_demo_session;
    #[path = "../../🎛️apps/📔️vdi3805/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_vdi3805_demo_session;
    #[path = "../../🎛️apps/📕️din4108/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_din4108_demo_session;
    #[path = "../../🎛️apps/📗️din16798/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_din16798_demo_session;
    #[path = "../../🎛️apps/📘️en1990/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1990_demo_session;
    #[path = "../../🎛️apps/📘️en1991/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1991_demo_session;
    #[path = "../../🎛️apps/📘️en1992/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1992_demo_session;
    #[path = "../../🎛️apps/📘️en1993/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1993_demo_session;
    #[path = "../../🎛️apps/📘️en1994/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1994_demo_session;
    #[path = "../../🎛️apps/📘️en1995/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1995_demo_session;
    #[path = "../../🎛️apps/📘️en1996/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1996_demo_session;
    #[path = "../../🎛️apps/📘️en1997/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1997_demo_session;
    #[path = "../../🎛️apps/📘️en1998/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1998_demo_session;
    #[path = "../../🎛️apps/📘️en1999/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_en1999_demo_session;
    #[path = "../../🎛️apps/📙️din18599/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_din18599_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
