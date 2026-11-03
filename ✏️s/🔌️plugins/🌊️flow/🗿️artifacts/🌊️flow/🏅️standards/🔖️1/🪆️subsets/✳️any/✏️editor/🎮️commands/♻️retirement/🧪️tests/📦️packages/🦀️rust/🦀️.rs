//! 🧪️ original-command-retirement-tests of `semio-s-artifact-flow-flow` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::mutations::FlowMutation;
pub use semio_s_artifact_flow_flow_schema::{FLOW_DOCUMENT_SCHEMA, FlowWorkingScene, cache_flow_content};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

#[cfg(test)]
fn fixture_clock()->Option<u64>{Some(0)}

#[cfg(test)]
pub(crate) fn self_funded_close<R>(grant:semio_framework_value::RetainedCloneGrant,advance:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<R,semio_framework_job::OriginalTurnFailure>{
 let stock=semio_framework_value::RetainedCloneGrant{maximum_items:grant.maximum_items+16,maximum_copy_bytes:grant.maximum_copy_bytes+4096,maximum_capacity_bytes:grant.maximum_capacity_bytes+65536,maximum_release_bytes:grant.maximum_release_bytes+65536,maximum_depth:grant.maximum_depth+8};
 let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(1),semio_framework_job::Generation(1),stock,fixture_clock,u64::MAX,20000);
 let result=caller.turn(grant,advance);caller.retire(stock);result
}
