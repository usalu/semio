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
                        #[path = "."]
                        pub mod inferences {
                            #[path = "../../../../../../🚪️io/📝️text/💡️inferences/🪑️components/🦀️.rs"]
                            pub mod components;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "../../../../../../🚪️io/📤️export/🧾️json/🦀️.rs"]
                        pub mod json;
                        #[path = "."]
                        pub mod sheets {
                            #[path = "../../../../../../🚪️io/📤️export/📄️sheets/🎨️svg/🦀️.rs"]
                            pub mod svg;
                        }
                        #[path = "."]
                        pub mod svg {
                            use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::path;
                            #[path = "../../../../../../🚪️io/📤️export/🎨️svg/📏️projection/🦀️.rs"]
                            pub mod projection;
                        }
                        #[path = "."]
                        pub mod ifc {
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧾️context/🦀️.rs"]
                            pub mod context;
                            #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🏛️spatial/🦀️.rs"]
                            pub mod spatial;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod element_solids {
                            #[path = "."]
                            pub mod wall_sweeps {
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🧷️wall-sweeps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod plan_kit {
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/📐️plan-kit/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod columns {
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🏛️columns/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod beams {
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/➖️beams/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod slabs {
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/⬜️slabs/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod ceilings {
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🔲️ceilings/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod roofs {
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🏠️roofs/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod stairs {
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🪜️stairs/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod railings {
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🛤️railings/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod components {
                                #[cfg(test)]
                                #[path = "../../../../../💡️inferences/🧊️element-solids/🪑️components/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../🧲️wall-depth/🦀️.rs"]
                        pub mod wall_depth;
                        #[path = "."]
                        pub mod wall_geometry {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
        }
    }
}
