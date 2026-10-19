//! 📏️ authored of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
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
                    #[path = "../../../../🚪️io/💬️bcf/🦀️.rs"]
                    pub mod bcf;
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod gltf {
                            #[path = "../../../../🚪️io/📤️export/🧊️gltf/📏️projection/🦀️.rs"]
                            pub mod projection;
                            #[cfg(any(test, feature = "testkit"))]
                            #[path = "../../../../🚪️io/📤️export/🧊️gltf/🧪️tests/🧰️testkit/🦀️.rs"]
                            pub mod testkit;
                        }
                        #[path = "."]
                        pub mod energy {
                            #[path = "../../../../🚪️io/📤️export/🔋️energy/🧱️library/🦀️.rs"]
                            pub mod library;
                        }
                        #[path = "."]
                        pub mod ifc {
                            #[path = "../../../../🚪️io/📤️export/🏗️ifc/🧭️frames/🦀️.rs"]
                            pub mod frames;
                        }
                        
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod ifc {
                            #[path = "."]
                            pub mod reader {
                                #[cfg(test)]
                                #[path = "../../../../🚪️io/📥️import/🏗️ifc/📖️reader/🧪️tests/🔬️unit/🦀️.rs"]
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
                            #[path = "../../../🔺️diff/🩹️patches/🤝️coordination/🦀️.rs"]
                            pub mod coordination;
                        }
                        #[path = "../../../🔺️diff/🧮️algebra/🦀️.rs"]
                        pub mod algebra;
                    }
                    
                    #[path = "."]
                    pub mod authored {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod formula {
                            #[cfg(test)]
                            #[path = "../../📝️formula/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod plan {
                            #[cfg(test)]
                            #[path = "../../🧭️plan/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod profile {
                            #[cfg(test)]
                            #[path = "../../▭️profile/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod phase_visibility {
                            #[path = "../../../💡️inferences/🎭️phase-visibility/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🧬️mutations/🖍️annotating/🦀️.rs"]
                        pub mod annotating;
                        #[path = "../../../🧬️mutations/🧿️horizontal-rules/🦀️.rs"]
                        pub mod horizontal_rules;
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
        pub mod entities {
            #[path = "../../../../✏️editor/🧩️entities/🔤️field-codecs/🦀️.rs"]
            pub mod field_codecs;
        }
    }
}
