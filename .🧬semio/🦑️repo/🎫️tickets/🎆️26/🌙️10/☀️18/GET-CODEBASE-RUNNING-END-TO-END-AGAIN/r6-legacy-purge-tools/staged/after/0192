#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
//! 🎪 `stdio.obj` artifact — stdio reference format.
use semio_framework_plugin::PluginBuilder;
#[cfg(test)] use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::ObjDiff;
pub use schema::mutations::ObjMutation;
pub use schema::snapshot::ObjSnapshot;
pub use schema::ObjArtifact;

pub use semio_s_artifact_stdio_obj_inferences::{STDIO_OBJ_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_obj_inferences::{OBJ_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_obj_inferences::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_obj_mutations::{definition};

pub use semio_s_artifact_stdio_obj_inferences::{formats};

pub use semio_s_artifact_stdio_obj_mutations::{native_codec};

pub use semio_s_artifact_stdio_obj_mutations::{native_codecs};

pub use semio_s_artifact_stdio_obj_io_registry::{contribution};

pub use semio_s_artifact_stdio_obj_io_registry::{assembly, declaration, pilot_languages};

pub use semio_s_artifact_stdio_obj_inferences::{artifact_kind};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_obj_io_registry::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v3_0 {
        // ⚙️→🚪️/🧬️ dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // real code now lives in `subsets::any::{io,schema}`; this stays an inline barrel
        // so every existing `standards::v3_0::engine::*`/root `engine::*` path still resolves.
        pub use semio_s_artifact_stdio_obj_io_registry::standards::v3_0::engine;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_obj_schema::standards::v3_0::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_obj_inferences::standards::v3_0::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_obj_diff::standards::v3_0::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_obj_inferences::standards::v3_0::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_obj_mutations::standards::v3_0::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_obj_io_registry::standards::v3_0::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_obj_io_registry::standards::v3_0::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_obj_inferences::standards::v3_0::subsets::any::io::binary::inferences;
}
pub mod sqlite {
    pub use semio_s_artifact_stdio_obj_diff::standards::v3_0::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_stdio_obj_inferences::standards::v3_0::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_stdio_obj_io_registry::standards::v3_0::subsets::any::io::text::*;
    pub use semio_s_artifact_stdio_obj_inferences::standards::v3_0::subsets::any::io::text::inferences;
}

                    pub use semio_s_artifact_stdio_obj_io_registry::standards::v3_0::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_obj_io_registry::standards::v3_0::subsets::any::io::export;
                }
            }
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----
pub mod schema {
    pub use super::standards::v3_0::subsets::any::schema::*;
}
pub mod engine {
    pub use super::standards::v3_0::engine::*;
}

pub use semio_s_artifact_stdio_obj_diff::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod obj {
        pub use semio_s_artifact_stdio_obj_mutations::editor::obj::*;

        pub use semio_s_artifact_stdio_obj_diff::editor::obj::edit_rules;
        pub use semio_s_artifact_stdio_obj_diff::editor::obj::presence;
        pub use semio_s_artifact_stdio_obj_diff::editor::obj::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_obj_inferences::editor::obj::modes::edit::*;

                pub use semio_s_artifact_stdio_obj_inferences::editor::obj::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod obj {
        pub use semio_s_artifact_stdio_obj_io_registry::viewer::obj::*;

        pub use semio_s_artifact_stdio_obj_diff::viewer::obj::presence;
        pub use semio_s_artifact_stdio_obj_diff::viewer::obj::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_obj_inferences::viewer::obj::modes::view::*;

                pub use semio_s_artifact_stdio_obj_inferences::viewer::obj::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v3_0::subsets::any::io::{ObjBuilderConstruction, ObjParts, ObjAnalyzerAnalysis, ObjBuilderFacets, ObjBuilder, ObjAnalyzer, ObjComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🦀️.rs"]
mod hoisted_receiving_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_1;
