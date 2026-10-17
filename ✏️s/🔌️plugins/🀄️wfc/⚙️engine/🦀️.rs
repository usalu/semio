//! 🀄️ Wave function collapse engine shared by every `wfc` plugin artifact. Domain agnostic: it
//! speaks only `PatternId`/`NodeId`/`RelationId`, never an artifact's document types. Every module
//! is production code — the only test-gated ones are the differential oracle (`oracle`) and the
//! shared model vectors (`model_vectors`), which exist purely to cross-check the engine.

#![allow(clippy::result_large_err)]

pub use semio_s_plugin_wfc_engine_foundation::{bitset, domain, error, heuristics, ids, model, propagate, sample, soft, topology, weights};
pub use semio_s_plugin_wfc_engine_kernel::{beam, constraint, constraints_card, constraints_conn, diag, flow, nogood, outcome, prop_ac3, prop_ac4, trail};
pub use semio_s_plugin_wfc_engine_job::job;
pub use semio_s_plugin_wfc_engine_solve::{chunk, evolve, hierarchy, parallel, repair, search, serial, solver_graph};
pub use semio_s_plugin_wfc_engine_lattice::{extract, grid2d, grid3d, motif, solver_grid2d, solver_grid3d, sparse3d, symmetry, tiled};

#[cfg(test)]
#[path = "🧪️tests/🔬️grid-job-drive/🦀️.rs"]
mod grid_job_drive_tests;
