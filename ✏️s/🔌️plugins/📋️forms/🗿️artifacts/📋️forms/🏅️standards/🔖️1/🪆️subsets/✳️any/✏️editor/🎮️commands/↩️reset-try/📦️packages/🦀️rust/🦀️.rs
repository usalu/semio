//! ▶️ The try-wizard commands of `semio-s-artifact-forms-forms`: stepping, submitting and the staged try values, read through the same `crate::` namespace the other commands use.

extern crate semio_framework_os_kernel as dsl;

pub use crate::schema::snapshot::FormsSnapshot;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_forms_forms_snapshot::{forms_steps};
pub use semio_s_artifact_forms_forms_mutations::op;

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
                    pub use semio_s_artifact_forms_forms_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::definition;
                    pub use semio_s_artifact_forms_forms_definition::standards::v1::subsets::any::schema::response;
                    pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_forms_forms_mutations::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_forms_forms_snapshot::standards::v1::subsets::any::io::text::snapshot::*;
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
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_forms_forms_config::editor::forms::config::*;
        }
        pub use semio_s_artifact_forms_forms_tests::editor::forms::support;
        #[path = "."]
        pub mod commands {
            #[path = "../../../▶️next-step/🦀️.rs"]
            pub mod next_step;
            #[path = "../../../◀️previous-step/🦀️.rs"]
            pub mod previous_step;
            #[path = "../../../✅️submit/🦀️.rs"]
            pub mod submit;
            #[path = "../../../🎯️set-try-value/🦀️.rs"]
            pub mod set_try_value;
            #[path = "../../../📍️set-try-value-step/🦀️.rs"]
            pub mod set_try_value_step;
            #[path = "../../../🗃️set-try-values/🦀️.rs"]
            pub mod set_try_values;
            #[path = "../../🦀️.rs"]
            pub mod reset_try;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod blueprint {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_forms_forms_tests_2::editor::forms::modes::blueprint::windows::try_wizard;
                }
            }
        }
    }
}
