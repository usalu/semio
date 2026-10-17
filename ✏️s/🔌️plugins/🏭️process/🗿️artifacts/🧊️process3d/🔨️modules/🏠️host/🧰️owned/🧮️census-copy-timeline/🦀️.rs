//! 🧮️ process3d owner census, snapshot copy and timeline: the census and copy cursors, the timeline owner and cursor, the retained mutation applier, the owner totals and the per-record copy functions.

use super::*;

pub(super) fn process3d_copy_string(source: &str) -> Result<String, &'static str> {
    if source.len() > PROCESS3D_OWNER_BYTES {
        return Err("process3d-owner.string-capacity");
    }
    let mut value = String::with_capacity(source.len());
    if value.capacity() > PROCESS3D_OWNER_BYTES {
        return Err("process3d-owner.string-observed-capacity");
    }
    value.push_str(source);
    Ok(value)
}

pub(super) struct Process3dOwnerCensusCursor {
    pub(super) phase: Process3dCensusPhase,
    totals: Process3dOwnerTotals,
}

impl Process3dOwnerCensusCursor {
    pub(super) fn new() -> Self {
        Self { phase: Process3dCensusPhase::Root, totals: Process3dOwnerTotals::default() }
    }

    fn string_bytes(strings: &[&String]) -> Result<usize, &'static str> {
        let mut bytes = 0usize;
        for value in strings {
            bytes = bytes.checked_add(value.capacity()).ok_or("process3d-owner.bytes-overflow")?;
        }
        Ok(bytes)
    }

    pub(super) fn machine(&mut self, machine: &WorkshopMachine) -> Result<(), &'static str> {
        let mut items = 4usize;
        let mut bytes = Self::string_bytes(&[&machine.id, &machine.label, &machine.icon_id])?;
        if let Some(catalog_id) = &machine.catalog_id {
            items += 1;
            bytes = bytes.checked_add(catalog_id.capacity()).ok_or("process3d-owner.bytes-overflow")?;
        }
        bytes = bytes.checked_add(machine.capabilities.capacity().saturating_mul(size_of::<Capability>())).ok_or("process3d-owner.bytes-overflow")?;
        for capability in &machine.capabilities {
            items = items.checked_add(4 + capability.parameters.len() * 3 + capability.rules.len() * 2).ok_or("process3d-owner.items-overflow")?;
            bytes = bytes.checked_add(Self::string_bytes(&[&capability.id, &capability.label, &capability.icon_id])?).ok_or("process3d-owner.bytes-overflow")?;
            bytes = bytes.checked_add(capability.parameters.capacity().saturating_mul(size_of::<CapabilityParameter>())).ok_or("process3d-owner.bytes-overflow")?;
            bytes = bytes.checked_add(capability.rules.capacity().saturating_mul(size_of::<CapabilityRule>())).ok_or("process3d-owner.bytes-overflow")?;
            for parameter in &capability.parameters {
                bytes = bytes.checked_add(Self::string_bytes(&[&parameter.id, &parameter.label])?).ok_or("process3d-owner.bytes-overflow")?;
            }
            for rule in &capability.rules {
                let parameter = match rule {
                    CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. } => parameter,
                };
                bytes = bytes.checked_add(parameter.capacity()).ok_or("process3d-owner.bytes-overflow")?;
            }
        }
        self.totals.admit(items, bytes, 8)
    }

    pub(super) fn step(&mut self, source: &Process3dSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(false);
        }
        match self.phase {
            Process3dCensusPhase::Root => {
                let bytes = Self::string_bytes(&[&source.stock_id, &source.stock_label])?
                    .checked_add(source.workshop.machines.capacity().saturating_mul(size_of::<WorkshopMachine>()))
                    .and_then(|value| value.checked_add(source.step_payloads.capacity().saturating_mul(size_of::<ProcessStep>())))
                    .and_then(|value| value.checked_add(source.tool_solids.capacity().saturating_mul(size_of_val(&process3d_empty_child::<semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot>()))))
                    .ok_or("process3d-owner.bytes-overflow")?;
                self.totals.admit(10, bytes, 4)?;
                self.totals.output_pages = PROCESS3D_MAXIMUM_OUTPUT_PAGES;
                self.totals.controls = PROCESS3D_RETAINED_STACK_CAPACITY;
                self.phase = Process3dCensusPhase::Machines(0);
            }
            Process3dCensusPhase::Machines(index) => {
                if let Some(machine) = source.workshop.machines.get(index) {
                    self.machine(machine)?;
                    self.phase = Process3dCensusPhase::Machines(index + 1);
                } else {
                    self.phase = Process3dCensusPhase::Steps(0);
                }
            }
            Process3dCensusPhase::Steps(index) => {
                if let Some(step) = source.step_payloads.get(index) {
                    let mut bytes = Self::string_bytes(&[&step.id, &step.label])?;
                    if let Some(origin) = &step.origin {
                        bytes = bytes.checked_add(Self::string_bytes(&[&origin.machine_id, &origin.capability_id])?).ok_or("process3d-owner.bytes-overflow")?;
                    }
                    self.totals.admit(6, bytes, 7)?;
                    self.phase = Process3dCensusPhase::Steps(index + 1);
                } else {
                    self.phase = Process3dCensusPhase::Tools(0);
                }
            }
            Process3dCensusPhase::Tools(index) => {
                if let Some(child) = source.tool_solids.get(index) {
                    let bytes = Self::string_bytes(&[&child.child_id, &child.target.artifact_id, &child.target.dialect.artifact_kind, &child.target.dialect.standard, &child.target.dialect.subset])?;
                    self.totals.admit(5, bytes, 5)?;
                    self.phase = Process3dCensusPhase::Tools(index + 1);
                } else {
                    self.phase = Process3dCensusPhase::Complete;
                }
            }
            Process3dCensusPhase::Complete => return Ok(true),
        }
        cx.consume_fuel(1);
        Ok(self.phase == Process3dCensusPhase::Complete)
    }
}

