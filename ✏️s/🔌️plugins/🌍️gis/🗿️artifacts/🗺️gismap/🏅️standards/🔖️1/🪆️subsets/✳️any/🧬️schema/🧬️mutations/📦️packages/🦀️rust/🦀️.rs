//! 🧬️ mut-any of `semio-s-artifact-gis-gismap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

pub use crate::schema::snapshot::GisMapSnapshot;
pub use crate::schema::feature::MapFeaturePatch;
pub use crate::schema::feature::MapFeature;
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
                        pub mod set_position_property {
                            #[path = "../../📌set-position-property/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../📌set-position-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../📌set-position-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_position_property {
                            #[path = "../../🧲remove-position-property/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🧲remove-position-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧲remove-position-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod set_route_property {
                            #[path = "../../🚏set-route-property/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🚏set-route-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🚏set-route-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_route_property {
                            #[path = "../../🔪remove-route-property/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔪remove-route-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🔪remove-route-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod set_region_property {
                            #[path = "../../🗾set-region-property/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗾set-region-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🗾set-region-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_region_property {
                            #[path = "../../🧨remove-region-property/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🧨remove-region-property/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧨remove-region-property/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod reorder_routes {
                            #[path = "../../🧭reorder-routes/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🧭reorder-routes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧭reorder-routes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_region {
                            #[path = "../../🌐create-region/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🌐create-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🌐create-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_region {
                            #[path = "../../🧹delete-region/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🧹delete-region/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🧹delete-region/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
