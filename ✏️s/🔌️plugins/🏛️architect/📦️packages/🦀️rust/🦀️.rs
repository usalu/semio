//! 🏛️ Architect plugin — the architectural-programming document app, bundled as a hot-swappable WASM
//! plugin.
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
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault>`, the exact signature
// `ArtifactApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned
// error type; boxing it here would diverge from the trait it must satisfy, and the lint does not fire on
// the trait impl itself (only on the free functions the taxonomy split creates), so this is a pure
// artefact of decomposition.
#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod program {
        #[path = "../../🗿️artifacts/🏛️program/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧱️kernel/🦀️.rs"]
                            pub mod kernel;
                            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🦀️.rs"]
                            pub mod registers;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_information_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_information_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_information_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_information_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/ℹ️information-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_sustainability_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_sustainability_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_sustainability_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_sustainability_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️sustainability-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_accessibility_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_accessibility_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_accessibility_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_accessibility_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♿️accessibility-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_conflict {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_conflict {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_conflict {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_conflict {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚔️conflict/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_option_evaluation {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_option_evaluation {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_option_evaluation {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_option_evaluation {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️option-evaluation/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_function {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_function {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_function {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_function {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️function/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_risk {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_risk {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_risk {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_risk {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚠️risk/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_decision {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_decision {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_decision {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_decision {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️decision/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_validation_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_validation_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_validation_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_validation_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✔️validation-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_priority_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_priority_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_priority_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_priority_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐️priority-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_flow_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_flow_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_flow_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_flow_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌊️flow-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_environmental_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_environmental_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_environmental_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_environmental_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌿️environmental-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_workshop {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_workshop {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_workshop {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_workshop {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎓️workshop/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_scenario {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_scenario {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_scenario {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_scenario {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬️scenario/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_benchmark_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_benchmark_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_benchmark_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_benchmark_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁️benchmark-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_activity {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_activity {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_activity {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_activity {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️activity/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_infrastructure_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_infrastructure_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_infrastructure_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_infrastructure_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️infrastructure-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_governance {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_governance {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏛️governance/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_organizational_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_organizational_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_organizational_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_organizational_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏢️organizational-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_meta {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_meta {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️meta/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_issue {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_issue {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_issue {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_issue {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🐛️issue/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_approval_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_approval_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_approval_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_approval_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👍️approval-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_stakeholder {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_stakeholder {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_stakeholder {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_stakeholder {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👥️stakeholder/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_quality_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_quality_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_quality_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_quality_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️quality-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_resilience_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_resilience_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_resilience_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_resilience_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💪️resilience-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_assumption {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_assumption {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_assumption {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_assumption {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💭️assumption/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_cost_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_cost_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_cost_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_cost_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💰️cost-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_project {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_project {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏙️project/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_document {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_document {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_document {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_document {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️document/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_schedule_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_schedule_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_schedule_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_schedule_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📅️schedule-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_growth_plan {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_growth_plan {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_growth_plan {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_growth_plan {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️growth-plan/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_performance_criterion {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_performance_criterion {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_performance_criterion {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_performance_criterion {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📊️performance-criterion/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_operational_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_operational_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_operational_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_operational_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📋️operational-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_site_context {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_site_context {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_site_context {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_site_context {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️site-context/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_template_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_template_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_template_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_template_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️template-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_report_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_report_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_report_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_report_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📑️report-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_audit_event {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_audit_event {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_audit_event {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_audit_event {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📒️audit-event/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_knowledge_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_knowledge_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_knowledge_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_knowledge_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚️knowledge-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_regulatory_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_regulatory_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_regulatory_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_regulatory_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️regulatory-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_change_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_change_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_change_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_change_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_communication_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_communication_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_communication_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_communication_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📡️communication-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_resource {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_resource {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_resource {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_resource {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️resource/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_status_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_status_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_status_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_status_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📶️status-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_process {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_process {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_process {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_process {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️process/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_search_filter {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_search_filter {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_search_filter {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_search_filter {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️search-filter/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_access_rule {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_access_rule {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_access_rule {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_access_rule {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔑️access-rule/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_privacy_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_privacy_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_privacy_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_privacy_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️privacy-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_relationship {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_relationship {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_relationship {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_relationship {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️relationship/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_quantity_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_quantity_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_quantity_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_quantity_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️quantity-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_analysis_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_analysis_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_analysis_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_analysis_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔬️analysis-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_storage_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_storage_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_storage_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_storage_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗄️storage-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_meeting_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_meeting_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_meeting_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_meeting_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗓️meeting-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_survey {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_survey {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_survey {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_survey {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗳️survey/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod connect_adjacency {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🧲️connect/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🧲️connect/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🧲️connect/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_delivery_constraint {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_delivery_constraint {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_delivery_constraint {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_delivery_constraint {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️delivery-constraint/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_constraint_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_constraint_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_constraint_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_constraint_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚧️constraint-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_compliance_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_compliance_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_compliance_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_compliance_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛂️compliance-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_service_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_service_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_service_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_service_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛎️service-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_equipment {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_equipment {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_equipment {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_equipment {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️equipment/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_security_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_security_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_security_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_security_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛡️security-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_collaboration_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_collaboration_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_collaboration_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_collaboration_record {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️collaboration-record/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_safety_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_safety_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_safety_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_safety_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦺️safety-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_user_profile {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_user_profile {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_user_profile {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_user_profile {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧑️user-profile/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_human_factor_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_human_factor_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_human_factor_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_human_factor_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧠️human-factor-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_flexibility_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_flexibility_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_flexibility_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_flexibility_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️flexibility-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_wayfinding_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_wayfinding_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_wayfinding_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_wayfinding_requirement {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️wayfinding-requirement/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod create_program_element {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🌱️create/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🌱️create/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🌱️create/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod delete_program_element {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🗑️delete/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🗑️delete/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🗑️delete/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod rename_program_element {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🏷️rename/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🏷️rename/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/🏷️rename/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_program_element {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/♻️replace/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/♻️replace/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱️program-element/♻️replace/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod connect_trace {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/🧵️connect/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/🧵️connect/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/🧵️connect/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod disconnect_trace {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/✂️disconnect/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/✂️disconnect/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️trace/✂️disconnect/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod disconnect_adjacency {
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🫷️disconnect/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🫷️disconnect/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧲️adjacency/🫷️disconnect/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod xlsx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::artifacts::program::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::program::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::program::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {

            pub use crate::artifacts::program::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::program::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::program::standards::v1::subsets::any::schema::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::program::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::artifacts::program::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::artifacts::program::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub mod kernel {
            pub use crate::artifacts::program::standards::v1::subsets::any::schema::kernel::*;
        }
        pub mod registers {
            pub use crate::artifacts::program::standards::v1::subsets::any::schema::registers::*;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod architect {
        #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs"]
        pub mod catalog;
        #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎨️chrome/🦀️.rs"]
        pub mod chrome;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️adjacency/🦀️.rs"]
            pub mod adjacency;
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔬️analysis/🦀️.rs"]
            pub mod analysis;
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️element/🦀️.rs"]
            pub mod element;
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️exchange/🦀️.rs"]
            pub mod exchange;
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️graph/🦀️.rs"]
            pub mod graph;
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️register/🦀️.rs"]
            pub mod register;
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔍️search/🦀️.rs"]
            pub mod search;
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️template/🦀️.rs"]
            pub mod template;
        }

        #[path = "."]
        pub mod modes {
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📊️report/🦀️.rs"]
            pub mod report;
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️review/🦀️.rs"]
            pub mod review;

            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🦀️.rs"]
                    pub mod adjacency;
                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs"]
                    pub mod graph;
                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🦀️.rs"]
                    pub mod register;
                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📓️report/🦀️.rs"]
                    pub mod report;
                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️trace/🦀️.rs"]
                    pub mod trace;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod architect {
        #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📋️register/🦀️.rs"]
                    pub mod register;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::ArchitectApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_architect_demo_session;
    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_program_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
    mod art_program_demo_tests;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
