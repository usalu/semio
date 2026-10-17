//! 🧬️ schema of `semio-s-artifact-wfc-bitmap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;

pub use crate::standards::v1::subsets::any::schema::mutations::BitmapMutation;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        #[path = "."]
                        pub mod change_seed {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🎲️change-seed/🧪️tests/🎲️reseeds/🦀️.rs"]
                            pub mod tests_reseeds_the_solve_from_7_to_99;
                        }
                        #[path = "."]
                        pub mod resize_input {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/📐️resize-input/🧪️tests/📐️grows/🦀️.rs"]
                            pub mod tests_grows_the_sample_to_6_by_4;
                        }
                        #[path = "."]
                        pub mod set_input_pixels {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🖌️set-input-pixels/🧪️tests/🖌️paints/🦀️.rs"]
                            pub mod tests_paints_a_2_by_2_block_of_colour_1;
                        }
                        #[path = "."]
                        pub mod add_palette_color {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🎨️add-palette-color/🧪️tests/🎨️appends/🦀️.rs"]
                            pub mod tests_appends_a_third_colour;
                        }
                        #[path = "."]
                        pub mod change_palette_color {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🖍️change-palette-color/🧪️tests/🖍️recolours/🦀️.rs"]
                            pub mod tests_recolours_the_second_entry;
                        }
                        #[path = "."]
                        pub mod remove_palette_color {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🧽️remove-palette-color/🧪️tests/🧽️drops/🦀️.rs"]
                            pub mod tests_drops_the_unused_third_colour;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_wfc_bitmap_inferences::standards::v1::subsets::any::io::text::*;
                        pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
            }
        }
    }
}
