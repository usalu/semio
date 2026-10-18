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
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gltf::document;
                            #[path = "../../../../🚪️io/📤️export/🧊️gltf/📏️projection/🦀️.rs"]
                            pub mod projection;
                            #[cfg(any(test, feature = "testkit"))]
                            #[path = "../../../../🚪️io/📤️export/🧊️gltf/🧪️tests/🧰️testkit/🦀️.rs"]
                            pub mod testkit;
                        }
                        #[path = "."]
                        pub mod energy {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::energy::target;
                            #[path = "../../../../🚪️io/📤️export/🔋️energy/🧱️library/🦀️.rs"]
                            pub mod library;
                        }
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer;
                            #[path = "../../../../🚪️io/📤️export/🏗️ifc/🧭️frames/🦀️.rs"]
                            pub mod frames;
                        }
                        #[path = "."]
                        pub mod gbxml {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gbxml::codec;
                        }
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod ifc {
                            pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::import::ifc::frames;
                            #[path = "."]
                            pub mod reader {
                                pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::import::ifc::reader::*;
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
                    pub mod snapshot {
                        pub use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::*;
                        pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values;
                    }
                    #[path = "."]
                    pub mod authored {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod formula {
                            pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::formula::*;
                            #[cfg(test)]
                            #[path = "../../📝️formula/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod plan {
                            pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::plan::*;
                            #[cfg(test)]
                            #[path = "../../🧭️plan/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod profile {
                            pub use semio_s_artifact_bim_model_formula::standards::v1::subsets::any::schema::authored::profile::*;
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
