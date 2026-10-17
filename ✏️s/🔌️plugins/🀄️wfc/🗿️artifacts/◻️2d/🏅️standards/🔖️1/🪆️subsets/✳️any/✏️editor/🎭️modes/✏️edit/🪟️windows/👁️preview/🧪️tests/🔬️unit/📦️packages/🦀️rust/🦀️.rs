//! 🔬️ tests of `semio-s-artifact-wfc-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

#[cfg(test)]
pub mod inferences {
    pub use crate::standards::v1::subsets::any::schema::inferences::*;
    pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
}

#[path = "."]
pub mod host {
    pub use semio_s_artifact_wfc_2d_inferences::host::inferences;
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
                    pub use semio_s_artifact_wfc_2d_inferences::standards::v1::subsets::any::schema::inferences;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wfc2d {
        pub use semio_s_artifact_wfc_2d_transient::editor::wfc2d::transient;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod preview {
                        pub use semio_s_artifact_wfc_2d_v1::editor::wfc2d::modes::edit::windows::preview::*;
                        #[cfg(test)]
                        #[path = "../../🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod tools {
                    #[path = "."]
                    pub mod fill {
                        pub use semio_s_artifact_wfc_2d_v1::editor::wfc2d::modes::edit::tools::fill::*;
                        #[cfg(test)]
                        #[path = "../../../../../../🛠️tools/🔣️fill/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::*;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::two_room_corridor;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::wall_roof_facade_strip;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::hex_ring;
    pub use semio_s_artifact_wfc_2d_pin_slot::examples::terrain_ring;
}