pub(super) struct Process3dSnapshotCopyCursor {
    pub(super) phase: Process3dCopyPhase,
    pub(super) machine_capacity: usize,
    pub(super) candidate: std::mem::ManuallyDrop<Option<Process3dSnapshot>>,
    pub(super) retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub(super) terminal_handoff: bool,
}

impl Process3dSnapshotCopyCursor {
    pub(super) fn new(machine_capacity: usize) -> Self {
        Self { phase: Process3dCopyPhase::Shell, machine_capacity, candidate: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None), terminal_handoff: false }
    }

    pub(super) fn step(&mut self, source: &Process3dSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(false);
        }
        match self.phase {
            Process3dCopyPhase::Shell => {
                let mut machines = Vec::with_capacity(self.machine_capacity);
                let step_payloads = Vec::with_capacity(source.step_payloads.len());
                let tool_solids = Vec::with_capacity(source.tool_solids.len());
                if machines.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS || step_payloads.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS || tool_solids.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                    return Err("process3d-owner.clone-observed-capacity");
                }
                machines.clear();
                *self.candidate = Some(Process3dSnapshot {
                    workshop: semio_s_artifact_process_process3d_model::snapshot::Workshop { machines },
                    stock_id: process3d_copy_string(&source.stock_id)?,
                    stock_label: process3d_copy_string(&source.stock_label)?,
                    stock_pose: process3d_copy_pose(&source.stock_pose),
                    stock_payload: process3d_copy_stock(&source.stock_payload)?,
                    stock_solid: process3d_copy_child(&source.stock_solid)?,
                    steps: process3d_copy_child(&source.steps)?,
                    step_payloads,
                    tool_solids,
                });
                self.phase = Process3dCopyPhase::Machines(0);
            }
            Process3dCopyPhase::Machines(index) => {
                if let Some(machine) = source.workshop.machines.get(index) {
                    self.candidate.as_mut().expect("Process3d clone shell remains retained").workshop.machines.push(process3d_copy_machine(machine)?);
                    self.phase = Process3dCopyPhase::Machines(index + 1);
                } else {
                    self.phase = Process3dCopyPhase::Steps(0);
                }
            }
            Process3dCopyPhase::Steps(index) => {
                if let Some(step) = source.step_payloads.get(index) {
                    self.candidate.as_mut().expect("Process3d clone shell remains retained").step_payloads.push(process3d_copy_step(step)?);
                    self.phase = Process3dCopyPhase::Steps(index + 1);
                } else {
                    self.phase = Process3dCopyPhase::Tools(0);
                }
            }
            Process3dCopyPhase::Tools(index) => {
                if let Some(child) = source.tool_solids.get(index) {
                    self.candidate.as_mut().expect("Process3d clone shell remains retained").tool_solids.push(process3d_copy_child(child)?);
                    self.phase = Process3dCopyPhase::Tools(index + 1);
                } else {
                    self.phase = Process3dCopyPhase::Complete;
                }
            }
            Process3dCopyPhase::Complete => return Ok(true),
        }
        cx.consume_fuel(1);
        Ok(self.phase == Process3dCopyPhase::Complete)
    }

    pub(super) fn take(&mut self) -> Option<Process3dSnapshot> {
        if self.phase != Process3dCopyPhase::Complete || self.terminal_handoff {
            return None;
        }
        let value = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(value)
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(retirement) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(retirement, body);
        }
        store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.candidate)
    }

    pub(super) fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.retirement.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant)?;
            if self.retirement.is_none() {
                self.terminal_handoff = true;
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.candidate.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.candidate, &mut self.retirement, grant);
        }
        self.terminal_handoff = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.terminal_handoff && self.candidate.is_none() && self.retirement.is_none()
    }
}

