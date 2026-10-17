//! 🌍️ tile-coordinates of `semio-s-artifact-gis-gismap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

pub use crate::schema::snapshot::GisMapSnapshot;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

#[path = "."]
pub mod tile_coordinates {
    #[path = "../../🦀️.rs"]
    pub mod gis_policy;
}
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
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::snapshot;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
            }
        }
    }
}
