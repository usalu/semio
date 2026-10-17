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
                pub mod schema {
                    pub use semio_s_artifact_gis_gismap_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::feature;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::drawing_bridge;
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod bounds {
                            #[path = "../../../📦bounds/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../../../🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        #[cfg(test)]
                        #[path = "../../../../🧬️mutations/🧪️tests/🔬️kinds-conformance/🦀️.rs"]
                        pub mod kinds_conformance;
                        #[path = "."]
                        pub mod create_position {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_position::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_position::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_position::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🆕create-position/🧪️tests/💡️adds/🦀️.rs"]
                            pub mod tests_adds_a_lighthouse_position_after_the_harbor;
                        }
                        #[path = "."]
                        pub mod delete_position {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_position::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_position::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_position::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🗑️delete-position/🧪️tests/🚫️removes/🦀️.rs"]
                            pub mod tests_removes_the_lighthouse_position;
                        }
                        #[path = "."]
                        pub mod replace_position_data {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_position_data::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_position_data::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_position_data::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🔁replace-position-data/🧪️tests/⚓️rewrites/🦀️.rs"]
                            pub mod tests_rewrites_the_harbor_position_payload;
                        }
                        #[path = "."]
                        pub mod reorder_positions {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::reorder_positions::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::reorder_positions::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::reorder_positions::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🔀reorder-positions/🧪️tests/⚓️moves/🦀️.rs"]
                            pub mod tests_moves_the_harbor_position_to_the_end;
                        }
                        #[path = "."]
                        pub mod create_route {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_route::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_route::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::create_route::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🛣️create-route/🧪️tests/🚋️adds/🦀️.rs"]
                            pub mod tests_adds_a_tram_route_after_the_ferry;
                        }
                        #[path = "."]
                        pub mod delete_route {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_route::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_route::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::delete_route::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/✂️delete-route/🧪️tests/🚫️removes-tram-route/🦀️.rs"]
                            pub mod tests_removes_the_tram_route;
                        }
                        #[path = "."]
                        pub mod replace_route_data {
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_route_data::*;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_route_data::diff;
                            pub use semio_s_artifact_gis_gismap_mut_family::standards::v1::subsets::any::schema::mutations::replace_route_data::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/♻️replace-route-data/🧪️tests/⛴️rewrites/🦀️.rs"]
                            pub mod tests_rewrites_the_ferry_route_payload;
                        }
                        #[path = "."]
                        pub mod set_position_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_position_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_position_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_position_property::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/📌set-position-property/🧪️tests/🔠relabels/🦀️.rs"]
                            pub mod tests_relabels_position_middle_property;
                        }
                        #[path = "."]
                        pub mod remove_position_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_position_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_position_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_position_property::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🧲remove-position-property/🧪️tests/🧽drops/🦀️.rs"]
                            pub mod tests_drops_position_middle_property;
                        }
                        #[path = "."]
                        pub mod set_route_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_route_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_route_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_route_property::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🚏set-route-property/🧪️tests/🔠relabels/🦀️.rs"]
                            pub mod tests_relabels_route_middle_property;
                        }
                        #[path = "."]
                        pub mod remove_route_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_route_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_route_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_route_property::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🔪remove-route-property/🧪️tests/🧽drops/🦀️.rs"]
                            pub mod tests_drops_route_middle_property;
                        }
                        #[path = "."]
                        pub mod set_region_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_region_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_region_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::set_region_property::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🗾set-region-property/🧪️tests/🔠relabels/🦀️.rs"]
                            pub mod tests_relabels_region_middle_property;
                        }
                        #[path = "."]
                        pub mod remove_region_property {
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_region_property::*;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_region_property::diff;
                            pub use semio_s_artifact_gis_gismap_mut_any::standards::v1::subsets::any::schema::mutations::remove_region_property::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🧨remove-region-property/🧪️tests/🧽drops/🦀️.rs"]
                            pub mod tests_drops_region_middle_property;
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
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_gis_gismap_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::text::mutations;
                    }
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_gis_gismap_mutations::standards::v1::subsets::any::io::binary::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../../🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
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
        #[path = "../../../../../✏️editor/📋️constants-and-io/🦀️.rs"]
        pub mod constants_and_io;
        #[path = "../../../../../✏️editor/🗺️maphost/🦀️.rs"]
        pub mod maphost;
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
                    }
                }
            }
        }
    }
}