impl Drop for Process3dSnapshotCopyCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d snapshot copy reached Drop before candidate handoff or retained close");
    }
}

pub(super) fn process3d_fault_detail(code: &[u8]) -> Vec<u8> {
    assert!(code.len() <= PROCESS3D_OWNER_BYTES, "Process3d fault detail exceeds its fixed admission");
    let mut detail = Vec::new();
    detail.try_reserve_exact(code.len()).expect("Process3d fixed fault detail was pre-admitted");
    detail.extend_from_slice(code);
    detail
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Process3dTimelineOwner {
    pub(super) steps: Vec<ProcessStep>,
    tools: Vec<store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot>>,
    flow: Option<store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot>>,
    pub(super) phase: u8,
}

impl Process3dTimelineOwner {
    pub(super) fn empty() -> Self {
        Self { steps: Vec::new(), tools: Vec::new(), flow: None, phase: 0 }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Process3dTimelinePhase {
    Scan(usize),
    Allocate,
    Copy(usize),
    Tools(usize),
    Reference { index: usize, offset: usize },
    Edges(usize),
    Nodes { index: usize, tool: usize },
    Complete,
    Unchanged,
    Write,
    Hash(usize),
}

pub(super) struct Process3dTimelineCursor {
    pub(super) phase: Process3dTimelinePhase,
    found: Option<usize>,
    pub(super) length: usize,
    pub(super) payload: Process3dTimelineOwner,
    hash: framework_hash::Sha256,
    reference_hash: Option<framework_hash::Sha256>,
    writer: Option<semio_framework_pack_json::JsonWriteCursor<semio_framework_value::DslValue>>,
    output: Option<String>,
    resume: Option<Process3dTimelinePhase>,
    pending: std::mem::ManuallyDrop<Option<Process3dReleased>>,
    pub(super) retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl Process3dTimelineCursor {
    pub(super) fn new() -> Self {
        Self { phase: Process3dTimelinePhase::Scan(0), found: None, length: 0, payload: Process3dTimelineOwner::empty(), hash: framework_hash::Sha256::new(), reference_hash: None, writer: None, output: None, resume: None, pending: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    pub(super) fn target(mutation: &Process3dMutation) -> Option<&str> {
        use Process3dMutation::*;
        Some(match mutation {
            CreateStep(value) => &value.step.id,
            DeleteStep(value) => &value.id,
            RenameStep(value) => &value.id,
            ChangeStepEnabled(value) => &value.id,
            ChangeStepOrigin(value) => &value.id,
            ReplaceStepMeasure(value) => &value.id,
            ReorderSteps(value) => &value.id,
            _ => return None,
        })
    }

    fn source_index(&self, mutation: &Process3dMutation, index: usize, old_length: usize) -> Option<usize> {
        use Process3dMutation::*;
        match mutation {
            CreateStep(value) => {
                let target = value.index.min(old_length);
                if index == target { None } else { Some(index - usize::from(index > target)) }
            }
            DeleteStep(_) => Some(index + usize::from(index >= self.found.expect("retained delete target"))),
            ReorderSteps(value) => {
                let from = self.found.expect("retained reorder target");
                let to = value.to_index.min(old_length - 1);
                Some(if index == to { from } else if from < to && index >= from && index < to { index + 1 } else if to < from && index > to && index <= from { index - 1 } else { index })
            }
            _ => Some(index),
        }
    }

    pub(super) fn step(&mut self, source: &Process3dSnapshot, mutation: &Process3dMutation, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        use Process3dMutation::*;
        if self.retirement.is_some() || self.pending.is_some() {
            let grant = cx.retained_grant();
            let step = if self.retirement.is_some() {
                store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|_| "process3d-store.timeline-json-retirement")?
            } else {
                store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending, &mut self.retirement, grant).map_err(|_| "process3d-store.timeline-json-retirement-admission")?
            };
            cx.consume_retained(step.progress()).map_err(|_| "process3d-store.timeline-json-retirement-grant")?;
            return Ok(false);
        }
        match self.phase {
            Process3dTimelinePhase::Scan(index) => {
                if let Some(step) = source.step_payloads.get(index) {
                    if Some(step.id.as_str()) == Self::target(mutation) {
                        if self.found.is_some() { return Err("process3d-store.timeline-duplicate"); }
                        self.found = Some(index);
                    }
                    self.phase = Process3dTimelinePhase::Scan(index + 1);
                } else {
                    if matches!(mutation, CreateStep(_)) && self.found.is_some() { return Err("process3d-store.duplicate-step"); }
                    if !matches!(mutation, CreateStep(_)) && self.found.is_none() {
                        self.phase = Process3dTimelinePhase::Unchanged;
                        return Ok(true);
                    }
                    if let Some(found) = self.found {
                        let step = &source.step_payloads[found];
                        let unchanged = match mutation {
                            RenameStep(value) => step.label == value.new_label,
                            ChangeStepEnabled(value) => step.enabled == value.new_enabled,
                            ChangeStepOrigin(value) => step.origin == value.new_origin,
                            ReplaceStepMeasure(value) => step.measure == value.new_measure,
                            ReorderSteps(value) => found == value.to_index,
                            _ => false,
                        };
                        if unchanged {
                            self.phase = Process3dTimelinePhase::Unchanged;
                            return Ok(true);
                        }
                    }
                    self.length = source.step_payloads.len() + usize::from(matches!(mutation, CreateStep(_))) - usize::from(matches!(mutation, DeleteStep(_)));
                    if self.length > PROCESS3D_MAXIMUM_DOMAIN_ITEMS { return Err("process3d-store.timeline-capacity"); }
                    self.phase = Process3dTimelinePhase::Allocate;
                }
            }
            Process3dTimelinePhase::Allocate => {
                let extent = self.length.checked_mul(size_of::<ProcessStep>() + size_of::<store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot>>()).ok_or("process3d-store.timeline-allocation-overflow")?;
                if extent > PROCESS3D_MAXIMUM_DOMAIN_BYTES { return Err("process3d-store.timeline-bytes"); }
                self.payload.steps = Vec::with_capacity(self.length);
                self.payload.tools = Vec::with_capacity(self.length);
                if self.payload.steps.capacity().saturating_mul(size_of::<ProcessStep>()) + self.payload.tools.capacity().saturating_mul(size_of::<store::os_store::artifact_child::ArtifactChild<semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot>>()) > PROCESS3D_MAXIMUM_DOMAIN_BYTES { return Err("process3d-store.timeline-bytes"); }
                self.phase = Process3dTimelinePhase::Copy(0);
            }
            Process3dTimelinePhase::Copy(index) => {
                if index == self.length {
                    self.phase = Process3dTimelinePhase::Tools(0);
                } else {
                    let origin = self.source_index(mutation, index, source.step_payloads.len());
                    let mut step = match origin {
                        Some(origin) => process3d_copy_step(&source.step_payloads[origin])?,
                        None => match mutation { CreateStep(value) => process3d_copy_step(&value.step)?, _ => return Err("process3d-store.timeline-source") },
                    };
                    if origin == self.found && origin.is_some() {
                        match mutation {
                            RenameStep(value) => step.label = process3d_copy_string(&value.new_label)?,
                            ChangeStepEnabled(value) => step.enabled = value.new_enabled,
                            ChangeStepOrigin(value) => step.origin = value.new_origin.as_ref().map(process3d_copy_origin).transpose()?,
                            ReplaceStepMeasure(value) => step.measure = process3d_copy_measure(&value.new_measure)?,
                            _ => {}
                        }
                    }
                    self.payload.steps.push(step);
                    self.phase = Process3dTimelinePhase::Copy(index + 1);
                }
            }
            Process3dTimelinePhase::Tools(index) => {
                let Some(step) = self.payload.steps.get(index) else {
                    self.hash.update(b"{\"schema\":");
                    self.hash.update(semio_framework_pack_json::to_json_string(&semio_s_artifact_stdio_semio_flow::schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.to_string()).as_bytes());
                    self.hash.update(b",\"nodes\":[");
                    self.phase = Process3dTimelinePhase::Nodes { index: 0, tool: 0 };
                    return Ok(false);
                };
                let solid = match &step.measure { ProcessMeasure::Cut { tool, .. } => Some(tool), ProcessMeasure::Attach { component, .. } => Some(component), ProcessMeasure::Drill { .. } => None };
                match solid {
                    Some(WorkingSolid::Reference { .. }) => {
                        self.reference_hash = Some(framework_hash::Sha256::new());
                        self.phase = Process3dTimelinePhase::Reference { index, offset: 0 };
                    }
                    Some(solid) => {
                        self.payload.tools.push(semio_s_artifact_process_process3d_model::snapshot::working_solid_child_handle(&format!("tool-{}", step.id), solid));
                        self.phase = Process3dTimelinePhase::Tools(index + 1);
                    }
                    None => self.phase = Process3dTimelinePhase::Tools(index + 1),
                }
            }
            Process3dTimelinePhase::Reference { index, offset } => {
                let step = &self.payload.steps[index];
                let reference_id = match &step.measure { ProcessMeasure::Cut { tool: WorkingSolid::Reference { reference_id }, .. } | ProcessMeasure::Attach { component: WorkingSolid::Reference { reference_id }, .. } => reference_id, _ => return Err("process3d-store.reference-owner") };
                let bytes = semio_s_artifact_process_process3d_model::snapshot::reference_solid(reference_id).map_or(reference_id.as_bytes(), |reference| reference.step.as_bytes());
                let end = offset.saturating_add(4_096).min(bytes.len());
                self.reference_hash.as_mut().expect("retained reference hash").update(&bytes[offset..end]);
                if end == bytes.len() {
                    let digest = self.reference_hash.take().expect("retained reference digest").finalize();
                    let id = format!("tool-{}-brep-{}", step.id, framework_hash::hex_lower(&digest[..8]));
                    self.payload.tools.push(Self::child(&id, "brep"));
                    self.phase = Process3dTimelinePhase::Tools(index + 1);
                } else {
                    self.phase = Process3dTimelinePhase::Reference { index, offset: end };
                }
            }
            Process3dTimelinePhase::Edges(index) => {
                if index + 1 < self.payload.steps.len() {
                    if index > 0 { self.hash.update(b","); }
                    let left = &self.payload.steps[index];
                    let right = &self.payload.steps[index + 1];
                    let edge = semio_s_artifact_stdio_semio_flow::schema::snapshot::FlowEdge { id: format!("e-{}-{}", left.id, right.id), from: semio_s_artifact_stdio_semio_flow::schema::snapshot::PortRef { node: process3d_copy_string(&left.id)?, port: "out".into() }, to: semio_s_artifact_stdio_semio_flow::schema::snapshot::PortRef { node: process3d_copy_string(&right.id)?, port: "in".into() }, kind: "sequence".into() };
                    self.write_json(&edge, Process3dTimelinePhase::Edges(index + 1));
                } else {
                    self.hash.update(b"]}");
                    let digest = self.hash.clone().finalize();
                    self.payload.flow = Some(Self::child(&format!("steps-flow-{}", framework_hash::hex_lower(&digest[..8])), "flow"));
                    self.phase = Process3dTimelinePhase::Complete;
                }
            }
            Process3dTimelinePhase::Nodes { index, tool } => {
                if let Some(step) = self.payload.steps.get(index) {
                    if index > 0 { self.hash.update(b","); }
                    let has_tool = !matches!(step.measure, ProcessMeasure::Drill { .. });
                    let child = if has_tool { Some(self.payload.tools[tool].child_id.as_str()) } else { None };
                    let node = semio_s_artifact_process_process3d_model::snapshot::flow_node_from_process_step(step, index, child);
                    self.write_json(&node, Process3dTimelinePhase::Nodes { index: index + 1, tool: tool + usize::from(has_tool) });
                } else {
                    self.hash.update(b"],\"edges\":[");
                    self.phase = Process3dTimelinePhase::Edges(0);
                }
            }
            Process3dTimelinePhase::Write => {
                let mut accepted = |_| true;
                let mut control = semio_framework_value::NativeEncodeControl::new(PROCESS3D_MAXIMUM_DOMAIN_BYTES, &mut accepted);
                let writer = self.writer.as_mut().expect("retained timeline JSON writer");
                let stepped = writer.step(256, &mut control, cx.retained_grant());
                let progress = writer.normal_step_progress();
                cx.consume_retained(progress).map_err(|_| "process3d-store.timeline-json-receipt")?;
                if let Some(output) = stepped.map_err(|_| "process3d-store.timeline-json")? {
                    self.output = Some(output);
                    *self.pending = Some(Process3dReleased::Json(self.writer.take().expect("completed timeline JSON writer")));
                    self.phase = Process3dTimelinePhase::Hash(0);
                }
            }
            Process3dTimelinePhase::Hash(offset) => {
                let bytes = self.output.as_ref().expect("retained timeline JSON output").as_bytes();
                let end = offset.saturating_add(4_096).min(bytes.len());
                self.hash.update(&bytes[offset..end]);
                if end == bytes.len() {
                    *self.pending = Some(Process3dReleased::String(self.output.take().expect("hashed timeline JSON output")));
                    self.phase = self.resume.take().expect("retained timeline continuation");
                } else {
                    self.phase = Process3dTimelinePhase::Hash(end);
                }
            }
            Process3dTimelinePhase::Complete | Process3dTimelinePhase::Unchanged => return Ok(true),
        }
        Ok(matches!(self.phase, Process3dTimelinePhase::Complete | Process3dTimelinePhase::Unchanged))
    }

    fn write_json<T: semio_framework_value::ToValue>(&mut self, value: &T, resume: Process3dTimelinePhase) {
        self.writer = Some(semio_framework_pack_json::JsonWriteCursor::new(semio_framework_value::ToValue::to_value(value)));
        self.resume = Some(resume);
        self.phase = Process3dTimelinePhase::Write;
    }

    pub(super) fn child<S>(id: &str, subset: &str) -> store::os_store::artifact_child::ArtifactChild<S> {
        store::os_store::artifact_child::ArtifactChild::new(id.into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: id.into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() } })
    }

    pub(super) fn commit(&mut self, snapshot: &mut Process3dSnapshot) -> Option<Process3dReleased> {
        if self.phase == Process3dTimelinePhase::Unchanged { return None; }
        assert!(self.phase == Process3dTimelinePhase::Complete);
        let old = Process3dTimelineOwner {
            steps: std::mem::replace(&mut snapshot.step_payloads, std::mem::take(&mut self.payload.steps)),
            tools: std::mem::replace(&mut snapshot.tool_solids, std::mem::take(&mut self.payload.tools)),
            flow: Some(std::mem::replace(&mut snapshot.steps, self.payload.flow.take().expect("complete timeline flow"))),
            phase: 0,
        };
        Some(Process3dReleased::Timeline(old))
    }

    fn payload_is_empty(&self) -> bool {
        self.payload.steps.is_empty() && self.payload.steps.capacity() == 0 && self.payload.tools.is_empty() && self.payload.tools.capacity() == 0 && self.payload.flow.is_none()
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.payload_is_empty() && self.writer.is_none() && self.output.is_none() && self.pending.is_none() && self.retirement.is_none()
    }

    /// 📏️ Quotes the one staged original the next close turn retires.
    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(retirement) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(retirement, body);
        }
        if self.pending.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending);
        }
        if self.writer.is_some() || self.output.is_some() || !self.payload_is_empty() {
            return Ok(semio_framework_value::RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<Process3dReleased>(), depth: 2, ..Default::default() });
        }
        Ok(Default::default())
    }

    pub(super) fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.retirement.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.pending.is_none() {
            if let Some(writer) = self.writer.take() {
                *self.pending = Some(Process3dReleased::Json(writer));
            } else if let Some(output) = self.output.take() {
                *self.pending = Some(Process3dReleased::String(output));
            } else if !self.payload_is_empty() {
                *self.pending = Some(Process3dReleased::Timeline(std::mem::replace(&mut self.payload, Process3dTimelineOwner::empty())));
            }
        }
        if self.pending.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending, &mut self.retirement, grant);
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
}

