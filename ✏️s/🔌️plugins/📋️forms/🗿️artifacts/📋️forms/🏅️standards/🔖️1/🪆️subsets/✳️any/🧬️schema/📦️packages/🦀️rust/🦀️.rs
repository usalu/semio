//! 🧬️ schema of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::FormsSnapshot;
use semio_framework_artifact_playbook_playbook as playbook;
pub use crate::playbook::PlaybookBlock as FormQuestion;
pub use crate::playbook::PlaybookStep as FormStep;
pub use crate::playbook::PLAYBOOK_BUILTIN_KINDS as FORM_BUILTIN_KINDS;
#[cfg(test)]
pub use crate::schema::diff::FormsDiff;
#[cfg(test)]
pub use crate::playbook::PlaybookExpr as FormExpr;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_forms_forms_snapshot::{FORMS_DOCUMENT_SCHEMA, FormsResultsChild, FormsStructureChild, as_playbook_spec, forms_snapshot_with_state, forms_steps};

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
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::definition;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::response;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::validation;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::schema::snapshot::*;
                        #[cfg(test)]
                        #[path = "../../🧪️tests/💾️persistence/🦀️.rs"]
                        pub mod persistence_tests;
                    }
                    pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::diff;
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
                    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod forms {
        #[path = "."]
        pub mod questions {
            #[path = "."]
            pub mod visibility {
                pub use semio_s_artifact_forms_forms_diff::editor::forms::questions::visibility::*;
                #[cfg(test)]
                #[path = "../../../✏️editor/❓️questions/🫥️visibility/🧪️tests/🔬️editing/🦀️.rs"]
                pub mod tests;
            }
            #[path = "."]
            pub mod extensions {
                pub use semio_s_artifact_forms_forms_diff::editor::forms::questions::extensions::*;
                #[cfg(test)]
                #[path = "../../../✏️editor/❓️questions/🧩️extensions/🧪️tests/🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_forms_forms_config::editor::forms::config::*;
        }
        pub use semio_s_artifact_forms_forms_tests::editor::forms::terminology;
        pub use semio_s_artifact_forms_forms_tests::editor::forms::support;
        #[path = "../../../✏️editor/🧩️contributions/🦀️.rs"]
        pub mod contributions;
        #[path = "."]
        pub mod modes {
            #[path = "../../../✏️editor/🎭️modes/📨️responses/🦀️.rs"]
            pub mod responses;
        }
    }
}
