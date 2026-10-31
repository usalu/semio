import glob
from rep import rep
R='/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/'
G=glob.glob(R+'🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry')[0]
rep(G+'/⏱️phased-job/🦀️.rs',[
("use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, spend};","use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, settle, spend, unfunded};"),
("spend(cx, if matches!(step, TessellationStep::Working(_)) { budget } else { 1 });","settle(cx, budget, matches!(step, TessellationStep::Working(_)));"),
("spend(cx, if matches!(step, BrepOperationStep::Working(_)) { budget } else { 1 });","settle(cx, budget, matches!(step, BrepOperationStep::Working(_)));"),
("""            return Ok(Some(WidgetStep::Done(evaluation.clone())));
        }
        loop {""","""            return Ok(Some(WidgetStep::Done(evaluation.clone())));
        }
        if unfunded(cx) {
            return Ok(None);
        }
        loop {"""),
])
rep(G+'/🔗️brep-boolean/🦀️.rs',[
("use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, spend, StepContext, StepFault};","use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, settle, StepContext, StepFault};"),
("spend(cx, if matches!(step, BrepOperationStep::Working(_)) { budget } else { 1 });","settle(cx, budget, matches!(step, BrepOperationStep::Working(_)));"),
])
rep(G+'/✅️analysis-check/🦀️.rs',[
("""            return Ok(Some(WidgetStep::Done(evaluation.clone())));
        }
        loop {""","""            return Ok(Some(WidgetStep::Done(evaluation.clone())));
        }
        if unfunded(cx) {
            return Ok(None);
        }
        loop {"""),
("use semio_s_artifact_procedural_3d_geometry_compute::{spend, OriginalTurnFailure, StepContext, StepFault};","use semio_s_artifact_procedural_3d_geometry_compute::{spend, unfunded, OriginalTurnFailure, StepContext, StepFault};"),
])
rep(G+'/📏️analysis-measure/🦀️.rs',[
("""            return Ok(Some(WidgetStep::Done(evaluation.clone())));
        }
        while self.done < self.total {""","""            return Ok(Some(WidgetStep::Done(evaluation.clone())));
        }
        if unfunded(cx) {
            return Ok(None);
        }
        while self.done < self.total {"""),
("use semio_s_artifact_procedural_3d_geometry_compute::{spend, OriginalTurnFailure, StepContext};","use semio_s_artifact_procedural_3d_geometry_compute::{spend, unfunded, OriginalTurnFailure, StepContext};"),
])
rep(G+'/🧱️mesh-support/🦀️.rs',[
("use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, spend, OriginalTurnFailure, StepContext};","use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, settle, unfunded, OriginalTurnFailure, StepContext};"),
("""            return Ok(Some(self.fault(cancelled_fault())));
        }
        let fuel = fuel_budget(cx);""","""            return Ok(Some(self.fault(cancelled_fault())));
        }
        if unfunded(cx) {
            return Ok(None);
        }
        let fuel = fuel_budget(cx);"""),
("spend(cx, if matches!(outcome, Ok(Ok(Flow::Working(_)))) { fuel } else { 1 });","settle(cx, fuel, matches!(outcome, Ok(Ok(Flow::Working(_)))));"),
])
rep(G+'/../../🚪️io/📐️geometry/💾️brep-interchange/🦀️.rs',[
("use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, spend};","use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, settle};"),
("spend(cx, if step.is_some() { 1 } else { fuel });","settle(cx, fuel, step.is_none());"),
])
