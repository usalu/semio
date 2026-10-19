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
                    pub mod export {
                        
                        #[path = "."]
                        pub mod gltf {
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📤️export/🧊️gltf/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod svg {
                            use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::path;
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📤️export/🎨️svg/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "../../../../../../../🚪️io/📤️export/🦴️solver/🦀️.rs"]
                        pub mod solver;
                        
                        #[path = "."]
                        pub mod csv {
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
                            #[path = "."]
                            pub mod ramps {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🛝️ramps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod wall_sweeps {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🧷️wall-sweeps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod curtain {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🪟️curtain/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod attach {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🧗️attach/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod horizontal {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/⬜️horizontal/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod ceilings {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🔲️ceilings/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod energy {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🔥️energy/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod circulation {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🪜️circulation/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📥️import/🏗️ifc/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod components {
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
        #[path = "."]
        pub mod entities {
            #[path = "."]
            pub mod psets {
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🏷️psets/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod phasing {
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🕰️phasing/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod frame {
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🏗️frame/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod wall_sweeps {
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🧷️wall-sweeps/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod zoning {
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🏘️zoning/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod sheets {
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/📄️sheets/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod coordination {
                #[cfg(test)]
                #[path = "../../../../../../🧩️entities/🤝️coordination/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod utilities {
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
                            #[cfg(test)]
                            #[path = "../../../../../../🎭️modes/✏️edit/🪟️windows/🧬️family/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
