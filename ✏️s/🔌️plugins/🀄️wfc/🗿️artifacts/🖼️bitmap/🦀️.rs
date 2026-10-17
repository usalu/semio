//! 🖼️ WFC bitmap artifact — the classic OVERLAPPING model: an authored input bitmap is learned as a
//! universe of `N × N` patterns, and an output bitmap is collapsed so that every one of its `N × N`
//! windows is a pattern the input actually contains. The document persists only the PROBLEM (input
//! sample and palette, output extent and periodicity, model parameters, pins, seed); the collapsed
//! bitmap, the contradiction verdict and the entropy map are an INFERENCE
//! (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`), never persisted state.
//!
//! The solver itself is not here: it is the shared `semio-s-plugin-wfc-engine` crate every artifact
//! of this plugin calls. This crate is a `🧬️schema` plus a `🚪️io` system plus two surfaces — never
//! an engine.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::WFC_BITMAP_DOCUMENT_SCHEMA;

use crate::schema::snapshot::{BitmapColor, BitmapPinnedPixel};

pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::{WFC_BITMAP_DIALECT};

pub use semio_s_artifact_wfc_bitmap_bitmap::{artifact_kind};

pub use semio_s_artifact_wfc_bitmap_snapshot::{BITMAP_LAYERS_JSON_BUDGET_BYTES, bitmap_layers_at_block, bitmap_layers_json, bitmap_rect_layer};

pub use semio_s_artifact_wfc_bitmap_bitmap::{ArtifactApps, artifact, definition, pilot_languages};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_wfc_bitmap_bitmap::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_wfc_bitmap_bitmap::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_bitmap_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::inverse;

                        }
                        #[path = "."]
                        pub mod resize_input {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::resize_input::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::resize_input::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::resize_input::inverse;

                        }
                        #[path = "."]
                        pub mod set_input_pixels {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::inverse;

                        }
                        #[path = "."]
                        pub mod add_palette_color {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::add_palette_color::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::add_palette_color::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::add_palette_color::inverse;

                        }
                        #[path = "."]
                        pub mod change_palette_color {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_palette_color::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_palette_color::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_palette_color::inverse;

                        }
                        #[path = "."]
                        pub mod remove_palette_color {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::remove_palette_color::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::remove_palette_color::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::remove_palette_color::inverse;

                        }
                        #[path = "."]
                        pub mod resize_output {
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::resize_output::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::resize_output::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::resize_output::inverse;

                        }
                        #[path = "."]
                        pub mod change_model {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_model::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_model::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::change_model::inverse;

                        }
                        #[path = "."]
                        pub mod pin_pixel {
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::pin_pixel::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::pin_pixel::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::pin_pixel::inverse;

                        }
                        #[path = "."]
                        pub mod unpin_pixel {
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::unpin_pixel::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::unpin_pixel::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::unpin_pixel::inverse;

                        }
                        #[path = "."]
                        pub mod paint_input_stroke {
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::paint_input_stroke::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::paint_input_stroke::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family_2::standards::v1::subsets::any::schema::mutations::paint_input_stroke::inverse;

                        }
                    }
                    pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::schema::inferences;
                }

                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_wfc_bitmap_bitmap::standards::v1::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::sqlite::snapshot;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bitmap {
        pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::*;

        pub use semio_s_artifact_wfc_bitmap_mut_family::editor::bitmap::presence;
        pub use semio_s_artifact_wfc_bitmap_mut_family::editor::bitmap::config;

        pub mod transient {
    pub use semio_s_artifact_wfc_bitmap_transient::editor::bitmap::transient::*;
    pub mod io {
        pub use semio_s_artifact_wfc_bitmap_mut_family::editor::bitmap::transient::io::*;
        pub mod binary {
            pub use semio_s_artifact_wfc_bitmap_snapshot::editor::bitmap::transient::io::binary::*;
            pub use semio_s_artifact_wfc_bitmap_transient::editor::bitmap::transient::io::binary::mutations;
        }
        pub mod text {
            pub use semio_s_artifact_wfc_bitmap_snapshot::editor::bitmap::transient::io::text::*;
            pub use semio_s_artifact_wfc_bitmap_transient::editor::bitmap::transient::io::text::mutations;
        }
    }
}

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::editor::bitmap::commands::set_active_example;
            pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::editor::bitmap::commands::pin_solution;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::*;

                pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::tools;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::windows::input;
                    pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::windows::output;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bitmap {
        pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::viewer::bitmap::*;

        pub use semio_s_artifact_wfc_bitmap_mut_family::viewer::bitmap::presence;
        pub use semio_s_artifact_wfc_bitmap_mut_family::viewer::bitmap::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_wfc_bitmap_snapshot::viewer::bitmap::modes::view::*;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_wfc_bitmap_snapshot::viewer::bitmap::modes::view::windows::input;
                    pub use semio_s_artifact_wfc_bitmap_snapshot::viewer::bitmap::modes::view::windows::output;
                }
            }
        }
    }
}

//#region 📚️Examples
/// 📚️ The bundled bitmap problems this subset ships — one compact three-colour plan, one taller
/// four-colour meadow with a ground constraint. Each slug owns its own directory under
/// `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`, and the Rust builder there is the authority its
/// `🗣️.dsl.semio` asset is printed from.
pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::examples;
//#endregion 📚️Examples

/// 🪞️ Flat access from the artifact root, mirroring the sibling wfc artifacts: the subset's schema
/// tree reached as `crate::schema`, with the shims below doing the same for diff, mutations and
/// inferences.
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod inferences {
    pub use crate::standards::v1::subsets::any::schema::inferences::*;
    pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
}
pub use crate::standards::v1::subsets::any::schema::diff::BitmapDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::BitmapMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::BitmapSnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️store-fixture/🦀️.rs"]
mod store_fixture;
//#endregion 🧪️Tests

#[path="🔨️modules/🏠️host/🦀️.rs"]
pub mod host;

pub use host::inferences::*;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
