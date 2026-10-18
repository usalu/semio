//! 📸️ snapshot of `semio-s-artifact-bim-model` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
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
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod sheets {
                            #[path = "."]
                            pub mod pdf {
                                #[path = "."]
                                pub mod arcs {
                                    pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::sheets::pdf::arcs::*;
                                    #[cfg(test)]
                                    #[path = "../../../../🚪️io/📤️export/📄️sheets/📖️pdf/🌀️arcs/🧪️tests/🔬️unit/🦀️.rs"]
                                    pub mod tests;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod ifc {
                            #[path = "."]
                            pub mod codec {
                                pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::io::export::ifc::codec::*;
                                #[cfg(test)]
                                #[path = "../../../../🚪️io/📤️export/🏗️ifc/🧱️codec/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                            #[path = "."]
                            pub mod writer {
                                pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::ifc::writer::*;
                                #[cfg(test)]
                                #[path = "../../../../🚪️io/📤️export/🏗️ifc/🧰️writer/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod gltf {
                            #[path = "."]
                            pub mod codec {
                                pub use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gltf::codec::*;
                                #[cfg(test)]
                                #[path = "../../../../🚪️io/📤️export/🧊️gltf/🧱️codec/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::*;
                    #[path = "."]
                    pub mod authored {
                        #[path = "../../../📏️authored/🎚️option-rules/🦀️.rs"]
                        pub mod option_rules;
                    }
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod energy_envelope {
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::energy_envelope::*;
                            #[path = "."]
                            pub mod thermal {
                                pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::energy_envelope::thermal::*;
                                #[cfg(test)]
                                #[path = "../../../💡️inferences/🌡️energy-envelope/🧱️thermal/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                        #[path = "."]
                        pub mod families {
                            pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::families::*;
                            #[path = "."]
                            pub mod issues {
                                pub use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::inferences::families::issues::*;
                                #[cfg(test)]
                                #[path = "../../../💡️inferences/🧬️families/⚠️issues/🧪️tests/🔬️unit/🦀️.rs"]
                                pub mod tests;
                            }
                        }
                    }
                }
            }
        }
    }
}
