//! 🔬️ tests-2 of `semio-s-artifact-architect-program` — a path-preserving carve of the artifact root mounts.

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
                    #[path = "."]
                    pub mod kernel {
                        pub use semio_s_artifact_architect_program_tests::standards::v1::subsets::any::schema::kernel::*;
                        #[cfg(test)]
                        #[path = "../../🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod architect {
        pub use semio_s_artifact_architect_program_architect::editor::architect::*;
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../✏️editor/🎚️config/🚪️io/🦀️.rs"]
            pub mod io;
            #[path = "../../../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../✏️editor/👥️presence/🚪️io/🦀️.rs"]
            pub mod io;
        }
        #[path = "."]
        pub mod modes {
            #[path = "../../../../../../✏️editor/🎭️modes/📊️report/🦀️.rs"]
            pub mod report;
            #[path = "../../../../../../✏️editor/🎭️modes/🔍️review/🦀️.rs"]
            pub mod review;
        }
        #[path = "../../../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod demo_session;
        #[cfg(test)]
        #[path = "../../../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        pub mod example;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod architect {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
