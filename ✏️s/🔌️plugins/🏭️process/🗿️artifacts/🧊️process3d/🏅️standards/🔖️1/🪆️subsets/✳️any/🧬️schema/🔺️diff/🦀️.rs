//! 🧬️ Process3d diff schema — sparse field delta over the artifact.

use crate::{Pose, ProcessStep, Stock, Workshop, Process3dSnapshot};
use crate::schema::Process3dArtifact;
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the process3d artifact.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.process.process3d")]
pub struct Process3dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::Process3dArtifact>>,
    #[state(artifact)]
    pub workshop: Option<Workshop>,
    #[state(artifact)]
    pub stock_id: Option<String>,
    #[state(artifact)]
    pub stock_label: Option<String>,
    #[state(artifact)]
    pub stock_pose: Option<Pose>,
    #[state(artifact)]
    pub stock_payload: Option<Stock>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub stock_solid: Option<store::ArtifactChild<SemioBrepSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub steps: Option<store::ArtifactChild<SemioFlowSnapshot>>,
    #[state(artifact)]
    pub step_payloads: Option<Vec<ProcessStep>>,
    #[state(artifact)]
    pub tool_solids: Option<Process3dToolSolidChildList>,
    #[state(artifact)]
    pub resolved_up_to: Option<Option<usize>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧩️ Whole-list wrapper for the `tool_solids` composed CHILD COLLECTION diff field — same
/// `RunList` shape `✳️text`/`✳️kit`/`📐️cad`'s `CadDrawingChildList` use for their own
/// `Vec<ArtifactChild<S>>` diff fields.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dToolSolidChildList {
    pub values: Vec<store::ArtifactChild<SemioBrepSnapshot>>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply
impl Process3dDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &Process3dArtifact) -> protocol::MutationApplyResult<Process3dArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(workshop) = &self.workshop {
                next.workshop = workshop.clone();
            }
            if let Some(value) = &self.stock_id {
                next.stock_id = value.clone();
            }
            if let Some(value) = &self.stock_label {
                next.stock_label = value.clone();
            }
            if let Some(value) = &self.stock_pose {
                next.stock_pose = value.clone();
            }
            if let Some(value) = &self.stock_payload {
                next.stock_payload = value.clone();
            }
            if let Some(value) = &self.stock_solid {
                next.stock_solid = value.clone();
            }
            if let Some(value) = &self.steps {
                next.steps = value.clone();
            }
            if let Some(value) = &self.step_payloads {
                next.step_payloads = value.clone();
            }
            if let Some(value) = &self.tool_solids {
                next.tool_solids = value.values.clone();
            }
            if let Some(value) = &self.resolved_up_to {
                next.resolved_up_to = *value;
            }
            next
        })
    }
}

impl MutationDiff<Process3dSnapshot> for Process3dDiff {
    fn apply(&self, snapshot: &Process3dSnapshot) -> protocol::MutationApplyResult<Process3dSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(workshop) = &self.workshop {
                next.workshop = workshop.clone();
            }
            if let Some(value) = &self.stock_id {
                next.stock_id = value.clone();
            }
            if let Some(value) = &self.stock_label {
                next.stock_label = value.clone();
            }
            if let Some(value) = &self.stock_pose {
                next.stock_pose = value.clone();
            }
            if let Some(value) = &self.stock_payload {
                next.stock_payload = value.clone();
            }
            if let Some(value) = &self.stock_solid {
                next.stock_solid = value.clone();
            }
            if let Some(value) = &self.steps {
                next.steps = value.clone();
            }
            if let Some(value) = &self.step_payloads {
                next.step_payloads = value.clone();
            }
            if let Some(value) = &self.tool_solids {
                next.tool_solids = value.values.clone();
            }
            if let Some(value) = &self.resolved_up_to {
                next.resolved_up_to = *value;
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(workshop);
        take!(stock_id);
        take!(stock_label);
        take!(stock_pose);
        take!(stock_payload);
        take!(stock_solid);
        take!(steps);
        take!(step_payloads);
        take!(tool_solids);
        take!(resolved_up_to);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 📸️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &Process3dSnapshot) -> Process3dDiff {
    Process3dDiff { artifact: Some(Box::new(Process3dArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
