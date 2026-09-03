//! 🎞️ Animate plugin — presentation tile play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

// 🌀️ R7 — `fn` in public traits (`Sobject`, `Animation`) is deliberate under O1's universal-async
// ruling; callers cannot assume `Send` from the lint's suggested fix, and R3 answers that structurally
// (every dyn seam here is a `dyn_enum_close!`-generated enum, so `Send` falls out of the concrete
// variant types). Never resolved by `+ Send` on the trait method or by making the method sync.
#![allow(async_fn_in_trait)]


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault>`, the exact signature
// `ArtifactApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split creates),
// so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod presentation {
        #[path = "../../🗿️artifacts/🎬️presentation/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod resize_source_frame {
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲resize-source-frame/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲resize-source-frame/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲resize-source-frame/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲resize-source-frame/🧪️tests/no-ops-when-the-frame-is-already-identical/🦀️.rs"]
                                    mod tests_no_ops_when_the_frame_is_already_identical;
                                }
                                #[path = "."]
                                pub mod replace_source {
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼replace-source/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼replace-source/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼replace-source/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼replace-source/🧪️tests/no-ops-when-the-source-is-already-identical/🦀️.rs"]
                                    mod tests_no_ops_when_the_source_is_already_identical;
                                }
                                #[path = "."]
                                pub mod create_tile {
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-tile/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-tile/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-tile/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-tile/🧪️tests/rejects-a-duplicate-tile-id/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_tile_id;
                                }
                                #[path = "."]
                                pub mod delete_tile {
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-tile/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-tile/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-tile/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-tile/🧪️tests/rejects-deleting-a-missing-tile/🦀️.rs"]
                                    mod tests_rejects_deleting_a_missing_tile;
                                }
                                #[path = "."]
                                pub mod delete_tiles {
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-tiles/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-tiles/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-tiles/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-tiles/🧪️tests/rejects-when-every-addressed-tile-is-missing/🦀️.rs"]
                                    mod tests_rejects_when_every_addressed_tile_is_missing;
                                }
                                #[path = "."]
                                pub mod rename_tile {
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏rename-tile/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏rename-tile/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏rename-tile/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏rename-tile/🧪️tests/no-ops-when-the-tile-already-has-that-name/🦀️.rs"]
                                    mod tests_no_ops_when_the_tile_already_has_that_name;
                                }
                                #[path = "."]
                                pub mod resize_tile_crop {
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂resize-tile-crop/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂resize-tile-crop/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂resize-tile-crop/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂resize-tile-crop/🧪️tests/rejects-a-zero-width-crop/🦀️.rs"]
                                    mod tests_rejects_a_zero_width_crop;
                                }
                                #[path = "."]
                                pub mod reorder_tiles {
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-tiles/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-tiles/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-tiles/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-tiles/🧪️tests/no-ops-when-the-tile-is-already-at-that-index/🦀️.rs"]
                                    mod tests_no_ops_when_the_tile_is_already_at_that_index;
                                }
                                #[path = "."]
                                pub mod replace_tiles {
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-tiles/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-tiles/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-tiles/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-tiles/🧪️tests/no-ops-when-the-collection-is-already-empty/🦀️.rs"]
                                    mod tests_no_ops_when_the_collection_is_already_empty;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod pptx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                        pub mod pptx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
            pub use crate::artifacts::presentation::standards::v1::subsets::any::io::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::presentation::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::presentation::standards::v1::subsets::any::io::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::presentation::standards::v1::subsets::any::io::diff::text::*;
            pub use crate::artifacts::presentation::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::io::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::io::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::io::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::presentation::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::io::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::io::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::io::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::presentation::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::io::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::io::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::presentation::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
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
    pub mod animate {
        #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod engine {
            //! ⚙️ This app's own stateful behaviour (compiler/slide/video-export at the root, plus the
            //! Manim-class animation core and headless video renderer as sibling `<topic>/🦀️.rs`
            //! files) — a non-taxonomy, editor-only facet (ticket
            //! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET packet recipe step 4: only editor-side
            //! files reference it, so it moved wholesale into `✏️editor/⚙️engine/`).
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎞️animation/🦀️.rs"]
            pub mod animation;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📷️camera/🦀️.rs"]
            pub mod camera;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎛️config/🦀️.rs"]
            pub mod config;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️geometry/🦀️.rs"]
            pub mod geometry;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/⏱️rate/🦀️.rs"]
            pub mod rate;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️.rs"]
            pub mod scene;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔤️text/🦀️.rs"]
            pub mod text;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️.rs"]
            pub mod video;
        }

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🀄️add-tile/🦀️.rs"]
            pub mod add_tile;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️canvas-pointer-down/🦀️.rs"]
            pub mod canvas_pointer_down;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️clear-tiles/🦀️.rs"]
            pub mod clear_tiles;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️copy-prompt/🦀️.rs"]
            pub mod copy_prompt;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🀄️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🀄️delete-tile/🦀️.rs"]
            pub mod delete_tile;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⌨️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⌨️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️export-video-from-deck/🦀️.rs"]
            pub mod export_video_from_deck;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️no-operation/🦀️.rs"]
            pub mod no_operation;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🀄️patch-tile-crops/🦀️.rs"]
            pub mod patch_tile_crops;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🀄️rename-tiles/🦀️.rs"]
            pub mod rename_tiles;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️reset-grid/🦀️.rs"]
            pub mod reset_grid;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️seed-grid/🦀️.rs"]
            pub mod seed_grid;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️set-frame/🦀️.rs"]
            pub mod set_frame;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️set-source/🦀️.rs"]
            pub mod set_source;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod main {
                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🖊️main/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🖊️main/🪟️windows/🖼️tile-editor/🦀️.rs"]
                    pub mod tile_editor;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️.rs"]
            pub mod artifact;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod animate {
        #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️tile-editor/🦀️.rs"]
                    pub mod tile_editor;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::AnimateApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_presentation_demo_session;
    pub use crate::artifacts::presentation::examples::demo as art_presentation_demo;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
