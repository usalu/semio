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
                        pub mod ifc {
                            #[path = "."]
                            pub mod brep {
                                #[cfg(test)]
                                #[path = "../../../../../../🚪️io/📤️export/🏗️ifc/🧊️brep/🧪️tests/🔬️unit/🦀️.rs"]
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
                        pub mod view_linework {
                            #[path = "../../../../🖼️view-linework/✂️clip/🦀️.rs"]
                            pub mod clip;
                            #[path = "../../../../🖼️view-linework/🪓️cut/🦀️.rs"]
                            pub mod cut;
                        }
                        #[path = "."]
                        pub mod wall_layout {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod opening_frames {
                            #[path = "../../../../🪟️opening-frames/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod families {
                            #[path = "."]
                            pub mod metrics {
                                #[cfg(test)]
                                #[path = "../../../../🧬️families/📊️metrics/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod findings {
                                #[cfg(test)]
                                #[path = "../../../../🧬️families/🩺️findings/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod parameters {
                                #[cfg(test)]
                                #[path = "../../../../🧬️families/🧮️parameters/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod solids {
                                #[cfg(test)]
                                #[path = "../../../../🧬️families/🧊️solids/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod clash_sets {
                            #[cfg(test)]
                            #[path = "../../../../🧨️clash-sets/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            #[path = "."]
                            pub mod columns {
                                #[path = "../../../../🧊️element-solids/🏛️columns/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod ramps {
                                #[path = "../../../../🧊️element-solids/🛝️ramps/🦀️.rs"]
                                mod component;
                                pub use component::*;
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
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod family {
                        #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧬️family/🏷️vocabulary/🦀️.rs"]
                        pub mod vocabulary;
                    }
                    #[path = "."]
                    pub mod schedule {
                        #[path = "."]
                        pub mod edit {
                            #[cfg(test)]
                            #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧮️schedule/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
