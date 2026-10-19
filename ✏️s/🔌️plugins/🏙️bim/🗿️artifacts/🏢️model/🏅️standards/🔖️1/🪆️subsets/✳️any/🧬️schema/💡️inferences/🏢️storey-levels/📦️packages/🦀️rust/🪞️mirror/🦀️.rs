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
                    pub mod bcf {
                        #[cfg(test)]
                        #[path = "../../../../../../🚪️io/💬️bcf/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod inferences {
                            #[path = "../../../../../../🚪️io/📝️text/💡️inferences/🪜️stair-runs/🦀️.rs"]
                            pub mod stair_runs;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod ifc {
                            #[path = "."]
                            pub mod frames {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧭️frames/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        
                        #[path = "."]
                        pub mod gltf {
                            #[path = "."]
                            pub mod document {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🧊️gltf/🧬️document/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod energy {
                            #[path = "."]
                            pub mod library {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🔋️energy/🧱️library/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod diff {
                        #[path = "."]
                        pub mod patches {
                            #[path = "."]
                            pub mod building_elements {
                                #[path = "../../../../../🔺️diff/🩹️patches/🧱️building-elements/🪜️stairs-railings-ramps/🦀️.rs"]
                                pub mod stairs_railings_ramps;
                            }
                            #[path = "../../../../../🔺️diff/🩹️patches/🖋️hand-written/🦀️.rs"]
                            pub mod hand_written;
                        }
                    }
                    
                    
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod storey_levels {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod curtain_layout {
                            #[path = "../../../../🪞️curtain-layout/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stair_runs {
                            #[path = "../../../../🪜️stair-runs/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../🧬️mutations/🔩️family-rules/🦀️.rs"]
                        pub mod family_rules;
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
        pub mod gestures {
            #[path = "../../../../../../✏️editor/🧵️gestures/📐️plane/🦀️.rs"]
            pub mod plane;
        }
        #[path = "."]
        pub mod marks {
            #[path = "../../../../../../✏️editor/🖍️marks/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
