//! 🧬️ Process3d diff schema — sparse field delta over the artifact. The machines, the step timeline and the tool solids are
//! id-keyed row deltas (`added`/`removed`/`patched`/`reordered`): a mutation names exactly the rows it creates, deletes or patches.

use crate::{Capability, Pose, ProcessMeasure, ProcessStep, Process3dSnapshot, Stock, StepOrigin, Workshop, WorkshopMachine};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

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
    #[child(kind = "s.stdio.semio")]
    pub steps: Option<store::ArtifactChild<SemioFlowSnapshot>>,
    #[state(artifact)]
    pub step_payloads: Option<Process3dStepsDelta>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub tool_solids: Option<Process3dToolSolidsDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧩️ Id-keyed row delta for the workshop machines.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dMachinesDelta {
    pub added: Vec<WorkshopMachine>,
    pub removed: Vec<String>,
    pub patched: Vec<Process3dMachinePatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 Field patch of one machine; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dMachinePatch {
    pub id: String,
    pub label: Option<String>,
    pub icon_id: Option<String>,
    pub capabilities: Option<Vec<Capability>>,
}

/// 🧩️ Id-keyed row delta for the step timeline.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dStepsDelta {
    pub added: Vec<ProcessStep>,
    pub removed: Vec<String>,
    pub patched: Vec<Process3dStepPatch>,
    pub reordered: Option<Vec<String>>,
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
    pub id: String,
    pub label: Option<String>,
    pub enabled: Option<bool>,
    pub measure: Option<ProcessMeasure>,
    pub origin: Option<Process3dOptionalOrigin>,
}

/// 🧩️ Child-keyed row delta for the tool solids a timeline mints: a solid is an immutable content-addressed child, so it is only ever
/// added, removed or moved, never patched.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dToolSolidsDelta {
    pub added: Vec<BrepChild>,
    pub removed: Vec<String>,
    pub patched: Vec<Process3dToolSolidPatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 The (always empty) patch of an immutable tool solid.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dToolSolidPatch {
    pub child_id: String,
}
//#endregion 🔖️DeltaHelpers

//#region 🧺️KeyedDelta
/// 🧺️ Id-keyed ordered-collection delta (`added`/`removed`/`patched`/`reordered`) and its algebra: apply, composition (create∘delete
/// cancels, delete∘create replaces, patch∘patch composes, patch∘create folds), negative delta and state delta.
pub trait KeyedDelta: Sized {
    type Row: Clone;
    type Patch: Clone;
    fn added(&self) -> &[Self::Row];
    fn removed(&self) -> &[String];
    fn patched(&self) -> &[Self::Patch];
    fn reordered(&self) -> Option<&[String]>;
    fn assemble(added: Vec<Self::Row>, removed: Vec<String>, patched: Vec<Self::Patch>, reordered: Option<Vec<String>>) -> Self;
    fn row_key(row: &Self::Row) -> &str;
    fn patch_key(patch: &Self::Patch) -> &str;
    fn patch_fold(patch: &Self::Patch, row: &mut Self::Row) -> Result<(), protocol::MutationApplyError>;
    fn patch_compose(first: &Self::Patch, later: &Self::Patch) -> Self::Patch;
    fn patch_inverse(patch: &Self::Patch, base: &Self::Row) -> Self::Patch;
    fn patch_between(base: &Self::Row, other: &Self::Row) -> Option<Self::Patch>;
    fn patch_is_empty(patch: &Self::Patch) -> bool;
}

fn keyed_error(code: &str, message: &str, at: [&str; 2]) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(at)
}

