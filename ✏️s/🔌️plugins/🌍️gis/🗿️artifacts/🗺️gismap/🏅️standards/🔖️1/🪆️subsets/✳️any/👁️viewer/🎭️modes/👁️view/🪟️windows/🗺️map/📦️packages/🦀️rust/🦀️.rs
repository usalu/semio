//! 🗺️ windows of `semio-s-artifact-gis-gismap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::schema::snapshot::GisMapSnapshot;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

#[path = "."]
pub mod tile_coordinates {
    pub use semio_s_artifact_gis_gismap_tile_coordinates::tile_coordinates::gis_policy;
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
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod gismap {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "."]
                pub mod windows {
                    #[path = "../../🦀️.rs"]
                    pub mod map;
                }
            }
        }
    }
}
