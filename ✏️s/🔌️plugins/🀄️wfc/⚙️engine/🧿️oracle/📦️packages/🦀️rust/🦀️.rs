//! 🧿️ Brute-force reference solver, the shared model vectors and the job step context every optimized solver of the wfc engine is cross-checked and driven with in tests.

#![allow(clippy::result_large_err)]

#[path = "../../../🧪️tests/🧮️model-vectors/🦀️.rs"]
pub mod model_vectors;
#[path = "../../../🔮️oracles/🦀️.rs"]
pub mod oracle;
#[path = "../../🧰️step-context/🦀️.rs"]
pub mod step_context;

use semio_s_plugin_wfc_engine_foundation::{bitset, ids, model, weights};
