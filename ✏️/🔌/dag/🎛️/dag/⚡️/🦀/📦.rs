//! 🔀 DAG app — document entity facade (constitutional: general).
//!
//! Unlike most constitutional apps, `DagDocument`'s fields and the `DAG_DOCUMENT_SCHEMA` constant are
//! NOT owned by this app — they live in the shared DAG kernel crate ([`infinite_board_port_directed_dag`],
//! `framework/kernel/infinite/board/port/directed/dag/rs`) because the DAG board is shared
//! infrastructure used by more than this play app. This crate re-exports the app-facing surface so
//! sibling constitutional crates (`engine`, `dsl`, `op`, `pack`, `protocol`, `ui`) depend on a stable
//! app-owned name instead of every crate reaching into the kernel path directly.

//#region 🔖Types
pub use infinite_board_port_directed_dag::{DagDocument, DAG_DOCUMENT_SCHEMA};
//#endregion 🔖Types