pub fn keyed_apply<D: KeyedDelta>(rows: &[D::Row], delta: &D) -> Result<Vec<D::Row>, protocol::MutationApplyError> {
    let key = D::row_key;
    for (index, id) in delta.removed().iter().enumerate() {
        if delta.removed()[..index].contains(id) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is removed more than once", ["removed", &index.to_string()]));
        }
        if !rows.iter().any(|row| key(row) == id) {
            return Err(keyed_error("mutation.apply.missing-target", "removed row does not exist", ["removed", &index.to_string()]));
        }
    }
    let mut next: Vec<D::Row> = rows.iter().filter(|row| !delta.removed().iter().any(|id| id == key(row))).cloned().collect();
    for (index, row) in delta.added().iter().enumerate() {
        if next.iter().any(|existing| key(existing) == key(row)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "added row identity already exists", ["added", &index.to_string()]));
        }
        next.push(row.clone());
    }
    for (index, patch) in delta.patched().iter().enumerate() {
        if delta.patched()[..index].iter().any(|earlier| D::patch_key(earlier) == D::patch_key(patch)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is patched more than once", ["patched", &index.to_string()]));
        }
        let row = next.iter_mut().find(|row| key(row) == D::patch_key(patch)).ok_or_else(|| keyed_error("mutation.apply.missing-target", "patched row does not exist", ["patched", &index.to_string()]))?;
        D::patch_fold(patch, row).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    let Some(order) = delta.reordered() else { return Ok(next) };
    if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|row| key(row) == id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered".to_string()]));
    }
    Ok(order.iter().filter_map(|id| next.iter().find(|row| key(row) == id).cloned()).collect())
}

fn canonical<D: KeyedDelta>(mut added: Vec<D::Row>, mut removed: Vec<String>, mut patched: Vec<D::Patch>, reordered: Option<Vec<String>>) -> D {
    if let Some(order) = &reordered {
        added.sort_by_key(|row| order.iter().position(|id| id == D::row_key(row)).unwrap_or(usize::MAX));
    }
    let reordered = reordered.filter(|order| {
        let tail = added.len();
        !(order.len() <= tail + 1 && order.len() >= tail && order[order.len() - tail..].iter().map(String::as_str).eq(added.iter().map(D::row_key)))
    });
    removed.sort();
    removed.dedup();
    patched.retain(|patch| !D::patch_is_empty(patch));
    patched.sort_by(|left, right| D::patch_key(left).cmp(D::patch_key(right)));
    D::assemble(added, removed, patched, reordered)
}

/// ➕️ Normal form of `first` then `later`: create∘delete cancels, delete∘create replaces, patch∘patch composes, patch∘create folds.
pub fn keyed_absorb<D: KeyedDelta>(first: &D, later: &D) -> D {
    let mut added: Vec<D::Row> = first.added().to_vec();
    let mut removed: Vec<String> = first.removed().to_vec();
    let mut patched: Vec<D::Patch> = Vec::new();
    for patch in first.patched() {
        match added.iter_mut().find(|row| D::row_key(row) == D::patch_key(patch)) {
            Some(row) => {
                let _ = D::patch_fold(patch, row);
            }
            None => patched.push(patch.clone()),
        }
    }
    for id in later.removed() {
        if let Some(position) = added.iter().position(|row| D::row_key(row) == id) {
            added.remove(position);
        } else {
            patched.retain(|patch| D::patch_key(patch) != id);
            if !removed.contains(id) {
                removed.push(id.clone());
            }
        }
    }
    added.extend(later.added().iter().cloned());
    for patch in later.patched() {
        let key = D::patch_key(patch);
        if let Some(row) = added.iter_mut().find(|row| D::row_key(row) == key) {
            let _ = D::patch_fold(patch, row);
        } else if let Some(existing) = patched.iter_mut().find(|existing| D::patch_key(existing) == key) {
            *existing = D::patch_compose(existing, patch);
        } else {
            patched.push(patch.clone());
        }
    }
    let reordered = match (later.reordered(), first.reordered()) {
        (Some(order), _) => Some(order.to_vec()),
        (None, Some(order)) => Some(order.iter().filter(|id| !later.removed().contains(id)).cloned().chain(later.added().iter().map(|row| D::row_key(row).to_string())).collect()),
        (None, None) => None,
    };
    canonical::<D>(added, removed, patched, reordered)
}

