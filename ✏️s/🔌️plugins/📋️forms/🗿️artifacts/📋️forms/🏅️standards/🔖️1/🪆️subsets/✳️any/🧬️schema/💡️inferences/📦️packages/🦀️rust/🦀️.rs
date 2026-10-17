//! 💡️ inferences of `semio-s-artifact-forms-forms` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::FormsSnapshot;
pub use crate::playbook::PlaybookExpr as FormExpr;
pub use crate::playbook::PlaybookStep as FormStep;
pub use crate::schema::diff::FormsDiff;
pub use crate::playbook::PlaybookBlock as FormQuestion;
use semio_framework_artifact_playbook_playbook as playbook;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_forms_forms_snapshot::{forms_steps};

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
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod topology {
                            #[path = "../../🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_block {
                            #[path = "../../../🧬️mutations/➕create-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/➕create-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "../../../🧬️mutations/➕create-block/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                        #[path = "."]
                        pub mod create_step {
                            #[path = "../../../🧬️mutations/🌱create-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/🌱create-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "../../../🧬️mutations/🌱create-step/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                        #[path = "."]
                        pub mod delete_block {
                            #[path = "../../../🧬️mutations/➖delete-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/➖delete-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "../../../🧬️mutations/➖delete-block/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                        #[path = "."]
                        pub mod delete_step {
                            #[path = "../../../🧬️mutations/🗑️delete-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/🗑️delete-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "../../../🧬️mutations/🗑️delete-step/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                        #[path = "."]
                        pub mod replace_block {
                            #[path = "../../../🧬️mutations/🔁replace-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/🔁replace-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "../../../🧬️mutations/🔁replace-block/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                        #[path = "."]
                        pub mod change_block_field {
                            pub use semio_s_artifact_forms_forms_diff::standards::v1::subsets::any::schema::mutations::change_block_field::mutation;
                        }
                        #[path = "."]
                        pub mod change_step_description {
                            #[path = "../../../🧬️mutations/📝change-step-description/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/📝change-step-description/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "../../../🧬️mutations/📝change-step-description/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                        #[path = "."]
                        pub mod rename_step {
                            #[path = "../../../🧬️mutations/✏️rename-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/✏️rename-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[path = "../../../🧬️mutations/✏️rename-step/🦠️mutation/🦀️.rs"]
                            pub mod mutation;
                        }
                    }
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
