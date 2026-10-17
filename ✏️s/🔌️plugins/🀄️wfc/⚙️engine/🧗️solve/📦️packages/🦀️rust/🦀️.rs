//! 🧗️ Solvers of the wfc engine: the backtracking search, repair, parallel portfolios, chunked and hierarchical solving, evolution and checkpoint serialization.

#![allow(clippy::result_large_err)]

#[path = "../../../🍰️chunk/🦀️.rs"]
pub mod chunk;
#[path = "../../../🧬️evolve/🦀️.rs"]
pub mod evolve;
#[path = "../../../🪜️hierarchy/🦀️.rs"]
pub mod hierarchy;
#[path = "../../../🧵️parallel/🦀️.rs"]
pub mod parallel;
#[path = "../../../🔧️repair/🦀️.rs"]
pub mod repair;
#[path = "../../../🔍️search/🦀️.rs"]
pub mod search;
#[path = "../../../💾️serial/🦀️.rs"]
pub mod serial;
#[path = "../../../🕸️solver-graph/🦀️.rs"]
pub mod solver_graph;

use semio_s_plugin_wfc_engine_foundation::{bitset, domain, error, heuristics, ids, model, propagate, sample, soft, topology};
use semio_s_plugin_wfc_engine_kernel::{beam, constraint, diag, nogood, outcome, prop_ac3, trail};
use semio_s_plugin_wfc_engine_job::job;
#[cfg(test)]
use semio_s_plugin_wfc_engine_oracle::{model_vectors, oracle};
#[cfg(test)]
use semio_s_plugin_wfc_engine_kernel::{constraints_card, flow};