fn ids_after<D: KeyedDelta>(base: &[String], delta: &D) -> Vec<String> {
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => base.iter().filter(|id| !delta.removed().contains(id)).cloned().chain(delta.added().iter().map(|row| D::row_key(row).to_string())).collect(),
    }
}

/// 🔁️ The negative delta: removes what `delta` added, restores what it removed, undoes its patches, restores the base order.
pub fn keyed_inverse<D: KeyedDelta>(delta: &D, base: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = delta.added().iter().map(|row| D::row_key(row).to_string()).collect();
    let added: Vec<D::Row> = delta.removed().iter().filter_map(|id| base.iter().find(|row| D::row_key(row) == id).cloned()).collect();
    let patched: Vec<D::Patch> = delta
        .patched()
        .iter()
        .filter(|patch| !removed.iter().any(|id| id == D::patch_key(patch)))
        .filter_map(|patch| base.iter().find(|row| D::row_key(row) == D::patch_key(patch)).map(|row| D::patch_inverse(patch, row)))
        .collect();
    let after = ids_after(&base_ids, delta);
    let natural: Vec<String> = after.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != base_ids).then_some(base_ids);
    canonical::<D>(added, removed, patched, reordered)
}

/// 🧭️ The delta turning `base` into `other` (sync/import only).
pub fn keyed_between<D: KeyedDelta>(base: &[D::Row], other: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let other_ids: Vec<String> = other.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = base_ids.iter().filter(|id| !other_ids.contains(id)).cloned().collect();
    let added: Vec<D::Row> = other.iter().filter(|row| !base_ids.iter().any(|id| id == D::row_key(row))).cloned().collect();
    let patched: Vec<D::Patch> = other.iter().filter_map(|row| base.iter().find(|candidate| D::row_key(candidate) == D::row_key(row)).and_then(|candidate| D::patch_between(candidate, row))).collect();
    let natural: Vec<String> = base_ids.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != other_ids).then_some(other_ids);
    canonical::<D>(added, removed, patched, reordered)
}

pub fn keyed_is_empty<D: KeyedDelta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().iter().all(D::patch_is_empty) && delta.reordered().is_none()
}
//#endregion 🧺️KeyedDelta

impl KeyedDelta for Process3dMachinesDelta {
    type Row = WorkshopMachine;
    type Patch = Process3dMachinePatch;
    fn added(&self) -> &[WorkshopMachine] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[Process3dMachinePatch] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<WorkshopMachine>, removed: Vec<String>, patched: Vec<Process3dMachinePatch>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &WorkshopMachine) -> &str {
        &row.id
    }
    fn patch_key(patch: &Process3dMachinePatch) -> &str {
        &patch.id
    }
    fn patch_fold(patch: &Process3dMachinePatch, row: &mut WorkshopMachine) -> Result<(), protocol::MutationApplyError> {
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
    fn patch_compose(first: &Process3dMachinePatch, later: &Process3dMachinePatch) -> Process3dMachinePatch {
        Process3dMachinePatch {
            id: first.id.clone(),
            label: later.label.clone().or_else(|| first.label.clone()),
            icon_id: later.icon_id.clone().or_else(|| first.icon_id.clone()),
            capabilities: later.capabilities.clone().or_else(|| first.capabilities.clone()),
        }
    }
    fn patch_inverse(patch: &Process3dMachinePatch, base: &WorkshopMachine) -> Process3dMachinePatch {
        Process3dMachinePatch {
            id: patch.id.clone(),
            label: patch.label.as_ref().map(|_| base.label.clone()),
            icon_id: patch.icon_id.as_ref().map(|_| base.icon_id.clone()),
            capabilities: patch.capabilities.as_ref().map(|_| base.capabilities.clone()),
        }
    }
    fn patch_between(base: &WorkshopMachine, other: &WorkshopMachine) -> Option<Process3dMachinePatch> {
        let patch = Process3dMachinePatch {
            id: other.id.clone(),
            label: (base.label != other.label).then(|| other.label.clone()),
            icon_id: (base.icon_id != other.icon_id).then(|| other.icon_id.clone()),
            capabilities: (base.capabilities != other.capabilities).then(|| other.capabilities.clone()),
        };
        (!Self::patch_is_empty(&patch)).then_some(patch)
    }
    fn patch_is_empty(patch: &Process3dMachinePatch) -> bool {
        patch.label.is_none() && patch.icon_id.is_none() && patch.capabilities.is_none()
    }
}

