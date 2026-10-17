//! 🧬️ schema of `semio-s-artifact-gis-gismap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

pub use crate::schema::snapshot::GisMapSnapshot;
pub use crate::schema::feature::MapFeature;
pub use crate::schema::feature::MapFeaturePatch;
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
                pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::map_composition;
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::feature;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::drawing_bridge;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::snapshot;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
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
                            #[path = "."]
                            pub mod io {
                                #[path = "../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🚪️io/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🚪️io/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧬️schema/🦀️.rs"]
                            pub mod schema;
                        }
                    }
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
        pub mod presence {
            #[path = "../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
