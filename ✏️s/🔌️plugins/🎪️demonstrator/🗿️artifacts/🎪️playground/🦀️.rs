//! 🎪️ Playground artifact — demonstrator's owned document entity (minimal schema stub).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

pub use semio_s_artifact_demonstrator_playground_schema::{PLAYGROUND_DOCUMENT_SCHEMA};

pub use semio_s_artifact_demonstrator_playground_editor::{PLAYGROUND_DIALECT};

pub use semio_s_artifact_demonstrator_playground_editor::{artifact_kind};

pub use semio_s_artifact_demonstrator_playground_schema::{PlaygroundCapabilityRow};

pub use semio_s_artifact_demonstrator_playground_examples::{build_pilot_languages, declaration, definition, pilot_languages, playground_diff_language, playground_document_language, playground_op_language, playground_pack_language, playground_spr_language};

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
                    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_demonstrator_playground_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod change_schema {
                            pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::change_schema::*;

                            pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::change_schema::diff;
                            pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::schema::mutations::change_schema::inverse;

                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_demonstrator_playground_examples::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_demonstrator_playground_examples::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_demonstrator_playground_editor::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_demonstrator_playground_inferences::standards::v1::subsets::any::io::binary::inferences;
}
pub mod sqlite {
    pub use semio_s_artifact_demonstrator_playground_inferences::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_demonstrator_playground_schema::standards::v1::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_demonstrator_playground_examples::standards::v1::subsets::any::io::text::*;
    pub use semio_s_artifact_demonstrator_playground_inferences::standards::v1::subsets::any::io::text::inferences;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_demonstrator_playground_editor::standards::v1::subsets::any::io::import::deserializers::artifacts::zip;
                                pub use semio_s_artifact_demonstrator_playground_editor::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_demonstrator_playground_editor::standards::v1::subsets::any::io::export::serializers::artifacts::zip;
                                pub use semio_s_artifact_demonstrator_playground_editor::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_demonstrator_playground_inferences::examples::demo::*;

        pub use semio_s_artifact_demonstrator_playground_examples::examples::demo::tests;
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod playground {
        pub use semio_s_artifact_demonstrator_playground_editor::editor::playground::*;

        pub use semio_s_artifact_demonstrator_playground_inferences::editor::playground::presence;
        pub use semio_s_artifact_demonstrator_playground_inferences::editor::playground::config;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_demonstrator_playground_editor::editor::playground::commands::change_schema;
            pub use semio_s_artifact_demonstrator_playground_inferences::editor::playground::commands::set_active_example;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_demonstrator_playground_editor::editor::playground::modes::edit::*;

                pub use semio_s_artifact_demonstrator_playground_editor::editor::playground::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod playground {
        pub use semio_s_artifact_demonstrator_playground_editor::viewer::playground::*;

        pub use semio_s_artifact_demonstrator_playground_inferences::viewer::playground::presence;
        pub use semio_s_artifact_demonstrator_playground_inferences::viewer::playground::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_demonstrator_playground_editor::viewer::playground::modes::view::*;

                pub use semio_s_artifact_demonstrator_playground_editor::viewer::playground::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::io::derived_construction::PlaygroundBuilderConstruction;
pub use crate::standards::v1::subsets::any::io::derived_analysis::{PlaygroundParts, PlaygroundAnalyzerAnalysis};
pub use crate::standards::v1::subsets::any::io::{PlaygroundBuilderFacets, PlaygroundBuilder, PlaygroundAnalyzer, PlaygroundComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
