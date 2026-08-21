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
//! `PlaybookConfigMutation`/`PlaybookCommand`, and the `PlaybookPlayApp` `ArtifactApp` impl.

extern crate semio_framework as semio_framework;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
pub use flow::playbook;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<PlaybookMutation, PlaybookConfigMutation>, Fault>`, the exact signature
// `ArtifactApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned
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

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod move_block {
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀move-block/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀move-block/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀move-block/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀move-block/🧪️tests/rejects-moving-a-block-into-a-missing-step/🦀️component.rs"]
                                    mod tests_rejects_moving_a_block_into_a_missing_step;
                                }
                                #[path = "."]
                                pub mod move_step {
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-step/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-step/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-step/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-step/🧪️tests/no-ops-when-the-step-is-already-at-that-index/🦀️component.rs"]
                                    mod tests_no_ops_when_the_step_is_already_at_that_index;
                                }
                                #[path = "."]
                                pub mod add_block {
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱add-block/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱add-block/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱add-block/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱add-block/🧪️tests/rejects-adding-a-block-to-a-missing-step/🦀️component.rs"]
                                    mod tests_rejects_adding_a_block_to_a_missing_step;
                                }
                                #[path = "."]
                                pub mod add_step {
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-step/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-step/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-step/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-step/🧪️tests/no-ops-on-a-duplicate-step-id/🦀️component.rs"]
                                    mod tests_no_ops_on_a_duplicate_step_id;
                                }
                                #[path = "."]
                                pub mod remove_block {
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-block/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-block/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-block/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-block/🧪️tests/rejects-removing-a-block-missing-from-its-step/🦀️component.rs"]
                                    mod tests_rejects_removing_a_block_missing_from_its_step;
                                }
                                #[path = "."]
                                pub mod remove_step {
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-step/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-step/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-step/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-step/🧪️tests/rejects-removing-a-missing-step/🦀️component.rs"]
                                    mod tests_rejects_removing_a_missing_step;
                                }
                                #[path = "."]
                                pub mod change_title {
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/🧪️tests/changes-the-playbook-title/🦀️component.rs"]
                                    mod tests_changes_the_playbook_title;
                                }
                                #[path = "."]
                                pub mod replace_block {
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-block/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-block/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-block/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-block/🧪️tests/no-ops-when-the-block-is-already-identical/🦀️component.rs"]
                                    mod tests_no_ops_when_the_block_is_already_identical;
                                }
                                #[path = "."]
                                pub mod update_step {
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-step/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-step/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-step/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-step/🧪️tests/no-ops-when-the-header-is-already-current/🦀️component.rs"]
                                    mod tests_no_ops_when_the_header_is_already_current;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
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
                                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod docx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod docx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
            pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::mutations::text::*;
            pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::mutations::{apply_playbook_mutation, PlaybookMutation};
        }
        pub mod dsl {
            pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::artifacts::playbook::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
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
    pub mod playbook {
        #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️component.rs"]
        pub mod engine;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️add-block/🦀️component.rs"]
            pub mod add_block;
            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️add-step/🦀️component.rs"]
            pub mod add_step;
            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️move-block/🦀️component.rs"]
            pub mod move_block;
            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️move-step/🦀️component.rs"]
            pub mod move_step;
            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️remove-block/🦀️component.rs"]
            pub mod remove_block;
            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️remove-step/🦀️component.rs"]
            pub mod remove_step;
            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️component.rs"]
            pub mod set_contributions;
            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️component.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️update-playbook/🦀️component.rs"]
            pub mod update_playbook;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod builder {
                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod builder {
                        #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod playbook {
        #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod steps {
                        #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️steps/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_playbook_demo_session;
    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_playbook_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
    mod art_playbook_demo_tests;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