impl Drop for Process3dTimelineCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d timeline reached Drop before handoff or retained retirement");
    }
}

pub(super) fn process3d_apply_retained_mutation(snapshot: &mut Process3dSnapshot, mutation: &Process3dMutation) -> Result<Option<Process3dReleased>, &'static str> {
    use Process3dMutation::*;
    let retired = match mutation {
        CreateStep(_) | DeleteStep(_) | RenameStep(_) | ChangeStepEnabled(_) | ChangeStepOrigin(_) | ReplaceStepMeasure(_) | ReorderSteps(_) => return Err("process3d-store.timeline-requires-cursor"),
        CreateMachine(value) => {
            if snapshot.workshop.machines.iter().any(|machine| machine.id == value.machine.id) {
                return Err("process3d-store.duplicate-machine");
            }
            if snapshot.workshop.machines.len() == snapshot.workshop.machines.capacity() {
                return Err("process3d-store.machine-capacity");
            }
            snapshot.workshop.machines.push(process3d_copy_machine(&value.machine)?);
            None
        }
        DeleteMachine(value) => {
            let index = snapshot.workshop.machines.iter().position(|machine| machine.id == value.id).ok_or("process3d-store.machine-missing")?;
            let old = snapshot.workshop.machines.remove(index);
            Some(Process3dReleased::Machine(old))
        }
        RenameMachine(value) => {
            let machine = snapshot.workshop.machines.iter_mut().find(|machine| machine.id == value.id).ok_or("process3d-store.machine-missing")?;
            let old = std::mem::replace(&mut machine.label, process3d_copy_string(&value.new_label)?);
            Some(Process3dReleased::String(old))
        }
        ChangeMachineIcon(value) => {
            let machine = snapshot.workshop.machines.iter_mut().find(|machine| machine.id == value.id).ok_or("process3d-store.machine-missing")?;
            let old = std::mem::replace(&mut machine.icon_id, process3d_copy_string(&value.new_icon_id)?);
            Some(Process3dReleased::String(old))
        }
        ReplaceMachineCapabilities(value) => {
            let machine = snapshot.workshop.machines.iter_mut().find(|machine| machine.id == value.id).ok_or("process3d-store.machine-missing")?;
            let mut next = Vec::with_capacity(value.new_capabilities.len());
            if next.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                return Err("process3d-store.capability-capacity");
            }
            for capability in &value.new_capabilities {
                next.push(process3d_copy_capability(capability)?);
            }
            let old = std::mem::replace(&mut machine.capabilities, next);
            Some(Process3dReleased::Capabilities(old))
        }
        MoveStock(value) => {
            if !value.new_pose.position.iter().chain(value.new_pose.axis.iter()).chain(std::iter::once(&value.new_pose.angle)).all(|scalar| scalar.is_finite()) {
                return Err("process3d-store.stock-pose-nonfinite");
            }
            snapshot.stock_pose = process3d_copy_pose(&value.new_pose);
            None
        }
        ChangeStockLabel(value) => {
            let old = std::mem::replace(&mut snapshot.stock_label, process3d_copy_string(&value.new_label)?);
            Some(Process3dReleased::String(old))
        }
        ReplaceStockSolid(value) => {
            let old = std::mem::replace(&mut snapshot.stock_solid, process3d_copy_child(&value.new_solid)?);
            Some(Process3dReleased::Child(old))
        }
    };
    Ok(retired)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Process3dOwnerTotals {
    pub items: usize,
    pub bytes: usize,
    pub output_pages: usize,
    pub controls: usize,
    pub combined_depth: usize,
}

