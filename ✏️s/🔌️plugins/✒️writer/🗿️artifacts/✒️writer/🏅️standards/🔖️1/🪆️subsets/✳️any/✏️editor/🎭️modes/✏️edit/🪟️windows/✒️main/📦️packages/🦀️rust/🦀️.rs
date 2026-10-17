//! ✒️ editor of `semio-s-artifact-writer-writer` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::editor::writer::modes::edit::windows::main::config::WriterEditorSettings;
pub use crate::editor::writer::modes::edit::windows::main::config::WriterCamera;
pub use crate::standards::v1::subsets::any::schema::snapshot::WriterSnapshot;
pub use crate::editor::writer::modes::edit::windows::main::transient::WriterEditorSelection;
pub use crate::editor::writer::modes::edit::windows::main::config::default_camera;
pub use crate::editor::writer::modes::edit::windows::main::config::default_zoom;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_writer_writer_schema::{writer_text};

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
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_writer_writer_schema::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_writer_writer_rename_writer::standards::v1::subsets::any::io::text::inferences;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod writer {
        pub use semio_s_artifact_writer_writer_inferences::editor::writer::base;
        pub use semio_s_artifact_writer_writer_tests::editor::writer::terminology;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod options {
                            #[path = "../../☑️options/🔤️font-size/🦀️.rs"]
                            pub mod font_size;
                            #[path = "../../☑️options/📏️line-height/🦀️.rs"]
                            pub mod line_height;
                            #[path = "../../☑️options/🔢️line-numbers/🦀️.rs"]
                            pub mod line_numbers;
                            #[path = "../../☑️options/🗂️tab/🦀️.rs"]
                            pub mod tab_size;

                        #[cfg(test)]
                        #[path = "../../☑️options/🧪️tests/🔬️unit/🦀️.rs"]
                        mod leaf_tests;
                        }
                    }
                }
            }
        }
    }
}
