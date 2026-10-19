//! 🏠️ spaces of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

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
                            #[path = "../../../../../🚪️io/📝️text/💡️inferences/🏠️spaces/🦀️.rs"]
                            pub mod spaces;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod view_linework {
                            #[path = "../../../🖼️view-linework/👁️hidden-lines/🦀️.rs"]
                            pub mod hidden_lines;
                        }
                        #[path = "."]
                        pub mod spaces {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod bodies {
                            #[path = "../../../📦️bodies/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod analytical_members {
                            #[path = "../../../🦴️analytical-members/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod energy_envelope {
                            #[path = "../../../🌡️energy-envelope/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            #[path = "."]
                            pub mod walls {
                                #[path = "../../../🧊️element-solids/🧱️walls/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod fillers {
                                #[path = "../../../🧊️element-solids/🚪️fillers/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod beams {
                                #[path = "../../../🧊️element-solids/➖️beams/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod railings {
                                #[path = "../../../🧊️element-solids/🛤️railings/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod ramps {
                                #[cfg(test)]
                                #[path = "../../../🧊️element-solids/🛝️ramps/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod rail_hosts {
                                #[path = "../../../🧊️element-solids/🪝️rail-hosts/🦀️.rs"]
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
