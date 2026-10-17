//! 💡️ inferences of `semio-s-artifact-wfc-bitmap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::mutations::BitmapMutation;
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}

#[path = "."]
pub mod host {
    #[path="../../../../../../../../🔨️modules/🏠️host/💡️inferences/🦀️.rs"]
    pub mod inferences;
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
                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
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
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
