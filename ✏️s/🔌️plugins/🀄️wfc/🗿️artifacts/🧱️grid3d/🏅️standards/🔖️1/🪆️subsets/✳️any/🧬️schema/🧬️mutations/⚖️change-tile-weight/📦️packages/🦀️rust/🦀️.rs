//! ⚖️ mut-family of `semio-s-artifact-wfc-grid3d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod examples {
    pub use crate::standards::v1::subsets::any::examples::*;
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
                pub mod examples {
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::examples::blocks;
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::examples::pipes_3d;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::io::text::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_tile_weight {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            #[path = "../../../🖼️change-tile-media/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🖼️change-tile-media/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🖼️change-tile-media/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            #[path = "../../../🚦️create-rule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🚦️create-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🚦️create-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[path = "../../../❌️delete-rule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../❌️delete-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../❌️delete-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod pin_cell {
                            #[path = "../../../📌️pin-cell/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../📌️pin-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../📌️pin-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod unpin_cell {
                            #[path = "../../../📍️unpin-cell/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../📍️unpin-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../📍️unpin-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod mask_cell {
                            #[path = "../../../🚫️mask-cell/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🚫️mask-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🚫️mask-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod unmask_cell {
                            #[path = "../../../🔓️unmask-cell/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔓️unmask-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔓️unmask-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
