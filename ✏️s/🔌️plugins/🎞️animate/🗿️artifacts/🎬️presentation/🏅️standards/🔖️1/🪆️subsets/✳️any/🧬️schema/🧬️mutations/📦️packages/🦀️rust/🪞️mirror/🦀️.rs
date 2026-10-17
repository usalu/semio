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
                    pub use semio_s_artifact_animate_presentation_tile_grid::standards::v1::subsets::any::schema::tile_grid;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::schema::diff::*;
                        pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod resize_source_frame {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_source_frame::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_source_frame::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_source_frame::inverse;
                            #[cfg(test)]
                            #[path = "../../../🔲resize-source-frame/🧪️tests/🧪️no/🦀️.rs"]
                            pub mod tests_no_ops_when_the_frame_is_already_identical;
                        }
                        #[path = "."]
                        pub mod replace_source {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::replace_source::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::replace_source::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::replace_source::inverse;
                            #[cfg(test)]
                            #[path = "../../../🖼️replace-source/🧪️tests/🧪️no/🦀️.rs"]
                            pub mod tests_no_ops_when_the_source_is_already_identical;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::create_tile::inverse;
                            #[cfg(test)]
                            #[path = "../../../🆕create-tile/🧪️tests/🧪️rejects/🦀️.rs"]
                            pub mod tests_rejects_a_duplicate_tile_id;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tile::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tile::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tile::inverse;
                            #[cfg(test)]
                            #[path = "../../../🗑️delete-tile/🧪️tests/🧪️rejects/🦀️.rs"]
                            pub mod tests_rejects_deleting_a_missing_tile;
                        }
                        #[path = "."]
                        pub mod delete_tiles {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tiles::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tiles::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::delete_tiles::inverse;
                            #[cfg(test)]
                            #[path = "../../../🧹delete-tiles/🧪️tests/🧪️rejects/🦀️.rs"]
                            pub mod tests_rejects_when_every_addressed_tile_is_missing;
                        }
                        #[path = "."]
                        pub mod rename_tile {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::rename_tile::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::rename_tile::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::rename_tile::inverse;
                            #[cfg(test)]
                            #[path = "../../../✏️rename-tile/🧪️tests/🧪️no/🦀️.rs"]
                            pub mod tests_no_ops_when_the_tile_already_has_that_name;
                        }
                        #[path = "."]
                        pub mod resize_tile_crop {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_tile_crop::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_tile_crop::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::resize_tile_crop::inverse;
                            #[cfg(test)]
                            #[path = "../../../✂️resize-tile-crop/🧪️tests/🧪️rejects/🦀️.rs"]
                            pub mod tests_rejects_a_zero_width_crop;
                        }
                        #[path = "."]
                        pub mod reorder_tiles {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::reorder_tiles::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::reorder_tiles::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::reorder_tiles::inverse;
                            #[cfg(test)]
                            #[path = "../../../🔀reorder-tiles/🧪️tests/🧪️no/🦀️.rs"]
                            pub mod tests_no_ops_when_the_tile_is_already_at_that_index;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_animate_presentation_viewer::standards::v1::subsets::any::io::text::*;
                        pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_animate_presentation_viewer::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_animate_presentation_viewer::standards::v1::subsets::any::io::import::deserializers::artifacts::pdf;
                                pub use semio_s_artifact_animate_presentation_viewer::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_animate_presentation_viewer::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_animate_presentation_viewer::standards::v1::subsets::any::io::export::serializers::artifacts::pdf;
                                pub use semio_s_artifact_animate_presentation_tests::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_animate_presentation_inferences::diff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::mutations::*;
    }

}
pub use semio_s_artifact_animate_presentation_inferences::snapshot;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        #[path = "."]
        pub mod engine {
            #[path = "../../../../../✏️editor/⚙️engine/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod scene {
                pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::scene::section;
            }
            pub use semio_s_artifact_animate_presentation_inferences::editor::animate::engine::config;
            pub use semio_s_artifact_animate_presentation_video::editor::animate::engine::video;
        }
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_animate_presentation_tile_grid::editor::animate::config::*;
        }
        #[path = "../../../../../✏️editor/🧰️support/🦀️.rs"]
        pub mod support;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../✏️editor/🎮️commands/🀄️add-tile/🦀️.rs"]
            pub mod add_tile;
            #[path = "../../../../../✏️editor/🎮️commands/👇️canvas-pointer-down/🦀️.rs"]
            pub mod canvas_pointer_down;
            #[path = "../../../../../✏️editor/🎮️commands/🧹️clear-tiles/🦀️.rs"]
            pub mod clear_tiles;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod animate {
        #[path = "../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_animate_presentation_tests::viewer::animate::modes::view::*;
                pub use semio_s_artifact_animate_presentation_tests::viewer::animate::modes::view::windows;
            }
        }
    }
}
