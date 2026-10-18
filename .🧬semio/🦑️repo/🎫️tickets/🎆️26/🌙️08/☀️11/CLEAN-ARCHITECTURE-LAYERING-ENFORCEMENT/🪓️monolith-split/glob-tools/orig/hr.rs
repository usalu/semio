//! 🧭️ The routers of the plugin host: IO, inference, plugin graph, mutation, instance directory, transaction coordinator, app and opening resolver.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use std::sync::{Arc, Mutex};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_plugin_guest_contract::{IoEntryRouteConflict, IoRouteConflict, PluginHostError};
use semio_framework_plugin_host_post_turn_relay::PluginInstanceHandle;
use semio_framework::PluginManifest;

#[path = "🚪️io-router/🦀️.rs"]
mod io_router;
pub use io_router::*;

#[path = "💡️inference-router/🦀️.rs"]
mod inference_router;
pub use inference_router::*;

#[path = "🕸️plugin-graph-and-instance-directory/🦀️.rs"]
mod plugin_graph_and_instance_directory;
pub use plugin_graph_and_instance_directory::*;

#[path = "🎯️mutation-router/🦀️.rs"]
mod mutation_router;
pub use mutation_router::*;

#[path = "🧾️transaction-coordinator/🦀️.rs"]
mod transaction_coordinator;
pub use transaction_coordinator::*;

#[path = "🧩️app-router-and-opening-resolver/🦀️.rs"]
mod app_router_and_opening_resolver;
pub use app_router_and_opening_resolver::*;

//#region 💡️InferenceRouter

#[cfg(test)]
#[path = "../🧪️tests/🔬️artifact-inference-router/🦀️.rs"]
mod artifact_inference_router_tests;
//#endregion 💡️InferenceRouter

//#region 🔖️PluginGraph

#[cfg(test)]
#[path = "../🧪️tests/🔬️plugin-graph/🦀️.rs"]
mod plugin_graph_tests;
//#endregion 🔖️PluginGraph

//#region 🎯️MutationRouter

#[cfg(test)]
#[path = "../🧪️tests/🔬️artifact-mutation-router/🦀️.rs"]
mod artifact_mutation_router_tests;
//#endregion 🎯️MutationRouter

//#region 🔖️InstanceDirectory

#[cfg(test)]
#[path = "../🧪️tests/🔬️instance-directory/🦀️.rs"]
mod instance_directory_tests;
//#endregion 🔖️InstanceDirectory

//#region 🎯️TransactionCoordinator

#[cfg(test)]
#[path = "../🧪️tests/🔬️owned-error/🦀️.rs"]
mod owned_error_tests;

#[cfg(test)]
#[path = "../🧪️tests/🔬️host-transaction-coordinator/🦀️.rs"]
mod host_transaction_coordinator_tests;
//#endregion 🎯️TransactionCoordinator

//#region 🔖️AppRouter

#[cfg(test)]
#[path = "../🧪️tests/🔬️app-router/🦀️.rs"]
mod app_router_tests;
//#endregion 🔖️AppRouter

//#region 🔖️OpeningResolver

#[cfg(test)]
#[path = "../🧪️tests/🔬️opening-resolver/🦀️.rs"]
mod opening_resolver_tests;
//#endregion 🔖️OpeningResolver

//#region 🔧️SharedWasmtimeEngine
/// 🧪️ Only `host_transaction_coordinator_tests`' `FakeCluster` fixture calls this now — the old
/// `HostState`'s WIT `host.*` import handlers (which encoded EVERY fault this way) are gone with
/// `WasmPluginRuntime`. Kept (not deleted) because that fixture still needs it; `cfg_attr` silences
/// the resulting "never used" warning on a plain (non-test) `--lib` build without hiding a REAL
/// dead-code case under a blanket `#[allow]`.
#[cfg_attr(not(test), allow(dead_code))]
// 🚫️async: E1 — pure in-memory encoder, no suspension point; reverted per R9 (its only
// consumer FakeCluster::exchange must itself be sync to satisfy run_transaction/undo_group
// production FnMut(...) -> Result<...> closure signature).
fn host_fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {
    let code = code.into();
    semio_framework_diagnostic::encode_fault_bytes(&semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Os, semio_framework_diagnostic::FaultCode::new(code), message))
}
//#endregion 🔧️SharedWasmtimeEngine
