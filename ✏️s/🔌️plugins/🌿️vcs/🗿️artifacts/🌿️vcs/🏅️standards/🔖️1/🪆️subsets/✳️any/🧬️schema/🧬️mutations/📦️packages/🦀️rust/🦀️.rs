//! 🧬️ mutations of `semio-s-artifact-vcs-vcs` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::VcsSnapshot;
pub use crate::standards::v1::subsets::any::schema::diff::VcsDiff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::VcsDemoMutation;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
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
                    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::diff;
                    #[path = "../../../⚙️operations/🦀️.rs"]
                    pub mod operations;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod add_tag {
                            #[path = "../../🏷️add-tag/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🏷️add-tag/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🏷️add-tag/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod rename_vcs {
                            #[path = "../../✏️rename-vcs/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../✏️rename-vcs/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../✏️rename-vcs/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_notes {
                            #[path = "../../📝change-notes/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../📝change-notes/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../📝change-notes/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_counter {
                            #[path = "../../🔢change-counter/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🔢change-counter/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🔢change-counter/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_tag {
                            #[path = "../../🗑️remove-tag/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗑️remove-tag/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🗑️remove-tag/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_status {
                            #[path = "../../🚦change-status/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🚦change-status/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../🚦change-status/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_vcs_vcs_inferences::diff;
