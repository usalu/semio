//! 🌀️ Procedural plugin — 2D and 3D flow apps bundled as one hot-swappable WASM component.
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
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
// 🧩️ `wfc_engine`'s copied-verbatim `🗺️topology`/`🎲️sample`/`🔮️oracle`/… leaves reference these two
// by the exact same aliases `semio-framework-math` used before this wave — the legal plugin→framework
// dependency direction, not a new coupling.
extern crate semio_framework_geometry as geometry;
extern crate semio_framework_graph as graph_core;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<...Mutation, ...ConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod generation2d {
        #[path = "../../🗿️artifacts/🌀️generation2d/🦀️.rs"]
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
                            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod clear_widget_layout {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹clear-widget-layout/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹clear-widget-layout/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹clear-widget-layout/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹clear-widget-layout/🧪️tests/drops-the-note-a-layout-entry/🦀️.rs"]
                                    mod tests_drops_the_note_a_layout_entry;
                                }
                                #[path = "."]
                                pub mod disconnect_synapse {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🧪️tests/severs-link-ab-leaving-both-notes/🦀️.rs"]
                                    mod tests_severs_link_ab_leaving_both_notes;
                                }
                                #[path = "."]
                                pub mod delete_widget {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/🧪️tests/removes-note-a-and-flags-the-dangling-synapse/🦀️.rs"]
                                    mod tests_removes_note_a_and_flags_the_dangling_synapse;
                                }
                                #[path = "."]
                                pub mod set_camera {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-camera/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-camera/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-camera/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-camera/🧪️tests/pans-and-zooms-the-graph-camera/🦀️.rs"]
                                    mod tests_pans_and_zooms_the_graph_camera;
                                }
                                #[path = "."]
                                pub mod move_widget {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/🧪️tests/repositions-note-a-on-the-canvas/🦀️.rs"]
                                    mod tests_repositions_note_a_on_the_canvas;
                                }
                                #[path = "."]
                                pub mod change_schema {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/🧪️tests/restamps-the-fixture-schema/🦀️.rs"]
                                    mod tests_restamps_the_fixture_schema;
                                }
                                #[path = "."]
                                pub mod connect_synapse {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-synapse/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-synapse/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-synapse/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-synapse/🧪️tests/joins-note-b-to-note-c-at-index-1/🦀️.rs"]
                                    mod tests_joins_note_b_to_note_c_at_index_1;
                                }
                                #[path = "."]
                                pub mod replace_widget {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-widget/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-widget/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-widget/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-widget/🧪️tests/rewrites-the-note-b-body-in-place/🦀️.rs"]
                                    mod tests_rewrites_the_note_b_body_in_place;
                                }
                                #[path = "."]
                                pub mod replace_synapse {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-synapse/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-synapse/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-synapse/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-synapse/🧪️tests/repoints-link-ab-onto-the-alt-port/🦀️.rs"]
                                    mod tests_repoints_link_ab_onto_the_alt_port;
                                }
                                #[path = "."]
                                pub mod create_generation {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-generation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-generation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-generation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-generation/🧪️tests/appends-generation-2-and-selects-it/🦀️.rs"]
                                    mod tests_appends_generation_2_and_selects_it;
                                }
                                #[path = "."]
                                pub mod delete_generation {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-generation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-generation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-generation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-generation/🧪️tests/removes-the-selected-generation-2-and-falls-back-to-generation-1/🦀️.rs"]
                                    mod tests_removes_the_selected_generation_2_and_falls_back_to_generation_1;
                                }
                                #[path = "."]
                                pub mod rename_generation {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-generation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-generation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-generation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-generation/🧪️tests/retitles-generation-1/🦀️.rs"]
                                    mod tests_retitles_generation_1;
                                }
                                #[path = "."]
                                pub mod change_generation_value {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-generation-value/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-generation-value/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-generation-value/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-generation-value/🧪️tests/raises-the-height-answer-in-generation-1/🦀️.rs"]
                                    mod tests_raises_the_height_answer_in_generation_1;
                                }
                                #[path = "."]
                                pub mod create_widget {
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-widget/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-widget/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-widget/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-widget/🧪️tests/inserts-note-c-at-index-2/🦀️.rs"]
                                    mod tests_inserts_note_c_at_index_2;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️.rs"]
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
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️.rs"]
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
            pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::diff::Generation2dDiff;
        pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::mutations::Generation2dMutation;
        pub use crate::artifacts::generation2d::standards::v1::subsets::any::schema::snapshot::Generation2dSnapshot;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod generation3d {
        #[path = "../../🗿️artifacts/🧊️generation3d/🦀️.rs"]
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
                            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod delete_widget_position {
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-widget-position/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-widget-position/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-widget-position/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-widget-position/🧪️tests/unpins-the-node-a-position/🦀️.rs"]
                                    mod tests_unpins_the_node_a_position;
                                }
                                #[path = "."]
                                pub mod disconnect_synapse {
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🧪️tests/cuts-wire-ab-leaving-both-nodes/🦀️.rs"]
                                    mod tests_cuts_wire_ab_leaving_both_nodes;
                                }
                                #[path = "."]
                                pub mod delete_widget {
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/🧪️tests/removes-node-a-and-leaves-wire-ab-dangling/🦀️.rs"]
                                    mod tests_removes_node_a_and_leaves_wire_ab_dangling;
                                }
                                #[path = "."]
                                pub mod update_camera {
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷update-camera/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷update-camera/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷update-camera/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷update-camera/🧪️tests/frames-the-graph-at-double-zoom/🦀️.rs"]
                                    mod tests_frames_the_graph_at_double_zoom;
                                }
                                #[path = "."]
                                pub mod move_widget {
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-widget/🧪️tests/repositions-node-a-in-the-graph/🦀️.rs"]
                                    mod tests_repositions_node_a_in_the_graph;
                                }
                                #[path = "."]
                                pub mod change_schema {
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤change-schema/🧪️tests/restamps-the-fixture-schema-id/🦀️.rs"]
                                    mod tests_restamps_the_fixture_schema_id;
                                }
                                #[path = "."]
                                pub mod update_synapse {
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄update-synapse/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄update-synapse/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄update-synapse/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄update-synapse/🧪️tests/repoints-wire-ab-onto-the-cap-port/🦀️.rs"]
                                    mod tests_repoints_wire_ab_onto_the_cap_port;
                                }
                                #[path = "."]
                                pub mod update_widget {
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/🧪️tests/retunes-the-knob-slider-value/🦀️.rs"]
                                    mod tests_retunes_the_knob_slider_value;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
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
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
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
            pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::diff::Generation3dDiff;
        pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
        pub use crate::artifacts::generation3d::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshot;
    }

    #[path = "."]
    pub mod assembly {
        #[path = "../../🗿️artifacts/🧩️assembly/🦀️.rs"]
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
                            #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                            pub mod snapshot;
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod create_slot {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-slot/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-slot/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-slot/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-slot/🧪️tests/appends-slot-c-at-index-2/🦀️.rs"]
                                    mod tests_appends_slot_c_at_index_2;
                                }
                                #[path = "."]
                                pub mod delete_slot {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-slot/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-slot/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-slot/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-slot/🧪️tests/removes-slot-a-and-cascades-edge-ab/🦀️.rs"]
                                    mod tests_removes_slot_a_and_cascades_edge_ab;
                                }
                                #[path = "."]
                                pub mod create_rule {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-rule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-rule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-rule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-rule/🧪️tests/appends-a-rule-forbidding-roof-over-wall/🦀️.rs"]
                                    mod tests_appends_a_rule_forbidding_roof_over_wall;
                                }
                                #[path = "."]
                                pub mod delete_rule {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-rule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-rule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-rule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-rule/🧪️tests/removes-the-wall-roof-rule/🦀️.rs"]
                                    mod tests_removes_the_wall_roof_rule;
                                }
                                #[path = "."]
                                pub mod change_weight {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-weight/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-weight/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-weight/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-weight/🧪️tests/raises-the-wall-module-selection-bias/🦀️.rs"]
                                    mod tests_raises_the_wall_module_selection_bias;
                                }
                                #[path = "."]
                                pub mod remove_weight {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-weight/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-weight/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-weight/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-weight/🧪️tests/drops-the-wall-module-weight-override/🦀️.rs"]
                                    mod tests_drops_the_wall_module_weight_override;
                                }
                                #[path = "."]
                                pub mod connect_slots {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-slots/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-slots/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-slots/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-slots/🧪️tests/joins-slot-b-to-slot-c-at-index-1/🦀️.rs"]
                                    mod tests_joins_slot_b_to_slot_c_at_index_1;
                                }
                                #[path = "."]
                                pub mod disconnect_slots {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-slots/🧪️tests/severs-edge-ab-leaving-both-slots/🦀️.rs"]
                                    mod tests_severs_edge_ab_leaving_both_slots;
                                }
                                #[path = "."]
                                pub mod change_seed {
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲change-seed/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲change-seed/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲change-seed/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲change-seed/🧪️tests/reseeds-the-solve-from-7-to-99/🦀️.rs"]
                                    mod tests_reseeds_the_solve_from_7_to_99;
                                }
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: flat access from the artifact root, mirroring generation2d/generation3d ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod diff {
            pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::diff::*;
        }
        pub mod mutations {
            pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod inferences {
            pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::inferences::*;
        }
        pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::diff::AssemblyDiff;
        pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::mutations::AssemblyMutation;
        pub use crate::artifacts::assembly::standards::v1::subsets::any::schema::snapshot::AssemblySnapshot;
    }
}
//#endregion 🗿️Artifacts

//#region 🧩️WfcEngine
#[path = "."]
pub(crate) mod wfc_engine {
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔦️beam/🦀️.rs"]
    pub(crate) mod beam;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🎛️bitset/🦀️.rs"]
    pub mod bitset;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🍰️chunk/🦀️.rs"]
    pub(crate) mod chunk;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⛓️constraint/🦀️.rs"]
    pub mod constraint;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔢️constraints-card/🦀️.rs"]
    pub mod constraints_card;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔗️constraints-conn/🦀️.rs"]
    pub mod constraints_conn;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🩺️diag/🦀️.rs"]
    pub mod diag;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🌐️domain/🦀️.rs"]
    pub mod domain;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⚠️error/🦀️.rs"]
    pub mod error;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧬️evolve/🦀️.rs"]
    pub mod evolve;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⛏️extract/🦀️.rs"]
    pub mod extract;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🌊️flow/🦀️.rs"]
    pub mod flow;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔲️grid-2d/🦀️.rs"]
    pub mod grid2d;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧊️grid-3d/🦀️.rs"]
    pub mod grid3d;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧭️heuristics/🦀️.rs"]
    pub mod heuristics;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪜️hierarchy/🦀️.rs"]
    pub(crate) mod hierarchy;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🆔️ids/🦀️.rs"]
    pub mod ids;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️job/🦀️.rs"]
    pub mod job;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🏗️model/🦀️.rs"]
    pub mod model;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🎼️motif/🦀️.rs"]
    pub(crate) mod motif;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🚫️nogood/🦀️.rs"]
    pub(crate) mod nogood;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔮️oracle/🦀️.rs"]
    pub mod oracle;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🏁️outcome/🦀️.rs"]
    pub mod outcome;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️parallel/🦀️.rs"]
    pub(crate) mod parallel;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔁️prop-ac3/🦀️.rs"]
    pub(crate) mod prop_ac3;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔄️prop-ac4/🦀️.rs"]
    pub(crate) mod prop_ac4;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/📣️propagate/🦀️.rs"]
    pub(crate) mod propagate;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔧️repair/🦀️.rs"]
    pub(crate) mod repair;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🎲️sample/🦀️.rs"]
    pub mod sample;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔍️search/🦀️.rs"]
    pub mod search;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/💾️serial/🦀️.rs"]
    pub mod serial;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪶️soft/🦀️.rs"]
    pub mod soft;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🕸️solver-graph/🦀️.rs"]
    pub mod solver_graph;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔳️solver-grid-2d/🦀️.rs"]
    pub mod solver_grid2d;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧱️solver-grid-3d/🦀️.rs"]
    pub mod solver_grid3d;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🕳️sparse-3d/🦀️.rs"]
    pub mod sparse3d;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪞️symmetry/🦀️.rs"]
    pub mod symmetry;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🀄️tiled/🦀️.rs"]
    pub mod tiled;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🗺️topology/🦀️.rs"]
    pub(crate) mod topology;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🐾️trail/🦀️.rs"]
    pub(crate) mod trail;
    #[path = "../../🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/⚖️weights/🦀️.rs"]
    pub mod weights;
}
//#endregion 🧩️WfcEngine

//#region ✏️Editor
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod generation2d {
        #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️add-generation/🦀️.rs"]
            pub mod add_generation;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️add-widget/🦀️.rs"]
            pub mod add_widget;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️canvas-pointer-down/🦀️.rs"]
            pub mod canvas_pointer_down;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️canvas-pointer-move/🦀️.rs"]
            pub mod canvas_pointer_move;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️canvas-pointer-up/🦀️.rs"]
            pub mod canvas_pointer_up;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️canvas-wheel/🦀️.rs"]
            pub mod canvas_wheel;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️connect-media-ports/🦀️.rs"]
            pub mod connect_media_ports;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️enter-generate/🦀️.rs"]
            pub mod enter_generate;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-tick/🦀️.rs"]
            pub mod flow_eval_tick;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️move-media-node/🦀️.rs"]
            pub mod move_media_node;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs"]
            pub mod node_graph_edit;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️remove-generation/🦀️.rs"]
            pub mod remove_generation;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️remove-widget/🦀️.rs"]
            pub mod remove_widget;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️rename-generation/🦀️.rs"]
            pub mod rename_generation;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️reorganize/🦀️.rs"]
            pub mod reorganize;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️select-generation/🦀️.rs"]
            pub mod select_generation;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️set-eval-outputs/🦀️.rs"]
            pub mod set_eval_outputs;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-show-mode/🦀️.rs"]
            pub mod set_show_mode;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️update-generation-values/🦀️.rs"]
            pub mod update_generation_values;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs"]
                    pub mod flow;
                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }

            #[path = "."]
            pub mod generate {
                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs"]
                    pub mod form;
                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs"]
                    pub mod generations;
                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod generation3d {
        #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️add-generation/🦀️.rs"]
            pub mod add_generation;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️add-widget/🦀️.rs"]
            pub mod add_widget;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-resolve/🦀️.rs"]
            pub mod flow_eval_resolve;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-eval-tick/🦀️.rs"]
            pub mod flow_eval_tick;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️flow-tessellate-resolve/🦀️.rs"]
            pub mod flow_tessellate_resolve;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️graph-pointer-down/🦀️.rs"]
            pub mod graph_pointer_down;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️move-media-node/🦀️.rs"]
            pub mod move_media_node;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs"]
            pub mod node_graph_edit;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️patch-flow-widgets/🦀️.rs"]
            pub mod patch_flow_widgets;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️remove-generation/🦀️.rs"]
            pub mod remove_generation;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️remove-widget/🦀️.rs"]
            pub mod remove_widget;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️rename-generation/🦀️.rs"]
            pub mod rename_generation;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️reorganize/🦀️.rs"]
            pub mod reorganize;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️rotate-selection/🦀️.rs"]
            pub mod rotate_selection;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️scale-selection/🦀️.rs"]
            pub mod scale_selection;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️select-generation/🦀️.rs"]
            pub mod select_generation;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-active-utility/🦀️.rs"]
            pub mod set_active_utility;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-lod-mode/🦀️.rs"]
            pub mod set_lod_mode;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-show-mode/🦀️.rs"]
            pub mod set_show_mode;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌞️set-sun-azimuth/🦀️.rs"]
            pub mod set_sun_azimuth;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌞️set-sun-elevation/🦀️.rs"]
            pub mod set_sun_elevation;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌞️set-sun-intensity/🦀️.rs"]
            pub mod set_sun_intensity;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌞️toggle-sun/🦀️.rs"]
            pub mod toggle_sun;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️translate-selection/🦀️.rs"]
            pub mod translate_selection;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️update-generation-values/🦀️.rs"]
            pub mod update_generation_values;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️world-pointer-down/🦀️.rs"]
            pub mod world_pointer_down;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs"]
                    pub mod flow;
                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }

            #[path = "."]
            pub mod generate {
                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs"]
                    pub mod form;
                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs"]
                    pub mod generations;
                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }

    // 🚧️ assembly's ✏️editor is authored on disk (`🗿️artifacts/🧩️assembly/…/✏️editor/`) but NOT mounted
    // here: `impl ArtifactEditor for AssemblyEditor` cannot satisfy the trait's own bounds
    // (`Snapshot: ArtifactDsl + ArtifactPack`, `Mutation: OpText + OpBinary`) until assembly's schema
    // gains its missing artifact-facet descriptor + JSON-Schema/GraphQL/Protobuf leaves (see
    // `📓️w2-p5-assembly-notes.md`'s "Blocking gap" — confirmed by a real `cargo check` attempt, not
    // just the theoretical typestate reading that report already gave). Mount once that lands.
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod generation2d {
        #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }

    #[path = "."]
    pub mod generation3d {
        #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }

    // 🚧️ assembly's 👁️viewer is authored on disk (`🗿️artifacts/🧩️assembly/…/👁️viewer/`) but NOT mounted
    // here, for the same reason as the sibling ✏️editor block above — see its comment.
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
pub(crate) use plugin::{generation_form, generation_tree, scene_surface, ui_label, ui_node_list, ui_text, ui_value_list, ui_value_map, ui_value_text};

#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::ProceduralApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_2d_demo_session;
    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_3d_demo_session;
    #[path = "../../🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_generation2d_demo;
    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️box-fillet-preview/🦀️.rs"]
    pub mod art_generation3d_box_fillet_preview;
    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️box-shell-preview/🦀️.rs"]
    pub mod art_generation3d_box_shell_preview;
    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️face-sweep-extrude/🦀️.rs"]
    pub mod art_generation3d_face_sweep_extrude;
    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️rectangle-extrude-volume/🦀️.rs"]
    pub mod art_generation3d_rectangle_extrude_volume;
    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️rectangle-wire-preview/🦀️.rs"]
    pub mod art_generation3d_rectangle_wire_preview;
    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️sphere-box-fuse/🦀️.rs"]
    pub mod art_generation3d_sphere_box_fuse;
    #[path = "../../🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️sphere-cut-with-torus/🦀️.rs"]
    pub mod art_generation3d_sphere_cut_with_torus;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
