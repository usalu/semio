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
                pub mod schema {
                    pub use semio_s_artifact_wfc_grid2d_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_wfc_grid2d_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::inferences;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod mask_cell {
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::mask_cell::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_cuts_the_pinned_corner_out_of_the_problem;
                        }
                        #[path = "."]
                        pub mod unmask_cell {
                            pub use semio_s_artifact_wfc_grid2d_inferences::standards::v1::subsets::any::schema::mutations::unmask_cell::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🔳️unmask-cell/🧪️tests/🔳️puts/🦀️.rs"]
                            pub mod tests_puts_the_hole_back_into_the_problem;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_wfc_grid2d_diff::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_wfc_grid2d_diff::examples::*;
    #[path = "."]
    pub mod grid2d {
        pub use semio_s_artifact_wfc_grid2d_diff::examples::grid2d::*;
        #[path = "."]
        pub mod terrain {
            pub use semio_s_artifact_wfc_grid2d_diff::examples::grid2d::terrain::*;
            #[cfg(test)]
            #[path = "../../../../../../../../📚️examples/🏜️terrain/🧪️tests/🧩️example/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod pipes {
            pub use semio_s_artifact_wfc_grid2d_diff::examples::grid2d::pipes::*;
            #[cfg(test)]
            #[path = "../../../../../../../../📚️examples/🚰️pipes/🧪️tests/🧩️example/🦀️.rs"]
            pub mod tests;
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod grid2d {
        #[path = "../../../../../../../../✏️editor/🪟️window/🦀️.rs"]
        pub mod window;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🔲️grid/🦀️.rs"]
                    pub mod grid;
                    #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
                #[path = "."]
                pub mod tools {
                    #[path = "."]
                    pub mod fill {
                        #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🛠️tools/🪣fill/🦀️.rs"]
                        mod component;
                        pub use component::*;
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
        #[path = "../../../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_wfc_grid2d_create_tile::viewer::grid2d::modes::view::*;
                pub use semio_s_artifact_wfc_grid2d_diff::viewer::grid2d::modes::view::windows;
            }
        }
    }
}
#[path = "."]
pub mod host {
    #[path = "."]
    pub mod inferences {
        pub use semio_s_artifact_wfc_grid2d_inferences::host::inferences::*;
        #[cfg(test)]
        #[path="../../../../../../../../../../../../🔨️modules/🏠️host/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
