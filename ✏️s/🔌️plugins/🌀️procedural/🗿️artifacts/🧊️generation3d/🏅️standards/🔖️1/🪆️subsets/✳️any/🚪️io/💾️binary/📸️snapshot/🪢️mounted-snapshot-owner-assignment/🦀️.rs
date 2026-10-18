//! 🪢️ Mounted snapshot owner assignment: the json, dsl, scalar, dictionary, record and container assignment and the closing steps of the typed snapshot owner. See [`super`].

use super::*;

//#region 🔖️MountedCanonicalPackSession
impl Generation3dMountedTypedSnapshotOwner {
    pub(super) fn assign_json(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
        match self.json_stack.last_mut() {
            Some(Generation3dMountedJsonFrame::Array(values)) => values.push(value),
            Some(Generation3dMountedJsonFrame::Object { values, key }) => {
                values.push((key.take().ok_or("generation3d-mounted.json-value-key")?, value));
            }
            None => {
                let (table, row) = self.json_destination.take().ok_or("generation3d-mounted.json-destination")?;
                let values = match value {
                    semio_framework_value::DslValue::Object(values) => values,
                    _ => return Err("generation3d-mounted.generation-values-shape"),
                };
                match self.stack.get_mut(table) {
                    Some(Generation3dMountedContainerOwner::Generations { rows, .. }) => rows.get_mut(row).ok_or("generation3d-mounted.generation-row")?.values = values,
                    _ => return Err("generation3d-mounted.generation-values-table"),
                }
            }
        }
        Ok(())
    }

