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
                    pub use semio_s_artifact_forms_forms_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::definition;
                    #[path = "."]
                    pub mod response {
                        pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::response::*;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        #[path = "."]
                        pub mod topology {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::inferences::topology::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../../../💡️inferences/🧭topology/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod commit_response {
                            pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations::commit_response::*;
                            pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations::commit_response::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../../🧬️mutations/📨️commit-response/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod discard_response {
                            pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations::discard_response::*;
                            pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations::discard_response::mutation;
                            #[cfg(test)]
                            #[path = "../../../../../../🧬️mutations/↩️discard-response/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        #[path = "../../../../../../🧬️mutations/🧪️tests/🔬️kinds-catalog/🦀️.rs"]
                        pub mod kinds_catalog;
                        #[path = "../../../../../../🧬️mutations/🌉️apply/🦀️.rs"]
                        pub mod apply;
                        #[path = "."]
                        pub mod create_step {
                            pub use semio_s_artifact_forms_forms_inferences::standards::v1::subsets::any::schema::mutations::create_step::mutation;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::export::serializers::artifacts::csv;
                                #[path = "."]
                                pub mod xlsx {
                                    pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::export::serializers::artifacts::xlsx::*;
                                    #[path = "."]
                                    pub mod v_ecma_376 {
                                        pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::export::serializers::artifacts::xlsx::v_ecma_376::*;
                                        #[path = "."]
                                        pub mod any {
                                            pub use semio_s_artifact_forms_forms_archive_codecs::standards::v1::subsets::any::io::export::serializers::artifacts::xlsx::v_ecma_376::any::*;
                                            #[cfg(test)]
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🧪️tests/🔬️unit/🦀️.rs"]
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
pub use semio_s_artifact_forms_forms_mutations::op;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod forms {
        #[path = "."]
        pub mod questions {
            pub use semio_s_artifact_forms_forms_diff::editor::forms::questions::extensions;
        }
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_forms_forms_config::editor::forms::config::*;
        }
        pub use semio_s_artifact_forms_forms_tests::editor::forms::terminology;
        pub use semio_s_artifact_forms_forms_tests::editor::forms::support;
        pub use semio_s_artifact_forms_forms_schema::editor::forms::contributions;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod responses {
                #[path = "."]
                pub mod results {
                    pub use semio_s_artifact_forms_forms_diff::editor::forms::modes::responses::results::*;
                    #[cfg(test)]
                    #[path = "../../../../../../../✏️editor/🎭️modes/📨️responses/🪟️windows/📊️results/🧪️tests/🦀️.rs"]
                    pub mod tests;
                }
            }
            #[path = "."]
            pub mod blueprint {
                #[path = "../../../../../../../✏️editor/🎭️modes/📝️blueprint/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../../✏️editor/🎭️modes/📝️blueprint/🪟️windows/🧱️builder/🦀️.rs"]
                    pub mod builder;
                    #[path = "../../../../../../../✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🦀️.rs"]
                    pub mod try_wizard;
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod inspection {
                #[path = "../../../../../../../✏️editor/📌️panels/🔍️inspection/🧱️controls/🦀️.rs"]
                pub mod controls;
            }
            #[path = "../../../../../../../✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../../../../../../✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod forms {
        #[path = "../../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/▶️try/🦀️.rs"]
                    pub mod try_wizard;
                }
            }
        }
    }
}
