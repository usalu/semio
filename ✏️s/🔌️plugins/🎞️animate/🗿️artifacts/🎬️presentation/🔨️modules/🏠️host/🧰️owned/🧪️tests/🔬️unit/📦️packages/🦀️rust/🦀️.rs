//! 🔬️ host of `semio-s-artifact-animate-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

pub use crate::snapshot::schema::PresentationSnapshot;
pub use crate::snapshot::schema::default_snapshot;
pub use crate::snapshot::schema::demo_snapshot;
pub use semio_s_artifact_animate_presentation_tile_grid::{FigureTileDraft, FigureTileFrame, FigureTileSource};
pub use semio_s_artifact_animate_presentation_inferences::{PRESENTATION_DOCUMENT_SCHEMA, presentation_working_scene};
pub use semio_s_artifact_animate_presentation_tests::{default_presentation_snapshot};
pub use crate::editor::animate::commands::engagement_input::artifact_items_9::{demo_presentation_snapshot};

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
                        #[path = "."]
                        pub mod create_tile {
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::create_tile::*;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::create_tile::diff;
                            pub use semio_s_artifact_animate_presentation_mut_family::standards::v1::subsets::any::schema::mutations::create_tile::inverse;
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_animate_presentation_mutations::mutations;
pub use semio_s_artifact_animate_presentation_inferences::snapshot;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_animate_presentation_mutations::editor::animate::engine::*;
        }
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_animate_presentation_tile_grid::editor::animate::config::*;
        }
        pub use semio_s_artifact_animate_presentation_mutations::editor::animate::support;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⌨️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️export-video-from-deck/🦀️.rs"]
            pub mod export_video_from_deck;
            #[path = "../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏸️no-operation/🦀️.rs"]
            pub mod no_operation;
            #[path = "../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️patch-tile-crops/🦀️.rs"]
            pub mod patch_tile_crops;
            #[path = "../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-tiles/🦀️.rs"]
            pub mod rename_tiles;
            #[path = "../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↩️reset-grid/🦀️.rs"]
            pub mod reset_grid;
            #[path = "../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱️seed-grid/🦀️.rs"]
            pub mod seed_grid;
            #[path = "../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️set-frame/🦀️.rs"]
            pub mod set_frame;
            #[path = "../../../../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️set-source/🦀️.rs"]
            pub mod set_source;
        }
    }
}
#[path = "."]
pub mod host {
    pub use semio_s_artifact_animate_presentation_schema::host::*;
    #[path = "."]
    pub mod owned {
        pub use semio_s_artifact_animate_presentation_schema::host::owned::*;
        #[cfg(test)]
        #[path = "../../🦀️.rs"]
        pub mod tests;
    }
}
