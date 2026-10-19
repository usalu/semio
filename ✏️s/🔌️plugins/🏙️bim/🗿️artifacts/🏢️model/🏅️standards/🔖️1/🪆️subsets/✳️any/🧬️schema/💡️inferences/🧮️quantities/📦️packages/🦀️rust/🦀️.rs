//! 🧮️ quantities of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

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
                            #[path = "../../../../../🚪️io/📝️text/💡️inferences/🧮️quantities/🦀️.rs"]
                            pub mod quantities;
                            #[path = "../../../../../🚪️io/📝️text/💡️inferences/📋️schedules/🦀️.rs"]
                            pub mod schedules;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod gbxml {
                            #[path = "."]
                            pub mod pairing {
                                #[cfg(test)]
                                #[path = "../../../../../🚪️io/📤️export/🌿️gbxml/🔗️pairing/🧪️tests/🔬️unit/🦀️.rs"]
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
                        pub mod quantities {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod option_scope {
                            #[path = "../../../🧭️option-scope/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod finishes {
                            #[path = "../../../🎨️finishes/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zones {
                            #[path = "../../../🏘️zones/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod effective_properties {
                            #[path = "../../../🏷️effective-properties/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod schedules {
                            #[path = "../../../📋️schedules/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod annotation_layout {
                            #[path = "../../../🪧️annotation-layout/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        
                    }
                }
            }
        }
    }
}
