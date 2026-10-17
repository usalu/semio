//! 🔲 mut-family of `semio-s-artifact-animate-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::snapshot::schema::PresentationSnapshot;
pub use crate::snapshot::schema::default_snapshot;
pub use crate::snapshot::schema::demo_snapshot;
pub use semio_s_artifact_animate_presentation_tile_grid::{FigureTileDraft, FigureTileFrame, FigureTileSource};
pub use semio_s_artifact_animate_presentation_inferences::{presentation_working_scene};

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
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod resize_source_frame {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_source {
                            #[path = "../../../🖼️replace-source/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🖼️replace-source/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🖼️replace-source/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_tile {
                            #[path = "../../../🆕create-tile/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🆕create-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🆕create-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_tile {
                            #[path = "../../../🗑️delete-tile/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🗑️delete-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🗑️delete-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_tiles {
                            #[path = "../../../🧹delete-tiles/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧹delete-tiles/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧹delete-tiles/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod rename_tile {
                            #[path = "../../../✏️rename-tile/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../✏️rename-tile/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../✏️rename-tile/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod resize_tile_crop {
                            #[path = "../../../✂️resize-tile-crop/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../✂️resize-tile-crop/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../✂️resize-tile-crop/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod reorder_tiles {
                            #[path = "../../../🔀reorder-tiles/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔀reorder-tiles/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔀reorder-tiles/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_animate_presentation_inferences::diff;
pub use semio_s_artifact_animate_presentation_inferences::snapshot;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        #[path = "."]
        pub mod engine {
            #[path = "../../../../../✏️editor/⚙️engine/🎬️scene/🦀️.rs"]
            pub mod scene;
        }
    }
}
