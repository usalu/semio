//! 🌿️ VCS artifact — the document entity the `vcs-play` app edits.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use semio_s_artifact_vcs_vcs_inferences::{VCS_DOCUMENT_SCHEMA};

pub use semio_s_artifact_vcs_vcs_editor::{VCS_DIALECT};

pub use semio_s_artifact_vcs_vcs_editor::{artifact_kind};
pub use semio_s_artifact_vcs_vcs_inferences::{definition};

pub use semio_s_artifact_vcs_vcs_v1::{artifact};

pub use semio_s_artifact_vcs_vcs_tests::{VcsApplication};

impl<A> VcsApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::vcs::VcsPlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::vcs::VcsViewer>>>
{
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_vcs_vcs_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_vcs_vcs_tests::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_vcs_vcs_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::operations;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod add_tag {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::add_tag::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::add_tag::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::add_tag::inverse;

                        }
                        #[path = "."]
                        pub mod rename_vcs {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::rename_vcs::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::rename_vcs::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::rename_vcs::inverse;

                        }
                        #[path = "."]
                        pub mod change_notes {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_notes::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_notes::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_notes::inverse;

                        }
                        #[path = "."]
                        pub mod change_counter {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_counter::inverse;

                        }
                        #[path = "."]
                        pub mod remove_tag {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::remove_tag::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::remove_tag::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::remove_tag::inverse;

                        }
                        #[path = "."]
                        pub mod change_status {
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_status::*;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_status::diff;
                            pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::schema::mutations::change_status::inverse;

                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_vcs_vcs_tests::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_vcs_vcs_viewer::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_vcs_vcs_tests::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::binary::inferences;
    pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::io::binary::mutations;
    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::binary::snapshot;
}
pub mod sqlite {
    pub use semio_s_artifact_vcs_vcs_viewer::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_vcs_vcs_viewer::standards::v1::subsets::any::io::text::*;
    pub use semio_s_artifact_vcs_vcs_tests::standards::v1::subsets::any::io::text::diff;
    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::text::inferences;
    pub use semio_s_artifact_vcs_vcs_mutations::standards::v1::subsets::any::io::text::mutations;
    pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::text::snapshot;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::zip;
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_vcs_vcs_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::csv;
                                pub use semio_s_artifact_vcs_vcs_tests::standards::v1::subsets::any::io::import::deserializers::artifacts::xlsx;
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::zip;
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_vcs_vcs_tests::standards::v1::subsets::any::io::export::serializers::artifacts::csv;
                                pub use semio_s_artifact_vcs_vcs_tests::standards::v1::subsets::any::io::export::serializers::artifacts::xlsx;
                                pub use semio_s_artifact_vcs_vcs_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::VcsDemoMutation;
}

pub use semio_s_artifact_vcs_vcs_inferences::diff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_vcs_vcs_inferences::snapshot;
pub use crate::standards::v1::subsets::any::schema::diff::VcsDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::VcsDemoMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::VcsSnapshot;

pub use semio_s_artifact_vcs_vcs_inferences::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod vcs {
        pub use semio_s_artifact_vcs_vcs_editor::editor::vcs::*;
pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::demo_session;

        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::config::*;

            pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::config::schema;
        }

        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::presence::*;

            pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::presence::schema;
        }
        pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::terminology;

        pub use semio_s_artifact_vcs_vcs_editor::editor::vcs::commands;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_vcs_vcs_editor::editor::vcs::modes::edit::*;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_vcs_vcs_editor::editor::vcs::modes::edit::windows::editor;
                    pub use semio_s_artifact_vcs_vcs_viewer::editor::vcs::modes::edit::windows::history;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_vcs_vcs_editor::editor::vcs::panels::document;
            pub use semio_s_artifact_vcs_vcs_editor::editor::vcs::panels::inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod vcs {
        pub use semio_s_artifact_vcs_vcs_tests::viewer::vcs::*;

        pub use semio_s_artifact_vcs_vcs_viewer::viewer::vcs::presence;
        pub use semio_s_artifact_vcs_vcs_viewer::viewer::vcs::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_vcs_vcs_tests::viewer::vcs::modes::view::*;

                pub use semio_s_artifact_vcs_vcs_tests::viewer::vcs::modes::view::windows;
            }
        }
    }
}

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_1;
