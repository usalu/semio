//! 🧬️ schema of `semio-s-artifact-animate-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::snapshot::schema::default_snapshot;
pub use crate::snapshot::schema::PresentationSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
pub use crate::snapshot::schema::demo_snapshot;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::diff::PresentationDiff;
pub use semio_s_artifact_animate_presentation_tile_grid::{FigureTileDraft, FigureTileFrame, FigureTileSource};
pub use semio_s_artifact_animate_presentation_inferences::{AnimationChild, PRESENTATION_DOCUMENT_SCHEMA, PresentationChild, default_figure_tile_source, presentation_snapshot_with_tiles};

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
                    pub use semio_s_artifact_animate_presentation_tile_grid::standards::v1::subsets::any::schema::tile_grid;
                    pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_animate_presentation_mutations::standards::v1::subsets::any::schema::mutations::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_animate_presentation_inferences::standards::v1::subsets::any::io::text::snapshot;
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_animate_presentation_mutations::standards::v1::subsets::any::io::binary::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
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
        pub mod config {
            pub use semio_s_artifact_animate_presentation_tile_grid::editor::animate::config::*;
        }
        pub use semio_s_artifact_animate_presentation_mutations::editor::animate::support;
        #[path = "."]
        pub mod commands {
            #[path = "../../../✏️editor/🎮️commands/📋️copy-prompt/🦀️.rs"]
            pub mod copy_prompt;
            #[path = "../../../✏️editor/🎮️commands/🚮️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "../../../✏️editor/🎮️commands/🗑️delete-tile/🦀️.rs"]
            pub mod delete_tile;
        }
    }
}
#[path = "."]
pub mod host {
#[path = "../../../../../../../🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
