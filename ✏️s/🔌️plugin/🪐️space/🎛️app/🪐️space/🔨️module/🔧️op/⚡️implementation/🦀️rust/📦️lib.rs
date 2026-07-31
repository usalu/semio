//! ⚡️ S Studio app — operation enum + laws (constitutional: op).
//!
//! 🕳️ Deviation from the constitutional split recipe's usual "op" content (a locally-defined
//! `#[derive(dsl::DslOps)]` enum + `impl Operation`/`OperationDiff` + a private `apply_X_operation` fn
//! matching on that enum): the Studio app has no operation type of its own. Its
//! `DocumentApp::Operation` is `semio_framework_os::OsOperation`, whose enum, `Operation`/
//! `OperationDiff` impls, and `apply_os_operation` all live in `framework/product/os/core/rs`, entirely
//! outside this plugin — this app only ever *constructs* `OsOperation` values from arguments (pure
//! compute, see `semio-s-app-space-space-engine`), it never matches on the enum to interpret/apply it.
//! There is therefore nothing left for this layer to hold; it is kept as an empty crate solely so the
//! plugin's constitutional 7-crate shape stays uniform across every app in the workspace.
