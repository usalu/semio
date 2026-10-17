//! ⚖️ Flow host retirement, work continuation and source read laws: the suites that drive a retained host together with its evaluation session.

pub use semio_framework_os_flow_host_core::*;
pub use semio_framework_os_flow_host_read::*;
pub use semio_framework_os_flow_host_session::*;
pub use semio_framework_os_flow_host_source::{seeds::FlowEvaluationSeeds, work::FlowEvaluationWork, FlowEvaluationSource};

#[cfg(test)] use semio_framework_os_infinite::board::ports::directed_dag as dag;
#[cfg(test)] use neural_engine as neural;
#[cfg(test)] use std::collections::{BTreeMap, HashMap};
#[cfg(test)] use std::sync::Arc;
#[cfg(test)] use protocol::causal::transition::HistoryFoldIndex;
#[cfg(test)] use semio_framework::MeshData;
#[cfg(test)] use semio_framework_value::retirement::RetireOwned;
#[cfg(test)] use semio_framework_value::RetirementDemand;
#[cfg(test)] use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
#[cfg(test)] use semio_framework_value::retirement::controlled::ControlledRetirement;
#[cfg(test)] use dag::DagHost;
#[cfg(test)] use semio_framework_os_infinite_directed_presentation::DagHostSnapshot;
#[cfg(test)] use semio_framework_os_infinite_directed_presentation::{DagNodeKind, DagNodeSpec, IoPortSpec};
#[cfg(test)] use semio_framework_artifact_flow_flow::{FlowStore, FLOW_DOCUMENT_SCHEMA};
#[cfg(test)] use semio_framework_graph_manifest::PropertyBag;
#[cfg(test)] use neural::{Atom, Dictionary, EvalChannels, Neuron, OperatorInfo, Tree, TreeSnapshot, Value as NeuralValue};
#[cfg(test)] use semio_framework_artifact_flow_flow::artifact::*;
#[cfg(test)] use semio_framework_os_flow_operators::bridge::*;
#[cfg(test)] use semio_framework_os_flow_operators::catalogue::*;
#[cfg(test)] use semio_framework_os_flow_drawing::*;
#[cfg(test)] use semio_framework_os_kernel::os_store::document_materialize::create_document_envelope;
#[cfg(test)] use semio_framework_os_flow_operators::registry::*;

#[cfg(test)]
#[path = "../🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs"]
mod session_retirement_tests;
#[cfg(test)]
#[path = "../📥️evaluation-source/🧵️work/🧪️tests/🦀️.rs"]
mod work_tests;
#[cfg(test)]
#[path = "../📥️evaluation-source/🔎️read/🧪️tests/🦀️.rs"]
mod read_tests;

/// 🧾️ Reads one file below the host node at test time instead of compiling it into the suite; the text lives for the whole test process.
#[cfg(test)]
fn read_node(path: &str) -> &'static str {
    let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(path);
    Box::leak(std::fs::read_to_string(&file).unwrap_or_else(|error| panic!("{}: {error}", file.display())).into_boxed_str())
}

#[cfg(test)]
#[path = "../📤️publication/📡️invocation/🧪️tests/🦀️.rs"]
mod invocation_tests;
#[cfg(test)]
#[path = "../📥️evaluation/🧪️tests/🦀️.rs"]
mod evaluation_tests;
#[cfg(test)]
#[path = "../📥️preview/🧪️tests/🦀️.rs"]
mod preview_tests;
