//! ✏️ editor of `semio-s-artifact-gis-gisterrain` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::schema::snapshot::GisTerrainSnapshot;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_gis_gisterrain_inferences::{GISTERRAIN_DIALECT, GIS_3D_TERRAIN_SCHEMA};

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
                    pub use semio_s_artifact_gis_gisterrain_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::standards::v1::subsets::any::schema::terrain_descriptor;
                    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::imported_map;
                    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_gis_gisterrain_operations::standards::v1::subsets::any::schema::mutations::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_gis_gisterrain_operations::mutations;
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::examples::demo::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gis3d {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_gis_gisterrain_terrain_descriptor::editor::gis3d::presence::*;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../🎮️commands/🎨️example/🦀️.rs"]
            pub mod example;
            #[path = "../../🎮️commands/🏔️exaggeration/🦀️.rs"]
            pub mod exaggeration;
            #[path = "../../🎮️commands/👁️view/🦀️.rs"]
            pub mod view;

        #[cfg(test)]
        #[path = "../../🎮️commands/🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::editor::gis3d::modes::view::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::editor::gis3d::modes::view::windows::*;
                    #[path = "."]
                    pub mod terrain {
                        pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::editor::gis3d::modes::view::windows::terrain::*;
                        #[cfg(test)]
                        #[path = "../../🎭️modes/👁️view/🪟️windows/🏔️terrain/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        #[path = "."]
                        pub mod config {
                            pub use semio_s_artifact_gis_gisterrain_relocated_engine_tests::editor::gis3d::modes::view::windows::terrain::config::*;
                            #[cfg(test)]
                            #[path = "../../🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
