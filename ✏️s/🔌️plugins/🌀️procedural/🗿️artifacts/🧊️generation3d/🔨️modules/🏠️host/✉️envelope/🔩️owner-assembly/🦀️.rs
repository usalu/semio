//! 🔩️ Assembly half of the retained mutation owner: construction, the string, symbol, JSON and DSL value assembly and the dictionary, widget and record frames. See [`super`].

use super::*;

//#region 🔖️TypedOwnedEnvelopeCatalog
impl Generation3dRetainedMutationOwner {
    pub(super) fn new(ordinal: u8) -> Result<Self, &'static str> {
        if usize::from(ordinal) >= GENERATION3D_MUTATION_VARIANT_COUNT {
            return Err("generation3d-mutation.variant");
        }
        let mut stack = Vec::new();
        stack.try_reserve_exact(GENERATION3D_RETAINED_STACK_CAPACITY).map_err(|_| "generation3d-mutation.stack-preflight")?;
        let mut json_stack = Vec::new();
        json_stack.try_reserve_exact(GENERATION3D_RETAINED_STACK_CAPACITY).map_err(|_| "generation3d-mutation.json-stack-preflight")?;
        let mut dsl_stack = Vec::new();
        dsl_stack.try_reserve_exact(GENERATION3D_RETAINED_STACK_CAPACITY).map_err(|_| "generation3d-mutation.dsl-stack-preflight")?;
        Ok(Self {
            ordinal,
            stack,
            string: None,
            strings: std::array::from_fn(|_| String::new()),
            targets: Vec::new(),
            closing_targets: Default::default(),
            numbers: [0.0; 4],
            index: 0,
            index_present: false,
            widget: None,
            synapse: None,
            layout: None,
            camera: None,
            generation: None,
            json: semio_framework_value::DslValue::Null,
            json_stack,
            json_destination: None,
            dsl_stack,
            dsl_destination: None,
            pending_table_rows: None,
            value: std::mem::ManuallyDrop::new(None),
            complete: false,
            handed_back: false,
        })
    }

    pub(super) fn push(&mut self, frame: Generation3dMutationFrame) -> Result<(), &'static str> {
        if self.stack.len() == self.stack.capacity() {
            return Err("generation3d-mutation.depth");
        }
        self.stack.push(frame);
        Ok(())
    }

    pub(super) fn root_field(&self) -> Option<u16> {
        self.stack.iter().find_map(|frame| match frame {
            Generation3dMutationFrame::Root { field } => *field,
            _ => None,
        })
    }

    fn string_target(&mut self) -> Result<Generation3dMutationStringTarget, &'static str> {
        let index = self.stack.len().checked_sub(1).ok_or("generation3d-mutation.string-owner")?;
        if self.json_destination.is_some() {
            return Ok(match self.json_stack.last() {
                Some(Generation3dMutationJsonFrame::Object { key: None, .. }) => Generation3dMutationStringTarget::JsonKey,
                _ => Generation3dMutationStringTarget::JsonValue,
            });
        }
        if self.dsl_destination.is_some() {
            return Ok(match self.dsl_stack.last() {
                Some(Generation3dMutationDslFrame::Object { key: None, .. }) => Generation3dMutationStringTarget::DslKey,
                _ => Generation3dMutationStringTarget::DslValue,
            });
        }
        match &mut self.stack[index] {
            Generation3dMutationFrame::Root { field: Some(field) } => Ok(Generation3dMutationStringTarget::Root(*field)),
            Generation3dMutationFrame::Statements { keyword: None } => Ok(Generation3dMutationStringTarget::Statement(index)),
            Generation3dMutationFrame::Widget { field: Some(field), .. } => Ok(Generation3dMutationStringTarget::Widget(index, *field)),
            Generation3dMutationFrame::Generation { field: Some(field), .. } => Ok(Generation3dMutationStringTarget::Generation(index, *field)),
            Generation3dMutationFrame::Dictionary { field: Some(0), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation3d-mutation.dictionary-key-row")?;
                *next = row + 1;
                Ok(Generation3dMutationStringTarget::DictionaryKey(index, row))
            }
            Generation3dMutationFrame::NeuralValue { field: Some(4), .. } => Ok(Generation3dMutationStringTarget::NeuralText(index)),
            Generation3dMutationFrame::DictionaryEntry { field: Some(0), .. } => Ok(Generation3dMutationStringTarget::DictionaryEntryKey(index)),
            Generation3dMutationFrame::Strings { .. } => Ok(Generation3dMutationStringTarget::Sequence(index)),
            Generation3dMutationFrame::Synapse { field: Some(0), .. } => Ok(Generation3dMutationStringTarget::SynapseId(index)),
            Generation3dMutationFrame::Wire { roles, roles_len, role, .. } if *role < *roles_len => {
                let target = roles[*role];
                *role += 1;
                Ok(Generation3dMutationStringTarget::Wire(index, target))
            }
            _ => Err("generation3d-mutation.string-role"),
        }
    }

    pub(super) fn begin_string(&mut self) -> Result<(), &'static str> {
        if self.string.is_some() {
            return Err("generation3d-mutation.string-overlap");
        }
        self.string = Some(Generation3dMutationStringOwner { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    pub(super) fn begin_symbol(&mut self, symbol: u64, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<(), &'static str> {
        if self.string.is_none() {
            self.begin_string()?;
        }
        let characters = body.symbol_chars(symbol).map_err(|_| "generation3d-mutation.symbol")?;
        let owner = self.string.as_mut().expect("P3 mutation string retained");
        owner.value.try_reserve_exact(characters).map_err(|_| "generation3d-mutation.symbol-preflight")?;
        owner.symbol = Some((symbol, 0, characters));
        if characters == 0 {
            self.finish_string()?;
        }
        Ok(())
    }

    pub(super) fn grant_symbol(&mut self, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<bool, &'static str> {
        let Some(owner) = self.string.as_mut() else { return Ok(false) };
        let Some((symbol, index, characters)) = owner.symbol else { return Ok(false) };
        owner.value.push(body.symbol_char(symbol, index).map_err(|_| "generation3d-mutation.symbol-char")?.ok_or("generation3d-mutation.symbol-short")?);
        if index + 1 == characters {
            self.finish_string()?;
        } else {
            self.string.as_mut().expect("P3 mutation symbol retained").symbol = Some((symbol, index + 1, characters));
        }
        Ok(true)
    }

    pub(super) fn finish_string(&mut self) -> Result<(), &'static str> {
        let owner = self.string.take().ok_or("generation3d-mutation.string-handoff")?;
        match owner.target {
            Generation3dMutationStringTarget::Root(field) => {
                let slot = match (self.ordinal, field) {
                    (2 | 5 | 6 | 7 | 9 | 11 | 14, 0) => 0,
                    (12 | 13 | 19, 0) => 0,
                    (12 | 13 | 19, 1) => 1,
                    (20 | 21, 0) => {
                        self.index = 1;
                        0
                    }
                    _ => return Err("generation3d-mutation.root-string-field"),
                };
                self.strings[slot] = owner.value;
                if let Some(Generation3dMutationFrame::Root { field }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMutationStringTarget::Statement(index) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Statements { keyword }) => *keyword = Some(owner.value),
                _ => return Err("generation3d-mutation.statement-owner"),
            },
            Generation3dMutationStringTarget::Widget(index, field) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Widget { field: active, owner: widget }) => {
                    *widget.strings.get_mut(field as usize).ok_or("generation3d-mutation.widget-string")? = owner.value;
                    *active = None;
                }
                _ => return Err("generation3d-mutation.widget-owner"),
            },
            Generation3dMutationStringTarget::Generation(index, field) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Generation { field: active, id, name, .. }) => {
                    if field == 0 {
                        *id = owner.value;
                    } else if field == 1 {
                        *name = owner.value;
                    } else {
                        return Err("generation3d-mutation.generation-string");
                    }
                    *active = None;
                }
                _ => return Err("generation3d-mutation.generation-owner"),
            },
            Generation3dMutationStringTarget::DictionaryKey(index, row) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Dictionary { rows, .. }) => rows.get_mut(row).ok_or("generation3d-mutation.dictionary-key-row")?.key = owner.value,
                _ => return Err("generation3d-mutation.dictionary-key-owner"),
            },
            Generation3dMutationStringTarget::DictionaryEntryKey(index) => {
                let (entries, row) = match self.stack.get_mut(index) {
                    Some(Generation3dMutationFrame::DictionaryEntry { entries, row, field }) => {
                        *field = None;
                        (*entries, *row)
                    }
                    _ => return Err("generation3d-mutation.dictionary-entry-owner"),
                };
                match self.stack.get_mut(entries) {
                    Some(Generation3dMutationFrame::DictionaryEntries { rows, .. }) => rows.get_mut(row).ok_or("generation3d-mutation.dictionary-entry-row")?.key = owner.value,
                    _ => return Err("generation3d-mutation.dictionary-entries-owner"),
                }
            }
            Generation3dMutationStringTarget::NeuralText(index) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(4) && value.is_none() => {
                    *value = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String(owner.value)));
                    *field = None;
                }
                _ => return Err("generation3d-mutation.neural-text-owner"),
            },
            Generation3dMutationStringTarget::Sequence(index) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Strings { values, .. }) => values.push(owner.value),
                _ => return Err("generation3d-mutation.sequence-owner"),
            },
            Generation3dMutationStringTarget::SynapseId(index) => match self.stack.get_mut(index) {
                Some(Generation3dMutationFrame::Synapse { field, owner: synapse }) => {
                    synapse.id = owner.value;
                    *field = None;
                }
                _ => return Err("generation3d-mutation.synapse-owner"),
            },
            Generation3dMutationStringTarget::Wire(index, role) => {
                let parent = match self.stack.get(index) {
                    Some(Generation3dMutationFrame::Wire { parent, .. }) => *parent,
                    _ => return Err("generation3d-mutation.wire-owner"),
                };
                let synapse = match self.stack.get_mut(parent) {
                    Some(Generation3dMutationFrame::Synapse { owner, .. }) => owner,
                    _ => return Err("generation3d-mutation.wire-parent"),
                };
                match role {
                    0 => synapse.from = owner.value,
                    2 => synapse.from_port = owner.value,
                    3 => synapse.to = owner.value,
                    5 => synapse.to_port = owner.value,
                    _ => drop(owner.value),
                }
            }
            Generation3dMutationStringTarget::JsonKey => match self.json_stack.last_mut() {
                Some(Generation3dMutationJsonFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation3d-mutation.json-key-owner"),
            },
            Generation3dMutationStringTarget::JsonValue => self.assign_json(semio_framework_value::DslValue::String(owner.value))?,
            Generation3dMutationStringTarget::DslKey => match self.dsl_stack.last_mut() {
                Some(Generation3dMutationDslFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation3d-mutation.dsl-key-owner"),
            },
            Generation3dMutationStringTarget::DslValue => self.assign_dsl(semio_framework_value::DslValue::String(owner.value))?,
        }
        Ok(())
    }

    pub(super) fn assign_json(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
        match self.json_stack.last_mut() {
            Some(Generation3dMutationJsonFrame::Array(values)) => values.push(value),
            Some(Generation3dMutationJsonFrame::Object { values, key }) => {
                let key = key.take().ok_or("generation3d-mutation.json-value-key")?;
                values.push((key, value));
            }
            None => match self.json_destination.take().ok_or("generation3d-mutation.json-destination")? {
                Generation3dMutationJsonDestination::ChangeValue => {
                    self.json = value;
                    match self.stack.last_mut() {
                        Some(Generation3dMutationFrame::Root { field }) if *field == Some(2) => *field = None,
                        _ => return Err("generation3d-mutation.change-value-owner"),
                    }
                }
                Generation3dMutationJsonDestination::Generation(index) => {
                    let values = match value {
                        semio_framework_value::DslValue::Object(values) => values,
                        _ => return Err("generation3d-mutation.generation-values-shape"),
                    };
                    match self.stack.get_mut(index) {
                        Some(Generation3dMutationFrame::Generation { field, values: target, .. }) => {
                            *target = values;
                            *field = None;
                        }
                        _ => return Err("generation3d-mutation.generation-values-owner"),
                    }
                }
            },
        }
        Ok(())
    }

    pub(super) fn begin_json(&mut self, destination: Generation3dMutationJsonDestination) -> Result<(), &'static str> {
        if self.json_destination.is_some() {
            return Err("generation3d-mutation.json-overlap");
        }
        self.json_destination = Some(destination);
        Ok(())
    }

    pub(super) fn assign_dsl(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
        match self.dsl_stack.last_mut() {
            Some(Generation3dMutationDslFrame::Array(values)) => values.push(value),
            Some(Generation3dMutationDslFrame::Object { values, key }) => values.push((key.take().ok_or("generation3d-mutation.dsl-value-key")?, value)),
            None => {
                let (parent, slot) = self.dsl_destination.take().ok_or("generation3d-mutation.dsl-destination")?;
                match self.stack.get_mut(parent) {
                    Some(Generation3dMutationFrame::Widget { field, owner }) if *field == Some((slot + 2) as u16) => {
                        owner.dynamic[slot] = Some(value);
                        *field = None;
                    }
                    _ => return Err("generation3d-mutation.dsl-widget-owner"),
                }
            }
        }
        Ok(())
    }

    pub(super) fn begin_dsl(&mut self) -> bool {
        if self.dsl_destination.is_some() {
            return true;
        }
        let Some(parent) = self.stack.len().checked_sub(1) else { return false };
        let field = match self.stack.get(parent) {
            Some(Generation3dMutationFrame::Widget { field: Some(field @ (2 | 3)), owner }) if owner.keyword == "cluster" => *field,
            _ => return false,
        };
        self.dsl_destination = Some((parent, usize::from(field - 2)));
        true
    }

    pub(super) fn end_dsl(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer) -> Result<bool, &'static str> {
        if self.dsl_destination.is_none() {
            return Ok(false);
        }
        let value = match self.dsl_stack.pop().ok_or("generation3d-mutation.dsl-end")? {
            Generation3dMutationDslFrame::Array(values) if kind == store::mounted_pack_rt::RetainedValueContainer::List => semio_framework_value::DslValue::Array(values),
            Generation3dMutationDslFrame::Object { values, key: None } if kind == store::mounted_pack_rt::RetainedValueContainer::Map => semio_framework_value::DslValue::Object(values),
            _ => return Err("generation3d-mutation.dsl-container-mismatch"),
        };
        self.assign_dsl(value)?;
        Ok(true)
    }

    pub(super) fn begin_dictionary(&mut self, destination: Generation3dMutationDictionaryDestination, count: u64) -> Result<(), &'static str> {
        let rows = usize::try_from(count).map_err(|_| "generation3d-mutation.dictionary-count")?;
        let mut values = Vec::new();
        values.try_reserve_exact(rows).map_err(|_| "generation3d-mutation.dictionary-preflight")?;
        values.resize_with(rows, Generation3dMutationDictionaryEntryOwner::default);
        let mut present = Vec::new();
        present.try_reserve_exact(rows).map_err(|_| "generation3d-mutation.dictionary-presence-preflight")?;
        present.resize(rows, false);
        self.push(Generation3dMutationFrame::Dictionary { destination, rows: values, field: None, present, next: 0 })
    }

    pub(super) fn finish_dictionary(&mut self, destination: Generation3dMutationDictionaryDestination, rows: Vec<Generation3dMutationDictionaryEntryOwner>) -> Result<(), &'static str> {
        let mut dictionary = semio_framework_artifact_flow_flow::neural::Dictionary::new();
        for row in rows {
            dictionary = dictionary.insert(row.key, row.value.ok_or("generation3d-mutation.dictionary-value")?);
        }
        match destination {
            Generation3dMutationDictionaryDestination::Widget { parent, field } => match self.stack.get_mut(parent) {
                Some(Generation3dMutationFrame::Widget { field: active, owner }) if *active == Some(field) => {
                    owner.dictionaries[if field == 1 { 1 } else { 0 }] = dictionary;
                    *active = None;
                }
                _ => return Err("generation3d-mutation.dictionary-widget-owner"),
            },
            Generation3dMutationDictionaryDestination::Value { parent } => match self.stack.get_mut(parent) {
                Some(Generation3dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(5) && value.is_none() => {
                    *value = Some(semio_framework_artifact_flow_flow::neural::Value::Dictionary(dictionary));
                    *field = None;
                }
                _ => return Err("generation3d-mutation.dictionary-value-owner"),
            },
        }
        Ok(())
    }

    pub(super) fn finish_widget(owner: Generation3dMutationWidgetOwner) -> Result<semio_framework_artifact_flow_flow::Widget, &'static str> {
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
                tree: semio_framework_value::FromValue::from_value(first_dynamic.ok_or("generation3d-mutation.cluster-tree")?).map_err(|_| "generation3d-mutation.cluster-tree-shape")?,
                flow: semio_framework_value::FromValue::from_value(second_dynamic.ok_or("generation3d-mutation.cluster-flow")?).map_err(|_| "generation3d-mutation.cluster-flow-shape")?,
            },
            _ => return Err("generation3d-mutation.widget-variant"),
        })
    }

    pub(super) fn begin_record(&mut self) -> Result<(), &'static str> {
        if self.stack.is_empty() {
            return self.push(Generation3dMutationFrame::Root { field: None });
        }
        let table = self.stack.len() - 1;
        if let Some(Generation3dMutationFrame::Dictionary { field: Some(1), present, next, .. }) = self.stack.get_mut(table) {
            let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation3d-mutation.dictionary-value-row")?;
            *next = row + 1;
            return self.push(Generation3dMutationFrame::NeuralValue { owner: Generation3dMutationNeuralOwner::TableRow { table, row }, field: None, value: None });
        }
        if let Some(Generation3dMutationFrame::DictionaryEntries { rows, .. }) = self.stack.get_mut(table) {
            let row = rows.len();
            rows.try_reserve(1).map_err(|_| "generation3d-mutation.dictionary-entry-preflight")?;
            rows.push(Generation3dMutationDictionaryEntryOwner::default());
            return self.push(Generation3dMutationFrame::DictionaryEntry { entries: table, row, field: None });
        }
        if let Some(Generation3dMutationFrame::DictionaryEntry { entries, row, field: Some(1) }) = self.stack.get_mut(table) {
            let owner = Generation3dMutationNeuralOwner::EntryRow { entries: *entries, row: *row };
            return self.push(Generation3dMutationFrame::NeuralValue { owner, field: None, value: None });
        }
        let root = self.root_field();
        let frame = match self.stack.last_mut() {
            Some(Generation3dMutationFrame::Statements { keyword }) => {
                let keyword = keyword.take().ok_or("generation3d-mutation.widget-keyword")?;
                Generation3dMutationFrame::Widget { field: None, owner: Generation3dMutationWidgetOwner { keyword, ..Default::default() } }
            }
            _ => match (self.ordinal, root) {
                (3, Some(1)) | (4, Some(0)) => Generation3dMutationFrame::Synapse { field: None, owner: Default::default() },
                (6, Some(1)) => Generation3dMutationFrame::Layout { field: None, value: semio_framework_artifact_flow_flow::WidgetLayout { x: 0.0, y: 0.0 } },
                (8, Some(0)) => Generation3dMutationFrame::Camera { field: None, value: semio_framework_artifact_flow_flow::CameraJson::default() },
                (10, Some(0)) => Generation3dMutationFrame::Generation { field: None, id: String::new(), name: String::new(), values: Vec::new() },
                _ => Generation3dMutationFrame::Structural(store::mounted_pack_rt::RetainedValueContainer::Record),
            },
        };
        self.push(frame)
    }
}
//#endregion 🔖️TypedOwnedEnvelopeCatalog
