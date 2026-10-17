//! ❓️ questions of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

pub use crate::playbook::PlaybookBlock as FormQuestion;
pub use crate::playbook::PlaybookVectorField as FormVectorField;
pub use crate::playbook::PlaybookBlockOption as FormQuestionOption;
use semio_framework_artifact_playbook_playbook as playbook;
pub use semio_s_artifact_forms_forms_snapshot::{FORMS_DIALECT};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::io::text::snapshot;
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
        #[path = "../../🦀️.rs"]
        pub mod questions;
        pub use semio_s_artifact_forms_forms_tests::editor::forms::terminology;
        pub use semio_s_artifact_forms_forms_tests::editor::forms::support;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod blueprint {
                pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::*;
                #[cfg(test)]
                #[path = "../../../🎭️modes/📝️blueprint/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::*;
                    pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::builder;
                    #[path = "."]
                    pub mod try_wizard {
                        pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::try_wizard::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/📝️blueprint/🪟️windows/▶️try/🧪️tests/🔬️control-vectors/🦀️.rs"]
                        pub mod control_vectors;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod forms {
        pub use semio_s_artifact_forms_forms_tests_2::viewer::forms::*;
        #[cfg(test)]
        #[path = "../../../../👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
