//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::map_composition;
                pub use semio_s_artifact_gis_gismap_tests::standards::v1::subsets::any::inference_service;
                #[path = "."]
                pub mod artifact_declaration {
                    #[path = "../../../../../../../../🪪️artifact-declaration/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_gis_gismap_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_gis_gismap_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::schema::mutations::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_gis_gismap_schema::standards::v1::subsets::any::io::binary;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_gis_gismap_tests::standards::v1::subsets::any::io::text::*;
                        pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                    pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::sqlite;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any;
                                        pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::geojson;
                                    }
                                }
                                pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::import::deserializers::artifacts::dwg;
                                pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::import::deserializers::artifacts::dxf;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::export::serializers::artifacts::svg;
                                pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::export::serializers::artifacts::pdf;
                                pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::export::serializers::artifacts::png;
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any;
                                        pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::geojson;
                                    }
                                }
                                pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::export::serializers::artifacts::dwg;
                                pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::export::serializers::artifacts::dxf;
                            }
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
        pub use semio_s_artifact_gis_gismap_inferences::editor::gis2d::constants_and_io;
        pub use semio_s_artifact_gis_gismap_snapshot::editor::gis2d::terminology;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod map {
                        #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod config {
                            pub use semio_s_artifact_gis_gismap_config::editor::gis2d::modes::edit::windows::map::config::*;
                        }
                        pub use semio_s_artifact_gis_gismap_tests::editor::gis2d::modes::edit::windows::map::options;
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
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_gis_gismap_tests::viewer::gismap::modes::view::*;
                pub use semio_s_artifact_gis_gismap_windows::viewer::gismap::modes::view::windows;
            }
        }
    }
}
