//! 🖱️ Architect app UI surface (constitutional: ui).

//#region 🔖️UiSurface
pub use architect::{Program, ARCHITECT_PROGRAM_SCHEMA};
pub use architect_engine::{adjacency_matrix, status_summary, undirected_edges};
pub use architect_op::ProgramOperation;
pub use architect_protocol::{decode as decode_operation, encode as encode_operation};
//#endregion 🔖️UiSurface
