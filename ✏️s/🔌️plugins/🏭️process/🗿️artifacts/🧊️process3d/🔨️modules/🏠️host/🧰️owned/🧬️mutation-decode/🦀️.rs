//! 🧬️ process3d mutation decode: the retained mutation reader, the mutation field decode authority and their state machines.

use super::*;

impl Process3dRetainedMutationReader {
    pub(super) fn new() -> Self {
        Self {
            phase: Process3dRetainedMutationPhase::Header,
            offset: 0,
            tag: 0,
            field: 0,
            index: 0,
            expected: 0,
            strings: [None, None, None],
            string: Default::default(),
            enabled: false,
            origin_tag: 0,
            step: None,
            measure: None,
            pose: None,
            child: None,
            machine: None,
            capability: None,
            capabilities: Vec::new(),
            value: std::mem::ManuallyDrop::new(None),
            terminal_handoff: false,
        }
    }

    pub(super) fn complete(&mut self, bytes: &[u8], value: Process3dMutation) -> Result<(), protocol::ProtocolError> {
        if self.offset != bytes.len() {
            return Err(process3d_protocol_error("retained mutation has trailing bytes".into()));
        }
        *self.value = Some(value);
        self.phase = Process3dRetainedMutationPhase::Complete;
        Ok(())
    }

    fn string_step(&mut self, bytes: &[u8]) -> Result<Option<String>, protocol::ProtocolError> {
        let value = process3d_retained_string_step(bytes, &mut self.offset, &mut self.string)?;
        if value.is_some() {
            self.string = Default::default();
        }
        Ok(value)
    }

