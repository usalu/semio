//! 🧬️ mutations of `semio-s-artifact-architect-program` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
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
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::create_register_row;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::delete_register_row;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::rename_register_row;
                        pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::mutations::replace_register_row;
                        pub use semio_s_artifact_architect_program_catalog::standards::v1::subsets::any::schema::mutations::rename_governance;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::replace_governance;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::rename_meta;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::replace_meta;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::rename_project;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::replace_project;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::connect_adjacency;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::connect_trace;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::disconnect_trace;
                        pub use semio_s_artifact_architect_program_mut_any::standards::v1::subsets::any::schema::mutations::disconnect_adjacency;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "../../../../📊️tables/🦀️.rs"]
                    pub mod tables;
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
        pub use semio_s_artifact_architect_program_snapshot::editor::architect::editor_vocabulary;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_architect_program_config::editor::architect::config::*;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../✏️editor/🎮️commands/📚️example/🦀️.rs"]
            pub mod example;
            #[path = "../../../../../✏️editor/🎮️commands/🔍️search/🦀️.rs"]
            pub mod search;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod architect {
        #[path = "../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
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
