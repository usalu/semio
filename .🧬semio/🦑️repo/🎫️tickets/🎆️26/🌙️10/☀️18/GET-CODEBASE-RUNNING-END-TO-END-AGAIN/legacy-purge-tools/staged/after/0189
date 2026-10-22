//! 🎪 `stdio.deflate` artifact — stdio reference format.
#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
use semio_framework_plugin::PluginBuilder;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

pub use semio_s_artifact_stdio_contract::{apply_mutation, apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};

pub use schema::diff::DeflateDiff;
pub use schema::mutations::DeflateMutation;
pub use schema::snapshot::DeflateSnapshot;
pub use schema::DeflateArtifact;
/// 🧬️ The wire contract a native test host decodes `🥒️.feature` witnesses through and inverts them with
/// (`Mutation::from_payload_value`/`Mutation::inverse`), re-exported because such a host links this crate alone.

pub use semio_s_artifact_stdio_deflate_snapshot::{STDIO_DEFLATE_DOCUMENT_SCHEMA};

pub use semio_s_artifact_stdio_deflate_examples::{DEFLATE_ARTIFACT_SCHEMA_ID};

pub use semio_s_artifact_stdio_deflate_examples::{ARTIFACT_DEFINITION_SCHEMA};

pub use semio_s_artifact_stdio_deflate_schema::{definition};

pub use semio_s_artifact_stdio_deflate_examples::{formats};

pub use semio_s_artifact_stdio_deflate_schema::{native_codec};

pub use semio_s_artifact_stdio_deflate_schema::{native_codecs};

pub use semio_s_artifact_stdio_deflate_tests::{contribution};

pub use semio_s_artifact_stdio_deflate_tests::{assembly, declaration, pilot_languages};

pub use semio_s_artifact_stdio_deflate_schema::{artifact_kind};
//#region 🚪️DerivedIoRegistry
pub use semio_s_artifact_stdio_deflate_tests::io_registry;
//#endregion 🚪️DerivedIoRegistry

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_rfc1950 {
        // 🐜️ `⚙️engine/` dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
        // pure format algorithms (Adler32/BitIO/Huffman/LZ77) + `zlib_compress`/
        // `zlib_decompress` + `encode_deflate_snapshot`/`decode_deflate_snapshot` +
        // `io_registry` + `register_schema_specs` all moved into `subsets::any::io`
        // (rule 6: deflate's Huffman/LZ77 is the clearest "keep with the codec" case);
        // `empty_deflate_snapshot`/`demo_deflate_snapshot` moved to `subsets::any::schema`;
        // `DeflateEngine` (zero construction sites repo-wide) deleted outright; tests moved
        // into `subsets::any::io` (codec tests) and `subsets::any::schema::inferences`
        // (conformance laws). `deflate` is NOT one of stdio's 10 protected imperative
        // plugin-root calls (it already used the declarative `ArtifactDeclaration` builder)
        // — every call site was repointed directly at the new locations, and no `engine`
        // shim survives here since nothing references it anymore.
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_deflate_schema::standards::v_rfc1950::subsets::any::schema::*;

                    pub use semio_s_artifact_stdio_deflate_snapshot::standards::v_rfc1950::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_deflate_mutations::standards::v_rfc1950::subsets::any::schema::inferences;
                    pub use semio_s_artifact_stdio_deflate_mutations::standards::v_rfc1950::subsets::any::schema::diff;
                    pub use semio_s_artifact_stdio_deflate_mutations::standards::v_rfc1950::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_deflate_tests::standards::v_rfc1950::subsets::any::io::*;

                    pub use semio_s_artifact_stdio_deflate_tests::standards::v_rfc1950::subsets::any::io::import;
                    pub use semio_s_artifact_stdio_deflate_tests::standards::v_rfc1950::subsets::any::io::export;
                }
            }
        }
    }
}

// ---- Crate-root module paths: the crate's public surface ----
pub mod schema {
    pub use super::standards::v_rfc1950::subsets::any::schema::*;
}

pub use semio_s_artifact_stdio_deflate_examples::examples;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod deflate {
        pub use semio_s_artifact_stdio_deflate_mutations::editor::deflate::*;

        pub use semio_s_artifact_stdio_deflate_examples::editor::deflate::edit_rules;
        pub use semio_s_artifact_stdio_deflate_examples::editor::deflate::presence;
        pub use semio_s_artifact_stdio_deflate_examples::editor::deflate::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_deflate_mutations::editor::deflate::modes::edit::*;

                pub use semio_s_artifact_stdio_deflate_mutations::editor::deflate::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod deflate {
        pub use semio_s_artifact_stdio_deflate_tests::viewer::deflate::*;

        pub use semio_s_artifact_stdio_deflate_examples::viewer::deflate::presence;
        pub use semio_s_artifact_stdio_deflate_examples::viewer::deflate::config;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_deflate_mutations::viewer::deflate::modes::view::*;

                pub use semio_s_artifact_stdio_deflate_mutations::viewer::deflate::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v_rfc1950::subsets::any::io::{DeflateBuilderConstruction, DeflateParts, DeflateAnalyzerAnalysis, DeflateBuilderFacets, DeflateBuilder, DeflateAnalyzer, DeflateComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🧪️tests/🦀️.rs"]
mod hoisted_receiving_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_1;
