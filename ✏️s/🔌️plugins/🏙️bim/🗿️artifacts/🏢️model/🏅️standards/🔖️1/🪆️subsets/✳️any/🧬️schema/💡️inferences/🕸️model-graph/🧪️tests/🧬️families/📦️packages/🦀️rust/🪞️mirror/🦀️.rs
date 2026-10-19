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
                            #[cfg(test)]
                            #[path = "../../../../../../../../🚪️io/📤️export/🔋️energy/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        
                        #[path = "."]
                        pub mod ifc {
                            #[path = "../../../../../../../../🚪️io/📤️export/🏗️ifc/🏠️spaces/🦀️.rs"]
                            pub mod spaces;
                            #[path = "../../../../../../../../🚪️io/📤️export/🏗️ifc/🪟️curtain/🦀️.rs"]
                            pub mod curtain;
                        }
                        #[path = "."]
                        pub mod gltf {
                            #[path = "."]
                            pub mod materials {
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📤️export/🧊️gltf/🎨️materials/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "../../../../../../../../🚪️io/📤️export/🧊️gltf/🌳️scene/🦀️.rs"]
                            pub mod scene;
                            #[path = "."]
                            pub mod projection {
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📤️export/🧊️gltf/📏️projection/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod model_graph {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod family_tests;
                        }
                        #[path = "."]
                        pub mod components {
                            #[cfg(test)]
                            #[path = "../../../../../../🪑️components/🧪️tests/🕸️graph/🦀️.rs"]
                            pub mod graph_tests;
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
    #[path = "."]
    pub mod world {
        #[cfg(test)]
        #[path = "../../../../../../../../🖌️render/🧊️world/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
