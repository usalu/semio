//! 🧮️ Dense lattice front ends of the wfc engine: symmetry, tiles, motifs, 2D and 3D grids, their solvers, overlapping extraction and sparse 3D.

#![allow(clippy::result_large_err)]

#[path = "../../../⛏️extract/🦀️.rs"]
pub mod extract;
#[path = "../../../🔲️grid-2d/🦀️.rs"]
pub mod grid2d;
#[path = "../../../🧊️grid-3d/🦀️.rs"]
pub mod grid3d;
#[path = "../../../🎼️motif/🦀️.rs"]
pub mod motif;
#[path = "../../../🔳️solver-grid-2d/🦀️.rs"]
pub mod solver_grid2d;
#[path = "../../../🧱️solver-grid-3d/🦀️.rs"]
pub mod solver_grid3d;
#[path = "../../../🕳️sparse-3d/🦀️.rs"]
pub mod sparse3d;
#[path = "../../../🪞️symmetry/🦀️.rs"]
pub mod symmetry;
#[path = "../../../🀄️tiled/🦀️.rs"]
pub mod tiled;

use semio_s_plugin_wfc_engine_foundation::{bitset, error, ids, model, topology};
use semio_s_plugin_wfc_engine_kernel::{constraint, outcome};
use semio_s_plugin_wfc_engine_solve::{chunk, search};
#[cfg(test)]
use semio_s_plugin_wfc_engine_kernel::constraints_card;
#[cfg(test)]
use semio_s_plugin_wfc_engine_solve::solver_graph;
