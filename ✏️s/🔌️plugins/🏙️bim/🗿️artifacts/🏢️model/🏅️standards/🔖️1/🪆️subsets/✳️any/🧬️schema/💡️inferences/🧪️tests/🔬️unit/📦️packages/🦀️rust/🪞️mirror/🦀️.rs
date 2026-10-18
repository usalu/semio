//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        #[path = "../../../../../../../🚪️io/📝️text/🔭️projection/🦀️.rs"]
                        pub mod projection;
                        #[path = "."]
                        pub mod inferences {
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::text::inferences::plan_linework;
                            pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::io::text::inferences::stair_runs;
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::io::text::inferences::quantities;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::io::text::inferences::spaces;
                            pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::io::text::inferences::diagnostics;
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::io::text::inferences::element_solids;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::io::text::inferences::ramp_runs;
                            pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::text::inferences::components;
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::text::inferences::view_linework;
                            pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::io::text::inferences::schedules;
                        }
                    }
                    pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::bcf;
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod json {
                            pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::json::*;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::json::codec;
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📤️export/🧾️json/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod sheets {
                            pub use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::*;
                            #[path = "."]
                            pub mod ink {
                                pub use semio_s_artifact_bim_model_incremental_tests::standards::v1::subsets::any::io::export::sheets::ink::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/📄️sheets/✒️ink/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod svg {
                                pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::sheets::svg::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/📄️sheets/🎨️svg/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod tables {
                                pub use semio_s_artifact_bim_model_incremental_tests::standards::v1::subsets::any::io::export::sheets::tables::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/📄️sheets/📊️tables/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📤️export/📄️sheets/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::io::export::holders;
                        #[path = "."]
                        pub mod gbxml {
                            pub use semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::*;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gbxml::codec;
                            pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::io::export::gbxml::geometry;
                            #[path = "."]
                            pub mod tables {
                                pub use semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::tables::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/🌿️gbxml/📊️tables/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📤️export/🌿️gbxml/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "../../../../../../../🚪️io/📤️export/🔋️energy/🦀️.rs"]
                        pub mod energy;
                        #[path = "."]
                        pub mod gltf {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gltf::document;
                            #[path = "../../../../../../../🚪️io/📤️export/🧊️gltf/🎨️materials/🦀️.rs"]
                            pub mod materials;
                        }
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::context;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::ifc::frames;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::io::export::ifc::brep;
                            pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::data;
                            #[path = "../../../../../../../🚪️io/📤️export/🏗️ifc/🏰️walls/🦀️.rs"]
                            pub mod walls;
                            #[path = "../../../../../../../🚪️io/📤️export/🏗️ifc/🧷️wall-sweeps/🦀️.rs"]
                            pub mod wall_sweeps;
                            #[path = "../../../../../../../🚪️io/📤️export/🏗️ifc/🪑️components/🦀️.rs"]
                            pub mod components;
                            #[path = "../../../../../../../🚪️io/📤️export/🏗️ifc/🌀️mep/🦀️.rs"]
                            pub mod mep;
                            #[path = "../../../../../../../🚪️io/📤️export/🏗️ifc/🏘️zoning/🦀️.rs"]
                            pub mod zoning;
                            #[path = "../../../../../../../🚪️io/📤️export/🏗️ifc/🪧️annotations/🦀️.rs"]
                            pub mod annotations;
                        }
                        #[path = "."]
                        pub mod svg {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::svg::codec;
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::path;
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::style;
                            pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::sheet;
                            pub use semio_s_artifact_bim_model_incremental_tests::standards::v1::subsets::any::io::export::svg::drawing;
                            #[path = "."]
                            pub mod projection {
                                pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::svg::projection::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/🎨️svg/📏️projection/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(any(test, feature = "testkit"))]
                            pub use semio_s_artifact_bim_model_svg::standards::v1::subsets::any::io::export::svg::testkit;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan;
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::profile;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::references;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::authored::sizes;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels;
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout;
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities;
                        pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::finishes;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::element_solids::walls;
                            pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::wall_sweeps;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod wall_depth {
                            pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_depth::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧬️mutations/🧲️wall-depth/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry;
                        #[path = "."]
                        pub mod modify {
                            pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::*;
                            pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🧬️mutations/🧙️modify/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod render {
    #[path = "../../../../../../../🖌️render/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod envelope {
        pub use semio_s_artifact_bim_model_modify::render::envelope::*;
        #[cfg(test)]
        #[path = "../../../../../../../🖌️render/🌡️envelope/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
