//! 📐️ mut-family of `semio-s-artifact-wfc-bitmap` — a path-preserving carve of the artifact root mounts.

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
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod resize_input {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod set_input_pixels {
                            #[path = "../../../🖌️set-input-pixels/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🖌️set-input-pixels/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🖌️set-input-pixels/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod add_palette_color {
                            #[path = "../../../🎨️add-palette-color/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🎨️add-palette-color/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🎨️add-palette-color/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_palette_color {
                            #[path = "../../../🖍️change-palette-color/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🖍️change-palette-color/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🖍️change-palette-color/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_palette_color {
                            #[path = "../../../🧽️remove-palette-color/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧽️remove-palette-color/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧽️remove-palette-color/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_model {
                            #[path = "../../../⚙️change-model/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../⚙️change-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../⚙️change-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bitmap {
        #[path = "."]
        pub mod transient {
            #[path = "../../../../../✏️editor/🫧️transient/🚪️io/🦀️.rs"]
            pub mod io;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bitmap {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
