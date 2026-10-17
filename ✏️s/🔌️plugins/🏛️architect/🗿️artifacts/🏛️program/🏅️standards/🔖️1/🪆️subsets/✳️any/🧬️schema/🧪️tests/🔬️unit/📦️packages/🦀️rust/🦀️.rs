//! 🔬️ tests of `semio-s-artifact-architect-program` — a path-preserving carve of the artifact root mounts.

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
                    pub use semio_s_artifact_architect_program_schema::standards::v1::subsets::any::schema::*;
                    #[path = "../../../../🧱️kernel/🦀️.rs"]
                    pub mod kernel;
                    #[path = "."]
                    pub mod registers {
                        #[path = "../../../../🗄️registers/🧱️kernel/🦀️.rs"]
                        pub mod kernel;
                    }
                    #[cfg(test)]
                    #[path = "../../🦀️.rs"]
                    pub mod tests;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            #[path="../../../../../🚪️io/🪶️sqlite/📸️snapshot/🧱️ddl/🦀️.rs"]pub mod ddl;
                        }
                    }
                    #[path = "../../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
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
        pub mod config {
            #[path = "."]
            pub mod io {
                #[path = "../../../../../✏️editor/🎚️config/🚪️io/💾️binary/🦀️.rs"]
                pub mod binary;
                #[path = "../../../../../✏️editor/🎚️config/🚪️io/📝️text/🦀️.rs"]
                pub mod text;
            }
        }
        #[path = "."]
        pub mod presence {
            #[path = "."]
            pub mod io {
                #[path = "../../../../../✏️editor/👥️presence/🚪️io/💾️binary/🦀️.rs"]
                pub mod binary;
                #[path = "../../../../../✏️editor/👥️presence/🚪️io/📝️text/🦀️.rs"]
                pub mod text;
            }
        }
    }
}
