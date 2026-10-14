#!/usr/bin/env python3
"""⚙️ Moves `🧪️sim` onto the current `semio_framework_job` close protocol (lane P1).

The job crate replaced the `InteractiveJobCloseStep::Refused` variant with a typed failure: `InteractiveJob::close_step`
now takes the turn's `StepContext` (its retained grant) and returns `Result<_, OriginalTurnFailure>`, and `step` fails
with `OriginalTurnFailure`. Energy's internal close ladders therefore return `Result<InteractiveJobCloseStep, ValueError>`
and propagate an owner refusal with `?` instead of folding it into a `Refused` step.
"""
import re, sys
P = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs"
s = open(P, encoding="utf-8").read()
if "OriginalTurnFailure" in s: sys.exit("already applied")
def sub(a, b, n=1):
    global s
    assert s.count(a) == n, (a[:90], s.count(a)); s = s.replace(a, b)
def span(start, end_marker="\n    }\n"):
    assert s.count(start) == 1, start[:90]; i = s.index(start); j = s.index(end_marker, i) + len(end_marker); return i, j
def okify(i, j):
    global s
    body = s[i:j]
    body = re.sub(r"return (energy_(?:done|idle|one|rung)\([^;]*\));", r"return Ok(\1);", body)
    body = re.sub(r"return (InteractiveJobCloseStep::[A-Za-z]+(?: \{[^;]*\})?);", r"return Ok(\1);", body)
    body = body.replace("return step;", "return Ok(step);")
    body = re.sub(r"energy_turn\(((?:[^;])*)\)(;|\.0)", lambda m: "energy_turn(" + m.group(1) + ")?" + m.group(2), body)
    body = re.sub(r"\n(\s+)energy_done\(\)\n(\s+)\}\n$", r"\n\1Ok(energy_done())\n\2}\n", body)
    s = s[:i] + body + s[j:]
sub("    StepBudget, StepContext,\n};", "    OriginalTurnFailure, StepBudget, StepContext,\n};")
sub("""fn energy_turn(step: Result<RetainedCloneStep, ValueError>) -> (InteractiveJobCloseStep, bool) {
    match step {
        Ok(RetainedCloneStep::Progress(progress)) => (InteractiveJobCloseStep::Pending { progress }, false),
        Ok(RetainedCloneStep::Complete(progress)) => (InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } }, true),
        Err(error) => (InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() }, false),
    }
}""", """fn energy_turn(step: Result<RetainedCloneStep, ValueError>) -> Result<(InteractiveJobCloseStep, bool), ValueError> {
    Ok(match step? {
        RetainedCloneStep::Progress(progress) => (InteractiveJobCloseStep::Pending { progress }, false),
        RetainedCloneStep::Complete(progress) => (InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } }, true),
    })
}""")
sub("/// 🧯️ Maps one owner close turn onto the job close vocabulary and reports whether that owner finished.", "/// 🧯️ Maps one owner close turn onto the job close vocabulary and reports whether that owner finished; an owner\n/// refusal propagates unchanged.")
i, j = span("    fn close_step(&mut self, grant: RetainedCloneGrant) -> Option<InteractiveJobCloseStep> {")
body = s[i:j]
body = body.replace("-> Option<InteractiveJobCloseStep> {", "-> Result<Option<InteractiveJobCloseStep>, ValueError> {")
body = re.sub(r"return Some\((InteractiveJobCloseStep::[^;]*)\);", r"return Ok(Some(\1));", body)
body = body.replace("let packet = self.slots[self.head].as_mut()?;", "let Some(packet) = self.slots[self.head].as_mut() else { return Ok(None) };")
body = body.replace("energy_turn(packet.close_step(grant));", "energy_turn(packet.close_step(grant))?;").replace("        Some(step)\n    }\n", "        Ok(Some(step))\n    }\n")
s = s[:i] + body + s[j:]
for start in ["    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {\n        if !self.packet.terminal_is_empty() {",
              "    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {\n        if let Some(packet) = self.replay_retiring.as_mut() {",
              "    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {\n        self.closing = true;"]:
    i, j = span(start)
    s = s[:i] + s[i:j].replace("-> InteractiveJobCloseStep {", "-> Result<InteractiveJobCloseStep, ValueError> {", 1) + s[j:]
    i, j = span(start.replace("-> InteractiveJobCloseStep {", "-> Result<InteractiveJobCloseStep, ValueError> {"))
    okify(i, j)
sub("""            return match rebuild.close_step(grant) {
                InteractiveJobCloseStep::Complete { progress } => {
                    self.rebuild = None;
                    InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } }
                }
                step => step,
            };""", """            return Ok(match EnergyJobAuthority::close_step(rebuild, grant)? {
                InteractiveJobCloseStep::Complete { progress } => {
                    self.rebuild = None;
                    InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } }
                }
                step => step,
            });""")
sub("""            if let Some(step) = queue.close_step(grant) {
                return Ok(step);
            }""", """            if let Some(step) = queue.close_step(grant)? {
                return Ok(step);
            }""") if "if let Some(step) = queue.close_step(grant) {\n                return Ok(step);" in s else sub("""            if let Some(step) = queue.close_step(grant) {
                return step;
            }""", """            if let Some(step) = queue.close_step(grant)? {
                return Ok(step);
            }""")
sub("    fn step<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {", "    fn step<'a>(&'a mut self, context: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {")
sub("""    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        EnergyJobAuthority::close_step(self, grant)
    }""", """    fn close_step(&mut self, context: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        Ok(EnergyJobAuthority::close_step(self, context.retained_grant())?)
    }""")
sub("""        let (step, _) = energy_turn(packet.close_step(grant));
        assert!(!matches!(step, InteractiveJobCloseStep::Refused { .. }), "a locally owned packet close is never refused");""", """        energy_turn(packet.close_step(grant)).expect("a locally owned packet close is never refused");""")
sub("""        let step = InteractiveJob::close_step(job, grant);
        assert!(step.progress().fits(grant), "an energy close receipt must fit its own quoted grant");
        assert!(!matches!(step, InteractiveJobCloseStep::Blocked | InteractiveJobCloseStep::Refused { .. }), "a locally owned energy job close has no external owner and no refusal");""", """        let step = EnergyJobAuthority::close_step(job, grant).expect("a locally owned energy job close is never refused");
        assert!(step.progress().fits(grant), "an energy close receipt must fit its own quoted grant");
        assert!(!matches!(step, InteractiveJobCloseStep::Blocked), "a locally owned energy job close has no external owner");""")
open(P, "w", encoding="utf-8").write(s)
print("ok")
