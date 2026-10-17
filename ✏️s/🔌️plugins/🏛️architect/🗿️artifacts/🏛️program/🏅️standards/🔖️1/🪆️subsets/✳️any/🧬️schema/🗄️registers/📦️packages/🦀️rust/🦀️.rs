//! 🗄️ registers of `semio-s-artifact-architect-program` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;

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
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::schema::registers::kernel;
                        pub use semio_s_artifact_architect_program_program_structure::standards::v1::subsets::any::schema::registers::program_structure;
                        pub use semio_s_artifact_architect_program_program_structure::standards::v1::subsets::any::schema::registers::operations_requirements;
                        pub use semio_s_artifact_architect_program_context_requirements::standards::v1::subsets::any::schema::registers::context_requirements;
                        pub use semio_s_artifact_architect_program_context_requirements::standards::v1::subsets::any::schema::registers::decisions;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod architect {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod report {
                pub use semio_s_artifact_architect_program_tests_2::editor::architect::modes::report::*;
                #[cfg(test)]
                #[path = "../../../../✏️editor/🎭️modes/📊️report/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod review {
                pub use semio_s_artifact_architect_program_tests_2::editor::architect::modes::review::*;
                #[cfg(test)]
                #[path = "../../../../✏️editor/🎭️modes/🔍️review/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
