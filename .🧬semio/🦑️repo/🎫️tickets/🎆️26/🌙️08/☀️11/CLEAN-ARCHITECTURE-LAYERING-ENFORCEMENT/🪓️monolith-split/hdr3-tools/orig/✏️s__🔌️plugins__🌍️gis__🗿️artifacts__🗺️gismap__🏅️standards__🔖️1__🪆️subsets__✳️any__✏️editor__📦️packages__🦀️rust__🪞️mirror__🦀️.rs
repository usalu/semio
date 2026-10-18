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
                pub use semio_s_artifact_gis_gismap_gismap::standards::v1::subsets::any::artifact_declaration;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::feature;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::snapshot;
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
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_gis_gismap_snapshot::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gis2d {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_gis_gismap_inferences::editor::gis2d::constants_and_io;
        pub use semio_s_artifact_gis_gismap_inferences::editor::gis2d::maphost;
        pub use semio_s_artifact_gis_gismap_snapshot::editor::gis2d::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "../../../🎮️commands/🎨️example/🦀️.rs"]
            pub mod example;
            #[path = "../../../🎮️commands/🗺️features/🦀️.rs"]
            pub mod features;
            #[path = "../../../🎮️commands/💡️inference/🦀️.rs"]
            pub mod inference;
            #[path = "../../../🎮️commands/🌐️shell/🦀️.rs"]
            pub mod shell;
            #[path = "../../../🎮️commands/👁️view/🦀️.rs"]
            pub mod view;

        #[cfg(test)]
        #[path = "../../../🎮️commands/🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_gis_gismap_gismap::editor::gis2d::modes::edit::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_gis_gismap_gismap::editor::gis2d::modes::edit::windows::*;
                    #[path = "."]
                    pub mod map {
                        pub use semio_s_artifact_gis_gismap_gismap::editor::gis2d::modes::edit::windows::map::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🗺️map/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        #[path = "."]
                        pub mod config {
                            pub use semio_s_artifact_gis_gismap_config::editor::gis2d::modes::edit::windows::map::config::*;
                            pub use semio_s_artifact_gis_gismap_config::editor::gis2d::modes::edit::windows::map::config::mutations;
                        }
                        pub use semio_s_artifact_gis_gismap_tests::editor::gis2d::modes::edit::windows::map::options;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../📌️panels/🗿️artifact/🦀️.rs"]
            pub mod artifact;
            #[path = "../../../📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../../📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;

        #[cfg(test)]
        #[path = "../../../📌️panels/🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
    }
}
pub use semio_s_artifact_gis_gismap_tests::host;
