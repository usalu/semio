//! ✏️ architect of `semio-s-artifact-architect-program` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

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
                    pub mod mutations {
                        pub use semio_s_artifact_architect_program_mutations::standards::v1::subsets::any::schema::mutations::*;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::create_register_row;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_architect_program_tests_exchange::standards::v1::subsets::any::io::export::serializers::artifacts::csv;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_architect_program_tests_2::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod architect {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_architect_program_snapshot::editor::architect::editor_vocabulary;
        pub use semio_s_artifact_architect_program_panels::editor::architect::behavior;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_architect_program_config::editor::architect::config::*;
            pub use semio_s_artifact_architect_program_tests_2::editor::architect::config::schema;
        }
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_architect_program_presence::editor::architect::presence::*;
        }
        pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::catalog;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::commands::adjacency;
            pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::commands::analysis;
            pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::commands::element;
            pub use semio_s_artifact_architect_program_mutations::editor::architect::commands::example;
            pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::commands::exchange;
            pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::commands::graph;
            pub use semio_s_artifact_architect_program_panels::editor::architect::commands::register;
            pub use semio_s_artifact_architect_program_mutations::editor::architect::commands::search;
            pub use semio_s_artifact_architect_program_panels::editor::architect::commands::template;
        }
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_architect_program_tests_2::editor::architect::modes::report;
            pub use semio_s_artifact_architect_program_tests_2::editor::architect::modes::review;
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::modes::edit::*;
                pub use semio_s_artifact_architect_program_tests_exchange::editor::architect::modes::edit::windows;
            }
        }
        pub use semio_s_artifact_architect_program_panels::editor::architect::panels;
    }
}
