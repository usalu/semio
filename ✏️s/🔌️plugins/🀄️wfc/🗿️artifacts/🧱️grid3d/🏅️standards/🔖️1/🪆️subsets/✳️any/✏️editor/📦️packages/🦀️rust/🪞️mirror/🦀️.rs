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
                    pub mod text {
                        #[path = "."]
                        pub mod inferences {
                            #[path = "."]
                            pub mod scene_projection {
                                pub use semio_s_artifact_wfc_grid3d_inferences::standards::v1::subsets::any::io::text::inferences::scene_projection::*;
                                #[cfg(test)]
                                #[path="../../../../🚪️io/📝️text/💡️inferences/🌐️scene-projection/🧪️tests/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_wfc_grid3d_inferences::standards::v1::subsets::any::schema::scene_internals;
                    pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod change_seed {
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::*;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::diff;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_seed::inverse;
                        }
                        #[path = "."]
                        pub mod resize_grid {
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::resize_grid::*;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::resize_grid::diff;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::resize_grid::inverse;
                        }
                        #[path = "."]
                        pub mod change_cell_sizes {
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_cell_sizes::*;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_cell_sizes::diff;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_cell_sizes::inverse;
                        }
                        #[path = "."]
                        pub mod change_periodicity {
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_periodicity::*;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_periodicity::diff;
                            pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::schema::mutations::change_periodicity::inverse;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::create_tile::inverse;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::*;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::diff;
                            pub use semio_s_artifact_wfc_grid3d_mutations::standards::v1::subsets::any::schema::mutations::delete_tile::inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_weight {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_weight::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_weight::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_weight::inverse;
                        }
                        #[path = "."]
                        pub mod change_tile_media {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::change_tile_media::inverse;
                        }
                        #[path = "."]
                        pub mod create_rule {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::create_rule::inverse;
                        }
                        #[path = "."]
                        pub mod delete_rule {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::delete_rule::inverse;
                        }
                        #[path = "."]
                        pub mod pin_cell {
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::pin_cell::*;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::pin_cell::diff;
                            pub use semio_s_artifact_wfc_grid3d_mut_family::standards::v1::subsets::any::schema::mutations::pin_cell::inverse;
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
                    pub use semio_s_artifact_wfc_grid3d_inferences::standards::v1::subsets::any::schema::inferences;
                }
                pub use semio_s_artifact_wfc_grid3d_snapshot::standards::v1::subsets::any::examples;
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod grid3d {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "../../../🪟️window/🦀️.rs"]
        pub mod window;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod tools {
                    #[path = "."]
                    pub mod fill {
                        #[path = "../../../🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod windows {
                    #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧱️grid/🦀️.rs"]
                    pub mod grid;
                    #[path = "../../../🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod grid3d {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }
}
#[path = "."]
pub mod host {
    #[path = "."]
    pub mod inferences {
        pub use semio_s_artifact_wfc_grid3d_inferences::host::inferences::*;
        #[cfg(test)]
        #[path="../../../../../../../../🔨️modules/🏠️host/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
