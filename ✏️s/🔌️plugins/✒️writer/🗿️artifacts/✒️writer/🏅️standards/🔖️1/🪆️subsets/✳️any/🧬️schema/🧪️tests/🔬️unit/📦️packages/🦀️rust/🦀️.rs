//! 🔬️ tests of `semio-s-artifact-writer-writer` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_schema as framework_schema;

#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::snapshot::WriterSnapshot;
pub use semio_s_artifact_writer_writer_schema::{WRITER_DOCUMENT_SCHEMA, WriterDocumentChild, document_child_handle_with_text};

#[path = "."]
pub mod graph_manifest {
    #[path = "../../../../../../../../../🤖️generated/🗣️writer-languages/🦀️.rs"]
    pub mod writer_languages;
}
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
                    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::schema::snapshot;
                    #[cfg(test)]
                    #[path = "../../🦀️.rs"]
                    pub mod tests;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🌐change-language/🦀️.rs"]
                            pub mod change_language;
                            #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/✏️edit-text/🦀️.rs"]
                            pub mod edit_text;
                            #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/✂️splice-text/🦀️.rs"]
                            pub mod splice_text;
                            #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🔗change-uri/🦀️.rs"]
                            pub mod change_uri;
                            #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🏷️rename-writer/🦀️.rs"]
                            pub mod rename_writer;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🌐change-language/🦀️.rs"]
                            pub mod change_language;
                            #[path = "../../../../../🚪️io/📝️text/🧬️mutations/✏️edit-text/🦀️.rs"]
                            pub mod edit_text;
                            #[path = "../../../../../🚪️io/📝️text/🧬️mutations/✂️splice-text/🦀️.rs"]
                            pub mod splice_text;
                            #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🔗change-uri/🦀️.rs"]
                            pub mod change_uri;
                            #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🏷️rename-writer/🦀️.rs"]
                            pub mod rename_writer;
                        }
                    }
                    #[path = "../../../../../🚪️io/🪶️sqlite/🦀️.rs"]
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
        #[path = "../../../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod writer {
        pub use semio_s_artifact_writer_writer_tests_2::editor::writer::*;
        #[path = "../../../../../✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "../../../../../✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;
        #[path = "../../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
        pub mod demo_session;
        #[cfg(test)]
        #[path = "../../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        pub mod example;
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod writer {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
