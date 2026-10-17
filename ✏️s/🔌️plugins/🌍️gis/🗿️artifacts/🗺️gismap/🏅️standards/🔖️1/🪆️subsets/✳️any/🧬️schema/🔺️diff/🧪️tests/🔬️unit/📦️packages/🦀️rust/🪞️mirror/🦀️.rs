//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::feature;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::drawing_bridge;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::diff::*;
                        pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
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
                            #[path = "../../../../../../🧬️mutations/🔃reorder-regions/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../../../🧬️mutations/🔃reorder-regions/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../../../🧬️mutations/🔃reorder-regions/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "../../../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                        #[path = "."]
                                        pub mod geojson {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dxf {
                                    #[path = "."]
                                    pub mod v_r12 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod svg {
                                    #[path = "."]
                                    pub mod v1_1 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod png {
                                    #[path = "."]
                                    pub mod v1_2 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                        #[path = "."]
                                        pub mod geojson {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dwg {
                                    #[path = "."]
                                    pub mod v_ac1018 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod dxf {
                                    #[path = "."]
                                    pub mod v_r12 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
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
                            #[cfg(test)]
                            #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
