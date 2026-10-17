//! 🧬️ schema of `semio-s-artifact-vcs-vcs` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::standards::v1::subsets::any::schema::snapshot::VcsSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::VcsDemoMutation;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::diff::VcsDiff;
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_vcs_vcs_inferences::{VCS_DOCUMENT_SCHEMA};

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
                    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod add_tag {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::add_tag::*;
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🏷️add-tag/🧪️tests/🧪️appends-urgent-tag/🦀️.rs"]
                            pub mod tests_appends_urgent_tag;
                        }
                        #[path = "."]
                        pub mod rename_vcs {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::rename_vcs::*;
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/✏️rename-vcs/🧪️tests/🧪️retitles-the-document/🦀️.rs"]
                            pub mod tests_retitles_the_document;
                        }
                        #[path = "."]
                        pub mod change_notes {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_notes::*;
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/📝change-notes/🧪️tests/🧪️rewrites-the-notes/🦀️.rs"]
                            pub mod tests_rewrites_the_notes;
                        }
                        #[path = "."]
                        pub mod change_counter {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::inverse;
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🔢change-counter/🧪️tests/🧪️sets-counter-to-seven/🦀️.rs"]
                            pub mod tests_sets_counter_to_seven;
                        }
                        #[path = "."]
                        pub mod remove_tag {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::remove_tag::*;
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🗑️remove-tag/🧪️tests/🧪️detaches-the-review-tag/🦀️.rs"]
                            pub mod tests_detaches_the_review_tag;
                        }
                        #[path = "."]
                        pub mod change_status {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_status::*;
                            #[cfg(test)]
                            #[path = "../../🧬️mutations/🚦change-status/🧪️tests/🧪️draft-to-review/🦀️.rs"]
                            pub mod tests_draft_to_review;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::io::binary::mutations::*;
                            #[cfg(test)]
                            #[path = "../../../🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                }
            }
        }
    }
}
