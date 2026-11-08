import re,sys
f,M=sys.argv[1],sys.argv[2]
t=open(f).read()
def sub(pat,rep,count=1,flags=0):
    global t
    t,n=re.subn(pat,rep,t,flags=flags)
    assert n==count,(pat,n)
sub(r"(fn advance_demands\(&self,[^\n]*?_body: usize)\)",r"\1, _maximum_work_units: u64)")
sub(r"(fn advance\(&mut self[^\n]*?-> Result<PagedOneItemEditStep<"+M+r">, )ValueError>",r"\1OriginalReceivingPrimary>")
sub(r"receiving\.commit_work\(self\.last_work\)\?;\n(\s*)result\n",r"receiving.commit_work(u128::from(self.last_work))?;\n\1result.map_err(Into::into)\n")
old=re.search(r"    fn close_demands\(&self, _mutations: &dyn ArtifactOwnedValueRetirementFactory<"+M+r">.*?\n    fn terminal_is_empty",t,re.S)
assert old
new=f'''    fn has_pending_receiving_recovery(&self) -> bool {{ false }}

    fn pending_receiving_demand(&self) -> Option<RetirementDemand> {{ None }}

    fn pending_receiving_recovery(&mut self) -> Option<semio_framework_value::native_decoding::construction::NativeRecoveryLoan<'_>> {{ None }}

    fn finish_receiving_recovery(&mut self, _: &mut semio_framework_value::native_decoding::NativeRetirementDecodeControl<'_, '_>) -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {{
        RetainedWorkOutcome {{ result: Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "{M} edit opens every native work action atomically and never retains a pending receiving frame").into()), work: 0 }}
    }}

    fn close_demands(&self, _mutations: &Arc<dyn ArtifactOwnedValueRetirementFactory<{M}>>, _body: usize) -> Result<RetirementDemand, ValueError> {{
        Ok(RetirementDemand::default())
    }}

    fn close_work_demand(&self, _mutations: &Arc<dyn ArtifactOwnedValueRetirementFactory<{M}>>, _body: usize) -> Result<u64, ValueError> {{
        Ok(0)
    }}

    fn close_step(&mut self, _mutations: &Arc<dyn ArtifactOwnedValueRetirementFactory<{M}>>, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {{
        self.closing = true;
        RetirementPollExit::settle(effect, Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())))
    }}

    fn terminal_is_empty'''
t=t[:old.start()]+new+t[old.end():]
sub(r"use std::sync::Arc;\n","use std::sync::Arc;\nuse semio_framework_value::{OriginalReceivingPrimary, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetirementPollExit};\n")
open(f,"w").write(t)
