//! 🎬️ Sequence plugin — declarative sequence play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that
//! is written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).
//!
//! `🎮️commands/🏃️run` is wired here as Rust module `playback` (not `run`) — its own payload submodules
//! are `run_command`/`stop_command`, so naming the owning module `run` too would trip clippy's
//! `module_inception`. The directory keeps its taxonomy name (🏃️run); only the Rust identifier differs.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<SequenceMutation, SequenceConfigMutation>, Fault>`, the exact signature
// `ArtifactApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split
// creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
extern crate infinite_canvas as infinite_board_port_directed_dag;

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod sequence {
        #[path = "../../🗿️artifacts/🎬️sequence/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                            pub mod operations;
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                    #[path = "."]
                    pub mod step {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod create_step {
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🌱create-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🌱create-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🌱create-step/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🌱create-step/🧪️tests/rejects-a-duplicate-step-id/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_step_id;
                                }
                                #[path = "."]
                                pub mod delete_step {
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🗑️delete-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🗑️delete-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🗑️delete-step/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🗑️delete-step/🧪️tests/rejects-deleting-a-missing-step/🦀️.rs"]
                                    mod tests_rejects_deleting_a_missing_step;
                                }
                                #[path = "."]
                                pub mod move_step {
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/📍move-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/📍move-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/📍move-step/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/📍move-step/🧪️tests/no-ops-when-the-step-is-already-at-that-position/🦀️.rs"]
                                    mod tests_no_ops_when_the_step_is_already_at_that_position;
                                }
                                #[path = "."]
                                pub mod edit_step_params {
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🔧edit-step-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🔧edit-step-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🔧edit-step-params/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🔧edit-step-params/🧪️tests/no-ops-when-the-params-are-already-identical/🦀️.rs"]
                                    mod tests_no_ops_when_the_params_are_already_identical;
                                }
                                #[path = "."]
                                pub mod change_step_collapsed {
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🗂️change-step-collapsed/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🗂️change-step-collapsed/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🗂️change-step-collapsed/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🗂️change-step-collapsed/🧪️tests/no-ops-when-the-step-is-already-collapsed/🦀️.rs"]
                                    mod tests_no_ops_when_the_step_is_already_collapsed;
                                }
                                #[path = "."]
                                pub mod duplicate_step {
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🧬duplicate-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🧬duplicate-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🧬duplicate-step/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️step/🧬️schema/🧬️mutations/🧬duplicate-step/🧪️tests/rejects-when-the-new-id-already-exists/🦀️.rs"]
                                    mod tests_rejects_when_the_new_id_already_exists;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod dependency {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod connect_steps {
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️dependency/🧬️schema/🧬️mutations/🔗connect-steps/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️dependency/🧬️schema/🧬️mutations/🔗connect-steps/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️dependency/🧬️schema/🧬️mutations/🔗connect-steps/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️dependency/🧬️schema/🧬️mutations/🔗connect-steps/🧪️tests/rejects-connecting-a-step-to-itself/🦀️.rs"]
                                    mod tests_rejects_connecting_a_step_to_itself;
                                }
                                #[path = "."]
                                pub mod disconnect_steps {
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️dependency/🧬️schema/🧬️mutations/✂️disconnect-steps/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️dependency/🧬️schema/🧬️mutations/✂️disconnect-steps/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️dependency/🧬️schema/🧬️mutations/✂️disconnect-steps/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️dependency/🧬️schema/🧬️mutations/✂️disconnect-steps/🧪️tests/rejects-disconnecting-a-missing-edge/🦀️.rs"]
                                    mod tests_rejects_disconnecting_a_missing_edge;
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
            pub use crate::artifacts::sequence::standards::v1::subsets::any::io::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::sequence::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::sequence::standards::v1::subsets::any::io::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::sequence::standards::v1::subsets::any::io::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::artifacts::sequence::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::artifacts::sequence::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
                mod tests;
            }
        }
    }
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sequence {
        #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[cfg(target_arch = "wasm32")]
        #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️connection/🦀️.rs"]
            pub mod connection;
            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️layout/🦀️.rs"]
            pub mod layout;
            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️locale/🦀️.rs"]
            pub mod locale;
            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph/🦀️.rs"]
            pub mod node_graph;
            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏃️run/🦀️.rs"]
            pub mod playback;
            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️step/🦀️.rs"]
            pub mod step;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️compiled/🦀️.rs"]
                    pub mod compiled;
                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🦀️.rs"]
                    pub mod main;
                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📜️script/🦀️.rs"]
                    pub mod script;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🦀️.rs"]
                mod tests;
            }
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod sequence {
        #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📽️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::SequenceApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_sequence_demo;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
