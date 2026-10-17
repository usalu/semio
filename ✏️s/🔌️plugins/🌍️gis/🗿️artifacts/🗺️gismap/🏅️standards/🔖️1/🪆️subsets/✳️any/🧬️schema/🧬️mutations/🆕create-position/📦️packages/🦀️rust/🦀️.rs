//! 🆕 mut-family of `semio-s-artifact-gis-gismap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

pub use crate::schema::snapshot::GisMapSnapshot;
pub use crate::schema::feature::MapFeature;
pub use crate::schema::feature::MapFeaturePatch;
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
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
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::feature;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_position {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_position {
                            #[path = "../../../🗑️delete-position/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🗑️delete-position/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🗑️delete-position/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_position_data {
                            #[path = "../../../🔁replace-position-data/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔁replace-position-data/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔁replace-position-data/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod reorder_positions {
                            #[path = "../../../🔀reorder-positions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔀reorder-positions/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔀reorder-positions/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_route {
                            #[path = "../../../🛣️create-route/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🛣️create-route/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🛣️create-route/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_route {
                            #[path = "../../../✂️delete-route/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../✂️delete-route/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../✂️delete-route/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_route_data {
                            #[path = "../../../♻️replace-route-data/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../♻️replace-route-data/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../♻️replace-route-data/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_region_data {
                            #[path = "../../../🔄replace-region-data/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔄replace-region-data/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔄replace-region-data/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gis2d {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod map {
                        #[path = "."]
                        pub mod config {
                            #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🚪️io/🦀️.rs"]
                            pub mod io;
                        }
                    }
                }
            }
        }
    }
}
