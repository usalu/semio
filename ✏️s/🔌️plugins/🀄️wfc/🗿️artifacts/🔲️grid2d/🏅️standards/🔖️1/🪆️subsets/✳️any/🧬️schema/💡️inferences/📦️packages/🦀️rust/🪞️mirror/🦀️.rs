//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod host {
    #[path="../../../../../../../../../🔨️modules/🏠️host/💡️inferences/🦀️.rs"]
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
                    pub use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🎲️change-seed/🧪️tests/🎲️reseeds/🦀️.rs"]
                            pub mod tests_reseeds_the_solve_from_7_to_99;
                        }
                        #[path = "."]
                        pub mod resize_grid {
                            #[path = "../../../../🧬️mutations/📐️resize-grid/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/📐️resize-grid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/📐️resize-grid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/📐️resize-grid/🧪️tests/📐️shrinks/🦀️.rs"]
                            pub mod tests_shrinks_the_board_and_drops_the_outside_cells;
                        }
                        #[path = "."]
                        pub mod change_cell_size {
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_cell_size::*;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_cell_size::diff;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_cell_size::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/📏️change-cell-size/🧪️tests/📏️widens-every-cell/🦀️.rs"]
                            pub mod tests_widens_every_cell;
                        }
                        #[path = "."]
                        pub mod change_periodicity {
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_periodicity::*;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_periodicity::diff;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_periodicity::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🔁️change-periodicity/🧪️tests/🔁️wraps-the-x-axis/🦀️.rs"]
                            pub mod tests_wraps_the_x_axis;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_tile::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🌱️create-tile/🧪️tests/🌱️inserts/🦀️.rs"]
                            pub mod tests_inserts_the_corner_tile_in_sorted_order;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_tile::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_tile::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_tile::inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_weight::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_weight::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_weight::inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_media::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_media::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::change_tile_media::inverse;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_rule::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_rule::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_rule::inverse;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_rule::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_rule::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::delete_rule::inverse;
                        }
                        #[path = "."]
                        pub mod pin_cell {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::pin_cell::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::pin_cell::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::pin_cell::inverse;
                        }
                        #[path = "."]
                        pub mod unpin_cell {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::unpin_cell::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::unpin_cell::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::unpin_cell::inverse;
                        }
                        #[path = "."]
                        pub mod mask_cell {
                            #[path = "../../../../🧬️mutations/🕳️mask-cell/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/🕳️mask-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/🕳️mask-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod unmask_cell {
                            #[path = "../../../../🧬️mutations/🔳️unmask-cell/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/🔳️unmask-cell/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/🔳️unmask-cell/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path="../../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                }
            }
        }
    }
}
