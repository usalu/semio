//! 🗺️ GIS map artifact — the document entity the 2d app edits (constitutional: general).

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

/// 📸️ Persisted GIS map snapshot — defined in `📸️ snapshot/🧬️ schema`, re-exported here.
pub use crate::schema::snapshot::GisMapSnapshot;

pub use semio_s_artifact_gis_gismap_inferences::{GISMAP_DIALECT, GIS_MAP_SCHEMA};

pub use crate::schema::feature::{MapFeature, MapFeaturePatch};

pub use crate::standards::v1::subsets::any::map_composition::*;

pub use crate::standards::v1::subsets::any::inference_service::*;

pub use crate::standards::v1::subsets::any::artifact_declaration::*;

//#region 🔹Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🦀️.rs"]
mod schema_document_contract;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🔬️window/🦀️.rs"]
mod map_window_ownership;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod demo_example_contract;
//#endregion 🔹Tests

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

                pub use semio_s_artifact_gis_gismap_gismap::standards::v1::subsets::any::artifact_declaration;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_gis_gismap_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_gis_gismap_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod create_position {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_position::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_position::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_position::inverse;

                        }
                        #[path = "."]
                        pub mod delete_position {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_position::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_position::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_position::inverse;

                        }
                        #[path = "."]
                        pub mod replace_position_data {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_position_data::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_position_data::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_position_data::inverse;

                        }
                        #[path = "."]
                        pub mod reorder_positions {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::reorder_positions::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::reorder_positions::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::reorder_positions::inverse;

                        }
                        #[path = "."]
                        pub mod create_route {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_route::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_route::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_route::inverse;

                        }
                        #[path = "."]
                        pub mod delete_route {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_route::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_route::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_route::inverse;

                        }
                        #[path = "."]
                        pub mod replace_route_data {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_route_data::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_route_data::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_route_data::inverse;

                        }
                        #[path = "."]
                        pub mod set_position_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_position_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_position_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_position_property::inverse;

                        }
                        #[path = "."]
                        pub mod remove_position_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_position_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_position_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_position_property::inverse;

                        }
                        #[path = "."]
                        pub mod set_route_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_route_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_route_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_route_property::inverse;

                        }
                        #[path = "."]
                        pub mod remove_route_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_route_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_route_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_route_property::inverse;

                        }
                        #[path = "."]
                        pub mod set_region_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_region_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_region_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_region_property::inverse;

                        }
                        #[path = "."]
                        pub mod remove_region_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_region_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_region_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_region_property::inverse;

                        }
                        #[path = "."]
                        pub mod reorder_routes {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::reorder_routes::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::reorder_routes::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::reorder_routes::inverse;

                        }
                        #[path = "."]
                        pub mod create_region {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::create_region::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::create_region::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::create_region::inverse;

                        }
                        #[path = "."]
                        pub mod delete_region {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::delete_region::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::delete_region::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::delete_region::inverse;

                        }
                        #[path = "."]
                        pub mod replace_region_data {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_region_data::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_region_data::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_region_data::inverse;

                        }
                        #[path = "."]
                        pub mod reorder_regions {
                            pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::schema::mutations::reorder_regions::*;
                            pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::schema::mutations::reorder_regions::diff;
                            pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::schema::mutations::reorder_regions::inverse;

                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_gis_gismap_gismap::standards::v1::subsets::any::io::*;

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
                                pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
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

// ---- Flat access from the artifact root ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod snapshot {
    pub use crate::standards::v1::subsets::any::schema::snapshot::*;
}
pub use crate::standards::v1::subsets::any::schema::diff::GisMapDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;

pub use semio_s_artifact_gis_gismap_snapshot::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gis2d {
        pub use semio_s_artifact_gis_gismap_gis2d::editor::gis2d::*;

        pub use semio_s_artifact_gis_gismap_inferences::editor::gis2d::maphost;
        pub use semio_s_artifact_gis_gismap_snapshot::editor::gis2d::terminology;
        pub use semio_s_artifact_gis_gismap_snapshot::editor::gis2d::wasm;

        #[path = "."]
        pub mod examples {
            pub use semio_s_artifact_gis_gismap_snapshot::editor::gis2d::examples::demo_session;

        }

        pub use semio_s_artifact_gis_gismap_gis2d::editor::gis2d::commands;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_gis_gismap_gismap::editor::gis2d::modes::edit::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod map {
                        pub use semio_s_artifact_gis_gismap_gismap::editor::gis2d::modes::edit::windows::map::*;

                        #[path = "."]
                        pub mod config {
                            pub use semio_s_artifact_gis_gismap_config::editor::gis2d::modes::edit::windows::map::config::*;
pub mod io {
    pub use semio_s_artifact_gis_gismap_mut_family::editor::gis2d::modes::edit::windows::map::config::io::*;
    pub use semio_s_artifact_gis_gismap_schema::editor::gis2d::modes::edit::windows::map::config::io::binary;
    pub mod text {
        pub use semio_s_artifact_gis_gismap_schema::editor::gis2d::modes::edit::windows::map::config::io::text::*;
        pub use semio_s_artifact_gis_gismap_config::editor::gis2d::modes::edit::windows::map::config::io::text::mutations;
    }
}

                            pub use semio_s_artifact_gis_gismap_schema::editor::gis2d::modes::edit::windows::map::config::schema;
                        }

                        pub use semio_s_artifact_gis_gismap_tests::editor::gis2d::modes::edit::windows::map::options;
                    }
                }
            }
        }

        pub use semio_s_artifact_gis_gismap_gis2d::editor::gis2d::panels;
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod gismap {
        pub use semio_s_artifact_gis_gismap_gismap::viewer::gismap::*;

        pub use semio_s_artifact_gis_gismap_schema::viewer::gismap::presence;
        pub use semio_s_artifact_gis_gismap_schema::viewer::gismap::config;

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

#[path = "💡️inference/🔌️client/🦀️.rs"]
pub mod inference_client;

#[path = "💡️inference/🧬️schema/🦀️.rs"]
pub mod inference_schema;

#[cfg(not(target_arch = "wasm32"))]
#[path = "💡️inference/👷️worker/🦀️.rs"]
pub mod inference_worker;

#[path = "🗺️tiles/🦀️.rs"]
pub mod tile_coordinates;

pub use crate::standards::v1::subsets::any::io::{GismapBuilderConstruction, GisMapParts, GisMapAnalyzerAnalysis, GismapBuilderFacets, GismapBuilder, GisMapAnalyzer, GisMapComposer};

pub use semio_s_artifact_gis_gismap_tests::host;

pub use semio_s_artifact_gis_gismap_snapshot::{gis_map_inference_demands};

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_1;
#[cfg(test)]
#[path = "🗺️tiles/🌍️gis-policy/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_2;
