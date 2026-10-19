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
                        pub mod ramp_runs {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod spaces {
                            #[cfg(test)]
                            #[path = "../../../../../../🏠️spaces/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod quantities {
                            #[cfg(test)]
                            #[path = "../../../../../../🧮️quantities/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod finishes {
                            #[cfg(test)]
                            #[path = "../../../../../../🎨️finishes/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod zones {
                            #[cfg(test)]
                            #[path = "../../../../../../🏘️zones/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod effective_properties {
                            #[cfg(test)]
                            #[path = "../../../../../../🏷️effective-properties/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod schedules {
                            #[cfg(test)]
                            #[path = "../../../../../../📋️schedules/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[cfg(test)]
                            #[path = "../../../../../../📋️schedules/🧪️tests/🗂️rows/🦀️.rs"]
                            pub mod tests_rows;
                            #[cfg(test)]
                            #[path = "../../../../../../📋️schedules/🧪️tests/🧮️table/🦀️.rs"]
                            pub mod tests_table;
                            #[cfg(test)]
                            #[path = "../../../../../../📋️schedules/🧪️tests/🕸️graph/🦀️.rs"]
                            pub mod tests_graph;
                        }
                        #[path = "."]
                        pub mod components {
                            #[cfg(test)]
                            #[path = "../../../../../../🪑️components/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod bodies {
                            #[cfg(test)]
                            #[path = "../../../../../../📦️bodies/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
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
                            pub mod zoning {
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📥️import/🏗️ifc/🏘️zoning/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod elements {
                                #[cfg(test)]
                                #[path = "../../../../../../../../🚪️io/📥️import/🏗️ifc/🏗️elements/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
        }
    }
}
