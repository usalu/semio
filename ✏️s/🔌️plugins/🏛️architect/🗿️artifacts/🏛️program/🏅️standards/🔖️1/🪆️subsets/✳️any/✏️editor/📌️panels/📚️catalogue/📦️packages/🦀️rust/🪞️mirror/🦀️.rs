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
                    pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::schema::kernel;
                    #[path = "."]
                    pub mod registers {
                        pub use semio_s_artifact_architect_program_registers::standards::v1::subsets::any::schema::registers::*;
                        pub use semio_s_artifact_architect_program_catalog::standards::v1::subsets::any::schema::registers::catalog;
                    }
                    pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_architect_program_mutations::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::create_register_row;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::connect_adjacency;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::connect_trace;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::disconnect_adjacency;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::io::program_json;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_architect_program_tests_exchange::standards::v1::subsets::any::io::import::deserializers::artifacts::csv;
                                pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    pub use semio_s_artifact_architect_program_tests_3::standards::v1::subsets::any::io::text;
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod zip {
                                    #[path = "."]
                                    pub mod v2_0 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                pub use semio_s_artifact_architect_program_tests_exchange::standards::v1::subsets::any::io::export::serializers::artifacts::csv;
                                pub use semio_s_artifact_architect_program_tests_exchange::standards::v1::subsets::any::io::export::serializers::artifacts::tsv;
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
                                pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                    pub use semio_s_artifact_architect_program_mutations::standards::v1::subsets::any::io::tables;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod architect {
        pub use semio_s_artifact_architect_program_snapshot::editor::architect::editor_vocabulary;
        #[path = "../../../../../🔧️behavior/🦀️.rs"]
        pub mod behavior;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_architect_program_config::editor::architect::config::*;
        }
        pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::catalog;
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod graph {
                pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::commands::graph::*;
                #[cfg(test)]
                #[path = "../../../../../🎮️commands/🕸️graph/🧪️tests/🦀️.rs"]
                pub mod tests;
            }
            #[path = "../../../../../🎮️commands/📋️register/🦀️.rs"]
            pub mod register;
            #[path = "../../../../../🎮️commands/📐️template/🦀️.rs"]
            pub mod template;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::modes::edit::windows;
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../🦀️.rs"]
            pub mod catalogue;
            #[path = "../../../../🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../../../🔍️inspection/🦀️.rs"]
            pub mod inspection;

        #[cfg(test)]
        #[path = "../../../../🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod architect {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_architect_program_schema::viewer::architect::modes::view::windows;
            }
        }
    }
}