impl Process3dOwnerTotals {
    pub(super) fn admit(&mut self, items: usize, bytes: usize, depth: usize) -> Result<(), &'static str> {
        self.items = self.items.checked_add(items).ok_or("process3d-owner.items-overflow")?;
        self.bytes = self.bytes.checked_add(bytes).ok_or("process3d-owner.bytes-overflow")?;
        self.combined_depth = self.combined_depth.max(depth);
        if self.items > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
            return Err("process3d-owner.items-capacity");
        }
        if self.bytes > PROCESS3D_MAXIMUM_DOMAIN_BYTES {
            return Err("process3d-owner.bytes-capacity");
        }
        if self.combined_depth >= PROCESS3D_RETAINED_STACK_CAPACITY {
            return Err("process3d-owner.combined-depth");
        }
        Ok(())
    }
}

fn process3d_empty_child<S>() -> store::os_store::artifact_child::ArtifactChild<S> {
    store::os_store::artifact_child::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } })
}

fn process3d_copy_pose(source: &Pose) -> Pose {
    Pose { position: source.position, axis: source.axis, angle: source.angle }
}

fn process3d_copy_solid(source: &WorkingSolid) -> Result<WorkingSolid, &'static str> {
    Ok(match source {
        WorkingSolid::Box { width, depth, height } => WorkingSolid::Box { width: *width, depth: *depth, height: *height },
        WorkingSolid::Cylinder { radius, height } => WorkingSolid::Cylinder { radius: *radius, height: *height },
        WorkingSolid::Sphere { radius } => WorkingSolid::Sphere { radius: *radius },
        WorkingSolid::ImportedMesh { mesh_url } => WorkingSolid::ImportedMesh { mesh_url: process3d_copy_string(mesh_url)? },
        WorkingSolid::ImportedSolid { solid_handle } => WorkingSolid::ImportedSolid { solid_handle: process3d_copy_string(solid_handle)? },
        WorkingSolid::Reference { reference_id } => WorkingSolid::Reference { reference_id: process3d_copy_string(reference_id)? },
    })
}

