//! 🪑️ components of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

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
                            #[path = "../../../../../🚪️io/📝️text/💡️inferences/⚠️diagnostics/🦀️.rs"]
                            pub mod diagnostics;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod holders {
                            #[cfg(test)]
                            #[path = "../../../../../🚪️io/📤️export/🧱️holders/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod gbxml {
                            #[path = "."]
                            pub mod geometry {
                                #[cfg(test)]
                                #[path = "../../../../../🚪️io/📤️export/🌿️gbxml/📐️geometry/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "../../../../../🚪️io/📤️export/🌿️gbxml/🔗️pairing/🦀️.rs"]
                            pub mod pairing;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    
                    #[path = "."]
                    pub mod inferences {
                        
                        #[path = "."]
                        pub mod components {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod mep {
                            #[path = "../../../🌀️mep/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod diagnostics {
                            #[path = "../../../⚠️diagnostics/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            #[path = "."]
                            pub mod components {
                                #[path = "../../../🧊️element-solids/🪑️components/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mep {
                                #[path = "../../../🧊️element-solids/🌀️mep/🦀️.rs"]
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
