//! 🏠️ Inference admission, publication and retained job lifecycle.
use semio_framework_job::InteractiveJob;
use crate::schema::snapshot::Wfc3dSnapshot;
use semio_s_plugin_wfc_engine as engine;
use std::collections::BTreeMap;
use crate::standards::v1::subsets::any::schema::inferences::*;

#[path = "📜️contract-and-request/🦀️.rs"]
mod contract_and_request;
pub use contract_and_request::*;

#[path = "🧮️job-core/🦀️.rs"]
mod job_core;
pub(crate) use job_core::*;

#[path = "🔁️job-lifecycle/🦀️.rs"]
mod job_lifecycle;

#[path = "🏭️factory-and-solve/🦀️.rs"]
mod factory_and_solve;
pub use factory_and_solve::*;


