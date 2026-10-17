//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../../../../🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_vcs_vcs_schema::standards::v1::subsets::any::schema::*;
                    #[cfg(test)]
                    #[path = "../../../🦀️.rs"]
                    pub mod tests;
                    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod summary {
                            pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::inferences::summary::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../../💡️inferences/📊summary/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../../🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod operations {
                        pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::operations::*;
                        #[cfg(test)]
                        #[path = "../../../../../⚙️operations/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod add_tag {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::add_tag::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::add_tag::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::add_tag::inverse;
                        }
                        #[path = "."]
                        pub mod rename_vcs {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::rename_vcs::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::rename_vcs::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::rename_vcs::inverse;
                        }
                        #[path = "."]
                        pub mod change_notes {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_notes::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_notes::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_notes::inverse;
                        }
                        #[path = "."]
                        pub mod change_counter {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::inverse;
                        }
                        #[path = "."]
                        pub mod remove_tag {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::remove_tag::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::remove_tag::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::remove_tag::inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::text::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../../../../🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "../../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                        #[path = "."]
                        pub mod mutations {
                            #[cfg(test)]
                            #[path = "../../../../../../🚪️io/📝️text/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::zip;
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod xlsx {
                                    #[path = "."]
                                    pub mod v_ecma_376 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::zip;
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod xlsx {
                                    #[path = "."]
                                    pub mod v_ecma_376 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    pub use semio_s_artifact_vcs_vcs_inferences::examples::*;
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_vcs_vcs_inferences::examples::demo::*;
        #[cfg(test)]
        #[path = "../../../../../../📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        pub mod example;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod vcs {
        pub use semio_s_artifact_vcs_vcs_editor::editor::vcs::*;
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_vcs_vcs_editor::editor::vcs::modes::*;
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_vcs_vcs_editor::editor::vcs::modes::edit::*;
                #[cfg(test)]
                #[path = "../../../../../../✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_vcs_vcs_editor::editor::vcs::modes::edit::windows::editor;
                    pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::modes::edit::windows::history;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod vcs {
        #[path = "../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/📜️history/🦀️.rs"]
                    pub mod history;
                }
            }
        }
    }
}
