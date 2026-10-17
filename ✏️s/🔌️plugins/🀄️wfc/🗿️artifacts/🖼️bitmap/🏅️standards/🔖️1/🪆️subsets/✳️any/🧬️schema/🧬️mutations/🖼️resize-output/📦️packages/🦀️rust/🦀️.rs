//! 🖼️ mut-family-2 of `semio-s-artifact-wfc-bitmap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
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
                        #[path = "."]
                        pub mod set_input_pixels {
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::*;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::diff;
                            pub use semio_s_artifact_wfc_bitmap_mut_family::standards::v1::subsets::any::schema::mutations::set_input_pixels::inverse;
                        }
                        #[path = "."]
                        pub mod resize_output {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod pin_pixel {
                            #[path = "../../../📌️pin-pixel/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../📌️pin-pixel/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../📌️pin-pixel/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod unpin_pixel {
                            #[path = "../../../📍️unpin-pixel/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../📍️unpin-pixel/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../📍️unpin-pixel/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod paint_input_stroke {
                            #[path = "../../../✍️paint-input-stroke/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../✍️paint-input-stroke/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../✍️paint-input-stroke/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::text::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