    fn structural_step(&mut self, bytes: &[u8]) -> Result<(), protocol::ProtocolError> {
        use semio_s_artifact_process_process3d_mut_machines::{change_machine_icon, create_machine, delete_machine, rename_machine, replace_machine_capabilities};
        use semio_s_artifact_process_process3d_mut_steps::{change_step_enabled, change_step_origin, create_step, delete_step, rename_step, reorder_steps, replace_step_measure};
        use semio_s_artifact_process_process3d_mut_stock::{change_stock_label, move_stock, replace_stock_solid};
        match self.tag {
            0 => {
                if self.field == 0 {
                    self.index = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_varint_u64().map(|value| value as usize).map_err(|error| error.to_string()))?;
                    self.field = 1;
                } else {
                    let cursor = self.step.get_or_insert_with(Default::default);
                    if let Some(step) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                        self.step = None;
                        self.complete(bytes, Process3dMutation::CreateStep(create_step::CreateStep { index: self.index, step }))?;
                    }
                }
            }
            1 | 8 | 13 => {
                let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                self.complete(
                    bytes,
                    match self.tag {
                        1 => Process3dMutation::DeleteStep(delete_step::DeleteStep { id: value }),
                        8 => Process3dMutation::DeleteMachine(delete_machine::DeleteMachine { id: value }),
                        13 => Process3dMutation::ChangeStockLabel(change_stock_label::ChangeStockLabel { new_label: value }),
                        _ => unreachable!("retained simple string tag"),
                    },
                )?;
            }
            2 | 9 | 10 => {
                let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                self.strings[self.field] = Some(value);
                self.field += 1;
                if self.field == 2 {
                    let first = self.strings[0].take().unwrap_or_default();
                    let second = self.strings[1].take().unwrap_or_default();
                    self.complete(
                        bytes,
                        match self.tag {
                            2 => Process3dMutation::RenameStep(rename_step::RenameStep { id: first, new_label: second }),
                            9 => Process3dMutation::RenameMachine(rename_machine::RenameMachine { id: first, new_label: second }),
                            10 => Process3dMutation::ChangeMachineIcon(change_machine_icon::ChangeMachineIcon { id: first, new_icon_id: second }),
                            _ => unreachable!("retained double string tag"),
                        },
                    )?;
                }
            }
            3 => {
                if self.field == 0 {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[0] = Some(value);
                    self.field = 1;
                } else {
                    self.enabled = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_u8().map(|value| value != 0).map_err(|error| error.to_string()))?;
                    let mutation = Process3dMutation::ChangeStepEnabled(change_step_enabled::ChangeStepEnabled { id: self.strings[0].take().unwrap_or_default(), new_enabled: self.enabled });
                    self.complete(bytes, mutation)?;
                }
            }
            4 => match self.field {
                0 => {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[0] = Some(value);
                    self.field = 1;
                }
                1 => {
                    self.origin_tag = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_u8().map_err(|error| error.to_string()))?;
                    match self.origin_tag {
                        0 => {
                            let mutation = Process3dMutation::ChangeStepOrigin(change_step_origin::ChangeStepOrigin { id: self.strings[0].take().unwrap_or_default(), new_origin: None });
                            self.complete(bytes, mutation)?;
                        }
                        1 => self.field = 2,
                        _ => return Err(process3d_protocol_error("retained step origin tag is invalid".into())),
                    }
                }
                2 | 3 => {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[self.field - 1] = Some(value);
                    self.field += 1;
                    if self.field == 4 {
                        let id = self.strings[0].take().unwrap_or_default();
                        let machine_id = self.strings[1].take().unwrap_or_default();
                        let capability_id = self.strings[2].take().unwrap_or_default();
                        let mutation = Process3dMutation::ChangeStepOrigin(change_step_origin::ChangeStepOrigin { id, new_origin: Some(StepOrigin { machine_id, capability_id }) });
                        self.complete(bytes, mutation)?;
                    }
                }
                _ => unreachable!("retained origin field"),
            },
            5 => {
                if self.field == 0 {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[0] = Some(value);
                    self.field = 1;
                } else {
                    let cursor = self.measure.get_or_insert_with(Default::default);
                    if let Some(new_measure) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                        self.measure = None;
                        let mutation = Process3dMutation::ReplaceStepMeasure(replace_step_measure::ReplaceStepMeasure { id: self.strings[0].take().unwrap_or_default(), new_measure });
                        self.complete(bytes, mutation)?;
                    }
                }
            }
            6 => {
                if self.field == 0 {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[0] = Some(value);
                    self.field = 1;
                } else {
                    let to_index = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_varint_u64().map(|value| value as usize).map_err(|error| error.to_string()))?;
                    let mutation = Process3dMutation::ReorderSteps(reorder_steps::ReorderSteps { id: self.strings[0].take().unwrap_or_default(), to_index });
                    self.complete(bytes, mutation)?;
                }
            }
            7 => {
                if self.field == 0 {
                    self.index = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_varint_u64().map(|value| value as usize).map_err(|error| error.to_string()))?;
                    self.field = 1;
                } else {
                    let cursor = self.machine.get_or_insert_with(Default::default);
                    if let Some(machine) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                        self.machine = None;
                        self.complete(bytes, Process3dMutation::CreateMachine(create_machine::CreateMachine { index: self.index, machine }))?;
                    }
                }
            }
            11 => match self.field {
                0 => {
                    let Some(value) = self.string_step(bytes)? else { return Ok(()) };
                    self.strings[0] = Some(value);
                    self.field = 1;
                }
                1 => {
                    self.expected = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_varint_u64().map(|value| value as usize).map_err(|error| error.to_string()))?;
                    if self.expected > PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                        return Err(protocol::ProtocolError::LimitExceeded("process3d retained capability count exceeds fixed catalog"));
                    }
                    self.capabilities.try_reserve_exact(self.expected).map_err(|_| process3d_protocol_error("retained capability admission failed".into()))?;
                    self.field = 2;
                    if self.expected == 0 {
                        let id = self.strings[0].take().unwrap_or_default();
                        let new_capabilities = std::mem::take(&mut self.capabilities);
                        let mutation = Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities { id, new_capabilities });
                        self.complete(bytes, mutation)?;
                    }
                }
                _ => {
                    let cursor = self.capability.get_or_insert_with(Default::default);
                    if let Some(capability) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                        self.capabilities.push(capability);
                        self.capability = None;
                        self.index += 1;
                        if self.index == self.expected {
                            let id = self.strings[0].take().unwrap_or_default();
                            let new_capabilities = std::mem::take(&mut self.capabilities);
                            let mutation = Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities { id, new_capabilities });
                            self.complete(bytes, mutation)?;
                        }
                    }
                }
            },
            12 => {
                let cursor = self.pose.get_or_insert_with(Default::default);
                if let Some(new_pose) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                    self.pose = None;
                    self.complete(bytes, Process3dMutation::MoveStock(move_stock::MoveStock { new_pose }))?;
                }
            }
            14 => {
                let cursor = self.child.get_or_insert_with(Default::default);
                if let Some(new_solid) = process3d_retained_advance(bytes, &mut self.offset, |reader| cursor.step(reader))? {
                    self.child = None;
                    self.complete(bytes, Process3dMutation::ReplaceStockSolid(replace_stock_solid::ReplaceStockSolid { new_solid }))?;
                }
            }
            _ => return Err(process3d_protocol_error("retained mutation tag is invalid".into())),
        }
        Ok(())
    }

    pub(super) fn step(&mut self, bytes: &[u8], cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, protocol::ProtocolError> {
        if cx.should_yield() || cx.fuel_remaining() == 0 || cx.is_cancelled() {
            return Ok(false);
        }
        match self.phase {
            Process3dRetainedMutationPhase::Header => {
                if bytes.first().copied() != Some(PROCESS3D_MUTATION_BINARY_FORMAT) || bytes.len() < 2 || bytes.len() > PROCESS3D_OWNER_BYTES {
                    return Err(process3d_protocol_error("invalid retained mutation header".into()));
                }
                self.offset = 1;
                self.phase = Process3dRetainedMutationPhase::Tag;
            }
            Process3dRetainedMutationPhase::Tag => {
                self.tag = process3d_retained_advance(bytes, &mut self.offset, |reader| reader.read_u8().map_err(|error| error.to_string()))?;
                if self.tag >= PROCESS3D_MUTATION_VARIANT_COUNT as u8 {
                    return Err(process3d_protocol_error("retained mutation tag is invalid".into()));
                }
                self.phase = Process3dRetainedMutationPhase::Structural;
            }
            Process3dRetainedMutationPhase::Structural => {
                self.structural_step(bytes)?;
            }
            Process3dRetainedMutationPhase::Complete => return Ok(true),
        }
        cx.consume_fuel(1);
        Ok(self.phase == Process3dRetainedMutationPhase::Complete)
    }

    pub(super) fn take(&mut self) -> Option<Process3dMutation> {
        if self.phase != Process3dRetainedMutationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let value = self.value.take()?;
        self.terminal_handoff = true;
        Some(value)
    }

    pub(super) fn take_partial(&mut self) -> Option<Process3dMutation> {
        use semio_s_artifact_process_process3d_mut_machines::{change_machine_icon, create_machine, delete_machine, rename_machine, replace_machine_capabilities};
        use semio_s_artifact_process_process3d_mut_steps::{change_step_enabled, change_step_origin, create_step, delete_step, rename_step, reorder_steps, replace_step_measure};
        use semio_s_artifact_process_process3d_mut_stock::{change_stock_label, move_stock, replace_stock_solid};
        if self.string.has_partial() {
            let partial = self.string.take_partial();
            match self.tag {
                1 | 8 | 13 => self.strings[0] = Some(partial),
                2 | 9 | 10 => self.strings[self.field.min(1)] = Some(partial),
                3 | 5 | 6 | 11 if self.field == 0 => self.strings[0] = Some(partial),
                4 if self.field == 0 => self.strings[0] = Some(partial),
                4 if matches!(self.field, 2 | 3) => self.strings[self.field - 1] = Some(partial),
                _ => drop(partial),
            }
            self.string = Default::default();
        }
        let value = match self.tag {
            0 => Some(Process3dMutation::CreateStep(create_step::CreateStep {
                index: self.index,
                step: self
                    .step
                    .take()
                    .map_or(ProcessStep { id: String::new(), label: String::new(), enabled: false, origin: None, measure: ProcessMeasure::Drill { radius: 0.0, depth: 0.0, pose: Default::default() } }, |mut cursor| cursor.take_partial()),
            })),
            1 => Some(Process3dMutation::DeleteStep(delete_step::DeleteStep { id: self.strings[0].take().unwrap_or_default() })),
            2 => Some(Process3dMutation::RenameStep(rename_step::RenameStep { id: self.strings[0].take().unwrap_or_default(), new_label: self.strings[1].take().unwrap_or_default() })),
            3 => Some(Process3dMutation::ChangeStepEnabled(change_step_enabled::ChangeStepEnabled { id: self.strings[0].take().unwrap_or_default(), new_enabled: self.enabled })),
            4 => Some(Process3dMutation::ChangeStepOrigin(change_step_origin::ChangeStepOrigin {
                id: self.strings[0].take().unwrap_or_default(),
                new_origin: (self.origin_tag == 1).then(|| StepOrigin { machine_id: self.strings[1].take().unwrap_or_default(), capability_id: self.strings[2].take().unwrap_or_default() }),
            })),
            5 => Some(Process3dMutation::ReplaceStepMeasure(replace_step_measure::ReplaceStepMeasure {
                id: self.strings[0].take().unwrap_or_default(),
                new_measure: self.measure.take().map_or(ProcessMeasure::Drill { radius: 0.0, depth: 0.0, pose: Default::default() }, |mut cursor| cursor.take_partial()),
            })),
            6 => Some(Process3dMutation::ReorderSteps(reorder_steps::ReorderSteps { id: self.strings[0].take().unwrap_or_default(), to_index: self.index })),
            7 => Some(Process3dMutation::CreateMachine(create_machine::CreateMachine {
                index: self.index,
                machine: self.machine.take().map_or(WorkshopMachine { id: String::new(), label: String::new(), icon_id: String::new(), catalog_id: None, capabilities: Vec::new() }, |mut cursor| cursor.take_partial()),
            })),
            8 => Some(Process3dMutation::DeleteMachine(delete_machine::DeleteMachine { id: self.strings[0].take().unwrap_or_default() })),
            9 => Some(Process3dMutation::RenameMachine(rename_machine::RenameMachine { id: self.strings[0].take().unwrap_or_default(), new_label: self.strings[1].take().unwrap_or_default() })),
            10 => Some(Process3dMutation::ChangeMachineIcon(change_machine_icon::ChangeMachineIcon { id: self.strings[0].take().unwrap_or_default(), new_icon_id: self.strings[1].take().unwrap_or_default() })),
            11 => {
                if let Some(mut cursor) = self.capability.take() {
                    self.capabilities.push(cursor.take_partial());
                }
                Some(Process3dMutation::ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities { id: self.strings[0].take().unwrap_or_default(), new_capabilities: std::mem::take(&mut self.capabilities) }))
            }
            12 => Some(Process3dMutation::MoveStock(move_stock::MoveStock { new_pose: self.pose.take().map(|mut cursor| cursor.take_partial()).unwrap_or_default() })),
            13 => Some(Process3dMutation::ChangeStockLabel(change_stock_label::ChangeStockLabel { new_label: self.strings[0].take().unwrap_or_default() })),
            14 => Some(Process3dMutation::ReplaceStockSolid(replace_stock_solid::ReplaceStockSolid {
                new_solid: self.child.take().map_or_else(
                    || store::os_store::artifact_child::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } }),
                    |mut cursor| cursor.take_partial(),
                ),
            })),
            _ => None,
        };
        for string in &mut self.strings {
            drop(string.take());
        }
        value
    }

    fn take_rejected(&mut self) -> Option<Process3dMutation> {
        if self.terminal_handoff {
            return None;
        }
        let value = self.value.take().or_else(|| self.take_partial());
        self.terminal_handoff = true;
        value
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.terminal_handoff
            && self.value.is_none()
            && self.strings.iter().all(Option::is_none)
            && self.string.terminal_is_empty()
            && self.step.is_none()
            && self.measure.is_none()
            && self.pose.is_none()
            && self.child.is_none()
            && self.machine.is_none()
            && self.capability.is_none()
            && self.capabilities.is_empty()
    }
}

