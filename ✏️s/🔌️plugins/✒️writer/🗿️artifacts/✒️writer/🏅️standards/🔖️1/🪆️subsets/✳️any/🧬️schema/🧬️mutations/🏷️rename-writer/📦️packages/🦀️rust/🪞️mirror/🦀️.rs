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
                    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod outline {
                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::schema::inferences::outline::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../../💡️inferences/🧾outline/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod rename_writer {
                            #[path = "../../../🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_uri {
                            #[path = "../../../../🔗change-uri/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🔗change-uri/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🔗change-uri/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_language {
                            #[path = "../../../../🌐change-language/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🌐change-language/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🌐change-language/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod edit_text {
                            #[path = "../../../../✏️edit-text/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../✏️edit-text/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../✏️edit-text/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod splice_text {
                            #[path = "../../../../✂️splice-text/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../✂️splice-text/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../✂️splice-text/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod diff {
                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::text::diff::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🚪️io/📝️text/🔺️diff/🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
                            pub mod semio_grammar_conformance;
                        }
                        #[path = "../../../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::*;
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::pdf::*;
                                    #[path = "."]
                                    pub mod v1_4 {
                                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::pdf::v1_4::*;
                                        #[path = "."]
                                        pub mod base {
                                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::pdf::v1_4::base::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/🧱️pdf-page-text/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod docx {
                                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::docx::*;
                                    #[path = "."]
                                    pub mod v_ecma_376 {
                                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::docx::v_ecma_376::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::docx::v_ecma_376::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::md::*;
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::md::v_commonmark::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::md::v_commonmark::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json::*;
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt::*;
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod pdf {
                                    pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::pdf::*;
                                    #[path = "."]
                                    pub mod v1_4 {
                                        pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::pdf::v1_4::*;
                                        #[path = "."]
                                        pub mod base {
                                            pub use semio_s_artifact_writer_writer_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::pdf::v1_4::base::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/🧱️pdf-page-text/🧪️tests/🔬️unit/🦀️.rs"]
                                            pub mod tests;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
