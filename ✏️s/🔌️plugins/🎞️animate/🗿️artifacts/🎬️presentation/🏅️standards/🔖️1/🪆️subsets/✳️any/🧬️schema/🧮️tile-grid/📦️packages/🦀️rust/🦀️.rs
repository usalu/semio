//! 🧮️ tile-grid of `semio-s-artifact-animate-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::editor::animate::config::artifact_items_3::{FigureTileDraft, FigureTileFrame, FigureTileSource};
pub use crate::editor::animate::config::artifact_items_13::{FigureTileDraftPatch};

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
                    pub mod tile_grid;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        #[path = "."]
        pub mod config {
            #[path = "../../../../✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod io {
                #[path = "."]
                pub mod text {
                    #[path = "../../../../✏️editor/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
                #[path = "."]
                pub mod binary {
                    #[path = "../../../../✏️editor/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
        }
        #[path = "."]
        pub mod presence {
            #[path = "."]
            pub mod io {
                #[path = "../../../../✏️editor/👥️presence/🚪️io/💾️binary/🦀️.rs"]
                pub mod binary;
                #[path = "../../../../✏️editor/👥️presence/🚪️io/📝️text/🦀️.rs"]
                pub mod text;
            }
        }
    }
}
