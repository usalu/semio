//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../../../../../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../../../../../../✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_mathematical_equation_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_mathematical_equation_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_mathematical_equation_mutations::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../../✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_mathematical_equation_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../../✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../../✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../../✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
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
                                pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::csv;
                                pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::md;
                                pub use semio_s_artifact_mathematical_equation_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
            #[path = "."]
            pub mod geometry {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::geometry::schema::mutations::insert_point;
                        pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::geometry::schema::mutations::remove_point;
                        pub use semio_s_artifact_mathematical_equation_mut_family::standards::v1::subsets::geometry::schema::mutations::set_point_positions;
                        #[cfg(test)]
                        #[path = "../../../🦀️.rs"]
                        pub mod leaf_tests;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_mathematical_equation_snapshot::snapshot;
pub use semio_s_artifact_mathematical_equation_examples::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod equation {
        pub use semio_s_artifact_mathematical_equation_editor::editor::equation::*;
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_mathematical_equation_editor::editor::equation::modes::*;
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_mathematical_equation_editor::editor::equation::modes::edit::*;
                #[cfg(test)]
                #[path = "../../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_mathematical_equation_editor::editor::equation::modes::edit::windows::*;
                    #[path = "."]
                    pub mod geometry {
                        pub use semio_s_artifact_mathematical_equation_editor::editor::equation::modes::edit::windows::geometry::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️geometry/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod graph {
                        pub use semio_s_artifact_mathematical_equation_editor::editor::equation::modes::edit::windows::graph::*;
                        #[cfg(test)]
                        #[path = "../../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                        #[path = "."]
                        pub mod config {
                            pub use semio_s_artifact_mathematical_equation_editor::editor::equation::modes::edit::windows::graph::config::*;
                            #[cfg(test)]
                            #[path = "../../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                            #[cfg(test)]
                            #[path = "../../../../../../../../✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧪️tests/🔬️mutation-vectors/🦀️.rs"]
                            pub mod mutation_vectors;
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod equation {
        pub use semio_s_artifact_mathematical_equation_mutations::viewer::equation::*;
    }
}
