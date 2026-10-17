//! 💡️ inferences of `semio-s-artifact-wfc-3d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::standards::v1::subsets::any::schema::mutations::Wfc3dMutation;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
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
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_3d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod drag_slots {
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::*;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::diff;
                            pub use semio_s_artifact_wfc_3d_mut_family::standards::v1::subsets::any::schema::mutations::drag_slots::inverse;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/🚦️create-rule/🧪️tests/🚦️forbids/🦀️.rs"]
                            pub mod tests_forbids_two_rooms_side_by_side;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/🚫️delete-rule/🧪️tests/🚫️drops/🦀️.rs"]
                            pub mod tests_drops_the_room_corridor_pairing;
                        }
                        #[path = "."]
                        pub mod change_seed {
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/🎲️change-seed/🧪️tests/🎲️reseeds/🦀️.rs"]
                            pub mod tests_reseeds_the_solve_from_7_to_99;
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
                    pub mod text {
                        #[path="../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod wfc3d {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod tools {
                    #[path = "../../../../✏️editor/🎭️modes/✏️edit/🛠️tools/✋️drag/🦀️.rs"]
                    pub mod drag;
                }
            }
        }
    }
}
