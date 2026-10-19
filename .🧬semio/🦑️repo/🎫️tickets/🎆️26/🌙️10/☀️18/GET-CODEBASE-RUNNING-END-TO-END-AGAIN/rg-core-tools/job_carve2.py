import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import rgl, deps
R = rgl.ROOT
J = R + "/🧰️framework/🔨️modules/🧵️job"
N = J + "/📬️outcome/📦️builder/📦️packages/🦀️rust"
os.makedirs(N, exist_ok=True)
root = J + "/🦀️.rs"
s = rgl.read(root)

def cut(s, start_pat, end_pat=None):
    m = re.search(start_pat, s, re.S)
    assert m, start_pat
    return s[:m.start()] + s[m.end():]

s = cut(s, r'#\[path = "📄️payload-pages-and-ledger/🦀️\.rs"\]\nmod payload_pages_and_ledger;\nuse payload_pages_and_ledger::\{[^}]*\};\npub use payload_pages_and_ledger::\{[^}]*\};\n\n')
s = cut(s, r'#\[path = "✍️payload-writer-and-grant/🦀️\.rs"\]\nmod payload_writer_and_grant;\npub use payload_writer_and_grant::\{[^}]*\};\n\n')
s = cut(s, r'#\[path="📬️outcome/📖️source/🦀️\.rs"\]\nmod original_payload_source;\n')
s = cut(s, r'#\[path="📬️outcome/📦️builder/🦀️\.rs"\]\nmod payload_builder;\npub use payload_builder::\{[^}]*\};\n')
payload_use = '''use semio_framework_job_payload::{JobPayloadStep, JobPayloadAuthority, JobPayloadOperationLedger, JobPayloadPage, JobPayloadPageSource, JobPayloadStream, JobPayloadAdmissionFault, JobPayloadRejectedPage, RetainedJobPayload, RetainedJobPayloadReader, RetainedJobPayloadWriter, RetainedPayloadBuilder, OriginalPayloadSourceIdentity, OriginalPayloadBuilderIdentity, JobPayloadPageGrant, JOB_PAYLOAD_PROCESS_OWNED_BYTES, JOB_PAYLOAD_PAGE_BYTES, JOB_PAYLOAD_OPERATION_PAGES, JOB_PAYLOAD_OPERATION_BYTES, JOB_PAYLOAD_PROCESS_BYTES};

'''
anchor = '#[path = "🕰️clock-identity-and-budget/🦀️.rs"]'
assert s.count(anchor) == 1
s = s.replace(anchor, payload_use + anchor)
impl_anchor = "impl<'a> StepContext<'a> {"
assert s.count(impl_anchor) == 1
step_impl = '''impl JobPayloadStep for StepContext<'_> {
    fn payload_ledger(&self) -> &Arc<JobPayloadOperationLedger> { &self.payload_ledger }
    fn payload_page_granted(&self) -> bool { self.payload_page_granted }
    fn grant_payload_page(&mut self) { self.payload_page_granted = true; }
    fn is_cancelled(&self) -> bool { StepContext::is_cancelled(self) }
    fn should_yield(&self) -> bool { StepContext::should_yield(self) }
    fn retained_grant(&self) -> RetainedCloneGrant { StepContext::retained_grant(self) }
    fn consume_retained(&mut self, progress: RetainedCloneProgress) -> Result<(), ValueError> { StepContext::consume_retained(self, progress) }
}

'''
s = s.replace(impl_anchor, step_impl + impl_anchor)
sup = '''#[path="📬️outcome/📦️builder/📮️return/🦀️.rs"]
mod builder_supplied_page_return;
pub use builder_supplied_page_return::RetainedPayloadBuilderSupply;
'''
s = s.replace('#[path="📬️outcome/📮️return/🦀️.rs"]\nmod original_payload_return;', '#[path="📬️outcome/📮️return/🦀️.rs"]\nmod original_payload_return;\n' + sup, 1)
assert "builder_supplied_page_return" in s
rgl.write(root, s)

