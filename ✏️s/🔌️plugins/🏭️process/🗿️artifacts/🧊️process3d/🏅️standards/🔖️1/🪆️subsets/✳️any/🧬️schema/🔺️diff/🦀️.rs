//! 🧬️ Process3d diff schema — sparse field delta over the artifact. The machines and the step timeline are positional row
//! deltas (`removed`/`inserted`/`moved`/`modified`, see `protocol::list_delta`): a mutation names exactly the rows it creates, deletes, moves or patches. The `steps` flow
//! handle and the `tool_solids` handles are derived from the step rows by `apply`, never carried.

use crate::{Capability, Pose, ProcessMeasure, ProcessStep, Process3dSnapshot, Stock, StepOrigin, Workshop, WorkshopMachine};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

type BrepChild = store::ArtifactChild<SemioBrepSnapshot>;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the process3d artifact.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.process.process3d")]
pub struct Process3dDiff {
    #[state(artifact)]
    pub workshop: Option<Process3dMachinesDelta>,
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
    pub stock_solid: Option<BrepChild>,
    #[state(artifact)]
    pub step_payloads: Option<Process3dStepsDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
protocol::list_delta! {
    pub Process3dMachinesDelta { removal: Process3dMachinesRemoval, insertion: Process3dMachinesInsertion, relocation: Process3dMachinesRelocation, modification: Process3dMachinesModification, row: WorkshopMachine, patch: Process3dMachinePatch, key: id, values_only }
}

/// 🩹 Field patch of one machine; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dMachinePatch {
    pub label: Option<String>,
    pub icon_id: Option<String>,
    pub capabilities: Option<Vec<Capability>>,
}

protocol::list_delta! {
    pub Process3dStepsDelta { removal: Process3dStepsRemoval, insertion: Process3dStepsInsertion, relocation: Process3dStepsRelocation, modification: Process3dStepsModification, row: ProcessStep, patch: Process3dStepPatch, key: id, values_only }
}

/// 🧱️ Carries the optional step origin as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dOptionalOrigin {
    pub value: Option<StepOrigin>,
}

/// 🩹 Field patch of one step; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dStepPatch {
    pub label: Option<String>,
    pub enabled: Option<bool>,
    pub measure: Option<ProcessMeasure>,
    pub origin: Option<Process3dOptionalOrigin>,
}

//#endregion 🔖️DeltaHelpers


