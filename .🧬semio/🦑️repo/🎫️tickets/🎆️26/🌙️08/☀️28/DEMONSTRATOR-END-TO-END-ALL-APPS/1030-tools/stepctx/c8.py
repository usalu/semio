import glob
from rep import rep
G=glob.glob('/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry')[0]
blk="""/// 🚫️ Why one step stopped: the widget faulted, or the caller's funded context refused the turn.
pub enum StepFault {
    Widget(WidgetFault),
    Turn(OriginalTurnFailure),
}

impl From<WidgetFault> for StepFault {
    fn from(fault: WidgetFault) -> Self {
        Self::Widget(fault)
    }
}

impl From<OriginalTurnFailure> for StepFault {
    fn from(failure: OriginalTurnFailure) -> Self {
        Self::Turn(failure)
    }
}

"""
rep(G+'/⏱️phased-job/🦀️.rs',[(blk,""),("use semio_s_artifact_procedural_3d_geometry_compute::{OriginalTurnFailure, StepContext};","use semio_s_artifact_procedural_3d_geometry_compute::{OriginalTurnFailure, StepContext, StepFault};")])
rep(G+'/⚙️compute/🦀️.rs',[("/// ⛽️ The units of work",blk+"/// ⛽️ The units of work")])
rep(G+'/🔗️brep-boolean/🦀️.rs',[("use semio_s_artifact_procedural_3d_phased_job::StepFault;\nuse semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, spend};\nuse semio_s_artifact_procedural_3d_geometry_compute::StepContext;","use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, spend, StepContext, StepFault};")])
