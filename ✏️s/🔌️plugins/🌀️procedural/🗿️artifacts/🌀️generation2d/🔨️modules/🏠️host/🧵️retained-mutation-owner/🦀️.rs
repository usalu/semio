//! 🧵️ Retained mutation owner: the owned field catalog, the maximum domain bytes, the retained mutation owner struct and its string, symbol, json, dsl, dictionary and record stepping. See [`super`].

use super::*;

/// 🎭️ Owner-local exact catalog for the Generation2d fresh-envelope decode cohort.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct Generation2dEnvelopeOwnedFieldCatalog;

pub(super) const GENERATION2D_MAXIMUM_DOMAIN_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

/// 🧬️ Fixed-depth typed owner for the exact fourteen Generation2d mutation records.
/// Dynamic JSON is admitted only at the one ChangeGenerationValue value leaf.
pub(super) struct Generation2dRetainedMutationOwner {
    pub(super) ordinal: u8,
    pub(super) stack: Vec<Generation2dMutationFrame>,
    pub(super) string: Option<Generation2dMutationStringOwner>,
    pub(super) strings: [String; 3],
    pub(super) targets: Vec<String>,
    pub(super) numbers: [f64; 2],
    pub(super) index: usize,
    pub(super) index_present: bool,
    pub(super) widget: Option<semio_framework_artifact_flow_flow::Widget>,
    pub(super) synapse: Option<semio_framework_artifact_flow_flow::SynapseSpec>,
    pub(super) layout: Option<semio_framework_artifact_flow_flow::WidgetLayout>,
    pub(super) camera: Option<semio_framework_artifact_flow_flow::CameraJson>,
    pub(super) generation: Option<semio_framework_artifact_playbook_playbook::FormGeneration>,
    pub(super) json: semio_framework_value::DslValue,
    pub(super) json_stack: Vec<Generation2dMutationJsonFrame>,
    pub(super) json_destination: Option<Generation2dMutationJsonDestination>,
    pub(super) dsl_stack: Vec<Generation2dMutationDslFrame>,
    pub(super) dsl_destination: Option<(usize, usize)>,
    pub(super) pending_table_rows: Option<u64>,
    pub(super) value: std::mem::ManuallyDrop<Option<Generation2dMutation>>,
    pub(super) complete: bool,
    pub(super) handed_back: bool,
}