fn process3d_copy_origin(source: &StepOrigin) -> Result<StepOrigin, &'static str> {
    Ok(StepOrigin { machine_id: process3d_copy_string(&source.machine_id)?, capability_id: process3d_copy_string(&source.capability_id)? })
}

fn process3d_copy_step(source: &ProcessStep) -> Result<ProcessStep, &'static str> {
    Ok(ProcessStep {
        id: process3d_copy_string(&source.id)?,
        label: process3d_copy_string(&source.label)?,
        enabled: source.enabled,
        origin: source.origin.as_ref().map(process3d_copy_origin).transpose()?,
        measure: process3d_copy_measure(&source.measure)?,
    })
}

fn process3d_copy_recipe(source: &MeasureRecipe) -> Result<MeasureRecipe, &'static str> {
    Ok(match source {
        MeasureRecipe::DiscCut { diameter, kerf } => MeasureRecipe::DiscCut { diameter: process3d_copy_string(diameter)?, kerf: process3d_copy_string(kerf)? },
        MeasureRecipe::BladeCut { kerf, length, depth } => MeasureRecipe::BladeCut { kerf: process3d_copy_string(kerf)?, length: process3d_copy_string(length)?, depth: process3d_copy_string(depth)? },
        MeasureRecipe::PocketCut { diameter, depth } => MeasureRecipe::PocketCut { diameter: process3d_copy_string(diameter)?, depth: process3d_copy_string(depth)? },
        MeasureRecipe::BoreDrill { radius, depth } => MeasureRecipe::BoreDrill { radius: process3d_copy_string(radius)?, depth: process3d_copy_string(depth)? },
        MeasureRecipe::CylinderAttach { radius, length } => MeasureRecipe::CylinderAttach { radius: process3d_copy_string(radius)?, length: process3d_copy_string(length)? },
        MeasureRecipe::BoxAttach { width, depth, height } => MeasureRecipe::BoxAttach { width: process3d_copy_string(width)?, depth: process3d_copy_string(depth)?, height: process3d_copy_string(height)? },
    })
}

