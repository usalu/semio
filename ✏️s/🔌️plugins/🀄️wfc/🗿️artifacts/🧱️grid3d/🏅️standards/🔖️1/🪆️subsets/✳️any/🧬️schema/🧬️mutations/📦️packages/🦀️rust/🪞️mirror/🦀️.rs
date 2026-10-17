//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
                #[path = "."]
                pub mod examples {
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::examples::blocks;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::inverse;
                            #[cfg(test)]
                            #[path = "../../../🎲️change-seed/🧪️tests/🎲️reseeds/🦀️.rs"]
                            pub mod tests_reseeds_the_solve_from_7_to_99;
                        }
                        #[path = "."]
                        pub mod resize_grid {
                            #[path = "../../../📐️resize-grid/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../📐️resize-grid/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../📐️resize-grid/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../📐️resize-grid/🧪️tests/📐️grows-the-grid-to-3x2x2/🦀️.rs"]
                            pub mod tests_grows_the_grid_to_3x2x2;
                        }
                        #[path = "."]
                        pub mod change_cell_sizes {
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_cell_sizes::*;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_cell_sizes::diff;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_cell_sizes::inverse;
                            #[cfg(test)]
                            #[path = "../../../📏️change-cell-sizes/🧪️tests/📏️stretches/🦀️.rs"]
                            pub mod tests_stretches_the_x_axis_columns;
                        }
                        #[path = "."]
                        pub mod change_periodicity {
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_periodicity::*;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_periodicity::diff;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_periodicity::inverse;
                            #[cfg(test)]
                            #[path = "../../../🔁️change-periodicity/🧪️tests/🔁️wraps-the-x-axis/🦀️.rs"]
                            pub mod tests_wraps_the_x_axis;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            #[path = "../../../🧱️create-tile/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧱️create-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧱️create-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../🧱️create-tile/🧪️tests/🧱️adds-the-roof-tile/🦀️.rs"]
                            pub mod tests_adds_the_roof_tile;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            #[path = "../../../🕳️delete-tile/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🕳️delete-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🕳️delete-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../🕳️delete-tile/🧪️tests/🕳️removes/🦀️.rs"]
                            pub mod tests_removes_the_air_tile_and_cascades;
                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_weight::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_weight::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_weight::inverse;
                            #[cfg(test)]
                            #[path = "../../../⚖️change-tile-weight/🧪️tests/⚖️raises/🦀️.rs"]
                            pub mod tests_raises_the_wall_tile_bias;
                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::inverse;
                            #[cfg(test)]
                            #[path = "../../../🖼️change-tile-media/🧪️tests/🖼️replaces/🦀️.rs"]
                            pub mod tests_replaces_the_wall_tile_mesh;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::inverse;
                            #[cfg(test)]
                            #[path = "../../../🚦️create-rule/🧪️tests/🚦️allows-air-above-air/🦀️.rs"]
                            pub mod tests_allows_air_above_air;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::inverse;
                            #[cfg(test)]
                            #[path = "../../../❌️delete-rule/🧪️tests/❌️removes/🦀️.rs"]
                            pub mod tests_removes_the_floor_wall_rule;
                        }
                        #[path = "."]
                        pub mod pin_cell {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::pin_cell::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::pin_cell::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::pin_cell::inverse;
                            #[cfg(test)]
                            #[path = "../../../📌️pin-cell/🧪️tests/📌️pins-the-far-cell-to-wall/🦀️.rs"]
                            pub mod tests_pins_the_far_cell_to_wall;
                        }
                        #[path = "."]
                        pub mod unpin_cell {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unpin_cell::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unpin_cell::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unpin_cell::inverse;
                        }
                        #[path = "."]
                        pub mod mask_cell {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::mask_cell::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::mask_cell::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::mask_cell::inverse;
                        }
                        #[path = "."]
                        pub mod unmask_cell {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unmask_cell::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unmask_cell::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::unmask_cell::inverse;
                        }
                    }
                }
            }
        }
    }
}
