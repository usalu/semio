import glob
from rep import rep
G=glob.glob('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry')[0]
rep(G+'/⚙️compute/🦀️.rs',[
("""/// 🧵️ A widget compute retains""","""/// ⛽️ The units of work the caller's context still funds this turn; a turn always admits at least one.
pub fn fuel_budget(cx: &semio_framework_job::StepContext<'_>) -> usize {
    usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX).max(1)
}

/// 🧾️ Charges the caller's context for `units` of work this turn actually did.
pub fn spend(cx: &mut semio_framework_job::StepContext<'_>, units: usize) {
    cx.consume_fuel(u64::try_from(units).unwrap_or(u64::MAX));
}

/// 🧵️ A widget compute retains""")])

p=G+'/⏱️phased-job/🦀️.rs'
rep(p,[
("use semio_s_artifact_procedural_3d_geometry_compute::finish;","use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, spend};\nuse semio_framework_job::{OriginalTurnFailure, StepContext};\nuse semio_s_artifact_procedural_3d_geometry_compute::finish;"),
("A step that is a lane J operation job is advanced with the remaining fuel of the grant, every other step is one unit","A step that is a lane J operation job is advanced with the remaining fuel of the caller's `StepContext`, every other step is one unit"),
("type Step = Box<dyn FnMut(&mut Work, usize) -> Result<Flow, WidgetFault> + Send>;","type Step = Box<dyn FnMut(&mut Work, &mut StepContext<'_>) -> Result<Flow, WidgetFault> + Send>;"),
("""        self.step(move |work, fuel| {
            if running.is_none() {
                running = Some(work.source(index)?.tessellate_job(deflection).map_err(|error| kernel_fault(&error))?);
            }
            let job = running.as_mut().ok_or_else(pipeline_fault)?;
            match job.step(fuel).map_err(|error| kernel_fault(&error))? {
                TessellationStep::Working(progress) => Ok(""",
"""        self.step(move |work, cx| {
            if running.is_none() {
                running = Some(work.source(index)?.tessellate_job(deflection).map_err(|error| kernel_fault(&error))?);
            }
            let job = running.as_mut().ok_or_else(pipeline_fault)?;
            let budget = fuel_budget(cx);
            let step = job.step(budget).map_err(|error| kernel_fault(&error))?;
            spend(cx, if matches!(step, TessellationStep::Working(_)) { budget } else { 1 });
            match step {
                TessellationStep::Working(progress) => Ok("""),
("pub fn step(mut self, run: impl FnMut(&mut Work, usize) -> Result<Flow, WidgetFault> + Send + 'static) -> Self {","pub fn step(mut self, run: impl FnMut(&mut Work, &mut StepContext<'_>) -> Result<Flow, WidgetFault> + Send + 'static) -> Self {"),
("""        self.step(move |work, fuel| {
            if let Some(plan) = plan.take() {""","""        self.step(move |work, cx| {
            if let Some(plan) = plan.take() {"""),
("""                match work.session.brep().step_operation_job_sync(job, fuel).map_err(|error| kernel_fault(&error))? {
                    BrepOperationStep::Working(progress) => return Ok(""","""                let budget = fuel_budget(cx);
                let step = work.session.brep().step_operation_job_sync(job, budget).map_err(|error| kernel_fault(&error))?;
                spend(cx, if matches!(step, BrepOperationStep::Working(_)) { budget } else { 1 });
                match step {
                    BrepOperationStep::Working(progress) => return Ok("""),
("""        self.step(move |work, fuel| {
            if work.values.len()""","""        self.step(move |work, cx| {
            if work.values.len()"""),
("""            let mut budget = fuel.max(1);
            while position.0 < work.groups.len() && budget > 0 {""","""            while position.0 < work.groups.len() && !cx.should_yield() {"""),
("""                done += 1;
                budget -= 1;
            }""","""                done += 1;
                spend(cx, 1);
            }"""),
("""    fn step(&mut self, fuel: usize) -> WidgetStep {
        if let Some(evaluation) = &self.finished {
            return WidgetStep::Done(evaluation.clone());
        }
        let mut left = fuel.max(1);
        loop {""","""    fn step(&mut self, cx: &mut StepContext<'_>) -> Result<Option<WidgetStep>, OriginalTurnFailure> {
        if let Some(evaluation) = &self.finished {
            return Ok(Some(WidgetStep::Done(evaluation.clone())));
        }
        loop {"""),
])
s=open(p,encoding='utf8').read()
a=s.index("impl WidgetJob for PhasedJob")
b=s.index("//#endregion 🔖️Job",a)
body=s[a:b]
body=body.replace("return self.end(Err(pipeline_fault()));","return Ok(Some(self.end(Err(pipeline_fault()))));")
body=body.replace("step(work, left)","step(work, cx)")
body=body.replace("Err(fault) => return self.end(Err(fault)),","Err(fault) => return Ok(Some(self.end(Err(fault)))),")
body=body.replace("Ok(Flow::Finish(outputs)) => return self.end(Ok(outputs)),","Ok(Flow::Finish(outputs)) => return Ok(Some(self.end(Ok(outputs)))),")
body=body.replace("return WidgetStep::Working { progress: self.progress().min(0.999) };","return Ok(Some(WidgetStep::Working { progress: self.progress().min(0.999) }));")
body=body.replace("""                    left = left.saturating_sub(1);
                    if left == 0 {""","""                    spend(cx, 1);
                    if cx.should_yield() {""")
s=s[:a]+body+s[b:]
open(p,'w',encoding='utf8').write(s)
