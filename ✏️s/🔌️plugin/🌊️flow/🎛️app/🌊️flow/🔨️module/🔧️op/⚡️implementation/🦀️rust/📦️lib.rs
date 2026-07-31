//! ⚡️ Flow app — operation type facade (constitutional: op).
//!
//! `FlowOperation`, its `protocol::Operation`/`OperationDiff` impls, and the private
//! `apply_flow_operation` fn all live in the shared flow kernel crate (`flow_core`,
//! `s/kernel/flow/core/rs`, `🔖️Operations` region) alongside the `FlowFixture` projection they mutate —
//! see `s/plugin/flow/app/rs/lib.rs` for why. Re-exported here so sibling constitutional crates
//! depend on the app-owned name instead of reaching into the kernel path directly.

//#region 🔖️Types
pub use flow_core::FlowOperation;
//#endregion 🔖️Types
