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
                        #[path = "../../../../../../../🚪️io/📝️text/🔭️projection/🦀️.rs"]
                        pub mod projection;
                        
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod json {
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📤️export/🧾️json/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod sheets {
                            #[path = "."]
                            pub mod ink {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/📄️sheets/✒️ink/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod svg {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/📄️sheets/🎨️svg/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod tables {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/📄️sheets/📊️tables/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path = "../../../../../../../🚪️io/📤️export/📄️sheets/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod gbxml {
                            #[path = "."]
                            pub mod tables {
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
                            #[path = "../../../../../../../🚪️io/📤️export/🧊️gltf/🎨️materials/🦀️.rs"]
                            pub mod materials;
                        }
                        #[path = "."]
                        pub mod ifc {
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
                            use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::io::export::svg::path;
                            #[path = "."]
                            pub mod projection {
                                #[cfg(test)]
                                #[path = "../../../../../../../🚪️io/📤️export/🎨️svg/📏️projection/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    
                    #[path = "."]
                    pub mod inferences {
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                        
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod wall_depth {
                            #[cfg(test)]
                            #[path = "../../../../../../🧬️mutations/🧲️wall-depth/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod modify {
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
        #[cfg(test)]
        #[path = "../../../../../../../🖌️render/🌡️envelope/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