impl KeyedDelta for Process3dStepsDelta {
    type Row = ProcessStep;
    type Patch = Process3dStepPatch;
    fn added(&self) -> &[ProcessStep] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[Process3dStepPatch] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<ProcessStep>, removed: Vec<String>, patched: Vec<Process3dStepPatch>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &ProcessStep) -> &str {
        &row.id
    }
    fn patch_key(patch: &Process3dStepPatch) -> &str {
        &patch.id
    }
    fn patch_fold(patch: &Process3dStepPatch, row: &mut ProcessStep) -> Result<(), protocol::MutationApplyError> {
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
    fn patch_compose(first: &Process3dStepPatch, later: &Process3dStepPatch) -> Process3dStepPatch {
        Process3dStepPatch {
            id: first.id.clone(),
            label: later.label.clone().or_else(|| first.label.clone()),
            enabled: later.enabled.or(first.enabled),
            measure: later.measure.clone().or_else(|| first.measure.clone()),
            origin: later.origin.clone().or_else(|| first.origin.clone()),
        }
    }
    fn patch_inverse(patch: &Process3dStepPatch, base: &ProcessStep) -> Process3dStepPatch {
        Process3dStepPatch {
            id: patch.id.clone(),
            label: patch.label.as_ref().map(|_| base.label.clone()),
            enabled: patch.enabled.map(|_| base.enabled),
            measure: patch.measure.as_ref().map(|_| base.measure.clone()),
            origin: patch.origin.as_ref().map(|_| Process3dOptionalOrigin { value: base.origin.clone() }),
        }
    }
    fn patch_between(base: &ProcessStep, other: &ProcessStep) -> Option<Process3dStepPatch> {
        let patch = Process3dStepPatch {
            id: other.id.clone(),
            label: (base.label != other.label).then(|| other.label.clone()),
            enabled: (base.enabled != other.enabled).then_some(other.enabled),
            measure: (base.measure != other.measure).then(|| other.measure.clone()),
            origin: (base.origin != other.origin).then(|| Process3dOptionalOrigin { value: other.origin.clone() }),
        };
        (!Self::patch_is_empty(&patch)).then_some(patch)
    }
    fn patch_is_empty(patch: &Process3dStepPatch) -> bool {
        patch.label.is_none() && patch.enabled.is_none() && patch.measure.is_none() && patch.origin.is_none()
    }
}

impl KeyedDelta for Process3dToolSolidsDelta {
    type Row = BrepChild;
    type Patch = Process3dToolSolidPatch;
    fn added(&self) -> &[BrepChild] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[Process3dToolSolidPatch] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<BrepChild>, removed: Vec<String>, patched: Vec<Process3dToolSolidPatch>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &BrepChild) -> &str {
        &row.child_id
    }
    fn patch_key(patch: &Process3dToolSolidPatch) -> &str {
        &patch.child_id
    }
    fn patch_fold(_patch: &Process3dToolSolidPatch, _row: &mut BrepChild) -> Result<(), protocol::MutationApplyError> {
        Ok(())
    }
    fn patch_compose(first: &Process3dToolSolidPatch, _later: &Process3dToolSolidPatch) -> Process3dToolSolidPatch {
        first.clone()
    }
    fn patch_inverse(patch: &Process3dToolSolidPatch, _base: &BrepChild) -> Process3dToolSolidPatch {
        patch.clone()
    }
    fn patch_between(_base: &BrepChild, _other: &BrepChild) -> Option<Process3dToolSolidPatch> {
        None
    }
    fn patch_is_empty(_patch: &Process3dToolSolidPatch) -> bool {
        true
    }
}

