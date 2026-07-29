//! 🌊 Flow app — document entity facade (constitutional: general).
//!
//! Unlike most constitutional apps, `FlowFixture`'s fields, `Widget`/`SynapseSpec` variants, and the
//! `FLOW_DOCUMENT_SCHEMA` constant are NOT owned by this app — they live in the shared flow kernel
//! crate ([`flow_core`], `s/kernel/flow/core/rs`) because multiple apps compile against the same flow
//! domain model. This crate re-exports the app-facing surface so sibling constitutional crates
//! (`engine`, `dsl`, `op`, `pack`, `protocol`, `ui`) depend on a stable app-owned name instead of every
//! crate reaching into the kernel path directly.

//#region 🔖Types
pub use flow_core::{FlowFixture, FLOW_DOCUMENT_SCHEMA};
//#endregion 🔖Types
