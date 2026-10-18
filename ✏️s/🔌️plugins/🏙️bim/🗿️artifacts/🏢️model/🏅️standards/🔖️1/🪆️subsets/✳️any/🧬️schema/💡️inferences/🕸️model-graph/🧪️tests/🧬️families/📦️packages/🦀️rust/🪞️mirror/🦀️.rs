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
                    #[path = "../../../../../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod energy {
                            pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::energy::*;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::energy::target;
                            pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::energy::build;
                            #[cfg(test)]
                            #[path = "../../../../../../../../🚪️io/📤️export/🔋️energy/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod json {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::json::codec;
                        }
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::context;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::ifc::frames;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::io::export::ifc::brep;
                            pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::walls;
                            #[path = "../../../../../../../../🚪️io/📤️export/🏗️ifc/🏠️spaces/🦀️.rs"]
                            pub mod spaces;
                            pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::zoning;
                            #[path = "../../../../../../../../🚪️io/📤️export/🏗️ifc/🪟️curtain/🦀️.rs"]
                            pub mod curtain;
                        }
                        #[path = "."]
                        pub mod gltf {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gltf::document;
                            #[path = "."]
                            pub mod materials {
                                pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::gltf::materials::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📤️export/🧊️gltf/🎨️materials/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "../../../../../../../../🚪️io/📤️export/🧊️gltf/🌳️scene/🦀️.rs"]
                            pub mod scene;
                            #[path = "."]
                            pub mod projection {
                                pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::gltf::projection::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📤️export/🧊️gltf/📏️projection/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(any(test, feature = "testkit"))]
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::gltf::testkit;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[cfg(any(test, feature = "testkit"))]
                            pub use semio_s_artifact_bim_model_svg::standards::v1::subsets::any::io::export::svg::testkit;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels;
                        #[path = "."]
                        pub mod model_graph {
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::*;
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::*;
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod family_tests;
                        }
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
                        #[path = "."]
                        pub mod components {
                            pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components::*;
                            pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🪑️components/🧪️tests/🕸️graph/🦀️.rs"]
                            pub mod graph_tests;
                        }
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout;
                        pub use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope;
                        pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::curtain_walls;
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
    pub mod bim {
        pub use semio_s_artifact_bim_model_tests::viewer::bim::terminology;
        #[path = "."]
        pub mod modes {
            #[path = "../../../../../../../../👁️viewer/🎭️modes/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod render {
    pub use semio_s_artifact_bim_model_tests_5::render::*;
    pub use semio_s_artifact_bim_model_tests::render::window_config;
    pub use semio_s_artifact_bim_model_plan_linework::render::plan;
    #[path = "."]
    pub mod world {
        pub use semio_s_artifact_bim_model_tests_5::render::world::*;
        #[cfg(test)]
        #[path = "../../../../../../../../🖌️render/🧊️world/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
