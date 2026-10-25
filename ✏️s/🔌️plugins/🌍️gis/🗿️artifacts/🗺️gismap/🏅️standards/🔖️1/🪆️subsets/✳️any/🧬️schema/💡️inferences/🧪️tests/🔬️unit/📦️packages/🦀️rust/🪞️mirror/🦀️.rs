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
                #[path = "."]
                pub mod inference_service {
                    #[path = "../../../../../../../../../../../🧠️inference-service/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::feature;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::drawing_bridge;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_gis_gismap_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_gis_gismap_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                    }
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
                            #[cfg(test)]
                            #[path = "../../../../../../🧬️mutations/🧭reorder-routes/🧪️tests/🚌️moves/🦀️.rs"]
                            pub mod tests_moves_the_bus_route_to_the_front;
                        }
                        #[path = "."]
                        pub mod create_region {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::create_region::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::create_region::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::create_region::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🧬️mutations/🌐create-region/🧪️tests/🏘️adds/🦀️.rs"]
                            pub mod tests_adds_the_old_town_region_after_the_harbor_district;
                        }
                        #[path = "."]
                        pub mod delete_region {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::delete_region::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::delete_region::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::delete_region::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🧬️mutations/🧹delete-region/🧪️tests/🚫️removes-old-town-region/🦀️.rs"]
                            pub mod tests_removes_the_old_town_region;
                        }
                        #[path = "."]
                        pub mod replace_region_data {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_region_data::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_region_data::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_region_data::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🧬️mutations/🔄replace-region-data/🧪️tests/🏘️rewrites/🦀️.rs"]
                            pub mod tests_rewrites_the_harbor_district_region_payload;
                        }
                        #[path = "."]
                        pub mod reorder_regions {
                            pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::schema::mutations::reorder_regions::*;
                            pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::schema::mutations::reorder_regions::diff;
                            pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::schema::mutations::reorder_regions::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🧬️mutations/🔃reorder-regions/🧪️tests/🌳️moves/🦀️.rs"]
                            pub mod tests_moves_the_park_region_between_the_two_districts;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[cfg(test)]
                    #[path = "../../../../../../../🚪️io/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                    #[path = "../../../../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
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
        #[path = "."]
        pub mod maphost {
            pub use semio_s_artifact_gis_gismap_inferences::editor::gis2d::maphost::*;
            #[cfg(test)]
            #[path = "../../../../../../../✏️editor/🗺️maphost/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        pub use semio_s_artifact_gis_gismap_snapshot::editor::gis2d::terminology;
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
                            pub use semio_s_artifact_gis_gismap_config::editor::gis2d::modes::edit::windows::map::config::*;
                        }
                        #[path = "."]
                        pub mod options {
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/☑️options/📏️layer-weights/🦀️.rs"]
                            pub mod layer_weights;
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/☑️options/👁️layers/🦀️.rs"]
                            pub mod layers;
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/☑️options/🔽️lod/🦀️.rs"]
                            pub mod lod_mode;
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/☑️options/🖼️render-mode/🦀️.rs"]
                            pub mod render_mode;
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/☑️options/🎨️vector-style/🦀️.rs"]
                            pub mod vector_style;

                        #[cfg(test)]
                        #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/☑️options/🧪️tests/🔬️unit/🦀️.rs"]
                        mod leaf_tests;
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
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_gis_gismap_windows::viewer::gismap::modes::view::windows::*;
                    #[path = "."]
                    pub mod map {
                        pub use semio_s_artifact_gis_gismap_windows::viewer::gismap::modes::view::windows::map::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        #[path = "."]
                        pub mod config {
                            pub use semio_s_artifact_gis_gismap_windows::viewer::gismap::modes::view::windows::map::config::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🗺️map/🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod host {
#[path = "../../../../../../../../../../../🔨️modules/🏠️host/🧰️owned/🦀️.rs"]
pub mod owned;
}
