//! ⚡ DAG app — operation type facade (constitutional: op).
//!
//! `DagOperation`, its `protocol::Operation`/`OperationDiff` impls, and the `apply`/`diff`/`backwards`
//! logic all live in the shared DAG kernel crate (`infinite_board_port_directed_dag`,
//! `framework/kernel/infinite/board/port/directed/dag/rs`, `🔖DocumentVcs` region) alongside the
//! `DagDocument` projection they mutate — see `s/plugin/dag/app/rs/lib.rs` for why. Re-exported here so
//! sibling constitutional crates depend on the app-owned name instead of reaching into the kernel path
//! directly.

//#region 🔖Types
pub use infinite_board_port_directed_dag::DagOperation;
//#endregion 🔖Types