    pub(super) fn assign_dsl(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
        match self.dsl_stack.last_mut() {
            Some(Generation3dMountedDslFrame::Array(values)) => values.push(value),
            Some(Generation3dMountedDslFrame::Object { values, key }) => values.push((key.take().ok_or("generation3d-mounted.dsl-value-key")?, value)),
            None => {
                let (parent, slot) = self.dsl_destination.take().ok_or("generation3d-mounted.dsl-destination")?;
                match self.stack.get_mut(parent) {
                    Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Widget(widget), field, .. }) if *field == Some((slot + 2) as u16) => {
                        widget.dynamic[slot] = Some(value);
                        *field = None;
                    }
                    _ => return Err("generation3d-mounted.dsl-widget-owner"),
                }
            }
        }
        Ok(())
    }

    pub(super) fn end_dsl(&mut self, kind: mounted::RetainedValueContainer) -> Result<bool, &'static str> {
        if self.dsl_destination.is_none() {
            return Ok(false);
        }
        let value = match self.dsl_stack.pop().ok_or("generation3d-mounted.dsl-end")? {
            Generation3dMountedDslFrame::Array(values) if kind == mounted::RetainedValueContainer::List => semio_framework_value::DslValue::Array(values),
            Generation3dMountedDslFrame::Object { values, key: None } if kind == mounted::RetainedValueContainer::Map => semio_framework_value::DslValue::Object(values),
            _ => return Err("generation3d-mounted.dsl-container-mismatch"),
        };
        self.assign_dsl(value)?;
        Ok(true)
    }

    pub(super) fn begin_dsl(&mut self) -> bool {
        if self.dsl_destination.is_some() {
            return true;
        }
        let Some(parent) = self.stack.len().checked_sub(1) else { return false };
        let field = match self.stack.get(parent) {
            Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Widget(widget), field: Some(field @ (2 | 3)), .. }) if widget.keyword == "cluster" => *field,
            _ => return false,
        };
        self.dsl_destination = Some((parent, usize::from(field - 2)));
        true
    }

    pub(super) fn assign_f64(&mut self, value: f64) -> Result<(), &'static str> {
        match self.stack.last_mut() {
            Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::NeuralValue { value: target, .. }, field, .. }) if *field == Some(3) && target.is_none() => {
                *target = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Decimal(value)));
                *field = None;
            }
            Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Camera(camera), field, .. }) => match field.take() {
                Some(0) => camera.x = value,
                Some(1) => camera.y = value,
                Some(2) => camera.zoom = value,
                _ => return Err("generation3d-mounted.camera-field"),
            },
            Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Widget(widget), field, .. }) => {
                let field = field.take().ok_or("generation3d-mounted.widget-number-owner")?;
                *widget.numbers.get_mut(field.checked_sub(2).ok_or("generation3d-mounted.widget-number-field")? as usize).ok_or("generation3d-mounted.widget-number-field")? = value;
            }
            Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Layout { value: layout, .. }, field, .. }) => match field.take() {
                Some(0) => layout.x = value,
                Some(1) => layout.y = value,
                _ => return Err("generation3d-mounted.layout-field"),
            },
            _ => return Err("generation3d-mounted.number-owner"),
        }
        Ok(())
    }

    pub(super) fn assign_bool(&mut self, value: bool) {
        match self.stack.last_mut() {
            Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Widget(widget), field, .. }) => {
                widget.boolean = value;
                *field = None;
            }
            Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::NeuralValue { value: target, .. }, field, .. }) if matches!(*field, Some(0 | 1)) && target.is_none() => {
                *target = Some(if *field == Some(0) {
                    semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Null)
                } else {
                    semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Boolean(value))
                });
                *field = None;
            }
            _ => {}
        }
    }

    pub(super) fn end_json(&mut self, kind: mounted::RetainedValueContainer) -> Result<bool, &'static str> {
        if self.json_destination.is_none() {
            return Ok(false);
        }
        let matches = matches!((self.json_stack.last(), kind), (Some(Generation3dMountedJsonFrame::Array(_)), mounted::RetainedValueContainer::List) | (Some(Generation3dMountedJsonFrame::Object { .. }), mounted::RetainedValueContainer::Map));
        if !matches {
            return Err("generation3d-mounted.json-container-mismatch");
        }
        let value = match self.json_stack.pop().ok_or("generation3d-mounted.json-end")? {
            Generation3dMountedJsonFrame::Array(values) => semio_framework_value::DslValue::Array(values),
            Generation3dMountedJsonFrame::Object { values, key: None } => semio_framework_value::DslValue::Object(values),
            Generation3dMountedJsonFrame::Object { .. } => return Err("generation3d-mounted.json-key-without-value"),
        };
        self.assign_json(value)?;
        Ok(true)
    }

    fn begin_dictionary(&mut self, destination: Generation3dMountedDictionaryDestination, count: u64) -> Result<(), &'static str> {
        let rows = usize::try_from(count).map_err(|_| "generation3d-mounted.dictionary-count")?;
        let mut values = Vec::new();
        values.try_reserve_exact(rows).map_err(|_| "generation3d-mounted.dictionary-preflight")?;
        values.resize_with(rows, Generation3dMountedDictionaryEntryOwner::default);
        let mut present = Vec::new();
        present.try_reserve_exact(rows).map_err(|_| "generation3d-mounted.dictionary-presence-preflight")?;
        present.resize(rows, false);
        self.push(Generation3dMountedContainerOwner::Dictionary { destination, rows: values, field: None, present, next: 0 })
    }

    fn finish_dictionary(&mut self, destination: Generation3dMountedDictionaryDestination, rows: Vec<Generation3dMountedDictionaryEntryOwner>) -> Result<(), &'static str> {
        let mut dictionary = semio_framework_artifact_flow_flow::neural::Dictionary::new();
        for row in rows {
            dictionary = dictionary.insert(row.key, row.value.ok_or("generation3d-mounted.dictionary-value")?);
        }
        match destination {
            Generation3dMountedDictionaryDestination::Widget { parent, field } => match self.stack.get_mut(parent) {
                Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Widget(widget), field: active, .. }) if *active == Some(field) => {
                    widget.dictionaries[if field == 1 { 1 } else { 0 }] = dictionary;
                    *active = None;
                }
                _ => return Err("generation3d-mounted.dictionary-widget-owner"),
            },
            Generation3dMountedDictionaryDestination::Value { parent } => match self.stack.get_mut(parent) {
                Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::NeuralValue { value, .. }, field, .. }) if *field == Some(5) && value.is_none() => {
                    *value = Some(semio_framework_artifact_flow_flow::neural::Value::Dictionary(dictionary));
                    *field = None;
                }
                _ => return Err("generation3d-mounted.dictionary-value-owner"),
            },
        }
        Ok(())
    }

    pub(super) fn begin_record(&mut self, count: u64) -> Result<(), &'static str> {
        if count > 64 {
            return Err("generation3d-mounted.record-field-count");
        }
        let root_field = self.current_root_field();
        if let Some(table) = self.stack.len().checked_sub(1) {
            if let Some(Generation3dMountedContainerOwner::Dictionary { field: Some(1), present, next, .. }) = self.stack.get_mut(table) {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation3d-mounted.dictionary-value-row")?;
                *next = row + 1;
                let owner = Generation3dMountedNeuralOwner::TableRow { table, row };
                return self.push(Generation3dMountedContainerOwner::Record { root_field, field: None, seen: 0, owner: Generation3dMountedRecordOwner::NeuralValue { owner, value: None } });
            }
            if let Some(Generation3dMountedContainerOwner::DictionaryEntries { rows, .. }) = self.stack.get_mut(table) {
                let row = rows.len();
                rows.try_reserve(1).map_err(|_| "generation3d-mounted.dictionary-entry-preflight")?;
                rows.push(Generation3dMountedDictionaryEntryOwner::default());
                return self.push(Generation3dMountedContainerOwner::DictionaryEntry { entries: table, row, field: None });
            }
            if let Some(Generation3dMountedContainerOwner::DictionaryEntry { entries, row, field: Some(1) }) = self.stack.get_mut(table) {
                let owner = Generation3dMountedNeuralOwner::EntryRow { entries: *entries, row: *row };
                return self.push(Generation3dMountedContainerOwner::Record { root_field, field: None, seen: 0, owner: Generation3dMountedRecordOwner::NeuralValue { owner, value: None } });
            }
        }
        let owner = if self.stack.is_empty() {
            Generation3dMountedRecordOwner::Root
        } else if root_field == Some(1) {
            Generation3dMountedRecordOwner::Camera(semio_framework_artifact_flow_flow::CameraJson::default())
        } else if root_field == Some(2) {
            let keyword = match self.stack.last_mut() {
                Some(Generation3dMountedContainerOwner::Statements { keyword, .. }) => keyword.take().ok_or("generation3d-mounted.widget-keyword")?,
                _ => return Err("generation3d-mounted.widget-statements-owner"),
            };
            Generation3dMountedRecordOwner::Widget(Generation3dMountedWidgetOwner { keyword, ..Generation3dMountedWidgetOwner::default() })
        } else if root_field == Some(4) {
            let key = match self.stack.last_mut() {
                Some(Generation3dMountedContainerOwner::LayoutMap { key }) => key.take().ok_or("generation3d-mounted.layout-key")?,
                _ => return Err("generation3d-mounted.layout-map-owner"),
            };
            Generation3dMountedRecordOwner::Layout { key, value: semio_framework_artifact_flow_flow::WidgetLayout { x: 0.0, y: 0.0 } }
        } else {
            Generation3dMountedRecordOwner::Structural
        };
        self.push(Generation3dMountedContainerOwner::Record { root_field, field: None, seen: 0, owner })
    }

    pub(super) fn begin_container(&mut self, kind: mounted::RetainedValueContainer, count: u64) -> Result<(), &'static str> {
        let root_field = self.current_root_field().ok_or("generation3d-mounted.container-root")?;
        match kind {
            mounted::RetainedValueContainer::Statements => self.push(Generation3dMountedContainerOwner::Statements { root_field, keyword: None }),
            mounted::RetainedValueContainer::Table if root_field == 2 => {
                let parent = self.stack.len().checked_sub(1).ok_or("generation3d-mounted.dictionary-parent")?;
                let destination = match self.stack.get(parent) {
                    Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Widget(_), field: Some(field @ (1 | 5)), .. }) => Generation3dMountedDictionaryDestination::Widget { parent, field: *field },
                    Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::NeuralValue { .. }, field: Some(5), .. }) => Generation3dMountedDictionaryDestination::Value { parent },
                    _ => return self.push(Generation3dMountedContainerOwner::Structural { kind, root_field: Some(root_field) }),
                };
                self.begin_dictionary(destination, count)
            }
            mounted::RetainedValueContainer::List | mounted::RetainedValueContainer::Tuple if root_field == 2 => {
                let (parent, field) = match self.stack.last() {
                    Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::NeuralValue { value: None, .. }, field: Some(5), .. }) => {
                        let parent = self.stack.len() - 1;
                        let mut rows = Vec::new();
                        rows.try_reserve_exact(usize::try_from(count).map_err(|_| "generation3d-mounted.dictionary-entry-count")?).map_err(|_| "generation3d-mounted.dictionary-entry-preflight")?;
                        return self.push(Generation3dMountedContainerOwner::DictionaryEntries { destination: Generation3dMountedDictionaryDestination::Value { parent }, rows });
                    }
                    Some(Generation3dMountedContainerOwner::Record { field: Some(field), .. }) => (self.stack.len() - 1, *field),
                    _ => return Err("generation3d-mounted.sequence-owner"),
                };
                let mut values = Vec::new();
                values.try_reserve_exact(count as usize).map_err(|_| "generation3d-mounted.sequence-preflight")?;
                self.push(Generation3dMountedContainerOwner::Strings { parent, field, values })
            }
            mounted::RetainedValueContainer::Table if root_field == 3 => {
                let rows = usize::try_from(count).map_err(|_| "generation3d-mounted.synapse-count")?;
                let mut values = Vec::new();
                values.try_reserve_exact(rows).map_err(|_| "generation3d-mounted.synapse-preflight")?;
                values.resize_with(rows, Generation3dMountedSynapseOwner::default);
                let mut present = Vec::new();
                present.try_reserve_exact(rows).map_err(|_| "generation3d-mounted.synapse-presence-preflight")?;
                present.resize(rows, false);
                self.push(Generation3dMountedContainerOwner::Synapses { rows: values, field: None, present, next: 0 })
            }
            mounted::RetainedValueContainer::Map if root_field == 4 => self.push(Generation3dMountedContainerOwner::LayoutMap { key: None }),
            mounted::RetainedValueContainer::Table if root_field == 7 => {
                let rows = usize::try_from(count).map_err(|_| "generation3d-mounted.generation-count")?;
                let mut values = Vec::new();
                values.try_reserve_exact(rows).map_err(|_| "generation3d-mounted.generation-preflight")?;
                values.resize_with(rows, Generation3dMountedGenerationOwner::default);
                let mut present = Vec::new();
                present.try_reserve_exact(rows).map_err(|_| "generation3d-mounted.generation-presence-preflight")?;
                present.resize(rows, false);
                self.push(Generation3dMountedContainerOwner::Generations { rows: values, field: None, present, next: 0 })
            }
            mounted::RetainedValueContainer::Map if root_field == 7 => {
                let table = self.stack.len().checked_sub(1).ok_or("generation3d-mounted.generation-table")?;
                let row = match self.stack.get_mut(table) {
                    Some(Generation3dMountedContainerOwner::Generations { field: Some(2), present, next, .. }) => {
                        let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation3d-mounted.generation-row")?;
                        *next = row + 1;
                        row
                    }
                    _ => return Err("generation3d-mounted.generation-values-owner"),
                };
                self.json_destination = Some((table, row));
                self.json_stack.push(Generation3dMountedJsonFrame::Object { values: Vec::new(), key: None });
                Ok(())
            }
            mounted::RetainedValueContainer::Wire if root_field == 3 => {
                let table = self.stack.len().checked_sub(1).ok_or("generation3d-mounted.wire-table")?;
                let row = match self.stack.get_mut(table) {
                    Some(Generation3dMountedContainerOwner::Synapses { present, next, .. }) => {
                        let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation3d-mounted.wire-row")?;
                        *next = row + 1;
                        row
                    }
                    _ => return Err("generation3d-mounted.wire-table"),
                };
                self.push(Generation3dMountedContainerOwner::Wire { table, row, roles: [0; 6], roles_len: 0, role: 0, nodes: 0 })
            }
            _ => self.push(Generation3dMountedContainerOwner::Structural { kind, root_field: Some(root_field) }),
        }
    }

    fn finish_widget(owner: Generation3dMountedWidgetOwner) -> Result<semio_framework_artifact_flow_flow::Widget, &'static str> {
        let [id, second, third, _fourth] = owner.strings;
        let [value, min, max, step] = owner.numbers;
        let [first_list, second_list] = owner.lists;
        let [first_dictionary, second_dictionary] = owner.dictionaries;
        let [first_dynamic, second_dynamic] = owner.dynamic;
        Ok(match owner.keyword.as_str() {
            "neuron" => semio_framework_artifact_flow_flow::Widget::Neuron { id, neuron_kind: second, params: first_dictionary, input_ports: first_list, output_ports: second_list, preview: owner.boolean },
            "input-slider" => semio_framework_artifact_flow_flow::Widget::InputSlider { id, label: second, value, min, max, step },
            "input-note" => semio_framework_artifact_flow_flow::Widget::InputNote { id, text: second },
            "input-image" => semio_framework_artifact_flow_flow::Widget::InputImage { id, src: second },
            "variable" => semio_framework_artifact_flow_flow::Widget::Variable { id, name: second, schema: third },
            "output-preview" => {
                let mut expanded = semio_framework_artifact_flow_flow::OrderedSet::new();
                for entry in first_list {
                    expanded.insert(entry);
                }
                semio_framework_artifact_flow_flow::Widget::OutputPreview { id, preview: second_dictionary, expanded }
            }
            "output-action" => semio_framework_artifact_flow_flow::Widget::OutputAction { id, action: second },
            "output-export" => semio_framework_artifact_flow_flow::Widget::OutputExport { id, format: second },
            "cluster" => semio_framework_artifact_flow_flow::Widget::Cluster {
                id,
                name: second,
                tree: semio_framework_value::FromValue::from_value(first_dynamic.ok_or("generation3d-mounted.cluster-tree")?).map_err(|_| "generation3d-mounted.cluster-tree-shape")?,
                flow: semio_framework_value::FromValue::from_value(second_dynamic.ok_or("generation3d-mounted.cluster-flow")?).map_err(|_| "generation3d-mounted.cluster-flow-shape")?,
            },
            _ => return Err("generation3d-mounted.widget-variant"),
        })
    }

    pub(super) fn end(&mut self, kind: mounted::RetainedValueContainer) -> Result<(), &'static str> {
        let owner = self.stack.pop().ok_or("generation3d-mounted.end-without-owner")?;
        match owner {
            Generation3dMountedContainerOwner::Record { root_field: None, seen, owner: Generation3dMountedRecordOwner::Root, field: None } => {
                if seen & GENERATION3D_REQUIRED_SNAPSHOT_FIELDS != GENERATION3D_REQUIRED_SNAPSHOT_FIELDS {
                    return Err("generation3d-mounted.snapshot-fields-missing");
                }
            }
            Generation3dMountedContainerOwner::Record { root_field: Some(1), owner: Generation3dMountedRecordOwner::Camera(camera), field: None, .. } => {
                self.candidate.as_mut().ok_or("generation3d-mounted.snapshot-owner")?.host_snapshot.camera = camera;
                if let Some(Generation3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMountedContainerOwner::Record { root_field: Some(2), owner: Generation3dMountedRecordOwner::Widget(widget), field: None, .. } => {
                self.candidate.as_mut().ok_or("generation3d-mounted.snapshot-owner")?.host_snapshot.widgets.push(Self::finish_widget(widget)?);
            }
            Generation3dMountedContainerOwner::Record { root_field: Some(4), owner: Generation3dMountedRecordOwner::Layout { key, value }, field: None, .. } => {
                self.candidate.as_mut().ok_or("generation3d-mounted.snapshot-owner")?.host_snapshot.layout.insert(key, value);
            }
            Generation3dMountedContainerOwner::Strings { parent, field, values } => match self.stack.get_mut(parent) {
                Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Widget(widget), field: active, .. }) => {
                    *widget.lists.get_mut(if field == 4 { 1 } else { 0 }).ok_or("generation3d-mounted.widget-list-field")? = values;
                    *active = None;
                }
                _ => return Err("generation3d-mounted.widget-list-owner"),
            },
            Generation3dMountedContainerOwner::Synapses { rows, .. } if kind == mounted::RetainedValueContainer::Table => {
                let target = &mut self.candidate.as_mut().ok_or("generation3d-mounted.snapshot-owner")?.host_snapshot.synapses;
                for row in rows {
                    target.push(semio_framework_artifact_flow_flow::SynapseSpec { id: row.id, from: row.from, to: row.to, from_port: row.from_port, to_port: row.to_port });
                }
                if let Some(Generation3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMountedContainerOwner::LayoutMap { key: None } if kind == mounted::RetainedValueContainer::Map => {
                if let Some(Generation3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMountedContainerOwner::Generations { rows, .. } if kind == mounted::RetainedValueContainer::Table => {
                let target = &mut self.candidate.as_mut().ok_or("generation3d-mounted.snapshot-owner")?.generation.cold_builder_mut()?.generations;
                for row in rows {
                    target.push(semio_framework_artifact_playbook_playbook::FormGeneration { id: row.id, name: row.name, values: row.values.into_iter().collect() });
                }
                if let Some(Generation3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMountedContainerOwner::Record { root_field: Some(2), field: None, owner: Generation3dMountedRecordOwner::NeuralValue { owner: Generation3dMountedNeuralOwner::TableRow { table, row }, value: Some(value) }, .. } if kind == mounted::RetainedValueContainer::Record => {
                match self.stack.get_mut(table) {
                    Some(Generation3dMountedContainerOwner::Dictionary { rows, field: Some(1), .. }) => {
                        rows.get_mut(row).ok_or("generation3d-mounted.dictionary-value-row")?.value = Some(value);
                    }
                    _ => return Err("generation3d-mounted.dictionary-value-table"),
                }
            }
            Generation3dMountedContainerOwner::Record { root_field: Some(2), field: None, owner: Generation3dMountedRecordOwner::NeuralValue { owner: Generation3dMountedNeuralOwner::EntryRow { entries, row }, value: Some(value) }, .. } if kind == mounted::RetainedValueContainer::Record => {
                match self.stack.get_mut(entries) {
                    Some(Generation3dMountedContainerOwner::DictionaryEntries { rows, .. }) => {
                        rows.get_mut(row).ok_or("generation3d-mounted.dictionary-entry-row")?.value = Some(value);
                    }
                    _ => return Err("generation3d-mounted.dictionary-entry-list"),
                }
                if let Some(Generation3dMountedContainerOwner::DictionaryEntry { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMountedContainerOwner::DictionaryEntry { field: None, .. } if kind == mounted::RetainedValueContainer::Record => {}
            Generation3dMountedContainerOwner::DictionaryEntries { destination, rows } if matches!(kind, mounted::RetainedValueContainer::List | mounted::RetainedValueContainer::Tuple) => self.finish_dictionary(destination, rows)?,
            Generation3dMountedContainerOwner::Dictionary { destination, rows, field: Some(1), .. } if kind == mounted::RetainedValueContainer::Table => self.finish_dictionary(destination, rows)?,
            Generation3dMountedContainerOwner::Wire { .. } if kind == mounted::RetainedValueContainer::Wire => {}
            Generation3dMountedContainerOwner::Statements { .. } => {
                if let Some(Generation3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMountedContainerOwner::Structural { kind: expected, .. } if expected == kind => {
                if let Some(Generation3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Structural, .. } => {}
            _ => return Err("generation3d-mounted.container-owner-mismatch"),
        }
        Ok(())
    }




}
//#endregion 🔖️MountedCanonicalPackSession
