//! 💡️ inferences of `semio-s-artifact-animate-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::snapshot::schema::PresentationSnapshot;
pub use crate::snapshot::schema::default_snapshot;
pub use crate::snapshot::schema::demo_snapshot;
pub use semio_s_artifact_animate_presentation_tile_grid::{FigureTileDraft, FigureTileFrame, FigureTileSource};
pub use crate::editor::animate::engine::camera::artifact_items::{PRESENTATION_DOCUMENT_SCHEMA};
pub use crate::editor::animate::engine::camera::artifact_items_4::{DEMO_FIGURE_SRC};
pub use crate::editor::animate::engine::camera::artifact_items_5::{DEMO_FIGURE_PIXEL_WIDTH};
pub use crate::editor::animate::engine::camera::artifact_items_6::{DEMO_FIGURE_PIXEL_HEIGHT};
pub use crate::editor::animate::engine::camera::artifact_items_7::{default_figure_tile_source};
pub use crate::editor::animate::engine::camera::artifact_items_10::{AnimationChild, PresentationChild, animation_child_handle, presentation_child_handle, presentation_snapshot_from_source_tiles, source_tiles_from_presentation_snapshot};
pub use crate::editor::animate::engine::camera::artifact_items_11::{PRESENTATION_CANVAS_FIT_EXTENT, PRESENTATION_CANVAS_SCALE, genesis_presentation_child_pack, presentation_canvas_camera, presentation_snapshot_with_tiles, presentation_working_scene};

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
                    pub mod snapshot {
                        #[path = "../../../📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod topology {
                            #[path = "../../🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
        }
    }
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }

}
pub mod snapshot {
    pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }

}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        #[path = "."]
        pub mod engine {
            #[path = "."]
            pub mod scene {
                pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::scene::sobject;
            }
            #[path = "../../../../✏️editor/⚙️engine/📷️camera/🦀️.rs"]
            pub mod camera;
            #[path = "../../../../✏️editor/⚙️engine/🎛️config/🦀️.rs"]
            pub mod config;
            pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::geometry;
            pub use semio_s_artifact_animate_presentation_examples::editor::animate::engine::text;
        }
    }
}