impl Drop for Process3dRetainedMutationReader {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d retained mutation reader reached Drop before handoff");
    }
}

pub(super) struct Process3dMutationDecodeAuthority {
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    pub(super) state: Process3dMutationDecodeState,
    hex: std::mem::ManuallyDrop<Option<store::os_store::owned_schema_decode::OwnedSchemaHexAuthority<PROCESS3D_OWNER_BYTES>>>,
    pub(super) reader: std::mem::ManuallyDrop<Option<Process3dRetainedMutationReader>>,
    pub(super) value: std::mem::ManuallyDrop<Option<Process3dMutation>>,
    pub(super) retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl Process3dMutationDecodeAuthority {
    pub(super) fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: Process3dMutationDecodeState::AwaitToken,
            hex: std::mem::ManuallyDrop::new(None),
            reader: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
        }
    }

    pub(super) fn diagnostic(&self, code: &'static str, offset: u64) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
        store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let native = |error: semio_framework_value::ValueError| self.diagnostic("process3d-envelope.mutation-close-demand", 0).with_native(error);
        if self.reader.is_some() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if self.hex.is_some() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: PROCESS3D_OWNER_BYTES, depth: 1, ..Default::default() });
        }
        if let Some(retirement) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(retirement, body).map_err(native);
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.value).map_err(native);
        }
        Ok(Default::default())
    }
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<Process3dMutation> for Process3dMutationDecodeAuthority {
    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.depth)
    }
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("process3d-envelope.mutation-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("process3d-envelope.mutation-cancelled", token.start));
        }
        if self.state == Process3dMutationDecodeState::AwaitToken {
            if !terminal {
                return Err(self.diagnostic("process3d-envelope.mutation-pack-must-be-scalar", token.start));
            }
            *self.hex = Some(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
            self.state = Process3dMutationDecodeState::Hex;
        }
        if self.state == Process3dMutationDecodeState::Hex {
            return match self.hex.as_mut().expect("Process3d mutation hex owner retained").step(source, cx) {
                store::os_store::owned_schema_decode::OwnedSchemaHexStep::Pending => Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending),
                store::os_store::owned_schema_decode::OwnedSchemaHexStep::Complete => {
                    *self.reader = Some(Process3dRetainedMutationReader::new());
                    self.state = Process3dMutationDecodeState::Structural;
                    Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending)
                }
                store::os_store::owned_schema_decode::OwnedSchemaHexStep::Cancelled => Err(self.diagnostic("process3d-envelope.mutation-pack-cancelled", token.start)),
                store::os_store::owned_schema_decode::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
            };
        }
        if self.state != Process3dMutationDecodeState::Structural {
            return Err(self.diagnostic("process3d-envelope.mutation-token-replayed", token.start));
        }
        let bytes = self.hex.as_ref().and_then(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority::as_bytes).ok_or_else(|| self.diagnostic("process3d-envelope.mutation-backing-missing", token.start))?;
        let complete = self.reader.as_mut().expect("Process3d retained mutation reader exists").step(bytes, cx).map_err(|_| self.diagnostic("process3d-envelope.mutation-structural-malformed", token.start))?;
        if !complete {
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        let value = self.reader.as_mut().expect("Process3d retained mutation reader exists").take().ok_or_else(|| self.diagnostic("process3d-envelope.mutation-handoff-missing", token.start))?;
        drop(self.reader.take());
        if !self.hex.as_mut().expect("Process3d mutation hex owner retained").release() {
            return Err(self.diagnostic("process3d-envelope.mutation-backing-release", token.start));
        }
        drop(self.hex.take());
        *self.value = Some(value);
        self.state = Process3dMutationDecodeState::Ready;
        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldTarget<Process3dMutation>,
        reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if self.state != Process3dMutationDecodeState::Ready {
            return Err(self.diagnostic("process3d-envelope.mutation-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("process3d-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = Process3dMutationDecodeState::Published;
        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(self.diagnostic("process3d-envelope.mutation-close-depth", 0));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(reader) = self.reader.as_mut() {
            if let Some(value) = reader.take_rejected() {
                *self.value = Some(value);
            }
            drop(self.reader.take());
            self.state = Process3dMutationDecodeState::Closing;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(hex) = self.hex.as_mut() {
            hex.cancel();
            drop(self.hex.take());
            self.state = Process3dMutationDecodeState::Closing;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        if self.retirement.is_none() {
            if let Some(value) = self.value.take() {
                let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
                return match semio_framework_value::retirement::admit_owned_retirement(value, child) {
                    Ok((owner, progress)) => {
                        *self.retirement = Some(owner);
                        if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes {
                            return Err(self.diagnostic("process3d-envelope.mutation-retirement-birth", 0).with_progress(progress));
                        }
                        Ok(RetainedCloneStep::Progress(progress))
                    }
                    Err((error, value)) => {
                        *self.value = Some(value);
                        Err(self.diagnostic("process3d-envelope.mutation-retirement-fault", 0).with_native(error))
                    }
                };
            }
            self.state = Process3dMutationDecodeState::Complete;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|error| self.diagnostic("process3d-envelope.mutation-retirement-fault", 0).with_native(error))?;
        if self.retirement.is_none() {
            self.state = Process3dMutationDecodeState::Complete;
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, Process3dMutationDecodeState::Published | Process3dMutationDecodeState::Complete) && self.hex.is_none() && self.reader.is_none() && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for Process3dMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d mutation field reached Drop before publication or terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dRetainedMutationPhase {
    Header,
    Tag,
    Structural,
    Complete,
}

fn process3d_retained_advance<T>(bytes: &[u8], offset: &mut usize, read: impl FnOnce(&mut store::ByteReader<'_>) -> Result<T, String>) -> Result<T, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes.get(*offset..).ok_or_else(|| process3d_protocol_error("retained mutation cursor escaped its backing".into()))?);
    let value = read(&mut reader).map_err(process3d_protocol_error)?;
    *offset = offset.checked_add(reader.position()).ok_or_else(|| process3d_protocol_error("retained mutation cursor overflow".into()))?;
    Ok(value)
}

fn process3d_retained_string_step(bytes: &[u8], offset: &mut usize, cursor: &mut Process3dRetainedStringCursor) -> Result<Option<String>, protocol::ProtocolError> {
    process3d_retained_advance(bytes, offset, |reader| cursor.step(reader))
}

struct Process3dRetainedMutationReader {
    pub(super) phase: Process3dRetainedMutationPhase,
    pub(super) offset: usize,
    pub(super) tag: u8,
    field: usize,
    pub(super) index: usize,
    pub(super) expected: usize,
    pub(super) strings: [Option<String>; 3],
    pub(super) string: Process3dRetainedStringCursor,
    pub(super) enabled: bool,
    origin_tag: u8,
    pub(super) step: Option<Process3dRetainedStepCursor>,
    pub(super) measure: Option<Process3dRetainedMeasureCursor>,
    pub(super) pose: Option<Process3dRetainedPoseCursor>,
    pub(super) child: Option<Process3dRetainedChildCursor>,
    pub(super) machine: Option<Process3dRetainedMachineCursor>,
    pub(super) capability: Option<Process3dRetainedCapabilityCursor>,
    pub(super) capabilities: Vec<Capability>,
    pub(super) value: std::mem::ManuallyDrop<Option<Process3dMutation>>,
    pub(super) terminal_handoff: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dMutationDecodeState {
    AwaitToken,
    Hex,
    Structural,
    Ready,
    Published,
    Closing,
    Complete,
}
