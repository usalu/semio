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
                    
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod plan_linework {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod view_linework {
                            #[cfg(test)]
                            #[path = "../../../../../../🖼️view-linework/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod energy_envelope {
                            #[cfg(test)]
                            #[path = "../../../../../../🌡️energy-envelope/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod annotation_layout {
                            #[cfg(test)]
                            #[path = "../../../../../../🪧️annotation-layout/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod diagnostics {
                            #[path = "."]
                            pub mod index {
                                #[cfg(test)]
                                #[path = "../../../../../../⚠️diagnostics/🗂️index/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod wall_depth {
                                #[cfg(test)]
                                #[path = "../../../../../../⚠️diagnostics/🧗️wall-depth/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path = "../../../../../../⚠️diagnostics/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            #[path = "."]
                            pub mod walls {
                                #[cfg(test)]
                                #[path = "../../../../../../🧊️element-solids/🧱️walls/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                                #[cfg(test)]
                                #[path = "../../../../../../🧊️element-solids/🧱️walls/🧪️tests/🏔️attached/🦀️.rs"]
                                pub mod attached;
                            }
                            #[path = "."]
                            pub mod curtain_walls {
                                #[cfg(test)]
                                #[path = "../../../../../../🧊️element-solids/🪟️curtain-walls/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[cfg(test)]
                            #[path = "../../../../../../🧊️element-solids/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[path = "."]
                            pub mod fillers {
                                #[cfg(test)]
                                #[path = "../../../../../../🧊️element-solids/🚪️fillers/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod ifc {
                            #[path = "."]
                            pub mod data {
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📥️import/🏗️ifc/🧬️data/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
        }
    }
}
