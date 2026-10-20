//! 🔬️ tests of `semio-s-artifact-wfc-bitmap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[cfg(test)]
pub use crate::schema::snapshot::WFC_BITMAP_DOCUMENT_SCHEMA;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::mutations::BitmapMutation;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::snapshot::BitmapSnapshot;
#[cfg(test)]
pub mod inferences {
    pub use crate::standards::v1::subsets::any::schema::inferences::*;
    pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
}
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
#[cfg(test)]
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::{WFC_BITMAP_DIALECT};
pub use semio_s_artifact_wfc_bitmap_bitmap::{artifact_kind};

#[path = "."]
pub mod host {
    pub use semio_s_artifact_wfc_bitmap_inferences::host::inferences;
}
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
                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod set_input_pixels {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::inverse;
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
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::text::snapshot;
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
        #[cfg(test)]
        #[path = "../../🦀️.rs"]
        pub mod tests;
        pub use semio_s_artifact_wfc_bitmap_transient::editor::bitmap::transient;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::editor::bitmap::commands::set_active_example;
        }
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::*;
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::*;
                #[path = "."]
                pub mod tools {
                    pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::tools::*;
                    #[path = "."]
                    pub mod fill {
                        pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::tools::fill::*;
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/✏️edit/🛠️tools/🌡️fill/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::windows::*;
                    #[path = "."]
                    pub mod input {
                        pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::windows::input::*;
                        #[path = "."]
                        pub mod utilities {
                            pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::windows::input::utilities::*;
                            #[path = "."]
                            pub mod brush {
                                pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::windows::input::utilities::brush::*;
                                #[cfg(test)]
                                #[path = "../../../🧪️brush-tool/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    pub use semio_s_artifact_wfc_bitmap_bitmap::editor::bitmap::modes::edit::windows::output;
                }
            }
        }
    }
}
pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::examples;