fn process3d_copy_capability(source: &Capability) -> Result<Capability, &'static str> {
    let mut parameters = Vec::with_capacity(source.parameters.len());
    if parameters.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        return Err("process3d-owner.parameter-observed-capacity");
    }
    for parameter in &source.parameters {
        parameters.push(CapabilityParameter { id: process3d_copy_string(&parameter.id)?, label: process3d_copy_string(&parameter.label)?, value: parameter.value });
    }
    let mut rules = Vec::with_capacity(source.rules.len());
    if rules.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        return Err("process3d-owner.rule-observed-capacity");
    }
    for rule in &source.rules {
        rules.push(match rule {
            CapabilityRule::Min { quantity, parameter, margin } => CapabilityRule::Min { quantity: *quantity, parameter: process3d_copy_string(parameter)?, margin: *margin },
            CapabilityRule::Max { quantity, parameter, margin } => CapabilityRule::Max { quantity: *quantity, parameter: process3d_copy_string(parameter)?, margin: *margin },
        });
    }
    Ok(Capability { id: process3d_copy_string(&source.id)?, label: process3d_copy_string(&source.label)?, icon_id: process3d_copy_string(&source.icon_id)?, recipe: process3d_copy_recipe(&source.recipe)?, parameters, rules })
}

fn process3d_copy_machine(source: &WorkshopMachine) -> Result<WorkshopMachine, &'static str> {
    let mut capabilities = Vec::with_capacity(source.capabilities.len());
    if capabilities.capacity() > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        return Err("process3d-owner.capability-observed-capacity");
    }
    for capability in &source.capabilities {
        capabilities.push(process3d_copy_capability(capability)?);
    }
    Ok(WorkshopMachine {
        id: process3d_copy_string(&source.id)?,
        label: process3d_copy_string(&source.label)?,
        icon_id: process3d_copy_string(&source.icon_id)?,
        catalog_id: source.catalog_id.as_deref().map(process3d_copy_string).transpose()?,
        capabilities,
    })
}