//#region 🔖️Apply
impl MutationDiff<Process3dSnapshot> for Process3dDiff {
    fn apply(&self, snapshot: &Process3dSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Process3dSnapshot> {
        let mut next = snapshot.clone();
        if let Some(delta) = &self.workshop {
            next.workshop = Workshop { machines: keyed_apply(&next.workshop.machines, delta).map_err(|error| error.under(["workshop"]))? };
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
        if let Some(delta) = &self.step_payloads {
            next.step_payloads = keyed_apply(&next.step_payloads, delta).map_err(|error| error.under(["stepPayloads"]))?;
        }
        if let Some(delta) = &self.tool_solids {
            next.tool_solids = keyed_apply(&next.tool_solids, delta).map_err(|error| error.under(["toolSolids"]))?;
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
                    (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
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
        take!(steps);
        compose!(step_payloads);
        compose!(tool_solids);
    }
}

impl protocol::DiffAlgebra<Process3dSnapshot> for Process3dDiff {
    fn inverse(&self, base: &Process3dSnapshot) -> Self {
        Self {
            workshop: self.workshop.as_ref().map(|delta| keyed_inverse(delta, &base.workshop.machines)),
            stock_id: self.stock_id.as_ref().map(|_| base.stock_id.clone()),
            stock_label: self.stock_label.as_ref().map(|_| base.stock_label.clone()),
            stock_pose: self.stock_pose.as_ref().map(|_| base.stock_pose.clone()),
            stock_payload: self.stock_payload.as_ref().map(|_| base.stock_payload.clone()),
            stock_solid: self.stock_solid.as_ref().map(|_| base.stock_solid.clone()),
            steps: self.steps.as_ref().map(|_| base.steps.clone()),
            step_payloads: self.step_payloads.as_ref().map(|delta| keyed_inverse(delta, &base.step_payloads)),
            tool_solids: self.tool_solids.as_ref().map(|delta| keyed_inverse(delta, &base.tool_solids)),
        }
    }
    fn between(base: &Process3dSnapshot, other: &Process3dSnapshot) -> Self {
        let workshop = keyed_between::<Process3dMachinesDelta>(&base.workshop.machines, &other.workshop.machines);
        let step_payloads = keyed_between::<Process3dStepsDelta>(&base.step_payloads, &other.step_payloads);
        let tool_solids = keyed_between::<Process3dToolSolidsDelta>(&base.tool_solids, &other.tool_solids);
        Self {
            workshop: (!keyed_is_empty(&workshop)).then_some(workshop),
            stock_id: (base.stock_id != other.stock_id).then(|| other.stock_id.clone()),
            stock_label: (base.stock_label != other.stock_label).then(|| other.stock_label.clone()),
            stock_pose: (base.stock_pose != other.stock_pose).then(|| other.stock_pose.clone()),
            stock_payload: (base.stock_payload != other.stock_payload).then(|| other.stock_payload.clone()),
            stock_solid: (base.stock_solid != other.stock_solid).then(|| other.stock_solid.clone()),
            steps: (base.steps != other.steps).then(|| other.steps.clone()),
            step_payloads: (!keyed_is_empty(&step_payloads)).then_some(step_payloads),
            tool_solids: (!keyed_is_empty(&tool_solids)).then_some(tool_solids),
        }
    }
    fn is_empty(&self) -> bool {
        self.workshop.as_ref().is_none_or(keyed_is_empty)
            && self.stock_id.is_none()
            && self.stock_label.is_none()
            && self.stock_pose.is_none()
            && self.stock_payload.is_none()
            && self.stock_solid.is_none()
            && self.steps.is_none()
            && self.step_payloads.as_ref().is_none_or(keyed_is_empty)
            && self.tool_solids.as_ref().is_none_or(keyed_is_empty)
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
