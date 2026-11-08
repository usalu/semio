//! 🧰️ Fixture access to private job-context turn state, built only on its narrow `StepFields` view.
use semio_framework_job_context::{StepContext,OperationId,Generation,RetainedCloneGrant,JobPayloadOperationLedger,RetirementDemand};
use std::sync::Arc;
/// 📄️ Lets the next payload page be granted again in the same turn.
pub fn renew_payload_page_grant(cx:&mut StepContext<'_>){*cx.testkit_fields().payload_page_granted=false}
/// 🆔️ Borrows the turn's operation identity for identity-mismatch laws.
pub fn operation_mut<'a>(cx:&'a mut StepContext<'_>)->&'a mut OperationId{cx.testkit_fields().operation}
/// 🎟️ Borrows the turn's retained grant for ceiling laws.
pub fn retained_grant_mut<'a>(cx:&'a mut StepContext<'_>)->&'a mut RetainedCloneGrant{cx.testkit_fields().retained}
/// 📊️ Borrows the observer's retained progress for receipt laws.
pub fn retained_progress_mut<'a>(cx:&'a mut StepContext<'_>)->&'a mut semio_framework_job_context::RetainedCloneProgress{cx.testkit_fields().retained_progress}
/// ⛔️ Runs one step under a replaced operation and capacity, restoring both even when it unwinds.
pub fn with_original_work_refusals<T>(cx:&mut StepContext<'_>,operation:Option<OperationId>,capacity:Option<usize>,run:impl FnOnce(&mut StepContext<'_>)->T)->T{
 struct Restore<'a>{context:*mut StepContext<'a>,operation:OperationId,retained:RetainedCloneGrant}
 impl Drop for Restore<'_>{fn drop(&mut self){let fields=unsafe{(*self.context).testkit_fields()};*fields.operation=self.operation;*fields.retained=self.retained}}
 let(saved_operation,saved_retained)={let fields=cx.testkit_fields();(*fields.operation,*fields.retained)};
 let restore=Restore{context:std::ptr::addr_of_mut!(*cx),operation:saved_operation,retained:saved_retained};
 {let fields=cx.testkit_fields();if let Some(operation)=operation{*fields.operation=operation}if let Some(capacity)=capacity{fields.retained.maximum_capacity_bytes=capacity}}
 let value=run(cx);drop(restore);value
}
/// 🧾️ Runs one step and restores the retained receipt policy it found.
pub fn with_retained_receipt_policy<T>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut StepContext<'_>)->T)->T{let original=*cx.testkit_fields().retained_policy;let result=run(cx);*cx.testkit_fields().retained_policy=original;result}
/// 🚫️ Makes the current turn refuse every further retained receipt item.
pub fn refuse_current_retained_receipt(cx:&mut StepContext<'_>){let grant=cx.retained_grant();let progress=cx.retained_progress();*cx.testkit_fields().retained_policy=Some((RetainedCloneGrant{maximum_items:0,..grant},progress));}
/// 🧱️ Creates one fresh payload operation ledger.
pub fn payload_ledger(operation:OperationId,generation:Generation)->Arc<JobPayloadOperationLedger>{Arc::new(JobPayloadOperationLedger::new(operation,generation))}
/// 📏️ Quotes the retirement demand of an optional payload ledger.
pub fn ledger_demands(ledger:&Option<Arc<JobPayloadOperationLedger>>)->RetirementDemand{semio_framework_job_payload::ledger_demands(ledger)}
/// ♻️ Closes an optional payload ledger under a grant.
pub fn close_ledger(ledger:&mut Option<Arc<JobPayloadOperationLedger>>,grant:RetainedCloneGrant)->semio_framework_value_retained_clone::RetainedWorkOutcome{semio_framework_job_payload::close_ledger(ledger,grant)}