fn process3d_copy_stock(source: &Stock) -> Result<Stock, &'static str> {
    Ok(Stock { id: process3d_copy_string(&source.id)?, label: process3d_copy_string(&source.label)?, solid: process3d_copy_solid(&source.solid)?, pose: process3d_copy_pose(&source.pose) })
}

fn process3d_copy_child<S>(source: &store::os_store::artifact_child::ArtifactChild<S>) -> Result<store::os_store::artifact_child::ArtifactChild<S>, &'static str> {
    Ok(store::os_store::artifact_child::ArtifactChild::new(
        process3d_copy_string(&source.child_id)?,
        semio_framework_artifact_reference::ArtifactRef {
            artifact_id: process3d_copy_string(&source.target.artifact_id)?,
            dialect: semio_framework_artifact_reference::ArtifactDialect {
                artifact_kind: process3d_copy_string(&source.target.dialect.artifact_kind)?,
                standard: process3d_copy_string(&source.target.dialect.standard)?,
                subset: process3d_copy_string(&source.target.dialect.subset)?,
            },
        },
    ))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dCensusPhase {
    Root,
    Machines(usize),
    Steps(usize),
    Tools(usize),
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dCopyPhase {
    Shell,
    Machines(usize),
    Steps(usize),
    Tools(usize),
    Complete,
}

fn process3d_copy_measure(source: &ProcessMeasure) -> Result<ProcessMeasure, &'static str> {
    Ok(match source {
        ProcessMeasure::Cut { tool, pose } => ProcessMeasure::Cut { tool: process3d_copy_solid(tool)?, pose: process3d_copy_pose(pose) },
        ProcessMeasure::Drill { radius, depth, pose } => ProcessMeasure::Drill { radius: *radius, depth: *depth, pose: process3d_copy_pose(pose) },
        ProcessMeasure::Attach { component, pose } => ProcessMeasure::Attach { component: process3d_copy_solid(component)?, pose: process3d_copy_pose(pose) },
    })
}
