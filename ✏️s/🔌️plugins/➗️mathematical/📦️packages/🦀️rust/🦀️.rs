//! 🧮️ Mathematical plugin — declarative mathematical play app (graph algorithms + computational
//! geometry) bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_number as number;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<EquationMutation, EquationConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
// 🚚 Wave M3a (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS): `cas`/
// `polynomial` migrated verbatim from `🧮️math`'s crate root (files physically relocated under
// `🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/`, the facet's
// Rust-only compute internals a named inference's `compute()` delegates into — mirrors stdio's
// `📐️step` io facet's `🪜️ladder`/`📐️part21`/`🧱️brep` precedent for deep Rust-only helper dirs under a
// facet). Mounted DIRECTLY at crate root, exactly as `🧮️math`'s own glue.rs mounted them — every
// `crate::cas::…`/`crate::polynomial::…` self-reference inside those two files (including references
// to non-`pub` inner modules, e.g. `crate::cas::canon`) is untouched, and privacy is structural in
// Rust: a re-export/alias layer (`pub use … as cas`) does NOT leak private inner items back out, so
// only a direct mount preserves them. `crate::algebra`'s two call sites became `math::algebra::MatG`/
// `math::algebra::VecG` against a since-removed `semio_framework_math as math` dependency — briefly a
// genuine pre-existing breakage (wave M3d had moved `algebra` out of `semio_framework_math` into
// `📸️remodel`, not knowing this wave had just created a second consumer here). Wave FIXALG (same
// ticket) relocated `VecG`/`MatG` out of `📸️remodel` into `semio_framework_number`'s own `algebra`
// module and repointed both sites at `number::MatG`/`number::VecG`, so `math` is no longer a
// dependency of this crate at all. `crate::number` became `number::` (wave MATHEND) against the
// `semio_framework_number` dependency below — `number` was relocated out of `🧮️math` into its own
// framework module, so it is a real top-level extern crate now, not a submodule of `math`.
#[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌿️cas-internals/🦀️.rs"]
pub mod cas;
#[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📈️polynomial-internals/🦀️.rs"]
pub mod polynomial;

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod equation {
        #[path = "../../🗿️artifacts/➗️equation/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                // 🚚 Wave M3a: first real `impl InferredField<P>` in this codebase — see its own
                                // doc header for why (every other named inference documents using the plain
                                // whole-snapshot pattern instead).
                                #[path = "."]
                                pub mod roots {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🌱roots/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                // 🚚 Wave M3a (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS):
                                // `🌿️cas-internals/`'s and `📈️polynomial-internals/`'s Rust-only compute code lives
                                // PHYSICALLY right here, under this facet — but is MOUNTED at crate root as `pub mod cas`/
                                // `pub mod polynomial` (see the top of this file), not nested under this module. Every
                                // `crate::cas::…`/`crate::polynomial::…` self-reference inside those two files is
                                // untouched from the original `🧮️math` crate, and privacy is structural in Rust: a
                                // `mod canon { … }` (non-`pub`) nested here would need `pub use component::*` to leak
                                // it back out, which does NOT re-export private items — `crate::cas::canon` would 404.
                                // Direct crate-root mounting is the only way to preserve every private inner `mod`
                                // unedited, so these two crates deliberately do NOT also appear as a submodule of
                                // `inferences` — see `🌿️cas-internals/🦀️.rs`'s doc header for the full story.
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                    pub mod graph {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod change_graph_directed {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔀️change-graph-directed/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔀️change-graph-directed/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔀️change-graph-directed/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔀️change-graph-directed/🧪️tests/keeps-an-already-directed-graph-directed/🦀️.rs"]
                                    mod tests_keeps_an_already_directed_graph_directed;
                                }
                                #[path = "."]
                                pub mod update_graph_algorithm {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🧮️update-graph-algorithm/🧪️tests/restates-the-unset-algorithm-and-its-absent-seed/🦀️.rs"]
                                    mod tests_restates_the_unset_algorithm_and_its_absent_seed;
                                }
                                #[path = "."]
                                pub mod replace_graph {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔁️replace-graph/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔁️replace-graph/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔁️replace-graph/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔁️replace-graph/🧪️tests/replays-the-identical-empty-graph/🦀️.rs"]
                                    mod tests_replays_the_identical_empty_graph;
                                }
                                #[path = "."]
                                pub mod create_node {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🟢️create-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🟢️create-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🟢️create-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🟢️create-node/🧪️tests/rejects-a-duplicate-node-id/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_node_id;
                                }
                                #[path = "."]
                                pub mod delete_node {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/❌️delete-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/❌️delete-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/❌️delete-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/❌️delete-node/🧪️tests/rejects-deleting-a-node-that-is-not-in-the-graph/🦀️.rs"]
                                    mod tests_rejects_deleting_a_node_that_is_not_in_the_graph;
                                }
                                #[path = "."]
                                pub mod delete_nodes {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-nodes/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-nodes/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-nodes/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-nodes/🧪️tests/rejects-a-bulk-delete-where-every-id-is-absent/🦀️.rs"]
                                    mod tests_rejects_a_bulk_delete_where_every_id_is_absent;
                                }
                                #[path = "."]
                                pub mod change_node_label {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏷️change-node-label/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏷️change-node-label/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏷️change-node-label/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏷️change-node-label/🧪️tests/rejects-relabelling-a-node-that-is-not-in-the-graph/🦀️.rs"]
                                    mod tests_rejects_relabelling_a_node_that_is_not_in_the_graph;
                                }
                                #[path = "."]
                                pub mod move_node {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🕹️move-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🕹️move-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🕹️move-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🕹️move-node/🧪️tests/rejects-moving-a-node-that-is-not-in-the-graph/🦀️.rs"]
                                    mod tests_rejects_moving_a_node_that_is_not_in_the_graph;
                                }
                                #[path = "."]
                                pub mod connect_nodes {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗️connect-nodes/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗️connect-nodes/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗️connect-nodes/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗️connect-nodes/🧪️tests/rejects-an-edge-between-two-absent-endpoints/🦀️.rs"]
                                    mod tests_rejects_an_edge_between_two_absent_endpoints;
                                }
                                #[path = "."]
                                pub mod disconnect_nodes {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️disconnect-nodes/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️disconnect-nodes/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️disconnect-nodes/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️disconnect-nodes/🧪️tests/rejects-severing-an-edge-that-is-not-in-the-graph/🦀️.rs"]
                                    mod tests_rejects_severing_an_edge_that_is_not_in_the_graph;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod geometry {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod replace_points {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/🌀️replace-points/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/🌀️replace-points/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/🌀️replace-points/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/🌀️replace-points/🧪️tests/replays-the-identical-empty-point-cloud/🦀️.rs"]
                                    mod tests_replays_the_identical_empty_point_cloud;
                                }
                                #[path = "."]
                                pub mod insert_point {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/➕️insert-point/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/➕️insert-point/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/➕️insert-point/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/➕️insert-point/🧪️tests/seeds-the-empty-cloud-with-its-first-point/🦀️.rs"]
                                    mod tests_seeds_the_empty_cloud_with_its_first_point;
                                }
                                #[path = "."]
                                pub mod remove_point {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/➖️remove-point/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/➖️remove-point/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/➖️remove-point/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/➖️remove-point/🧪️tests/rejects-removing-a-point-from-an-empty-cloud/🦀️.rs"]
                                    mod tests_rejects_removing_a_point_from_an_empty_cloud;
                                }
                                #[path = "."]
                                pub mod move_point {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/🎯️move-point/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/🎯️move-point/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/🎯️move-point/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️geometry/🧬️schema/🧬️mutations/🎯️move-point/🧪️tests/rejects-moving-a-point-that-is-not-in-the-cloud/🦀️.rs"]
                                    mod tests_rejects_moving_a_point_that_is_not_in_the_cloud;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod equation {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod change_coefficient {
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️equation/🧬️schema/🧬️mutations/🔄️change-coefficient/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️equation/🧬️schema/🧬️mutations/🔄️change-coefficient/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️equation/🧬️schema/🧬️mutations/🔄️change-coefficient/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️equation/🧬️schema/🧬️mutations/🔄️change-coefficient/🧪️tests/raises-the-leading-coefficient-to-three-halves/🦀️.rs"]
                                    mod tests_raises_the_leading_coefficient_to_three_halves;
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
            pub use crate::artifacts::equation::standards::v1::subsets::any::io::mutations::text::*;
            pub use crate::artifacts::equation::standards::v1::subsets::any::schema::mutations::EquationMutation;
        }
        pub mod dsl {
            pub use crate::artifacts::equation::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::equation::standards::v1::subsets::any::io::mutations::binary::*;
        }
        pub mod pack {
            pub use crate::artifacts::equation::standards::v1::subsets::any::io::snapshot::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::equation::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::equation::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::equation::standards::v1::subsets::any::io::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::equation::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::artifacts::equation::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::artifacts::equation::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }
        pub use crate::artifacts::equation::standards::v1::subsets::any::schema::diff::EquationDiff;
        pub use crate::artifacts::equation::standards::v1::subsets::any::schema::mutations::EquationMutation;
        pub use crate::artifacts::equation::standards::v1::subsets::any::schema::snapshot::EquationSnapshot;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
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
    pub mod equation {
        #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs"]
            pub mod node_graph_edit;
            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️set-algorithm/🦀️.rs"]
            pub mod set_algorithm;
            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-artifact/🦀️.rs"]
            pub mod set_artifact;
            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️set-directed/🦀️.rs"]
            pub mod set_directed;
            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️set-points/🦀️.rs"]
            pub mod set_points;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️geometry/🦀️.rs"]
                    pub mod geometry;
                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs"]
                    pub mod graph;
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
    pub mod equation {
        #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📐️geometry/🦀️.rs"]
                    pub mod geometry;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::MathematicalApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_equation_demo_session;
    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_equation_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
    mod art_equation_demo_tests;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
