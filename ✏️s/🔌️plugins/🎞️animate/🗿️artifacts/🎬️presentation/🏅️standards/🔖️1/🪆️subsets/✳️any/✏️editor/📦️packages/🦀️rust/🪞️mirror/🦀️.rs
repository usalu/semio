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
                    pub use semio_s_artifact_animate_presentation_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_animate_presentation_tile_grid::standards::v1::subsets::any::schema::tile_grid;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_animate_presentation_mutations::standards::v1::subsets::any::schema::mutations::*;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_animate_presentation_mutations::mutations;
pub use semio_s_artifact_animate_presentation_inferences::snapshot;
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_animate_presentation_examples::examples::demo::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_animate_presentation_mutations::editor::animate::engine::*;
        }
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_animate_presentation_tile_grid::editor::animate::config::*;
            pub use semio_s_artifact_animate_presentation_viewer::editor::animate::config::schema;
        }
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_animate_presentation_viewer::editor::animate::presence::*;
        }
        #[path = "../../../🗣️terminology/🦀️.rs"]
        pub mod terminology;
        pub use semio_s_artifact_animate_presentation_mutations::editor::animate::support;
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod add_tile {
                pub use semio_s_artifact_animate_presentation_mutations::editor::animate::commands::add_tile::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🀄️add-tile/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod canvas_pointer_down {
                pub use semio_s_artifact_animate_presentation_mutations::editor::animate::commands::canvas_pointer_down::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/👇️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_animate_presentation_mutations::editor::animate::commands::clear_tiles;
            #[path = "."]
            pub mod copy_prompt {
                pub use semio_s_artifact_animate_presentation_schema::editor::animate::commands::copy_prompt::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/📋️copy-prompt/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod delete_selection {
                pub use semio_s_artifact_animate_presentation_schema::editor::animate::commands::delete_selection::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🚮️delete-selection/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_animate_presentation_schema::editor::animate::commands::delete_tile;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::engagement_input;
            #[path = "."]
            pub mod engagement_submit {
                pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::engagement_submit::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/📤️engagement-submit/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod export_video_from_deck {
                pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::export_video_from_deck::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🎥️export-video-from-deck/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::no_operation;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::patch_tile_crops;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::rename_tiles;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::reset_grid;
            #[path = "."]
            pub mod seed_grid {
                pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::seed_grid::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/🌱️seed-grid/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::set_active_example;
            pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::set_frame;
            #[path = "."]
            pub mod set_source {
                pub use semio_s_artifact_animate_presentation_host::editor::animate::commands::set_source::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/📥️set-source/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod main {
                pub use semio_s_artifact_animate_presentation_tests::editor::animate::modes::main::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_animate_presentation_tests::editor::animate::modes::main::windows::*;
                    #[path = "."]
                    pub mod tile_editor {
                        pub use semio_s_artifact_animate_presentation_tests::editor::animate::modes::main::windows::tile_editor::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/🖊️main/🪟️windows/🖼️tile-editor/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../📌️panels/🗿️artifact/🦀️.rs"]
            pub mod artifact;
            #[path = "../../../📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../../📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}
pub use semio_s_artifact_animate_presentation_schema::host;
