//! 💻️ Semio framework OS kernel — wasm-safe document model (store, spr, dsl, pack).
//!
//! Infinite/flow component files exist under 🔨️modules/ but are unwired pending dep-DAG cleanup.

#![allow(
    unused_extern_crates,
    ambiguous_glob_reexports,
    unused_imports,
    dead_code,
    async_fn_in_trait,
    clippy::approx_constant,
    clippy::await_holding_lock,
    clippy::double_must_use,
    clippy::drop_non_drop,
    clippy::empty_line_after_doc_comments,
    clippy::explicit_counter_loop,
    clippy::extra_unused_type_parameters,
    clippy::field_reassign_with_default,
    clippy::len_without_is_empty,
    clippy::let_underscore_future,
    clippy::map_unwrap_or,
    clippy::needless_pass_by_value,
    clippy::needless_range_loop,
    clippy::result_large_err,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_get_then_check,
    clippy::unnecessary_wraps,
    clippy::vec_init_then_push
)]


#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

extern crate self as dsl;
extern crate self as dsl_grammar;

pub extern crate self as semio_format;
extern crate self as semio_framework_os_kernel;
extern crate self as spr;
extern crate self as store;
extern crate self as vcs;
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this alias only feeds the browser
// wasm-bindgen async-fn codegen in this crate's session/transport bridges, so it is narrowed to
// exclude the WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
extern crate semio_framework_async as wasm_bindgen_futures;

// 🏷️ Former standalone crate names — proc-macros (`dsl_derive`) and in-tree `use store::` /
// `use protocol::` style paths resolve through these aliases to this crate root.

include!("📥️mounts/🌱️wp1-leaves/🦀️.rs");


pub use semio_framework_ui_locale::{AppLabels, Label, LabelText, Locale, LocalizedLabel, Terminology};

pub use semio_framework_os_vcs as os_vcs;

pub use semio_framework_os_identity as os_identity;


pub use semio_framework_plugin_component_receipt as os_component_receipt;

pub mod os_directory {
 pub use semio_framework_os_directory::*;
 pub use semio_framework_os_directory_client as client;
 pub use semio_framework_os_directory_client::identity;
}

// 🔁️ ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1: `🔁️workflow/🦀️.rs`
// is NOT mounted here (tried, reverted — see this file's own header comment "pending dep-DAG
// cleanup"): its `use semio_framework::{AppDefinition, MediaClass, MediaType, ConfigSpec,
// Terminology, Locale, …}` lines need the FULL framework crate's surface, which this wasm-safe
// kernel crate cannot depend on without an actual `semio-framework` → `semio-framework-os-kernel`
// →(back to)→ `semio-framework` cargo dependency CYCLE (`semio-framework` already depends on this
// crate — see its Cargo.toml). It is mounted in `🧰️framework/📦️packages/🦀️rust/🦀️.rs`
// (the `semio-framework` crate) instead, where all of those symbols already live — see that
// file's own `os_workflow` mount for the real fix, and the run crate's glue.rs for the matching
// `extern crate semio_framework as workflow;` alias change.

pub mod io {
    pub use semio_framework_os_io::*;
    pub use semio_framework_os_store_codec_registry::lifted_io::*;
}
pub use semio_framework_os_io::{compose_thunk, io_run_thunk, io_sniff_thunk};

pub use semio_framework_io_schema as io_schema;

pub use semio_framework_io_sqlite_snapshot as sqlite_snapshot;


#[path = "."]
pub mod os_store {
    #[path = "../../🔨️modules/🏪️store/🦀️.rs"]
    mod component;
    pub use component::*;

    // 🌉️ Also excluded from `wasm32-wasip2`, not just gated on the `sync` feature. This module's
    // `use tokio::sync::{broadcast, mpsc}` is unconditional, and `tokio` is deliberately absent from
    // wasip2 (see this crate's `Cargo.toml`: the dependency is
    // `cfg(not(all(target_arch = "wasm32", target_env = "p2")))`). Cargo feature unification can still
    // switch `sync` on inside a wasip2 plugin's graph — `🌉️mcp` and the wgpu renderer both request it
    // — and the module would then fail to resolve `tokio`. Excluding it here matches what the feature's
    // own docstring already states: WASI-P2 guest plugins never link the sync actor's transport.
    #[cfg(all(feature = "sync", not(all(target_arch = "wasm32", target_env = "p2"))))]
    pub mod sync {
        pub use semio_framework_os_store_sync::*;
        #[cfg(not(target_arch = "wasm32"))]
        pub use semio_framework_os_store_sync_native::*;
        #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
        pub use semio_framework_os_store_sync_wasm::*;
    }

    // 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; `👷️worker/🦀️.rs` is an
    // unconditional `wasm_bindgen` Web Worker `postMessage` bridge with no internal target split of
    // its own, so the mount itself is narrowed to exclude the WASI component target (which has no
    // "Web Worker" concept for an in-guest plugin).
    #[cfg(all(feature = "worker", target_arch = "wasm32", not(target_env = "p2")))]
    #[path = "../../🔨️modules/🏪️store/👷️worker/🦀️.rs"]
    pub mod worker;
}

pub use semio_framework_os_inference as os_inference;

pub use crate::os_dsl::*;
pub use crate::os_inference::*;
pub use crate::os_pack::*;
/// 🎯️ The replication protocol's central applier, exported by this kernel's protocol facade for every crate that reaches the protocol through it (`extern crate semio_framework_os_kernel as protocol`): the function itself, not an `os_spr` alias.
pub use protocol::{apply_diff_cold, apply_diff_cold_into, ColdMutation, ColdMutationDiff, ColdDiffAlgebra, retained_row_patch};
pub use crate::os_spr::*;
pub use crate::os_store::test_support;
pub use crate::os_store::*;


#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub use crate::os_extension as extension;
pub use crate::os_semio::*;
pub use crate::os_vcs::*;

// Former dsl_notation crate root surface
pub use crate::os_dsl::grammar::{family_fragments,product_macros,verify_protocol_bytes};


use semio_framework_dsl_record::*;
pub use semio_framework_value::{DslValue,FromValue,ToValue};
use semio_framework_value::{ValueError, ValueRefusalKind, ValueShape, ValueEdit, edit_through_value, NativeEncodeControl};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;


pub use semio_framework_ui_viewport::{Viewport2d, Viewport3dOrbit};

/// 🧬️ The crates `sparse_record_diff!`/`field_set_mutations!`/`config_record!` expand against, re-exported so the expansion
/// resolves the same in every invoking crate.
#[doc(hidden)]
pub use semio_framework_diagnostic as __diagnostic;
#[doc(hidden)]
pub use semio_framework_pack_json as __pack_json;
#[doc(hidden)]
pub use semio_framework_value as __value;
#[doc(hidden)]
pub use semio_framework_value_derive as __value_derive;
#[doc(hidden)]
pub use semio_framework_dsl_record_derive as __dsl_record_derive;

/// 🌿️ Publishes the canonical retained ownership derives beside their owned value traits.


include!("📥️mounts/🧪️wp8-tests/🦀️.rs");



pub use semio_framework_mutation_diff::{sparse_record_diff,sparse_whole_diff};

/// 🧒️ Canonical projection owner for exported composition macros.
pub use semio_framework_composition_admission;