impl Generation2dRetainedMutationOwner {
    pub(super) fn new(ordinal: u8) -> Result<Self, &'static str> {
        if usize::from(ordinal) >= GENERATION2D_MUTATION_VARIANT_COUNT {
            return Err("generation2d-mutation.variant");
        }
        let mut stack = Vec::new();
        stack.try_reserve_exact(GENERATION2D_RETAINED_STACK_CAPACITY).map_err(|_| "generation2d-mutation.stack-preflight")?;
        let mut json_stack = Vec::new();
        json_stack.try_reserve_exact(GENERATION2D_RETAINED_STACK_CAPACITY).map_err(|_| "generation2d-mutation.json-stack-preflight")?;
        let mut dsl_stack = Vec::new();
        dsl_stack.try_reserve_exact(GENERATION2D_RETAINED_STACK_CAPACITY).map_err(|_| "generation2d-mutation.dsl-stack-preflight")?;
        Ok(Self {
            ordinal,
            stack,
            string: None,
            strings: std::array::from_fn(|_| String::new()),
            targets: Vec::new(),
            numbers: [0.0; 2],
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

    pub(super) fn push(&mut self, frame: Generation2dMutationFrame) -> Result<(), &'static str> {
        if self.stack.len() == self.stack.capacity() {
            return Err("generation2d-mutation.depth");
        }
        self.stack.push(frame);
        Ok(())
    }

    pub(super) fn root_field(&self) -> Option<u16> {
        self.stack.iter().find_map(|frame| match frame {
            Generation2dMutationFrame::Root { field } => *field,
            _ => None,
        })
    }

    fn string_target(&mut self) -> Result<Generation2dMutationStringTarget, &'static str> {
        let index = self.stack.len().checked_sub(1).ok_or("generation2d-mutation.string-owner")?;
        if self.json_destination.is_some() {
            return Ok(match self.json_stack.last() {
                Some(Generation2dMutationJsonFrame::Object { key: None, .. }) => Generation2dMutationStringTarget::JsonKey,
                _ => Generation2dMutationStringTarget::JsonValue,
            });
        }
        if self.dsl_destination.is_some() {
            return Ok(match self.dsl_stack.last() {
                Some(Generation2dMutationDslFrame::Object { key: None, .. }) => Generation2dMutationStringTarget::DslKey,
                _ => Generation2dMutationStringTarget::DslValue,
            });
        }
        match &mut self.stack[index] {
            Generation2dMutationFrame::Root { field: Some(field) } => Ok(Generation2dMutationStringTarget::Root(*field)),
            Generation2dMutationFrame::Statements { keyword: None } => Ok(Generation2dMutationStringTarget::Statement(index)),
            Generation2dMutationFrame::Widget { field: Some(field), .. } => Ok(Generation2dMutationStringTarget::Widget(index, *field)),
            Generation2dMutationFrame::Generation { field: Some(field), .. } => Ok(Generation2dMutationStringTarget::Generation(index, *field)),
            Generation2dMutationFrame::Dictionary { field: Some(0), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation2d-mutation.dictionary-key-row")?;
                *next = row + 1;
                Ok(Generation2dMutationStringTarget::DictionaryKey(index, row))
            }
            Generation2dMutationFrame::DictionaryEntry { field: Some(0), .. } => Ok(Generation2dMutationStringTarget::DictionaryEntryKey(index)),
            Generation2dMutationFrame::NeuralValue { field: Some(4), .. } => Ok(Generation2dMutationStringTarget::NeuralText(index)),
            Generation2dMutationFrame::Strings { .. } => Ok(Generation2dMutationStringTarget::Sequence(index)),
            Generation2dMutationFrame::Synapse { field: Some(0), .. } => Ok(Generation2dMutationStringTarget::SynapseId(index)),
            Generation2dMutationFrame::Wire { roles, roles_len, role, .. } if *role < *roles_len => {
                let target = roles[*role];
                *role += 1;
                Ok(Generation2dMutationStringTarget::Wire(index, target))
            }
            _ => Err("generation2d-mutation.string-role"),
        }
    }

