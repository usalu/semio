//! 📸️ snapshot of `semio-s-artifact-gis-gismap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::schema::snapshot::GisMapSnapshot;
pub use crate::schema::feature::MapFeature;
pub use crate::schema::feature::MapFeaturePatch;
pub use crate::standards::v1::subsets::any::schema::diff::GisMapDiff;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use crate::editor::gis2d::terminology::artifact_items_2::{gis_map_inference_demands};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod map_composition {
                    #[path = "../../../../../../../../🧩️map-composition/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../../📍️feature/🦀️.rs"]
                    pub mod feature;
                    #[path = "../../../🖼️drawing-bridge/🦀️.rs"]
                    pub mod drawing_bridge;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
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
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "../../../../🚪️io/💾️binary/📸️snapshot/📦️pack/🦀️.rs"]
                            pub mod owned_pack;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gis2d {
        #[path = "../../../../✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "../../../../✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;
        #[path = "."]
        pub mod examples {
            #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
            pub mod demo_session;
            #[cfg(test)]
            #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
            pub mod demo_session_tests;
        }
    }
}
