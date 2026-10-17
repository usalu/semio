//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod host {
    pub use semio_s_artifact_wfc_grid2d_inferences::host::inferences;
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_grid2d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::inferences;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_seed::inverse;
                        }
                        #[path = "."]
                        pub mod resize_grid {
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::resize_grid::*;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::resize_grid::diff;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::resize_grid::inverse;
                        }
                        #[path = "."]
                        pub mod change_cell_size {
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_cell_size::*;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_cell_size::diff;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_cell_size::inverse;
                        }
                        #[path = "."]
                        pub mod change_periodicity {
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_periodicity::*;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_periodicity::diff;
                            pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::mutations::change_periodicity::inverse;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_wfc_grid2d_create_tile::standards::v1::subsets::any::schema::mutations::create_tile::inverse;
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
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::mask_cell::*;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::mask_cell::diff;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::mask_cell::inverse;
                        }
                        #[path = "."]
                        pub mod unmask_cell {
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::unmask_cell::*;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::unmask_cell::diff;
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::unmask_cell::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::standards::v1::subsets::any::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_wfc_grid2d_diff::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod grid2d {
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod window {
            pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::editor::grid2d::window::*;
            #[cfg(test)]
            #[path = "../../../🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::editor::grid2d::modes::edit::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::editor::grid2d::modes::edit::windows::*;
                    #[path = "."]
                    pub mod preview {
                        pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::editor::grid2d::modes::edit::windows::preview::*;
                        #[cfg(test)]
                        #[path = "../../../🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod tools {
                    pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::editor::grid2d::modes::edit::tools::*;
                    #[path = "."]
                    pub mod fill {
                        pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::editor::grid2d::modes::edit::tools::fill::*;
                        #[cfg(test)]
                        #[path = "../../../🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣fill/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod grid2d {
        pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::viewer::grid2d::*;
    }
}
