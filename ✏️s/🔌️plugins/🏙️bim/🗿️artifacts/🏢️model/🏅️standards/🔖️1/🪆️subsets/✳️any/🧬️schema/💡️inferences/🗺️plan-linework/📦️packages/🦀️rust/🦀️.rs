//! 🗺️ plan-linework of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod render {
    #[path = "../../../../../🖌️render/🗺️plan/🦀️.rs"]
    pub mod plan;
}
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
                            #[path = "../../../../../🚪️io/📝️text/💡️inferences/🗺️plan-linework/🦀️.rs"]
                            pub mod plan_linework;
                            #[path = "../../../../../🚪️io/📝️text/💡️inferences/🖼️view-linework/🦀️.rs"]
                            pub mod view_linework;
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../../../../🚪️io/📤️export/🎨️svg/✒️path/🦀️.rs"]
                            pub mod path;
                            #[path = "../../../../../🚪️io/📤️export/🎨️svg/🎚️style/🦀️.rs"]
                            pub mod style;
                            #[path = "../../../../../🚪️io/📤️export/🎨️svg/📐️sheet/🦀️.rs"]
                            pub mod sheet;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod plan_linework {
                            #[path = "../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod view_linework {
                            #[path = "../../../🖼️view-linework/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod sheet_layout {
                            #[path = "../../../📄️sheet-layout/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod rule_results {
                            #[path = "../../../⚖️rule-results/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        
                    }
                }
            }
        }
    }
}
