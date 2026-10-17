//! ⚙️ operations of `semio-s-artifact-gis-gisterrain` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::GisTerrainSnapshot;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_gis_gisterrain_inferences::{GIS_3D_TERRAIN_SCHEMA};

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
                    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::imported_map;
                    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_gis_gisterrain_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "../../🦀️.rs"]
                    pub mod operations;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod change_exaggeration {
                            #[path = "../../../🧬️mutations/🎚️change-exaggeration/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧬️mutations/🎚️change-exaggeration/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/🎚️change-exaggeration/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/🎚️change-exaggeration/🧪️tests/⛰️raises/🦀️.rs"]
                            pub mod tests_raises_the_exaggeration_from_one_to_two_and_a_half;
                        }
                        #[path = "."]
                        pub mod change_imported_features {
                            #[path = "../../../🧬️mutations/📥change-imported-features/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧬️mutations/📥change-imported-features/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/📥change-imported-features/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/📥change-imported-features/🧪️tests/📥️imports/🦀️.rs"]
                            pub mod tests_imports_a_single_harbor_position_descriptor;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_gis_gisterrain_inferences::diff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::mutations::*;
    }

}
