//! 🧬️ schema of `semio-s-artifact-wfc-grid2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;

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
                    pub use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod delete_tile {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🗑️delete-tile/🧪️tests/🗑️removes/🦀️.rs"]
                            pub mod tests_removes_the_straight_tile_and_cascades_its_rule_and_pin;
                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/⚖️change-tile-weight/🧪️tests/⚖️biases/🦀️.rs"]
                            pub mod tests_biases_the_solve_towards_empty;
                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🎨️change-tile-media/🧪️tests/🎨️redraws/🦀️.rs"]
                            pub mod tests_redraws_the_empty_tile_as_a_bitmap;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🚦️create-rule/🧪️tests/🚦️lets/🦀️.rs"]
                            pub mod tests_lets_two_straights_stack_vertically;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/❌delete-rule/🧪️tests/❌️forbids/🦀️.rs"]
                            pub mod tests_forbids_the_straight_pair_again;
                        }
                        #[path = "."]
                        pub mod pin_cell {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/📌️pin-cell/🧪️tests/📌️fixes/🦀️.rs"]
                            pub mod tests_fixes_the_right_cell_to_the_straight_tile;
                        }
                        #[path = "."]
                        pub mod unpin_cell {
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/📍️unpin-cell/🧪️tests/📍️releases/🦀️.rs"]
                            pub mod tests_releases_the_pinned_straight_cell;
                        }
                    }
                }
            }
        }
    }
}