b = J + "/📬️outcome/📦️builder/🦀️.rs"
t = rgl.read(b)
t = t.replace('#[path="📮️return/🦀️.rs"]\nmod supplied_page_return;', '')
t = t.rstrip("\n") + '''
impl RetainedPayloadBuilder{
 /// 📦️ Lends the sealed in-place header once its last page is ready to supply.
 pub fn sealed_original_mut(&mut self)->Option<&mut RetainedJobPayload>{if !self.sealed||!self.ready||self.staged.is_some(){return None}Some(unsafe{self.original.assume_init_mut()})}
}
'''
rgl.write(b, t)
rf = J + "/📬️outcome/📦️builder/📮️return/🦀️.rs"
rgl.write(rf, '''//! 📮️ A sealed builder lends one original page per turn into a pre-reserved return owner in place.
use super::*;

/// 📦️ Supplies the last page of a sealed builder into a page return reservation.
pub trait RetainedPayloadBuilderSupply{
 /// 📦️ A valid sealed prefix remains in the same header while the last page keeps its original ordinal.
 fn supply_last_page<const N:usize>(&mut self,reservation:&mut OriginalJobPageReservation<N>,cx:&mut StepContext<'_>)->Result<Option<OriginalSuppliedJobPage>,OriginalTurnFailure>;
}
impl RetainedPayloadBuilderSupply for RetainedPayloadBuilder{
 fn supply_last_page<const N:usize>(&mut self,reservation:&mut OriginalJobPageReservation<N>,cx:&mut StepContext<'_>)->Result<Option<OriginalSuppliedJobPage>,OriginalTurnFailure>{
  let Some(original)=self.sealed_original_mut() else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original supplied page requires its sealed builder").into())};
  reservation.publish_last(original,cx)
 }
}
''')

rgl.write(N + "/Cargo.toml", '''[package]
workspace = "../../../../../../.."
name = "semio-framework-job-payload"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Job payload custody: the paid payload pages and operation ledger, the retained payload writer and grant, the in-place payload builder and its staged pages, funded through the job step."

[package.metadata.semio]
role = "framework"
id = "job-payload"

[package.metadata.semio.rust-source-boundary]
maxSourceBytes = 262144
maxSourceFiles = 32
maxActiveLines = 4000

[lints]
workspace = true

[lib]
name = "semio_framework_job_payload"
path = "🦀️.rs"

[dependencies]
semio-framework-trace = { workspace = true }
semio-framework-value = { workspace = true }
''')
cargo = rgl.read(N + "/Cargo.toml").replace('workspace = "../../../../../../.."', 'workspace = "' + os.path.relpath(R + "/🧰️framework", N) + '"')
rgl.write(N + "/Cargo.toml", cargo)
rgl.write(N + "/📋️project.json", rgl.project_json("@semio-tech/framework-job-payload-rs"))
rgl.write(N + "/🦀️.rs", '''//! 📬️ Job payload custody glue — wiring only. Domain lives at the owner `🦀️.rs` files.

use std::mem::{ManuallyDrop, MaybeUninit, size_of_val};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use semio_framework_trace::{Generation, OperationId};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

/// 🧭️ The funded step a payload custody call runs inside: its cancellation, yield, retained stock and ledger.
pub trait JobPayloadStep {
    fn payload_ledger(&self) -> &Arc<JobPayloadOperationLedger>;
    fn payload_page_granted(&self) -> bool;
    fn grant_payload_page(&mut self);
    fn is_cancelled(&self) -> bool;
    fn should_yield(&self) -> bool;
    fn retained_grant(&self) -> RetainedCloneGrant;
    fn consume_retained(&mut self, progress: RetainedCloneProgress) -> Result<(), ValueError>;
}

#[path = "../../../../📄️payload-pages-and-ledger/🦀️.rs"]
pub mod payload_pages_and_ledger;
pub use payload_pages_and_ledger::{JOB_PAYLOAD_OPERATION_BYTES, JOB_PAYLOAD_OPERATION_PAGES, JOB_PAYLOAD_PAGE_BYTES, JOB_PAYLOAD_PROCESS_BYTES, JOB_PAYLOAD_PROCESS_OWNED_BYTES, JobPayloadAdmissionFault, JobPayloadOperationLedger, JobPayloadPage, JobPayloadPageSource, JobPayloadRejectedPage, JobPayloadStream, RetainedJobPayload, RetainedJobPayloadReader};

#[path = "../../../../✍️payload-writer-and-grant/🦀️.rs"]
pub mod payload_writer_and_grant;
pub use payload_writer_and_grant::{JobPayloadPageGrant, JobPayloadStagedSource, RetainedJobPayloadWriter};

#[path = "../../../📖️source/🦀️.rs"]
mod original_payload_source;

#[path = "../../🦀️.rs"]
pub mod payload_builder;
pub use payload_builder::{JobPayloadAuthority, OriginalPayloadBuilderIdentity, OriginalPayloadSourceIdentity, RetainedPayloadBuilder};
''')
rgl.register(J + "/📦️packages/🦀️rust", "semio-framework-job", N, "semio-framework-job-payload", "@semio-tech/framework-job-rs", "@semio-tech/framework-job-payload-rs", "🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/📦️packages/🦀️rust", dep_ws=[FW] if (FW := R + "/🧰️framework/Cargo.toml") else None)
print(deps.add_dep(J + "/📦️packages/🦀️rust/Cargo.toml", "semio-framework-trace", "semio-framework-job-payload", N))
