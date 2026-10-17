//! 🌐️ context-requirements of `semio-s-artifact-architect-program` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
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
                        #[path = "../../🦀️.rs"]
                        pub mod context_requirements;
                        #[path = "../../../📋️decisions/🦀️.rs"]
                        pub mod decisions;
                    }
                }
            }
        }
    }
}
