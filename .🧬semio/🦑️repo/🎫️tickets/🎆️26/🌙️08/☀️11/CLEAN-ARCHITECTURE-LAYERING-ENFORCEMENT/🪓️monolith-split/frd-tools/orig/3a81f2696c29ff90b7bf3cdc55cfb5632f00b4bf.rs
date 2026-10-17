//! ⚖️ Flow VCS laws: the retained VCS operation, canonical history and lifecycle suite over the whole VCS producer.

pub use semio_framework_os_flow_vcs::*;

#[cfg(test)] use neural_engine as neural;
#[cfg(test)] use neural::{Neuron, Synapse};
#[cfg(test)] use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
#[cfg(test)]
use semio_framework_artifact_flow_flow::*;
#[cfg(test)] use semio_framework_os_flow_vcs::history_identity;
#[cfg(test)] use semio_framework_os_flow_vcs::original_retirement::admit_turn;

/// 🧾️ Reads one file below the VCS node at test time instead of compiling it into the suite; the text lives for the whole test process.
#[cfg(test)]
fn read_node(path: &str) -> &'static str {
    let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(path);
    Box::leak(std::fs::read_to_string(&file).unwrap_or_else(|error| panic!("{}: {error}", file.display())).into_boxed_str())
}

// #region 🔖️Laws
#[cfg(test)]
#[path = "../🧪️tests/🔬️flow-vcs/🦀️.rs"]
mod flow_vcs_tests;
// #endregion 🔖️Laws