    pub(super) fn begin_string(&mut self) -> Result<(), &'static str> {
        if self.string.is_some() {
            return Err("generation2d-mutation.string-overlap");
        }
        self.string = Some(Generation2dMutationStringOwner { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    pub(super) fn begin_symbol(&mut self, symbol: u64, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<(), &'static str> {
        if self.string.is_none() {
            self.begin_string()?;
        }
        let characters = body.symbol_chars(symbol).map_err(|_| "generation2d-mutation.symbol")?;
        let owner = self.string.as_mut().expect("P2 mutation string retained");
        owner.value.try_reserve_exact(characters).map_err(|_| "generation2d-mutation.symbol-preflight")?;
        owner.symbol = Some((symbol, 0, characters));
        if characters == 0 {
            self.finish_string()?;
        }
        Ok(())
    }

    pub(super) fn grant_symbol(&mut self, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<bool, &'static str> {
        let Some(owner) = self.string.as_mut() else { return Ok(false) };
        let Some((symbol, index, characters)) = owner.symbol else { return Ok(false) };
        owner.value.push(body.symbol_char(symbol, index).map_err(|_| "generation2d-mutation.symbol-char")?.ok_or("generation2d-mutation.symbol-short")?);
        if index + 1 == characters {
            self.finish_string()?;
        } else {
            self.string.as_mut().expect("P2 mutation symbol retained").symbol = Some((symbol, index + 1, characters));
        }
        Ok(true)
    }

    pub(super) fn finish_string(&mut self) -> Result<(), &'static str> {
        let owner = self.string.take().ok_or("generation2d-mutation.string-handoff")?;
        match owner.target {
            Generation2dMutationStringTarget::Root(field) => {
                let slot = match (self.ordinal, field) {
                    (2 | 5 | 6 | 7 | 9 | 11 | 14, 0) => 0,
                    (16, 0) => {
                        self.index = 1;
                        0
                    }
                    (12 | 13, 0) => 0,
                    (12 | 13, 1) => 1,
                    _ => return Err("generation2d-mutation.root-string-field"),
                };
                self.strings[slot] = owner.value;
                if let Some(Generation2dMutationFrame::Root { field }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation2dMutationStringTarget::Statement(index) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Statements { keyword }) => *keyword = Some(owner.value),
                _ => return Err("generation2d-mutation.statement-owner"),
            },
            Generation2dMutationStringTarget::Widget(index, field) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Widget { field: active, owner: widget }) => {
                    *widget.strings.get_mut(field as usize).ok_or("generation2d-mutation.widget-string")? = owner.value;
                    *active = None;
                }
                _ => return Err("generation2d-mutation.widget-owner"),
            },
            Generation2dMutationStringTarget::Generation(index, field) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Generation { field: active, id, name, .. }) => {
                    if field == 0 {
                        *id = owner.value;
                    } else if field == 1 {
                        *name = owner.value;
                    } else {
                        return Err("generation2d-mutation.generation-string");
                    }
                    *active = None;
                }
                _ => return Err("generation2d-mutation.generation-owner"),
            },
            Generation2dMutationStringTarget::DictionaryKey(index, row) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Dictionary { rows, .. }) => rows.get_mut(row).ok_or("generation2d-mutation.dictionary-key-row")?.key = owner.value,
                _ => return Err("generation2d-mutation.dictionary-key-owner"),
            },
            Generation2dMutationStringTarget::DictionaryEntryKey(index) => {
                let (entries, row) = match self.stack.get_mut(index) {
                    Some(Generation2dMutationFrame::DictionaryEntry { entries, row, field }) => {
                        *field = None;
                        (*entries, *row)
                    }
                    _ => return Err("generation2d-mutation.dictionary-entry-owner"),
                };
                match self.stack.get_mut(entries) {
                    Some(Generation2dMutationFrame::DictionaryEntries { rows, .. }) => rows.get_mut(row).ok_or("generation2d-mutation.dictionary-entry-row")?.key = owner.value,
                    _ => return Err("generation2d-mutation.dictionary-entries-owner"),
                }
            }
            Generation2dMutationStringTarget::NeuralText(index) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(4) && value.is_none() => {
                    *value = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String(owner.value)));
                    *field = None;
                }
                _ => return Err("generation2d-mutation.neural-text-owner"),
            },
            Generation2dMutationStringTarget::Sequence(index) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Strings { values, .. }) => values.push(owner.value),
                _ => return Err("generation2d-mutation.sequence-owner"),
            },
            Generation2dMutationStringTarget::SynapseId(index) => match self.stack.get_mut(index) {
                Some(Generation2dMutationFrame::Synapse { field, owner: synapse }) => {
                    synapse.id = owner.value;
                    *field = None;
                }
                _ => return Err("generation2d-mutation.synapse-owner"),
            },
            Generation2dMutationStringTarget::Wire(index, role) => {
                let parent = match self.stack.get(index) {
                    Some(Generation2dMutationFrame::Wire { parent, .. }) => *parent,
                    _ => return Err("generation2d-mutation.wire-owner"),
                };
                let synapse = match self.stack.get_mut(parent) {
                    Some(Generation2dMutationFrame::Synapse { owner, .. }) => owner,
                    _ => return Err("generation2d-mutation.wire-parent"),
                };
                match role {
                    0 => synapse.from = owner.value,
                    2 => synapse.from_port = owner.value,
                    3 => synapse.to = owner.value,
                    5 => synapse.to_port = owner.value,
                    _ => drop(owner.value),
                }
            }
            Generation2dMutationStringTarget::JsonKey => match self.json_stack.last_mut() {
                Some(Generation2dMutationJsonFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation2d-mutation.json-key-owner"),
            },
            Generation2dMutationStringTarget::JsonValue => self.assign_json(semio_framework_value::DslValue::String(owner.value))?,
            Generation2dMutationStringTarget::DslKey => match self.dsl_stack.last_mut() {
                Some(Generation2dMutationDslFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation2d-mutation.dsl-key-owner"),
            },
            Generation2dMutationStringTarget::DslValue => self.assign_dsl(semio_framework_value::DslValue::String(owner.value))?,
        }
        Ok(())
    }

    pub(super) fn assign_json(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
        match self.json_stack.last_mut() {
            Some(Generation2dMutationJsonFrame::Array(values)) => values.push(value),
            Some(Generation2dMutationJsonFrame::Object { values, key }) => {
                let key = key.take().ok_or("generation2d-mutation.json-value-key")?;
                values.push((key, value));
            }
            None => match self.json_destination.take().ok_or("generation2d-mutation.json-destination")? {
                Generation2dMutationJsonDestination::ChangeValue => {
                    self.json = value;
                    match self.stack.last_mut() {
                        Some(Generation2dMutationFrame::Root { field }) if *field == Some(2) => *field = None,
                        _ => return Err("generation2d-mutation.change-value-owner"),
                    }
                }
                Generation2dMutationJsonDestination::Generation(index) => {
                    let values = match value {
                        semio_framework_value::DslValue::Object(values) => values,
                        _ => return Err("generation2d-mutation.generation-values-shape"),
                    };
                    match self.stack.get_mut(index) {
                        Some(Generation2dMutationFrame::Generation { field, values: target, .. }) => {
                            *target = values;
                            *field = None;
                        }
                        _ => return Err("generation2d-mutation.generation-values-owner"),
                    }
                }
            },
        }
        Ok(())
    }

    pub(super) fn begin_json(&mut self, destination: Generation2dMutationJsonDestination) -> Result<(), &'static str> {
        if self.json_destination.is_some() {
            return Err("generation2d-mutation.json-overlap");
        }
        self.json_destination = Some(destination);
        Ok(())
    }

    pub(super) fn assign_dsl(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
        match self.dsl_stack.last_mut() {
            Some(Generation2dMutationDslFrame::Array(values)) => values.push(value),
            Some(Generation2dMutationDslFrame::Object { values, key }) => values.push((key.take().ok_or("generation2d-mutation.dsl-value-key")?, value)),
            None => {
                let (parent, slot) = self.dsl_destination.take().ok_or("generation2d-mutation.dsl-destination")?;
                match self.stack.get_mut(parent) {
                    Some(Generation2dMutationFrame::Widget { field, owner }) if *field == Some((slot + 2) as u16) => {
                        owner.dynamic[slot] = Some(value);
                        *field = None;
                    }
                    _ => return Err("generation2d-mutation.dsl-widget-owner"),
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
            Some(Generation2dMutationFrame::Widget { field: Some(field @ (2 | 3)), owner }) if owner.keyword == "cluster" => *field,
            _ => return false,
        };
        self.dsl_destination = Some((parent, usize::from(field - 2)));
        true
    }

    pub(super) fn end_dsl(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer) -> Result<bool, &'static str> {
        if self.dsl_destination.is_none() {
            return Ok(false);
        }
        let value = match self.dsl_stack.pop().ok_or("generation2d-mutation.dsl-end")? {
            Generation2dMutationDslFrame::Array(values) if kind == store::mounted_pack_rt::RetainedValueContainer::List => semio_framework_value::DslValue::Array(values),
            Generation2dMutationDslFrame::Object { values, key: None } if kind == store::mounted_pack_rt::RetainedValueContainer::Map => semio_framework_value::DslValue::Object(values),
            _ => return Err("generation2d-mutation.dsl-container-mismatch"),
        };
        self.assign_dsl(value)?;
        Ok(true)
    }

    pub(super) fn begin_dictionary(&mut self, destination: Generation2dMutationDictionaryDestination, count: u64) -> Result<(), &'static str> {
        let rows = usize::try_from(count).map_err(|_| "generation2d-mutation.dictionary-count")?;
        let mut values = Vec::new();
        values.try_reserve_exact(rows).map_err(|_| "generation2d-mutation.dictionary-preflight")?;
        values.resize_with(rows, Generation2dMutationDictionaryEntryOwner::default);
        let mut present = Vec::new();
        present.try_reserve_exact(rows).map_err(|_| "generation2d-mutation.dictionary-presence-preflight")?;
        present.resize(rows, false);
        self.push(Generation2dMutationFrame::Dictionary { destination, rows: values, field: None, present, next: 0 })
    }

    pub(super) fn finish_dictionary(&mut self, destination: Generation2dMutationDictionaryDestination, rows: Vec<Generation2dMutationDictionaryEntryOwner>) -> Result<(), &'static str> {
        let mut dictionary = semio_framework_artifact_flow_flow::neural::Dictionary::new();
        for row in rows {
            dictionary = dictionary.insert(row.key, row.value.ok_or("generation2d-mutation.dictionary-value")?);
        }
        match destination {
            Generation2dMutationDictionaryDestination::Widget { parent, field } => match self.stack.get_mut(parent) {
                Some(Generation2dMutationFrame::Widget { field: active, owner }) if *active == Some(field) => {
                    owner.dictionaries[if field == 1 { 1 } else { 0 }] = dictionary;
                    *active = None;
                }
                _ => return Err("generation2d-mutation.dictionary-widget-owner"),
            },
            Generation2dMutationDictionaryDestination::Value { parent } => match self.stack.get_mut(parent) {
                Some(Generation2dMutationFrame::NeuralValue { field, value, .. }) if *field == Some(5) && value.is_none() => {
                    *value = Some(semio_framework_artifact_flow_flow::neural::Value::Dictionary(dictionary));
                    *field = None;
                }
                _ => return Err("generation2d-mutation.dictionary-value-owner"),
            },
        }
        Ok(())
    }

    pub(super) fn finish_widget(owner: Generation2dMutationWidgetOwner) -> Result<semio_framework_artifact_flow_flow::Widget, &'static str> {
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
                tree: semio_framework_value::FromValue::from_value(first_dynamic.ok_or("generation2d-mutation.cluster-tree")?).map_err(|_| "generation2d-mutation.cluster-tree-shape")?,
                flow: semio_framework_value::FromValue::from_value(second_dynamic.ok_or("generation2d-mutation.cluster-flow")?).map_err(|_| "generation2d-mutation.cluster-flow-shape")?,
            },
            _ => return Err("generation2d-mutation.widget-variant"),
        })
    }

    pub(super) fn begin_record(&mut self) -> Result<(), &'static str> {
        if self.stack.is_empty() {
            return self.push(Generation2dMutationFrame::Root { field: None });
        }
        let table = self.stack.len() - 1;
        if let Some(Generation2dMutationFrame::Dictionary { field: Some(1), present, next, .. }) = self.stack.get_mut(table) {
            let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation2d-mutation.dictionary-value-row")?;
            *next = row + 1;
            return self.push(Generation2dMutationFrame::NeuralValue { owner: Generation2dMutationNeuralOwner::TableRow { table, row }, field: None, value: None });
        }
        if let Some(Generation2dMutationFrame::DictionaryEntries { rows, .. }) = self.stack.get_mut(table) {
            let row = rows.len();
            rows.try_reserve(1).map_err(|_| "generation2d-mutation.dictionary-entry-preflight")?;
            rows.push(Generation2dMutationDictionaryEntryOwner::default());
            return self.push(Generation2dMutationFrame::DictionaryEntry { entries: table, row, field: None });
        }
        if let Some(Generation2dMutationFrame::DictionaryEntry { entries, row, field: Some(1) }) = self.stack.get_mut(table) {
            let owner = Generation2dMutationNeuralOwner::EntryRow { entries: *entries, row: *row };
            return self.push(Generation2dMutationFrame::NeuralValue { owner, field: None, value: None });
        }
        let root = self.root_field();
        let frame = match self.stack.last_mut() {
            Some(Generation2dMutationFrame::Statements { keyword }) => {
                let keyword = keyword.take().ok_or("generation2d-mutation.widget-keyword")?;
                Generation2dMutationFrame::Widget { field: None, owner: Generation2dMutationWidgetOwner { keyword, ..Default::default() } }
            }
            _ => match (self.ordinal, root) {
                (3, Some(1)) | (4, Some(0)) => Generation2dMutationFrame::Synapse { field: None, owner: Default::default() },
                (6, Some(1)) => Generation2dMutationFrame::Layout { field: None, value: semio_framework_artifact_flow_flow::WidgetLayout { x: 0.0, y: 0.0 } },
                (8, Some(0)) => Generation2dMutationFrame::Camera { field: None, value: semio_framework_artifact_flow_flow::CameraJson::default() },
                (10, Some(0)) => Generation2dMutationFrame::Generation { field: None, id: String::new(), name: String::new(), values: Vec::new() },
                _ => Generation2dMutationFrame::Structural(store::mounted_pack_rt::RetainedValueContainer::Record),
            },
        };
        self.push(frame)
    }

}
