//! 🔬️ tests-3 of `semio-s-artifact-architect-program` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;

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
                        pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::schema::registers::kernel;
                        #[path = "."]
                        pub mod program_structure {
                            pub use semio_s_artifact_architect_program_program_structure::standards::v1::subsets::any::schema::registers::program_structure::*;
                            #[cfg(test)]
                            #[path = "../../🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod operations_requirements {
                            pub use semio_s_artifact_architect_program_program_structure::standards::v1::subsets::any::schema::registers::operations_requirements::*;
                            #[cfg(test)]
                            #[path = "../../../../../🔁️operations-requirements/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod context_requirements {
                            pub use semio_s_artifact_architect_program_context_requirements::standards::v1::subsets::any::schema::registers::context_requirements::*;
                            #[cfg(test)]
                            #[path = "../../../../../🌐️context-requirements/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[path = "."]
                        pub mod decisions {
                            pub use semio_s_artifact_architect_program_context_requirements::standards::v1::subsets::any::schema::registers::decisions::*;
                            #[cfg(test)]
                            #[path = "../../../../../📋️decisions/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    pub use semio_s_artifact_architect_program_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_architect_program_inferences::standards::v1::subsets::any::schema::diff;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
