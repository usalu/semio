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
                    #[path = "../../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::*;
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::io::export::ifc::codec;
                            pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::context;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
                            pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::io::export::ifc::brep;
                            #[path = "."]
                            pub mod spatial {
                                pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::io::export::ifc::spatial::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏛️spatial/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod data {
                                pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::data::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧬️data/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod walls {
                                pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::walls::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏰️walls/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod wall_sweeps {
                                pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::wall_sweeps::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧷️wall-sweeps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod frame {
                                pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::frame::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏗️frame/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod horizontal {
                                pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::horizontal::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/⬜️horizontal/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod ceilings {
                                pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::ceilings::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🔲️ceilings/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod circulation {
                                pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::circulation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🪜️circulation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod ramps {
                                pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::ramps::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🛝️ramps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod components {
                                pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::components::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🪑️components/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod mep {
                                pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::mep::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🌀️mep/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod grids {
                                pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::grids::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/📏️grids/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod spaces {
                                pub use semio_s_artifact_bim_model_family_tests::standards::v1::subsets::any::io::export::ifc::spaces::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏠️spaces/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod zoning {
                                pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::zoning::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏘️zoning/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod curtain {
                                pub use semio_s_artifact_bim_model_family_tests::standards::v1::subsets::any::io::export::ifc::curtain::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🪟️curtain/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod annotations {
                                pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::annotations::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🪧️annotations/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod ifc4 {
                                pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::ifc4::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/📚️ifc4/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod projection {
                                pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::projection::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🔬️projection/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod gltf {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gltf::document;
                            pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::gltf::materials;
                            #[path = "."]
                            pub mod scene {
                                pub use semio_s_artifact_bim_model_family_tests::standards::v1::subsets::any::io::export::gltf::scene::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🧊️gltf/🌳️scene/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::gltf::projection;
                            #[cfg(any(test, feature = "testkit"))]
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::gltf::testkit;
                        }
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::*;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::import::ifc::reader;
                            #[path = "."]
                            pub mod spatial {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::spatial::*;
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📥️import/🏗️ifc/🏛️spatial/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod model_graph {
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance;
                            pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::standalone;
                        }
                        pub use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep;
                        #[path = "."]
                        pub mod element_solids {
                            pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::*;
                            pub use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::schema::inferences::element_solids::curtain_walls;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade;
                        #[path = "../../../🦀️.rs"]
                        pub mod cascade_restore;
                        pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry;
                        #[path = "."]
                        pub mod split_wall {
                            pub use semio_s_artifact_bim_model_mut_any_177::standards::v1::subsets::any::schema::mutations::split_wall::*;
                            #[cfg(test)]
                            #[path = "../../../../🦈️split-wall/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            pub mod tests_follows_by_inference;
                        }
                        #[path = "."]
                        pub mod delete_curtain_wall {
                            pub use semio_s_artifact_bim_model_mut_any_83::standards::v1::subsets::any::schema::mutations::delete_curtain_wall::*;
                            #[cfg(test)]
                            #[path = "../../../../🦖️delete-curtain-wall/🧪️tests/🔗️follows-by-inference/🦀️.rs"]
                            pub mod tests_follows_by_inference;
                        }
                        pub use semio_s_artifact_bim_model_mut_any_87::standards::v1::subsets::any::schema::mutations::set_element_property;
                        pub use semio_s_artifact_bim_model_mut_any_88::standards::v1::subsets::any::schema::mutations::set_element_classification;
                        pub use semio_s_artifact_bim_model_mut_any_50::standards::v1::subsets::any::schema::mutations::set_element_option;
                        pub use semio_s_artifact_bim_model_mut_any_51::standards::v1::subsets::any::schema::mutations::set_element_workset;
                        pub use semio_s_artifact_bim_model_mut_any_156::standards::v1::subsets::any::schema::mutations::set_component_override;
                        pub use semio_s_artifact_bim_model_mut_any_56::standards::v1::subsets::any::schema::mutations::set_space_conditions;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_bim_model_tests_5::render;
