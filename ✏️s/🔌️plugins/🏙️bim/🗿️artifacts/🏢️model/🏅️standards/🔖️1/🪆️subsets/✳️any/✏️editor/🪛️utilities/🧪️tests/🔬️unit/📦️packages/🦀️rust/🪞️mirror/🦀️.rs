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
                pub mod schema {
                    pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::*;
                    #[path = "."]
                    pub mod authored {
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula;
                        pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan;
                    }
                    pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels;
                        pub use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph;
                        pub use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework;
                        pub use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results;
                        pub use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_bim_model_mut_any_144::standards::v1::subsets::any::schema::mutations::set_element_storey;
                        pub use semio_s_artifact_bim_model_mut_any_103::standards::v1::subsets::any::schema::mutations::set_element_phase;
                        pub use semio_s_artifact_bim_model_mut_any_172::standards::v1::subsets::any::schema::mutations::create_curtain_wall;
                        pub use semio_s_artifact_bim_model_mut_any_42::standards::v1::subsets::any::schema::mutations::set_family;
                        pub use semio_s_artifact_bim_model_mut_any_109::standards::v1::subsets::any::schema::mutations::set_family_parameter;
                        pub use semio_s_artifact_bim_model_mut_any_109::standards::v1::subsets::any::schema::mutations::remove_family_parameter;
                        pub use semio_s_artifact_bim_model_mut_any_110::standards::v1::subsets::any::schema::mutations::create_family_solid;
                        pub use semio_s_artifact_bim_model_mut_any_110::standards::v1::subsets::any::schema::mutations::delete_family_solid;
                        pub use semio_s_artifact_bim_model_mut_any_43::standards::v1::subsets::any::schema::mutations::set_family_solid;
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::*;
                    pub use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::text::snapshot;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::io::binary::snapshot;
                    }
                    #[path = "."]
                    pub mod export {
                        pub use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::export::*;
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_ifc::standards::v1::subsets::any::io::export::ifc::*;
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::io::export::ifc::codec;
                            pub use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::io::export::ifc::ceilings;
                            pub use semio_s_artifact_bim_model_tests_5::standards::v1::subsets::any::io::export::ifc::components;
                        }
                        #[path = "."]
                        pub mod gltf {
                            pub use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::io::export::gltf::*;
                            #[cfg(any(test, feature = "testkit"))]
                            pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::io::export::gltf::testkit;
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📤️export/🧊️gltf/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod svg {
                            pub use semio_s_artifact_bim_model_svg::standards::v1::subsets::any::io::export::svg::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📤️export/🎨️svg/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "../../../../../../../🚪️io/📤️export/🦴️solver/🦀️.rs"]
                        pub mod solver;
                        #[path = "."]
                        pub mod sheets {
                            #[cfg(any(test, feature = "testkit"))]
                            pub use semio_s_artifact_bim_model_sheets::standards::v1::subsets::any::io::export::sheets::testkit;
                        }
                        #[path = "."]
                        pub mod csv {
                            pub use semio_s_artifact_bim_model_io::standards::v1::subsets::any::io::export::csv::*;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::csv::codec;
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📤️export/📊️csv/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[cfg(test)]
                    #[path = "../../../../../../../🚪️io/🧪️tests/🔬️unit/🦀️.rs"]
                    pub mod tests;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::*;
                            #[path = "."]
                            pub mod ramps {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::ramps::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🛝️ramps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::import::ifc::frames;
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::import::ifc::reader;
                            #[path = "."]
                            pub mod wall_sweeps {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::wall_sweeps::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🧷️wall-sweeps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod curtain {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::curtain::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🪟️curtain/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod attach {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::attach::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🧗️attach/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod horizontal {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::horizontal::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/⬜️horizontal/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod ceilings {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::ceilings::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🔲️ceilings/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod energy {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::energy::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🔥️energy/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod circulation {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::circulation::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🪜️circulation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod components {
                                pub use semio_s_artifact_bim_model_ifc_2::standards::v1::subsets::any::io::import::ifc::components::*;
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🪑️components/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
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
    pub mod bim {
        pub use semio_s_artifact_bim_model_tests::editor::bim::terminology;
        #[path = "."]
        pub mod entities {
            pub use semio_s_artifact_bim_model_entities::editor::bim::entities::*;
            #[path = "."]
            pub mod psets {
                pub use semio_s_artifact_bim_model_entities::editor::bim::entities::psets::*;
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🏷️psets/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod phasing {
                pub use semio_s_artifact_bim_model_viewer::editor::bim::entities::phasing::*;
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🕰️phasing/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod frame {
                pub use semio_s_artifact_bim_model_viewer::editor::bim::entities::frame::*;
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🏗️frame/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod wall_sweeps {
                pub use semio_s_artifact_bim_model_entities::editor::bim::entities::wall_sweeps::*;
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🧷️wall-sweeps/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod zoning {
                pub use semio_s_artifact_bim_model_entities::editor::bim::entities::zoning::*;
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🏘️zoning/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod sheets {
                pub use semio_s_artifact_bim_model_entities::editor::bim::entities::sheets::*;
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/📄️sheets/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod coordination {
                pub use semio_s_artifact_bim_model_entities::editor::bim::entities::coordination::*;
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🤝️coordination/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod utilities {
            pub use semio_s_artifact_bim_model_kit::editor::bim::utilities::*;
            pub use semio_s_artifact_bim_model_kit::editor::bim::utilities::*;
            #[cfg(test)]
            #[path = "../../../🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod family {
                        #[path = "."]
                        pub mod edit {
                            pub use semio_s_artifact_bim_model_viewer::editor::bim::modes::edit::windows::family::edit::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🎭️modes/✏️edit/🪟️windows/🧬️family/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section;
                    pub use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::sheet;
                }
            }
        }
    }
}
