//! 🔺️ diff of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
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
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                        #[path = "."]
                        pub mod inferences {
                            #[path = "../../../../🚪️io/📝️text/💡️inferences/🧊️element-solids/🦀️.rs"]
                            pub mod element_solids;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    
                    
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod view_linework {
                            #[path = "../../../💡️inferences/🖼️view-linework/📐️frame/🦀️.rs"]
                            pub mod frame;
                        }
                        #[path = "."]
                        pub mod element_solids {
                            #[path = "../../../💡️inferences/🧊️element-solids/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod plan_kit {
                                #[path = "../../../💡️inferences/🧊️element-solids/📐️plan-kit/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod roofs {
                                #[path = "../../../💡️inferences/🧊️element-solids/🏠️roofs/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🧬️mutations/🌊️cascade/🦀️.rs"]
                        pub mod cascade;
                    }
                }
            }
        }
    }
}
