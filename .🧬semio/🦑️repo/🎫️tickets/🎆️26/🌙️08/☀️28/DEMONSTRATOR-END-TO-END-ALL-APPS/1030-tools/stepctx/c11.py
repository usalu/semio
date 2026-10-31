import glob
from rep import rep
G=glob.glob('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry')[0]
p=G+'/🧱️mesh-support/🦀️.rs'
rep(p,[
("under a widget fuel grant.","under the caller's `StepContext`."),
("are charged to the grant like every other unit.","are charged to the caller like every other unit."),
("use semio_s_artifact_procedural_3d_geometry_compute::WidgetStep;","use semio_s_artifact_procedural_3d_geometry_compute::WidgetStep;\nuse semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, spend, OriginalTurnFailure, StepContext};"),
("/// 🧵️ A started piece of mesh work that advances by widget fuel.","/// 🧵️ A started piece of mesh work that advances by the fuel its caller's context funds."),
("    /// ⏱️ Does at most `fuel` units of work.\n    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault>;","    /// ⏱️ Does at most the units of work `cx` funds.\n    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<Flow, WidgetFault>;"),
("""    fn step(&mut self, fuel: usize) -> WidgetStep {
        if self.finished {
            return self.fault(cancelled_fault());
        }
        let fuel = fuel.max(1);
        let outcome""","""    fn step(&mut self, cx: &mut StepContext<'_>) -> Result<Option<WidgetStep>, OriginalTurnFailure> {
        if self.finished {
            return Ok(Some(self.fault(cancelled_fault())));
        }
        let fuel = fuel_budget(cx);
        let outcome"""),
("Some(machine) => machine.advance(fuel),","Some(machine) => machine.advance(cx),"),
("""        match outcome {
            Ok(Ok(Flow::Working(progress))) => WidgetStep::Working { progress: progress.clamp(0.0, 0.999) },
            Ok(Ok(Flow::Done(outputs))) => {
                self.finished = true;
                self.machine = None;
                WidgetStep::Done(WidgetEvaluation::ok(outputs, self.quality))
            }
            Ok(Err(fault)) => self.fault(fault),
            Err(_) => self.fault(stopped_fault()),
        }
    }""","""        spend(cx, if matches!(outcome, Ok(Ok(Flow::Working(_)))) { fuel } else { 1 });
        Ok(Some(match outcome {
            Ok(Ok(Flow::Working(progress))) => WidgetStep::Working { progress: progress.clamp(0.0, 0.999) },
            Ok(Ok(Flow::Done(outputs))) => {
                self.finished = true;
                self.machine = None;
                WidgetStep::Done(WidgetEvaluation::ok(outputs, self.quality))
            }
            Ok(Err(fault)) => self.fault(fault),
            Err(_) => self.fault(stopped_fault()),
        }))
    }"""),
("""impl<J: Stepper> Machine for Modeling<J> {
    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault> {
        match self.0.step(fuel.saturating_mul(BATCH))""","""impl<J: Stepper> Machine for Modeling<J> {
    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<Flow, WidgetFault> {
        match self.0.step(fuel_budget(cx).saturating_mul(BATCH))"""),
("""    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault> {
        if self.job.is_none() {""","""    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<Flow, WidgetFault> {
        let fuel = fuel_budget(cx);
        if self.job.is_none() {"""),
])
p=G+'/🔀️mesh-convert/🦀️.rs'
rep(p,[
("use semio_s_artifact_procedural_3d_geometry_compute::KernelSession;","use semio_s_artifact_procedural_3d_geometry_compute::KernelSession;\nuse semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, StepContext};"),
("impl Machine for FromBrep {\n    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault> {","impl Machine for FromBrep {\n    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<Flow, WidgetFault> {\n        let fuel = fuel_budget(cx);"),
("impl Machine for ToBrep {\n    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault> {","impl Machine for ToBrep {\n    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<Flow, WidgetFault> {\n        let fuel = fuel_budget(cx);"),
])
