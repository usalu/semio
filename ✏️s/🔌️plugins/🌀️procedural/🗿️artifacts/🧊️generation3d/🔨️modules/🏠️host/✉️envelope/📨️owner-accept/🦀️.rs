//! 📨️ Token intake half of the retained mutation owner: the token dispatch that fills the typed frames, the handoff and the close steps. See [`super`].

use super::*;

//#region 🔖️TypedOwnedEnvelopeCatalog
impl Generation3dRetainedMutationOwner {
    pub(super) fn accept(&mut self, token: store::mounted_pack_rt::RetainedValueToken, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<(), &'static str> {
        use store::mounted_pack_rt::{RetainedValueContainer as Container, RetainedValueRole as Role, RetainedValueToken as Token};
        match token {
            Token::Tag { value: 0x11, .. } => {
                if self.dsl_destination.is_some() {
                    self.begin_dsl();
                } else if self.json_destination.is_none() {
                    if matches!(self.ordinal, 13 | 19) && self.root_field() == Some(2) {
                        self.begin_json(Generation3dMutationJsonDestination::ChangeValue)?;
                    } else {
                        self.begin_dsl();
                    }
                }
            }
            Token::Begin { kind: Container::List, count } if self.dsl_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mutation.dsl-count")?).map_err(|_| "generation3d-mutation.dsl-preflight")?;
                self.dsl_stack.push(Generation3dMutationDslFrame::Array(values));
            }
            Token::Begin { kind: Container::Map, count } if self.dsl_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mutation.dsl-count")?).map_err(|_| "generation3d-mutation.dsl-preflight")?;
                self.dsl_stack.push(Generation3dMutationDslFrame::Object { values, key: None });
            }
            Token::Begin { kind: Container::List, count } if self.json_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mutation.json-list-count")?).map_err(|_| "generation3d-mutation.json-list-preflight")?;
                self.json_stack.push(Generation3dMutationJsonFrame::Array(values));
            }
            Token::Begin { kind: Container::Map, .. } if self.json_destination.is_some() => {
                self.json_stack.push(Generation3dMutationJsonFrame::Object { values: Vec::new(), key: None });
            }
            Token::Begin { kind: Container::Map, .. } => {
                let index = self.stack.len().checked_sub(1).ok_or("generation3d-mutation.generation-values-owner")?;
                if matches!(self.stack.get(index), Some(Generation3dMutationFrame::Generation { field: Some(2), .. })) {
                    self.begin_json(Generation3dMutationJsonDestination::Generation(index))?;
                    self.json_stack.push(Generation3dMutationJsonFrame::Object { values: Vec::new(), key: None });
                } else {
                    self.push(Generation3dMutationFrame::Structural(Container::Map))?;
                }
            }
            Token::Begin { kind: Container::Table, count } => self.accept_begin_table(count)?,
            Token::Begin { kind: Container::Record, .. } => self.begin_record()?,
            Token::Begin { kind: Container::Statements, .. } => self.push(Generation3dMutationFrame::Statements { keyword: None })?,
            Token::Begin { kind: Container::List | Container::Tuple, count } => self.accept_begin_list_or_tuple(count)?,
            Token::Begin { kind: Container::Wire, .. } => {
                let parent = self.stack.len().checked_sub(1).ok_or("generation3d-mutation.wire-parent")?;
                self.push(Generation3dMutationFrame::Wire { parent, roles: [0; 6], roles_len: 0, role: 0, nodes: 0 })?;
            }
            Token::Begin { kind, .. } => self.push(Generation3dMutationFrame::Structural(kind))?,
            Token::Unsigned { role: Role::FieldId, value } if value <= u16::MAX as u64 => self.accept_field_id(value)?,
            Token::Unsigned { role: Role::TableRows, value } => self.pending_table_rows = Some(value),
            Token::Unsigned { role: Role::TableField, value } => if let Some(Generation3dMutationFrame::Dictionary { field, present, next, .. }) = self.stack.last_mut() {
                *field = Some(u16::try_from(value).map_err(|_| "generation3d-mutation.dictionary-field")?);
                present.fill(false);
                *next = 0;
            },
            Token::Unsigned { role: Role::Unsigned, value } if self.json_destination.is_none() && self.dsl_destination.is_none() => {
                self.index = usize::try_from(value).map_err(|_| "generation3d-mutation.index")?;
                self.index_present = true;
                if let Some(Generation3dMutationFrame::Root { field }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Token::Tag { value: 0x06 | 0x07, .. } => self.begin_string()?,
            Token::Unsigned { role: Role::StringLength, value } => {
                let owner = self.string.as_mut().ok_or("generation3d-mutation.string-length")?;
                owner.value.try_reserve_exact(value as usize).map_err(|_| "generation3d-mutation.string-preflight")?;
                owner.remaining = Some(value);
                if value == 0 {
                    self.finish_string()?;
                }
            }
            Token::StringChar(character) => {
                let owner = self.string.as_mut().ok_or("generation3d-mutation.string-char")?;
                owner.value.push(character);
                let remaining = owner.remaining.as_mut().ok_or("generation3d-mutation.string-width")?;
                *remaining = remaining.checked_sub(character.len_utf8() as u64).ok_or("generation3d-mutation.string-width")?;
                if *remaining == 0 {
                    self.finish_string()?;
                }
            }
            Token::Unsigned { role: Role::Symbol, value } => self.begin_symbol(value, body)?,
            Token::F64(bits) => self.accept_f64(bits)?,
            Token::Signed(value) if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::int(value))?,
            Token::Signed(value) if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::int(value))?,
            Token::Signed(value) => match self.stack.last_mut() {
                Some(Generation3dMutationFrame::NeuralValue { field, value: target, .. }) if *field == Some(2) && target.is_none() => {
                    *target = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(value)));
                    *field = None;
                }
                _ => return Err("generation3d-mutation.integer-owner"),
            },
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::uint(value))?,
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::uint(value))?,
            Token::Tag { value: 0x01 | 0x02, .. } => self.accept_inline_tag(token)?,
            Token::Tag { value: 0x12, .. } if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::Null)?,
            Token::Tag { value: 0x12, .. } if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::Null)?,
            Token::WirePresence(_) => {}
            Token::WireNodePresence(presence) => self.accept_wire_node_presence(presence)?,
            Token::TablePresence { rows, value } => match self.stack.last_mut() {
                Some(Generation3dMutationFrame::Dictionary { present, .. }) if rows as usize == present.len() && value == 0 => {
                    present.fill(true);
                }
                _ => {}
            },
            Token::TableBitmap { first_row, value } => if let Some(Generation3dMutationFrame::Dictionary { present, .. }) = self.stack.last_mut() {
                for bit in 0..8 {
                    let row = first_row as usize + bit;
                    if row < present.len() {
                        present[row] = value & (1 << bit) != 0;
                    }
                }
            },
            Token::End(kind) => self.accept_end(kind)?,
            Token::Complete { .. } => self.accept_complete()?,
            Token::Tag { .. } | Token::Unsigned { .. } | Token::Byte(_) | Token::WireLabelPresence(_) => {}
        }
        Ok(())
    }

    /// 🧱️ Opens a table container of the retained record.
    fn accept_begin_table(&mut self, count: u64) -> Result<(), &'static str> {
        use store::mounted_pack_rt::RetainedValueContainer as Container;
        if self.pending_table_rows.take() != Some(count) {
            return Err("generation3d-mutation.table-row-count");
        }
        let parent = self.stack.len().checked_sub(1).ok_or("generation3d-mutation.dictionary-parent")?;
        let destination = match self.stack.get(parent) {
            Some(Generation3dMutationFrame::Widget { field: Some(field @ (1 | 5)), .. }) => Generation3dMutationDictionaryDestination::Widget { parent, field: *field },
            Some(Generation3dMutationFrame::NeuralValue { field: Some(5), .. }) => Generation3dMutationDictionaryDestination::Value { parent },
            _ => {
                self.push(Generation3dMutationFrame::Structural(Container::Table))?;
                return Ok(());
            }
        };
        self.begin_dictionary(destination, count)?;
        Ok(())
    }

    /// 🪵️ Opens a list or tuple container of the retained record.
    fn accept_begin_list_or_tuple(&mut self, count: u64) -> Result<(), &'static str> {
        use store::mounted_pack_rt::RetainedValueContainer as Container;
        let (parent, field) = match self.stack.last() {
            Some(Generation3dMutationFrame::NeuralValue { field: Some(5), value: None, .. }) => {
                let parent = self.stack.len() - 1;
                let mut rows = Vec::new();
                rows.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mutation.dictionary-entry-count")?).map_err(|_| "generation3d-mutation.dictionary-entry-preflight")?;
                return self.push(Generation3dMutationFrame::DictionaryEntries { destination: Generation3dMutationDictionaryDestination::Value { parent }, rows });
            }
            Some(Generation3dMutationFrame::Widget { field: Some(field), .. }) => (self.stack.len() - 1, *field),
            Some(Generation3dMutationFrame::Root { field: Some(0) }) if (15..=18).contains(&self.ordinal) => (self.stack.len() - 1, 0),
            _ => {
                self.push(Generation3dMutationFrame::Structural(Container::List))?;
                return Ok(());
            }
        };
        let mut values = Vec::new();
        values.try_reserve_exact(count as usize).map_err(|_| "generation3d-mutation.sequence-preflight")?;
        self.push(Generation3dMutationFrame::Strings { parent, field, values })?;
        Ok(())
    }

    /// 🔩️ Binds the field id of the retained record.
    fn accept_field_id(&mut self, value: u64) -> Result<(), &'static str> {
        match self.stack.last_mut() {
            Some(
                Generation3dMutationFrame::Root { field }
                | Generation3dMutationFrame::Widget { field, .. }
                | Generation3dMutationFrame::Synapse { field, .. }
                | Generation3dMutationFrame::Layout { field, .. }
                | Generation3dMutationFrame::Camera { field, .. }
                | Generation3dMutationFrame::Generation { field, .. }
                | Generation3dMutationFrame::DictionaryEntry { field, .. }
                | Generation3dMutationFrame::NeuralValue { field, .. },
            ) if field.is_none() => *field = Some(value as u16),
            _ => return Err("generation3d-mutation.field-owner"),
        }
        Ok(())
    }

    /// ⚙️ Accepts the bits of one float of the retained record.
    fn accept_f64(&mut self, bits: u64) -> Result<(), &'static str> {
        if self.dsl_destination.is_some() {
            self.assign_dsl(semio_framework_value::DslValue::float(f64::from_bits(bits)))?;
        } else if self.json_destination.is_some() {
            self.assign_json(semio_framework_value::DslValue::float(f64::from_bits(bits)))?;
        } else {
            match self.stack.last_mut() {
                Some(Generation3dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(3) && value.is_none() => {
                    *value = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(f64::from_bits(bits))));
                    *field = None;
                }
                Some(Generation3dMutationFrame::Widget { field, owner }) => {
                    let field = field.take().ok_or("generation3d-mutation.widget-number")?;
                    *owner.numbers.get_mut(field.checked_sub(2).ok_or("generation3d-mutation.widget-number-field")? as usize).ok_or("generation3d-mutation.widget-number-field")? = f64::from_bits(bits);
                }
                Some(Generation3dMutationFrame::Layout { field, value }) => match field.take() {
                    Some(0) => value.x = f64::from_bits(bits),
                    Some(1) => value.y = f64::from_bits(bits),
                    _ => return Err("generation3d-mutation.layout-field"),
                },
                Some(Generation3dMutationFrame::Root { field }) if (14..=18).contains(&self.ordinal) => {
                    let slot = field.take().filter(|value| (1..=4).contains(value)).ok_or("generation3d-mutation.semantic-number-field")?;
                    self.numbers[usize::from(slot - 1)] = f64::from_bits(bits);
                }
                Some(Generation3dMutationFrame::Camera { field, value }) => match field.take() {
                    Some(0) => value.x = f64::from_bits(bits),
                    Some(1) => value.y = f64::from_bits(bits),
                    Some(2) => value.zoom = f64::from_bits(bits),
                    _ => return Err("generation3d-mutation.camera-field"),
                },
                _ => return Err("generation3d-mutation.number-owner"),
            }
        }
        Ok(())
    }

    /// 🧲️ Accepts the inline tag of a retained flag or bit mask.
    fn accept_inline_tag(&mut self, token: store::mounted_pack_rt::RetainedValueToken) -> Result<(), &'static str> {
        use store::mounted_pack_rt::RetainedValueToken as Token;
        let boolean = matches!(token, Token::Tag { value: 0x02, .. });
        if self.dsl_destination.is_some() {
            self.assign_dsl(semio_framework_value::DslValue::Bool(boolean))?;
        } else if self.json_destination.is_some() {
            self.assign_json(semio_framework_value::DslValue::Bool(boolean))?;
        } else {
            match self.stack.last_mut() {
                Some(Generation3dMutationFrame::Widget { field, owner }) => {
                    owner.boolean = boolean;
                    *field = None;
                }
                Some(Generation3dMutationFrame::NeuralValue { field, value, .. }) if matches!(*field, Some(0 | 1)) && value.is_none() => {
                    *value = Some(if *field == Some(0) {
                        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Null)
                    } else {
                        semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(boolean))
                    });
                    *field = None;
                }
                _ => return Err("generation3d-mutation.boolean-owner"),
            }
        }
        Ok(())
    }

    /// 🪛️ Accepts the presence marker of one wire node.
    fn accept_wire_node_presence(&mut self, presence: u8) -> Result<(), &'static str> {
        match self.stack.last_mut() {
            Some(Generation3dMutationFrame::Wire { roles, roles_len, nodes, .. }) => {
                let base = if *nodes == 0 { 0 } else { 3 };
                roles[*roles_len] = base;
                *roles_len += 1;
                if presence & 1 != 0 {
                    roles[*roles_len] = base + 1;
                    *roles_len += 1;
                }
                if presence & 2 != 0 {
                    roles[*roles_len] = base + 2;
                    *roles_len += 1;
                }
                *nodes += 1;
            }
            _ => return Err("generation3d-mutation.wire-node"),
        }
        Ok(())
    }

    /// 🔧️ Closes the innermost container of the retained record.
    fn accept_end(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer) -> Result<(), &'static str> {
        use store::mounted_pack_rt::RetainedValueContainer as Container;
        if self.end_dsl(kind)? {
            return Ok(());
        }
        if self.json_destination.is_some() && matches!(kind, Container::List | Container::Map) {
            let value = match self.json_stack.pop().ok_or("generation3d-mutation.json-end-owner")? {
                Generation3dMutationJsonFrame::Array(values) if kind == Container::List => semio_framework_value::DslValue::Array(values),
                Generation3dMutationJsonFrame::Object { values, key: None } if kind == Container::Map => semio_framework_value::DslValue::Object(values),
                _ => return Err("generation3d-mutation.json-end-mismatch"),
            };
            self.assign_json(value)?;
            return Ok(());
        }
        let frame = self.stack.pop().ok_or("generation3d-mutation.end-owner")?;
        match frame {
            Generation3dMutationFrame::Root { field: None } if kind == Container::Record => {}
            Generation3dMutationFrame::Widget { field: None, owner } if kind == Container::Record => self.widget = Some(Self::finish_widget(owner)?),
            Generation3dMutationFrame::Synapse { field: None, owner } if kind == Container::Record => {
                self.synapse = Some(semio_framework_artifact_flow_flow::SynapseSpec { id: owner.id, from: owner.from, to: owner.to, from_port: owner.from_port, to_port: owner.to_port });
            }
            Generation3dMutationFrame::Layout { field: None, value } if kind == Container::Record => self.layout = Some(value),
            Generation3dMutationFrame::Camera { field: None, value } if kind == Container::Record => self.camera = Some(value),
            Generation3dMutationFrame::Generation { field: None, id, name, values } if kind == Container::Record => {
                self.generation = Some(semio_framework_artifact_playbook_playbook::FormGeneration { id, name, values: values.into_iter().collect() });
            }
            Generation3dMutationFrame::NeuralValue { owner: Generation3dMutationNeuralOwner::TableRow { table, row }, field: None, value: Some(value) } if kind == Container::Record => match self.stack.get_mut(table) {
                Some(Generation3dMutationFrame::Dictionary { rows, field: Some(1), .. }) => {
                    rows.get_mut(row).ok_or("generation3d-mutation.dictionary-value-row")?.value = Some(value);
                }
                _ => return Err("generation3d-mutation.dictionary-value-table"),
            },
            Generation3dMutationFrame::NeuralValue { owner: Generation3dMutationNeuralOwner::EntryRow { entries, row }, field: None, value: Some(value) } if kind == Container::Record => {
                match self.stack.get_mut(entries) {
                    Some(Generation3dMutationFrame::DictionaryEntries { rows, .. }) => {
                        rows.get_mut(row).ok_or("generation3d-mutation.dictionary-entry-row")?.value = Some(value);
                    }
                    _ => return Err("generation3d-mutation.dictionary-entry-list"),
                }
                if let Some(Generation3dMutationFrame::DictionaryEntry { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMutationFrame::DictionaryEntry { field: None, .. } if kind == Container::Record => {}
            Generation3dMutationFrame::DictionaryEntries { destination, rows } if matches!(kind, Container::List | Container::Tuple) => self.finish_dictionary(destination, rows)?,
            Generation3dMutationFrame::Dictionary { destination, rows, field: Some(1), .. } if kind == Container::Table => self.finish_dictionary(destination, rows)?,
            Generation3dMutationFrame::Strings { parent, field, values } => match self.stack.get_mut(parent) {
                Some(Generation3dMutationFrame::Root { field: active }) if (15..=18).contains(&self.ordinal) && field == 0 => { self.targets = values; *active = None; }
                Some(Generation3dMutationFrame::Widget { field: active, owner }) => {
                    owner.lists[if field == 4 { 1 } else { 0 }] = values;
                    *active = None;
                }
                _ => return Err("generation3d-mutation.sequence-parent"),
            },
            Generation3dMutationFrame::Wire { parent, .. } if kind == Container::Wire => {
                if let Some(Generation3dMutationFrame::Synapse { field, .. }) = self.stack.get_mut(parent) {
                    *field = None;
                }
            }
            Generation3dMutationFrame::Statements { .. } if kind == Container::Statements => {}
            Generation3dMutationFrame::Structural(expected) if expected == kind => {}
            _ => return Err("generation3d-mutation.end-mismatch"),
        }
        if let Some(Generation3dMutationFrame::Root { field }) = self.stack.last_mut() {
            *field = None;
        }
        Ok(())
    }

    /// 🔨️ Completes the retained record and publishes its destination.
    fn accept_complete(&mut self) -> Result<(), &'static str> {
        if !self.stack.is_empty() || self.string.is_some() || self.json_destination.is_some() || !self.json_stack.is_empty() || self.dsl_destination.is_some() || !self.dsl_stack.is_empty() || self.pending_table_rows.is_some() {
            return Err("generation3d-mutation.terminal-populated");
        }
        let strings = std::mem::replace(&mut self.strings, std::array::from_fn(|_| String::new()));
        let [first, second, _third] = strings;
        let mutation = match self.ordinal {
            0 => Generation3dMutation::CreateWidget(CreateWidget { index: self.index, widget: self.widget.take().ok_or("generation3d-mutation.create-widget")? }),
            1 => Generation3dMutation::UpdateWidget(UpdateWidget { widget: self.widget.take().ok_or("generation3d-mutation.update-widget")? }),
            2 => Generation3dMutation::DeleteWidget(DeleteWidget { id: first }),
            3 => Generation3dMutation::ConnectSynapse(ConnectSynapse { index: self.index, synapse: self.synapse.take().ok_or("generation3d-mutation.connect-synapse")? }),
            4 => Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: self.synapse.take().ok_or("generation3d-mutation.update-synapse")? }),
            5 => Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: first }),
            6 => Generation3dMutation::MoveWidget(MoveWidget { id: first, layout: self.layout.take().ok_or("generation3d-mutation.move-widget")? }),
            7 => Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: first }),
            8 => Generation3dMutation::UpdateCamera(UpdateCamera { camera: self.camera.take().ok_or("generation3d-mutation.update-camera")? }),
            9 => Generation3dMutation::ChangeSchema(ChangeSchema { new_schema: first }),
            10 => Generation3dMutation::CreateGeneration(CreateGeneration { generation: self.generation.take().ok_or("generation3d-mutation.create-generation")?, index: self.index_present.then_some(self.index) }),
            11 => Generation3dMutation::DeleteGeneration(DeleteGeneration { id: first }),
            12 => Generation3dMutation::RenameGeneration(RenameGeneration { id: first, new_name: second }),
            13 => Generation3dMutation::ChangeGenerationValue(ChangeGenerationValue { id: first, question_id: second, new_value: std::mem::replace(&mut self.json, semio_framework_value::DslValue::Null) }),
            14 => Generation3dMutation::ChangeSliderValue(ChangeSliderValue { id: first, value: self.numbers[0] }),
            15 => Generation3dMutation::DragTransforms(DragTransforms { targets: std::mem::take(&mut self.targets), dx: self.numbers[0], dy: self.numbers[1], dz: self.numbers[2] }),
            16 => Generation3dMutation::RotateTransforms(RotateTransforms { targets: std::mem::take(&mut self.targets), ax: self.numbers[0], ay: self.numbers[1], az: self.numbers[2], angle: self.numbers[3] }),
            17 => Generation3dMutation::ScaleTransforms(ScaleTransforms { targets: std::mem::take(&mut self.targets), sx: self.numbers[0], sy: self.numbers[1], sz: self.numbers[2] }),
            18 => Generation3dMutation::MoveNodes(MoveNodes { ids: std::mem::take(&mut self.targets), dx: self.numbers[0], dy: self.numbers[1] }),
            19 => Generation3dMutation::ChangeWidgetInput(ChangeWidgetInput { id: first, channel: second, input: <WidgetInputValue as semio_framework_value::FromValue>::from_value(std::mem::replace(&mut self.json, semio_framework_value::DslValue::Null)).map_err(|_| "generation3d-mutation.widget-input")? }),
            20 => Generation3dMutation::SelectGeneration(SelectGeneration { generation_id: (self.index == 1).then_some(first) }),
            21 => Generation3dMutation::ChangeGenerationPreview(ChangeGenerationPreview { text: (self.index == 1).then_some(first) }),
            _ => return Err("generation3d-mutation.variant"),
        };
        *self.value = Some(mutation);
        self.complete = true;
        Ok(())
    }

    pub(super) fn take(&mut self) -> Option<Generation3dMutation> {
        if !self.complete || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.value.take()
    }

    pub(super) fn close_step(&mut self) -> bool {
        self.string = None;
        if !self.closing_targets.terminal_is_empty() {
            generation3d_close_flow_frontier(&mut self.closing_targets).expect("admitted semantic target retirement");
            return false;
        }
        if let Some(frame) = self.stack.pop() {
            if let Generation3dMutationFrame::Strings { values, .. } = frame { generation3d_park_flow_strings(&mut self.closing_targets, values); }
            return false;
        }
        if !self.targets.is_empty() {
            generation3d_park_flow_strings(&mut self.closing_targets, std::mem::take(&mut self.targets));
            return false;
        }
        if let Some(mut mutation) = self.value.take() {
            let targets = match &mut mutation {
                Generation3dMutation::DragTransforms(value) => Some(std::mem::take(&mut value.targets)),
                Generation3dMutation::RotateTransforms(value) => Some(std::mem::take(&mut value.targets)),
                Generation3dMutation::ScaleTransforms(value) => Some(std::mem::take(&mut value.targets)),
                Generation3dMutation::MoveNodes(value) => Some(std::mem::take(&mut value.ids)),
                _ => None,
            };
            if let Some(targets) = targets { generation3d_park_flow_strings(&mut self.closing_targets, targets); }
            mutation.retire_cold();
            return false;
        }
        drop(self.widget.take());
        drop(self.synapse.take());
        self.layout = None;
        self.camera = None;
        drop(self.generation.take());
        self.json_stack.clear();
        self.json_destination = None;
        self.dsl_stack.clear();
        self.dsl_destination = None;
        self.pending_table_rows = None;
        self.handed_back = true;
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.handed_back
            && self.value.is_none()
            && self.stack.is_empty()
            && self.string.is_none()
            && self.targets.is_empty()
            && self.closing_targets.terminal_is_empty()
            && self.widget.is_none()
            && self.synapse.is_none()
            && self.layout.is_none()
            && self.camera.is_none()
            && self.generation.is_none()
            && self.json_stack.is_empty()
            && self.json_destination.is_none()
            && self.dsl_stack.is_empty()
            && self.dsl_destination.is_none()
            && self.pending_table_rows.is_none()
    }
}
//#endregion 🔖️TypedOwnedEnvelopeCatalog
