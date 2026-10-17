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
                    pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::schema::kernel;
                    #[path = "."]
                    pub mod registers {
                        pub use semio_s_artifact_architect_program_registers::standards::v1::subsets::any::schema::registers::*;
                        pub use semio_s_artifact_architect_program_catalog::standards::v1::subsets::any::schema::registers::catalog;
                    }
                    pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[cfg(test)]
                        //#region 🧪️ExchangeReadsTests
                        #[path = "../../../🦀️.rs"]
                        pub mod tests_exchange;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_architect_program_mutations::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::create_register_row;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::delete_register_row;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::replace_register_row;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::connect_adjacency;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::connect_trace;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::disconnect_trace;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::disconnect_adjacency;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::io::text::inferences;
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
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
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod tsv {
                                    #[path = "."]
                                    pub mod v_iana {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "../../../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📑️tsv/🔖️iana/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
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
        pub use semio_s_artifact_architect_program_snapshot::editor::architect::base;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_architect_program_config::editor::architect::config::*;
        }
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_architect_program_presence::editor::architect::presence::*;
        }
        #[path = "../../../../../../../✏️editor/🗂️catalog/🦀️.rs"]
        pub mod catalog;
        pub use semio_s_artifact_architect_program_snapshot::editor::architect::chrome;
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../../../✏️editor/🎮️commands/↔️adjacency/🦀️.rs"]
            pub mod adjacency;
            #[path = "../../../../../../../✏️editor/🎮️commands/🔬️analysis/🦀️.rs"]
            pub mod analysis;
            #[path = "../../../../../../../✏️editor/🎮️commands/🏗️element/🦀️.rs"]
            pub mod element;
            #[path = "../../../../../../../✏️editor/🎮️commands/📤️exchange/🦀️.rs"]
            pub mod exchange;
            #[path = "../../../../../../../✏️editor/🎮️commands/🕸️graph/🦀️.rs"]
            pub mod graph;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🦀️.rs"]
                    pub mod adjacency;
                    #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs"]
                    pub mod graph;
                    #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🦀️.rs"]
                    pub mod register;
                    #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/📓️report/🦀️.rs"]
                    pub mod report;
                    #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️trace/🦀️.rs"]
                    pub mod trace;

                #[cfg(test)]
                #[path = "../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🧪️tests/🔬️unit/🦀️.rs"]
                mod leaf_tests;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod architect {
        pub use semio_s_artifact_architect_program_mutations::viewer::architect::*;
        #[cfg(test)]
        #[path = "../../../../../../../👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_architect_program_schema::viewer::architect::modes::view::*;
                pub use semio_s_artifact_architect_program_schema::viewer::architect::modes::view::windows;
            }
        }
    }
}