impl protocol::list_delta::RowPatch<WorkshopMachine> for Process3dMachinePatch {
    fn commit_into(&self, row: &mut WorkshopMachine, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        let patch = self;
        if let Some(label) = &patch.label {
            row.label = label.clone();
        }
        if let Some(icon_id) = &patch.icon_id {
            row.icon_id = icon_id.clone();
        }
        if let Some(capabilities) = &patch.capabilities {
            row.capabilities = capabilities.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        let earlier = self.clone();
        let first = &earlier;
        let later = &later;
        *self = {
        Process3dMachinePatch {
            id: first.id.clone(),
            label: later.label.clone().or_else(|| first.label.clone()),
            icon_id: later.icon_id.clone().or_else(|| first.icon_id.clone()),
            capabilities: later.capabilities.clone().or_else(|| first.capabilities.clone()),
        }
        };
    }
    fn inverse(&self, row: &WorkshopMachine) -> Self {
        let patch = self;
        let base = row;
        Process3dMachinePatch {
                        label: patch.label.as_ref().map(|_| base.label.clone()),
            icon_id: patch.icon_id.as_ref().map(|_| base.icon_id.clone()),
            capabilities: patch.capabilities.as_ref().map(|_| base.capabilities.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        let patch = self;
        patch.label.is_none() && patch.icon_id.is_none() && patch.capabilities.is_none()
    }
}

impl protocol::list_delta::RowPatch<ProcessStep> for Process3dStepPatch {
    fn commit_into(&self, row: &mut ProcessStep, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        let patch = self;
        if let Some(label) = &patch.label {
            row.label = label.clone();
        }
        if let Some(enabled) = patch.enabled {
            row.enabled = enabled;
        }
        if let Some(measure) = &patch.measure {
            row.measure = measure.clone();
        }
        if let Some(origin) = &patch.origin {
            row.origin = origin.value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        let earlier = self.clone();
        let first = &earlier;
        let later = &later;
        *self = {
        Process3dStepPatch {
            id: first.id.clone(),
            label: later.label.clone().or_else(|| first.label.clone()),
            enabled: later.enabled.or(first.enabled),
            measure: later.measure.clone().or_else(|| first.measure.clone()),
            origin: later.origin.clone().or_else(|| first.origin.clone()),
        }
        };
    }
    fn inverse(&self, row: &ProcessStep) -> Self {
        let patch = self;
        let base = row;
        Process3dStepPatch {
                        label: patch.label.as_ref().map(|_| base.label.clone()),
            enabled: patch.enabled.map(|_| base.enabled),
            measure: patch.measure.as_ref().map(|_| base.measure.clone()),
            origin: patch.origin.as_ref().map(|_| Process3dOptionalOrigin { value: base.origin.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        let patch = self;
        patch.label.is_none() && patch.enabled.is_none() && patch.measure.is_none() && patch.origin.is_none()
    }
}

//#region 🔖️Apply
impl MutationDiff<Process3dSnapshot> for Process3dDiff {
    fn apply(&self, snapshot: &Process3dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Process3dSnapshot> {
        let mut next = snapshot.clone();
        if let Some(delta) = &self.workshop {
            next.workshop = Workshop { machines: delta.commit_onto(&next.workshop.machines, capability).map_err(|error| error.under(["workshop"]))? };
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
        if let Some(delta) = &self.step_payloads {
            next.step_payloads = delta.commit_onto(&next.step_payloads, capability).map_err(|error| error.under(["stepPayloads"]))?;
            let derived = crate::process_working_scene_to_snapshot(&crate::process_working_scene_from_snapshot(&next), next.workshop.clone());
            next.steps = derived.steps;
            next.tool_solids = derived.tool_solids;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        macro_rules! compose {
            ($field:ident) => {
                self.$field = match (self.$field.take(), other.$field) {
                    (Some(mut first), Some(later)) => {
                        first.absorb(later);
                        Some(first)
                    }
                    (first, later) => later.or(first),
                };
            };
        }
        compose!(workshop);
        take!(stock_id);
        take!(stock_label);
        take!(stock_pose);
        take!(stock_payload);
        take!(stock_solid);
        compose!(step_payloads);
    }
}

impl protocol::DiffAlgebra<Process3dSnapshot> for Process3dDiff {
    fn inverse(&self, base: &Process3dSnapshot) -> Self {
        Self {
            workshop: self.workshop.as_ref().map(|delta| delta.inverse(&base.workshop.machines)),
            stock_id: self.stock_id.as_ref().map(|_| base.stock_id.clone()),
            stock_label: self.stock_label.as_ref().map(|_| base.stock_label.clone()),
            stock_pose: self.stock_pose.as_ref().map(|_| base.stock_pose.clone()),
            stock_payload: self.stock_payload.as_ref().map(|_| base.stock_payload.clone()),
            stock_solid: self.stock_solid.as_ref().map(|_| base.stock_solid.clone()),
            step_payloads: self.step_payloads.as_ref().map(|delta| delta.inverse(&base.step_payloads)),
        }
    }
    fn is_empty(&self) -> bool {
        self.workshop.as_ref().is_none_or(Process3dMachinesDelta::is_empty)
            && self.stock_id.is_none()
            && self.stock_label.is_none()
            && self.stock_pose.is_none()
            && self.stock_payload.is_none()
            && self.stock_solid.is_none()
            && self.step_payloads.as_ref().is_none_or(Process3dStepsDelta::is_empty)
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
