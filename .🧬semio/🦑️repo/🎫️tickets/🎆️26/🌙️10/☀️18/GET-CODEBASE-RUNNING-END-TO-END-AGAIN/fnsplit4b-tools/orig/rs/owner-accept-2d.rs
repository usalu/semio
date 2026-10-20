//! 🧰️ Retained mutation owner accept: the accept step of the retained mutation owner with its take, close step, terminal check and drop. See [`super`].

use super::*;
use semio_s_artifact_procedural_2d_mut_generations::create_generation;

impl Generation2dRetainedMutationOwner {
    pub(super) fn accept(&mut self, token: store::mounted_pack_rt::RetainedValueToken, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<(), &'static str> {
        use store::mounted_pack_rt::{RetainedValueContainer as Container, RetainedValueRole as Role, RetainedValueToken as Token};
        match token {
            Token::Tag { value: 0x11, .. } => {
                if self.dsl_destination.is_some() {
                    self.begin_dsl();
                } else if self.json_destination.is_none() {
                    if self.ordinal == 13 && self.root_field() == Some(2) {
                        self.begin_json(Generation2dMutationJsonDestination::ChangeValue)?;
                    } else {
                        self.begin_dsl();
                    }
                }
            }
            Token::Begin { kind: Container::List, count } if self.dsl_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation2d-mutation.dsl-count")?).map_err(|_| "generation2d-mutation.dsl-preflight")?;
                self.dsl_stack.push(Generation2dMutationDslFrame::Array(values));
            }
            Token::Begin { kind: Container::Map, count } if self.dsl_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation2d-mutation.dsl-count")?).map_err(|_| "generation2d-mutation.dsl-preflight")?;
                self.dsl_stack.push(Generation2dMutationDslFrame::Object { values, key: None });
            }
            Token::Begin { kind: Container::List, count } if self.json_destination.is_some() => {
                let mut values = Vec::new();
                values.try_reserve_exact(usize::try_from(count).map_err(|_| "generation2d-mutation.json-list-count")?).map_err(|_| "generation2d-mutation.json-list-preflight")?;
                self.json_stack.push(Generation2dMutationJsonFrame::Array(values));
            }
            Token::Begin { kind: Container::Map, .. } if self.json_destination.is_some() => {
                self.json_stack.push(Generation2dMutationJsonFrame::Object { values: Vec::new(), key: None });
            }
            Token::Begin { kind: Container::Map, .. } => {
                let index = self.stack.len().checked_sub(1).ok_or("generation2d-mutation.generation-values-owner")?;
                if matches!(self.stack.get(index), Some(Generation2dMutationFrame::Generation { field: Some(2), .. })) {
                    self.begin_json(Generation2dMutationJsonDestination::Generation(index))?;
                    self.json_stack.push(Generation2dMutationJsonFrame::Object { values: Vec::new(), key: None });
                } else {
                    self.push(Generation2dMutationFrame::Structural(Container::Map))?;
                }
            }
            Token::Begin { kind: Container::Table, count } => {
                if self.pending_table_rows.take() != Some(count) {
                    return Err("generation2d-mutation.table-row-count");
                }
                let parent = self.stack.len().checked_sub(1).ok_or("generation2d-mutation.dictionary-parent")?;
                let destination = match self.stack.get(parent) {
                    Some(Generation2dMutationFrame::Widget { field: Some(field @ (1 | 5)), .. }) => Generation2dMutationDictionaryDestination::Widget { parent, field: *field },
                    Some(Generation2dMutationFrame::NeuralValue { field: Some(5), .. }) => Generation2dMutationDictionaryDestination::Value { parent },
                    _ => {
                        self.push(Generation2dMutationFrame::Structural(Container::Table))?;
                        return Ok(());
                    }
                };
                self.begin_dictionary(destination, count)?;
            }
            Token::Begin { kind: Container::Record, .. } => self.begin_record()?,
            Token::Begin { kind: Container::Statements, .. } => self.push(Generation2dMutationFrame::Statements { keyword: None })?,
            Token::Begin { kind: Container::List | Container::Tuple, count } => {
                let (parent, field) = match self.stack.last() {
                    Some(Generation2dMutationFrame::NeuralValue { field: Some(5), value: None, .. }) => {
                        let parent = self.stack.len() - 1;
                        let mut rows = Vec::new();
                        rows.try_reserve_exact(usize::try_from(count).map_err(|_| "generation2d-mutation.dictionary-entry-count")?).map_err(|_| "generation2d-mutation.dictionary-entry-preflight")?;
                        return self.push(Generation2dMutationFrame::DictionaryEntries { destination: Generation2dMutationDictionaryDestination::Value { parent }, rows });
                    }
                    Some(Generation2dMutationFrame::Widget { field: Some(field), .. }) => (self.stack.len() - 1, *field),
                    Some(Generation2dMutationFrame::Root { field: Some(0) }) if self.ordinal == 15 => (self.stack.len() - 1, 0),
                    _ => {
                        self.push(Generation2dMutationFrame::Structural(Container::List))?;
                        return Ok(());
                    }
                };
                let mut values = Vec::new();
                values.try_reserve_exact(count as usize).map_err(|_| "generation2d-mutation.sequence-preflight")?;
                self.push(Generation2dMutationFrame::Strings { parent, field, values })?;
            }
            Token::Begin { kind: Container::Wire, .. } => {
                let parent = self.stack.len().checked_sub(1).ok_or("generation2d-mutation.wire-parent")?;
                self.push(Generation2dMutationFrame::Wire { parent, roles: [0; 6], roles_len: 0, role: 0, nodes: 0 })?;
            }
            Token::Begin { kind, .. } => self.push(Generation2dMutationFrame::Structural(kind))?,
            Token::Unsigned { role: Role::FieldId, value } if value <= u16::MAX as u64 => match self.stack.last_mut() {
                Some(
                    Generation2dMutationFrame::Root { field }
                    | Generation2dMutationFrame::Widget { field, .. }
                    | Generation2dMutationFrame::Synapse { field, .. }
                    | Generation2dMutationFrame::Layout { field, .. }
                    | Generation2dMutationFrame::Camera { field, .. }
                    | Generation2dMutationFrame::Generation { field, .. }
                    | Generation2dMutationFrame::DictionaryEntry { field, .. }
                    | Generation2dMutationFrame::NeuralValue { field, .. },
                ) if field.is_none() => *field = Some(value as u16),
                _ => return Err("generation2d-mutation.field-owner"),
            },
            Token::Unsigned { role: Role::TableRows, value } => self.pending_table_rows = Some(value),
            Token::Unsigned { role: Role::TableField, value } => if let Some(Generation2dMutationFrame::Dictionary { field, present, next, .. }) = self.stack.last_mut() {
                *field = Some(u16::try_from(value).map_err(|_| "generation2d-mutation.dictionary-field")?);
                present.fill(false);
                *next = 0;
            },
            Token::Unsigned { role: Role::Unsigned, value } if self.json_destination.is_none() && self.dsl_destination.is_none() => {
                self.index = usize::try_from(value).map_err(|_| "generation2d-mutation.index")?;
                self.index_present = true;
                if let Some(Generation2dMutationFrame::Root { field }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Token::Tag { value: 0x06 | 0x07, .. } => self.begin_string()?,
            Token::Unsigned { role: Role::StringLength, value } => {
                let owner = self.string.as_mut().ok_or("generation2d-mutation.string-length")?;
                owner.value.try_reserve_exact(value as usize).map_err(|_| "generation2d-mutation.string-preflight")?;
                owner.remaining = Some(value);
                if value == 0 {
                    self.finish_string()?;
                }
            }
            Token::StringChar(character) => {
                let owner = self.string.as_mut().ok_or("generation2d-mutation.string-char")?;
                owner.value.push(character);
                let remaining = owner.remaining.as_mut().ok_or("generation2d-mutation.string-width")?;
                *remaining = remaining.checked_sub(character.len_utf8() as u64).ok_or("generation2d-mutation.string-width")?;
                if *remaining == 0 {
                    self.finish_string()?;
                }
            }
            Token::Unsigned { role: Role::Symbol, value } => self.begin_symbol(value, body)?,
            Token::F64(bits) => {
                if self.dsl_destination.is_some() {
                    self.assign_dsl(semio_framework_value::DslValue::float(f64::from_bits(bits)))?;
                } else if self.json_destination.is_some() {
                    self.assign_json(semio_framework_value::DslValue::float(f64::from_bits(bits)))?;
                } else {
                    match self.stack.last_mut() {
                        Some(Generation2dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(3) && value.is_none() => {
                            *value = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(f64::from_bits(bits))));
                            *field = None;
                        }
                        Some(Generation2dMutationFrame::Widget { field, owner }) => {
                            let field = field.take().ok_or("generation2d-mutation.widget-number")?;
                            *owner.numbers.get_mut(field.checked_sub(2).ok_or("generation2d-mutation.widget-number-field")? as usize).ok_or("generation2d-mutation.widget-number-field")? = f64::from_bits(bits);
                        }
                        Some(Generation2dMutationFrame::Layout { field, value }) => match field.take() {
                            Some(0) => value.x = f64::from_bits(bits),
                            Some(1) => value.y = f64::from_bits(bits),
                            _ => return Err("generation2d-mutation.layout-field"),
                        },
                        Some(Generation2dMutationFrame::Root { field }) if (14..=15).contains(&self.ordinal) => {
                            let slot = field.take().filter(|value| (1..=2).contains(value)).ok_or("generation2d-mutation.gesture-number-field")?;
                            self.numbers[usize::from(slot - 1)] = f64::from_bits(bits);
                        }
                        Some(Generation2dMutationFrame::Camera { field, value }) => match field.take() {
                            Some(0) => value.x = f64::from_bits(bits),
                            Some(1) => value.y = f64::from_bits(bits),
                            Some(2) => value.zoom = f64::from_bits(bits),
                            _ => return Err("generation2d-mutation.camera-field"),
                        },
                        _ => return Err("generation2d-mutation.number-owner"),
                    }
                }
            }
            Token::Signed(value) if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::int(value))?,
            Token::Signed(value) if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::int(value))?,
            Token::Signed(value) => match self.stack.last_mut() {
                Some(Generation2dMutationFrame::NeuralValue { field, value: target, .. }) if *field == Some(2) && target.is_none() => {
                    *target = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(value)));
                    *field = None;
                }
                _ => return Err("generation2d-mutation.integer-owner"),
            },
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::uint(value))?,
            Token::Unsigned { role: Role::Integer | Role::Unsigned | Role::Enum, value } if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::uint(value))?,
            Token::Tag { value: 0x01 | 0x02, .. } => {
                let boolean = matches!(token, Token::Tag { value: 0x02, .. });
                if self.dsl_destination.is_some() {
                    self.assign_dsl(semio_framework_value::DslValue::Bool(boolean))?;
                } else if self.json_destination.is_some() {
                    self.assign_json(semio_framework_value::DslValue::Bool(boolean))?;
                } else {
                    match self.stack.last_mut() {
                        Some(Generation2dMutationFrame::Widget { field, owner }) => {
                            owner.boolean = boolean;
                            *field = None;
                        }
                        Some(Generation2dMutationFrame::NeuralValue { field, value, .. }) if matches!(*field, Some(0 | 1)) && value.is_none() => {
                            *value = Some(if *field == Some(0) {
                                semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Null)
                            } else {
                                semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(boolean))
                            });
                            *field = None;
                        }
                        _ => return Err("generation2d-mutation.boolean-owner"),
                    }
                }
            }
            Token::Tag { value: 0x12, .. } if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::Null)?,
            Token::Tag { value: 0x12, .. } if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::Null)?,
            Token::WirePresence(_) => {}
            Token::WireNodePresence(presence) => match self.stack.last_mut() {
                Some(Generation2dMutationFrame::Wire { roles, roles_len, nodes, .. }) => {
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
                _ => return Err("generation2d-mutation.wire-node"),
            },
            Token::TablePresence { rows, value } => match self.stack.last_mut() {
                Some(Generation2dMutationFrame::Dictionary { present, .. }) if rows as usize == present.len() && value == 0 => {
                    present.fill(true);
                }
                _ => {}
            },
            Token::TableBitmap { first_row, value } => if let Some(Generation2dMutationFrame::Dictionary { present, .. }) = self.stack.last_mut() {
                for bit in 0..8 {
                    let row = first_row as usize + bit;
                    if row < present.len() {
                        present[row] = value & (1 << bit) != 0;
                    }
                }
            },
            Token::End(kind) => {
                if self.end_dsl(kind)? {
                    return Ok(());
                }
                if self.json_destination.is_some() && matches!(kind, Container::List | Container::Map) {
                    let value = match self.json_stack.pop().ok_or("generation2d-mutation.json-end-owner")? {
                        Generation2dMutationJsonFrame::Array(values) if kind == Container::List => semio_framework_value::DslValue::Array(values),
                        Generation2dMutationJsonFrame::Object { values, key: None } if kind == Container::Map => semio_framework_value::DslValue::Object(values),
                        _ => return Err("generation2d-mutation.json-end-mismatch"),
                    };
                    self.assign_json(value)?;
                    return Ok(());
                }
                let frame = self.stack.pop().ok_or("generation2d-mutation.end-owner")?;
                match frame {
                    Generation2dMutationFrame::Root { field: None } if kind == Container::Record => {}
                    Generation2dMutationFrame::Widget { field: None, owner } if kind == Container::Record => self.widget = Some(Self::finish_widget(owner)?),
                    Generation2dMutationFrame::Synapse { field: None, owner } if kind == Container::Record => {
                        self.synapse = Some(semio_framework_artifact_flow_flow::SynapseSpec { id: owner.id, from: owner.from, to: owner.to, from_port: owner.from_port, to_port: owner.to_port });
                    }
                    Generation2dMutationFrame::Layout { field: None, value } if kind == Container::Record => self.layout = Some(value),
                    Generation2dMutationFrame::Camera { field: None, value } if kind == Container::Record => self.camera = Some(value),
                    Generation2dMutationFrame::Generation { field: None, id, name, values } if kind == Container::Record => {
                        self.generation = Some(semio_framework_artifact_playbook_playbook::FormGeneration { id, name, values: values.into_iter().collect() });
                    }
                    Generation2dMutationFrame::NeuralValue { owner: Generation2dMutationNeuralOwner::TableRow { table, row }, field: None, value: Some(value) } if kind == Container::Record => {
                        match self.stack.get_mut(table) {
                            Some(Generation2dMutationFrame::Dictionary { rows, field: Some(1), .. }) => {
                                rows.get_mut(row).ok_or("generation2d-mutation.dictionary-value-row")?.value = Some(value);
                            }
                            _ => return Err("generation2d-mutation.dictionary-value-table"),
                        }
                    }
                    Generation2dMutationFrame::NeuralValue { owner: Generation2dMutationNeuralOwner::EntryRow { entries, row }, field: None, value: Some(value) } if kind == Container::Record => {
                        match self.stack.get_mut(entries) {
                            Some(Generation2dMutationFrame::DictionaryEntries { rows, .. }) => {
                                rows.get_mut(row).ok_or("generation2d-mutation.dictionary-entry-row")?.value = Some(value);
                            }
                            _ => return Err("generation2d-mutation.dictionary-entry-list"),
                        }
                        if let Some(Generation2dMutationFrame::DictionaryEntry { field, .. }) = self.stack.last_mut() {
                            *field = None;
                        }
                    }
                    Generation2dMutationFrame::DictionaryEntry { field: None, .. } if kind == Container::Record => {}
                    Generation2dMutationFrame::DictionaryEntries { destination, rows } if matches!(kind, Container::List | Container::Tuple) => self.finish_dictionary(destination, rows)?,
                    Generation2dMutationFrame::Dictionary { destination, rows, field: Some(1), .. } if kind == Container::Table => self.finish_dictionary(destination, rows)?,
                    Generation2dMutationFrame::Strings { parent, field, values } => match self.stack.get_mut(parent) {
                        Some(Generation2dMutationFrame::Root { field: active }) if self.ordinal == 15 && field == 0 => {
                            self.targets = values;
                            *active = None;
                        }
                        Some(Generation2dMutationFrame::Widget { field: active, owner }) => {
                            owner.lists[if field == 4 { 1 } else { 0 }] = values;
                            *active = None;
                        }
                        _ => return Err("generation2d-mutation.sequence-parent"),
                    },
                    Generation2dMutationFrame::Wire { parent, .. } if kind == Container::Wire => {
                        if let Some(Generation2dMutationFrame::Synapse { field, .. }) = self.stack.get_mut(parent) {
                            *field = None;
                        }
                    }
                    Generation2dMutationFrame::Statements { .. } if kind == Container::Statements => {}
                    Generation2dMutationFrame::Structural(expected) if expected == kind => {}
                    _ => return Err("generation2d-mutation.end-mismatch"),
                }
                if let Some(Generation2dMutationFrame::Root { field }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Token::Complete { .. } => {
                if !self.stack.is_empty() || self.string.is_some() || self.json_destination.is_some() || !self.json_stack.is_empty() || self.dsl_destination.is_some() || !self.dsl_stack.is_empty() || self.pending_table_rows.is_some() {
                    return Err("generation2d-mutation.terminal-populated");
                }
                use semio_s_artifact_procedural_2d_mutations::*;
                let strings = std::mem::replace(&mut self.strings, std::array::from_fn(|_| String::new()));
                let [first, second, _third] = strings;
                let mutation = match self.ordinal {
                    0 => create_widget(self.index, self.widget.take().ok_or("generation2d-mutation.create-widget")?),
                    1 => replace_widget(self.widget.take().ok_or("generation2d-mutation.replace-widget")?),
                    2 => delete_widget(first),
                    3 => connect_synapse(self.index, self.synapse.take().ok_or("generation2d-mutation.connect-synapse")?),
                    4 => replace_synapse(self.synapse.take().ok_or("generation2d-mutation.replace-synapse")?),
                    5 => disconnect_synapse(first),
                    6 => move_widget(first, self.layout.take().ok_or("generation2d-mutation.move-widget")?),
                    7 => clear_widget_layout(first),
                    8 => update_camera(self.camera.take().ok_or("generation2d-mutation.update-camera")?),
                    9 => change_schema(first),
                    10 => {
                        let generation = self.generation.take().ok_or("generation2d-mutation.create-generation")?;
                        Generation2dMutation::CreateGeneration(create_generation::CreateGeneration { generation, index: self.index_present.then_some(self.index) })
                    }
                    11 => delete_generation(first),
                    12 => rename_generation(first, second),
                    13 => change_generation_value(first, second, std::mem::replace(&mut self.json, semio_framework_value::DslValue::Null)),
                    14 => change_slider_value(first, self.numbers[0]),
                    15 => move_nodes(std::mem::take(&mut self.targets), self.numbers[0], self.numbers[1]),
                    16 => select_generation((self.index == 1).then_some(first)),
                    _ => return Err("generation2d-mutation.variant"),
                };
                *self.value = Some(mutation);
                self.complete = true;
            }
            Token::Tag { .. } | Token::Unsigned { .. } | Token::Byte(_) | Token::WireLabelPresence(_) => {}
        }
        Ok(())
    }

    pub(super) fn take(&mut self) -> Option<Generation2dMutation> {
        if !self.complete || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.value.take()
    }

    pub(super) fn close_step(&mut self) -> bool {
        self.string = None;
        if self.stack.pop().is_some() {
            return false;
        }
        drop(self.value.take());
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
        self.targets = Vec::new();
        self.handed_back = true;
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.handed_back
            && self.value.is_none()
            && self.stack.is_empty()
            && self.string.is_none()
            && self.targets.is_empty()
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

impl Drop for Generation2dRetainedMutationOwner {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "Generation2d retained mutation owner reached Drop before handoff or terminal-empty close");
    }
}
