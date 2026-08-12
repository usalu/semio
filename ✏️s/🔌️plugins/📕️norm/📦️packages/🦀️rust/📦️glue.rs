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
// `Result<Emit< Mutation, NormConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle` and
// `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it here
// would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself (only
// on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 📄️Document kernel
// 🐛️ W5a fix: dir is `📄️artifact` on disk (renamed from `📄️document` by the framework-wide
// document->artifact terminology migration, ticket 26/08/05); this `#[path]` still pointed at the
// old name and broke every downstream build. Module name stays `document` — `crate::document::…`
// is used pervasively across this crate's engines/apps.
#[path = "../../📄️artifact/🦀️component.rs"]
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
    pub mod iso16757 {
        #[path = "../../🗿️artifacts/📓️iso16757/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📓️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::snapshot::Iso16757Snapshot;
        pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::mutations::Iso16757Mutation;
        pub use crate::artifacts::iso16757::standards::v1::subsets::any::schema::diff::Iso16757Diff;

        #[path = "../../🗿️artifacts/📓️iso16757/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📓️iso16757/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📓️iso16757/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📓️iso16757/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod vdi3805 {
        #[path = "../../🗿️artifacts/📔️vdi3805/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📔️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::snapshot::Vdi3805Snapshot;
        pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::mutations::Vdi3805Mutation;
        pub use crate::artifacts::vdi3805::standards::v1::subsets::any::schema::diff::Vdi3805Diff;

        #[path = "../../🗿️artifacts/📔️vdi3805/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📔️vdi3805/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📔️vdi3805/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📔️vdi3805/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod din4108 {
        #[path = "../../🗿️artifacts/📕️din4108/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::snapshot::Din4108Snapshot;
        pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::mutations::Din4108Mutation;
        pub use crate::artifacts::din4108::standards::v1::subsets::any::schema::diff::Din4108Diff;

        #[path = "../../🗿️artifacts/📕️din4108/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📕️din4108/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📕️din4108/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📕️din4108/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod din16798 {
        #[path = "../../🗿️artifacts/📗️din16798/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::snapshot::Din16798Snapshot;
        pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::mutations::Din16798Mutation;
        pub use crate::artifacts::din16798::standards::v1::subsets::any::schema::diff::Din16798Diff;

        #[path = "../../🗿️artifacts/📗️din16798/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📗️din16798/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📗️din16798/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📗️din16798/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod en1990 {
        #[path = "../../🗿️artifacts/📘️en1990/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::snapshot::En1990Snapshot;
        pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::mutations::En1990Mutation;
        pub use crate::artifacts::en1990::standards::v1::subsets::any::schema::diff::En1990Diff;

        #[path = "../../🗿️artifacts/📘️en1990/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📘️en1990/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📘️en1990/🎹️composer/🦀️component.rs"]
        pub mod composer;

    }
    #[path = "."]
    pub mod en1991 {
        #[path = "../../🗿️artifacts/📘️en1991/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::snapshot::En1991Snapshot;
        pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::mutations::En1991Mutation;
        pub use crate::artifacts::en1991::standards::v1::subsets::any::schema::diff::En1991Diff;

        #[path = "../../🗿️artifacts/📘️en1991/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📘️en1991/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📘️en1991/🎹️composer/🦀️component.rs"]
        pub mod composer;

    }
    #[path = "."]
    pub mod en1992 {
        #[path = "../../🗿️artifacts/📘️en1992/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::snapshot::En1992Snapshot;
        pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::mutations::En1992Mutation;
        pub use crate::artifacts::en1992::standards::v1::subsets::any::schema::diff::En1992Diff;

        #[path = "../../🗿️artifacts/📘️en1992/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📘️en1992/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📘️en1992/🎹️composer/🦀️component.rs"]
        pub mod composer;

    }
    #[path = "."]
    pub mod en1993 {
        #[path = "../../🗿️artifacts/📘️en1993/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::snapshot::En1993Snapshot;
        pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::mutations::En1993Mutation;
        pub use crate::artifacts::en1993::standards::v1::subsets::any::schema::diff::En1993Diff;

        #[path = "../../🗿️artifacts/📘️en1993/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📘️en1993/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📘️en1993/🎹️composer/🦀️component.rs"]
        pub mod composer;

    }
    #[path = "."]
    pub mod en1994 {
        #[path = "../../🗿️artifacts/📘️en1994/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::snapshot::En1994Snapshot;
        pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::mutations::En1994Mutation;
        pub use crate::artifacts::en1994::standards::v1::subsets::any::schema::diff::En1994Diff;

        #[path = "../../🗿️artifacts/📘️en1994/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📘️en1994/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📘️en1994/🎹️composer/🦀️component.rs"]
        pub mod composer;

    }
    #[path = "."]
    pub mod en1995 {
        #[path = "../../🗿️artifacts/📘️en1995/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::snapshot::En1995Snapshot;
        pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::mutations::En1995Mutation;
        pub use crate::artifacts::en1995::standards::v1::subsets::any::schema::diff::En1995Diff;

        #[path = "../../🗿️artifacts/📘️en1995/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📘️en1995/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📘️en1995/🎹️composer/🦀️component.rs"]
        pub mod composer;

    }
    #[path = "."]
    pub mod en1996 {
        #[path = "../../🗿️artifacts/📘️en1996/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::snapshot::En1996Snapshot;
        pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::mutations::En1996Mutation;
        pub use crate::artifacts::en1996::standards::v1::subsets::any::schema::diff::En1996Diff;

        #[path = "../../🗿️artifacts/📘️en1996/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📘️en1996/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📘️en1996/🎹️composer/🦀️component.rs"]
        pub mod composer;

    }
    #[path = "."]
    pub mod en1997 {
        #[path = "../../🗿️artifacts/📘️en1997/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::snapshot::En1997Snapshot;
        pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::mutations::En1997Mutation;
        pub use crate::artifacts::en1997::standards::v1::subsets::any::schema::diff::En1997Diff;

        #[path = "../../🗿️artifacts/📘️en1997/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📘️en1997/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📘️en1997/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📘️en1997/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod en1998 {
        #[path = "../../🗿️artifacts/📘️en1998/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::snapshot::En1998Snapshot;
        pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::mutations::En1998Mutation;
        pub use crate::artifacts::en1998::standards::v1::subsets::any::schema::diff::En1998Diff;

        #[path = "../../🗿️artifacts/📘️en1998/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📘️en1998/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📘️en1998/🎹️composer/🦀️component.rs"]
        pub mod composer;

    }
    #[path = "."]
    pub mod en1999 {
        #[path = "../../🗿️artifacts/📘️en1999/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::snapshot::En1999Snapshot;
        pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::mutations::En1999Mutation;
        pub use crate::artifacts::en1999::standards::v1::subsets::any::schema::diff::En1999Diff;

        #[path = "../../🗿️artifacts/📘️en1999/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📘️en1999/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📘️en1999/🎹️composer/🦀️component.rs"]
        pub mod composer;

    }
    #[path = "."]
    pub mod din18599 {
        #[path = "../../🗿️artifacts/📙️din18599/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                pub mod engine;
                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_snapshot {
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::snapshot::Din18599Snapshot;
        pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::mutations::Din18599Mutation;
        pub use crate::artifacts::din18599::standards::v1::subsets::any::schema::diff::Din18599Diff;

        #[path = "../../🗿️artifacts/📙️din18599/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📙️din18599/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📙️din18599/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📙️din18599/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
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
            #[path = "../../🎛️apps/📕️din4108/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📗️din16798/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📙️din18599/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📘️en1990/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📘️en1991/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📘️en1992/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📘️en1993/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📘️en1994/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📘️en1995/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📘️en1996/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📘️en1997/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📘️en1998/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📘️en1999/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📓️iso16757/📌️panels/📄️artifact/🦀️component.rs"]
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
            #[path = "../../🎛️apps/📔️vdi3805/📌️panels/📄️artifact/🦀️component.rs"]
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
#[path = "../../🔧️setup/🦀️component.rs"]
mod setup;
pub use setup::register_norm_exports;

#[path = "../../🦀️component.rs"]
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
