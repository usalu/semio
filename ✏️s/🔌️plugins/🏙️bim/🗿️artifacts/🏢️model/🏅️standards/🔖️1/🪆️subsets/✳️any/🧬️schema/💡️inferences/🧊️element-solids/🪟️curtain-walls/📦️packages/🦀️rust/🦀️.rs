//! 🪟️ curtain-walls of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as schema;

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
                        #[path = "../../../../../../🚪️io/📤️export/🧱️holders/🦀️.rs"]
                        pub mod holders;
                        #[path = "."]
                        pub mod gbxml {
                            #[path = "../../../../../../🚪️io/📤️export/🌿️gbxml/📐️geometry/🦀️.rs"]
                            pub mod geometry;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod element_solids {
                            #[path = "."]
                            pub mod curtain_walls {
                                #[path = "../../🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod wall_sweeps {
                                #[path = "../../../🧷️wall-sweeps/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod rail_hosts {
                                #[cfg(test)]
                                #[path = "../../../🪝️rail-hosts/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
        }
    }
}
