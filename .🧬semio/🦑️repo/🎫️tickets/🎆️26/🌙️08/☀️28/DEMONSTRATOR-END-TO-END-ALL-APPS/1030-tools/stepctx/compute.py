import glob,os
from rep import rep
G=glob.glob('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry')[0]
p=G+'/⚙️compute/🦀️.rs'
s=open(p,encoding='utf8').read()
a=s.index('/// 🧊️ Runs one cold-oracle')
b=s.index('/// 🧵️ A widget compute retains')
s=s[:a]+s[b:]
open(p,'w',encoding='utf8').write(s)
rep(p,[
("""/// Cold oracle stepping uses fuel; original stepping consumes the initiating caller directly.
pub trait WidgetJob: Send {
    fn step(&mut self, fuel: usize) -> WidgetStep;
    fn cancel(&mut self);
    fn step_on_original(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<WidgetStep>, semio_framework_job::OriginalTurnFailure> {
        Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "widget job has no original funded producer").into())
    }
}""",
"""/// Every turn is funded by the caller's `StepContext`; `None` means the caller's grant admitted no progress this turn.
pub trait WidgetJob: Send {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<WidgetStep>, semio_framework_job::OriginalTurnFailure>;
    fn cancel(&mut self);
}"""),
("""    fn step(&mut self, fuel: usize) -> WidgetStep {
        if fuel == 0 || self.cancelled { return WidgetStep::Working { progress: 1.0 }; }
        self.evaluation.take().map_or(WidgetStep::Working { progress: 1.0 }, WidgetStep::Done)
    }

    fn cancel(&mut self) { self.cancelled = true; }

    fn step_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<WidgetStep>, semio_framework_job::OriginalTurnFailure> {
        let source = cx.original_source_parent()?.source_identity()?;
        if self.original_source != Some(source) {
            return Err(""",
"""    fn cancel(&mut self) { self.cancelled = true; }

    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<WidgetStep>, semio_framework_job::OriginalTurnFailure> {
        if let Some(expected) = self.original_source {
            if cx.original_source_parent()?.source_identity()? != expected {
                return Err("""),
])
