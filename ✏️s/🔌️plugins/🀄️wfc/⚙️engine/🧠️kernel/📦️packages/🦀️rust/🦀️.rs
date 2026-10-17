//! 🧠️ Propagation kernel of the wfc engine: diagnostics, outcomes, trail, arc-consistency propagators, nogood learning, beam search and the constraint families.

#![allow(clippy::result_large_err)]

#[path = "../../../🔦️beam/🦀️.rs"]
pub mod beam;
#[path = "../../../⛓️constraint/🦀️.rs"]
pub mod constraint;
#[path = "../../../🔢️constraints-card/🦀️.rs"]
pub mod constraints_card;
#[path = "../../../🔗️constraints-conn/🦀️.rs"]
pub mod constraints_conn;
#[path = "../../../🩺️diag/🦀️.rs"]
pub mod diag;
#[path = "../../../🌊️flow/🦀️.rs"]
pub mod flow;
#[path = "../../../🚫️nogood/🦀️.rs"]
pub mod nogood;
#[path = "../../../🏁️outcome/🦀️.rs"]
pub mod outcome;
#[path = "../../../🔁️prop-ac3/🦀️.rs"]
pub mod prop_ac3;
#[path = "../../../🔄️prop-ac4/🦀️.rs"]
pub mod prop_ac4;
#[path = "../../../🐾️trail/🦀️.rs"]
pub mod trail;

use semio_s_plugin_wfc_engine_foundation::{bitset, domain, error, heuristics, ids, model, propagate, topology, weights};
#[cfg(test)]
use semio_s_plugin_wfc_engine_oracle::{model_vectors, oracle};
