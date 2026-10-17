//! 🔬️ config-tests of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;

#[cfg(test)]
pub use crate::playbook::PlaybookStep as FormStep;
#[cfg(test)]
pub use crate::schema::snapshot::FormsSnapshot;
#[cfg(test)]
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_forms_forms_snapshot::{FORMS_DOCUMENT_SCHEMA, forms_snapshot_with_state};

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
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::definition;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::response;
                    pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::schema::snapshot;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "."]
                            pub mod response {
                                pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::io::text::snapshot::response::export;
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod forms {
        pub use semio_s_artifact_forms_forms_forms::editor::forms::*;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_forms_forms_config::editor::forms::config::*;
        }
        pub use semio_s_artifact_forms_forms_tests::editor::forms::support;
        pub use semio_s_artifact_forms_forms_commands::editor::forms::commands;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod blueprint {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod try_wizard {
                        pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::try_wizard::*;
                        #[path = "."]
                        pub mod config {
                            pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::try_wizard::config::*;
                            #[cfg(test)]
                            #[path = "../../🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
