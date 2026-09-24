import sys
p=sys.argv[1]
s=open(p,encoding="utf-8").read()
start=s.index("    /// ⚙️ `Pump`: one bounded worker transition.")
end=s.index("    /// 🧾️ `OutcomeClose`: one page of the checked-out outcome's")
old=s[start:end]
body_start=old.index("        match crate::app::inference_cancelled")
body=old[body_start:old.rindex("    }\n")]
lines=body.split("\n")
out=[]
for l in lines:
    if "return self.fail(" in l:
        l=l.replace("return self.fail(","return PumpTransition::Settled(self.fail(")
        r=l.rstrip()
        if r.endswith(");"): l=r[:-2]+"));"
        elif r.endswith("),"): l=r[:-2]+")),"
        else: raise SystemExit("unexpected: "+l)
    out.append(l)
body="\n".join(out)
def rep(a,b):
    global body
    assert body.count(a)==1, a
    body=body.replace(a,b)
rep("""        if !matches!(poll, semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) {
            return JobStep::Running(Some(progress));
        }""","""        if !matches!(poll, semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal) {
            return PumpTransition::Settled(JobStep::Running(Some(progress)));
        }""")
rep("""                if let Some(item) = self.bridge.take_preview() {
                    progress = encode_bridge_item(&item);
                }
                None
            }""","""                if let Some(item) = self.bridge.take_preview() {
                    progress = encode_bridge_item(&item);
                    preview = true;
                }
                None
            }""")
rep("""        let scheduled = self.bridge.scheduled();
        let mut progress = encode_bridge_item(&scheduled);""","""        let scheduled = self.bridge.scheduled();
        let mut progress = encode_bridge_item(&scheduled);
        let mut preview = false;""")
tail_old="""        self.result = result;
        self.outcome = Some(outcome);
        self.phase = InteractivePhase::OutcomeClose;
        JobStep::Running(Some(progress))"""
assert body.rstrip().endswith(tail_old), body[-300:]
body=body.rstrip()[:-len(tail_old)]+"""        let mut outcome = outcome;
        if result.is_none() && matches!(outcome, StepOutcome::Yield | StepOutcome::PreviewReady(_)) && retire_absorbed_outcome(&mut outcome) {
            return match self.session.as_mut().map(InferenceSession::resume) {
                Some(Ok(())) => PumpTransition::Absorbed { progress, preview },
                _ => PumpTransition::Settled(self.fail(super::fault("job.infer.resume", "interactive inference outcome lost its exact resume authority"))),
            };
        }
        self.result = result;
        self.outcome = Some(outcome);
        self.phase = InteractivePhase::OutcomeClose;
        PumpTransition::Settled(JobStep::Running(Some(progress)))
"""
new=('''    /// ⚙️ `Pump`: drives the mounted session transition after transition inside ONE host crossing,
    /// spending the crossing's own grant — `budget.fuel` at [`WORK_UNITS_PUMP`] per transition and
    /// `budget.deadline_ms` against the monotonic clock — instead of ending the crossing at the first
    /// outcome. A `Yield`/`PreviewReady` outcome whose payload retires within
    /// [`ABSORB_CLOSE_PAGES`] is retired and resumed in place, the latest preview coalescing into the
    /// crossing's progress bytes; anything lossless or terminal still leaves through `OutcomeClose`.
    /// One transition per crossing made the host relay pay a whole `step-job` round trip (plus an
    /// `OutcomeClose` one) for every 16-unit preview a WFC solve publishes: the 24 × 24 genesis solve
    /// that settles natively in 16.6 s crossed 541 650 times in 582 s over the semio MCP without
    /// finishing (ticket 26/09/23, `📓️wp-g5.md`). Without a clock the grant is one transition, as before.
    // 🚫️async: E1 state action consumed by the sync `BoundedJob::step` dispatch table.
    fn pump(&mut self, budget: JobBudget) -> JobStep {
        let deadline_us = semio_framework_job::default_now_us().and_then(|now_us| now_us.checked_add(u64::from(budget.deadline_ms).saturating_mul(1_000)));
        let mut granted = budget.fuel;
        let mut latest: Option<(Vec<u8>, bool)> = None;
        loop {
            granted = granted.saturating_sub(WORK_UNITS_PUMP);
            match self.pump_transition() {
                PumpTransition::Settled(step) => return step,
                PumpTransition::Absorbed { progress, preview } => {
                    if preview || !latest.as_ref().is_some_and(|(_, kept_preview)| *kept_preview) {
                        latest = Some((progress, preview));
                    }
                }
            }
            let clock_spent = deadline_us.is_none_or(|deadline_us| semio_framework_job::default_now_us().is_none_or(|now_us| now_us >= deadline_us));
            if granted < WORK_UNITS_PUMP || clock_spent {
                return JobStep::Running(Some(latest.map_or_else(|| self.retirement_progress(), |(progress, _)| progress)));
            }
        }
    }

    /// 🔁️ One mounted-session transition of [`Self::pump`]: submits and settles one worker step,
    /// then either absorbs its non-terminal outcome in place or hands the crossing back.
    // 🚫️async: E1 state action body consumed by the sync `pump` loop above.
    fn pump_transition(&mut self) -> PumpTransition {
''' + body + "    }\n\n")
s=s[:start]+new+s[end:]
assert s.count("            InteractivePhase::Pump => self.pump(),")==1
s=s.replace("            InteractivePhase::Pump => self.pump(),","            InteractivePhase::Pump => self.pump(budget),",1)
anchor="struct InteractiveInferenceJob {"
helper='''/// 📄️ Pages a `Yield`/`PreviewReady` outcome may retire inside a `Pump` crossing before the machine
/// hands it to `OutcomeClose` instead — a preview the size of a WFC trace frame retires in one.
const ABSORB_CLOSE_PAGES: usize = 4;

/// 🔁️ What one mounted-session transition left the `Pump` crossing with.
enum PumpTransition {
    Absorbed { progress: Vec<u8>, preview: bool },
    Settled(JobStep),
}

/// 🧾️ Retires an absorbed outcome's payload in place; `false` leaves it for `OutcomeClose`.
fn retire_absorbed_outcome(outcome: &mut StepOutcome) -> bool {
    (0..ABSORB_CLOSE_PAGES).any(|_| matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) && outcome.terminal_is_empty())
}

'''
assert s.count(anchor)==1
s=s.replace(anchor,helper+anchor,1)
s=s.replace("""/// opportunity: `Dispatch` admits the worker session, `Pump` advances it by one bounded worker
/// step,""","""/// opportunity: `Dispatch` admits the worker session, `Pump` advances it by as many bounded worker
/// steps as the crossing's grant covers,""",1)
open(p,"w",encoding="utf-8").write(s)
print("ok")
