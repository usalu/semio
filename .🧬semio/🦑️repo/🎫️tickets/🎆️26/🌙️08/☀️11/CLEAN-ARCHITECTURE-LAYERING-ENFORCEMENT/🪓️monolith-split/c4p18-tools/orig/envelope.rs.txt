//! ✉️ Generation3d typed owned envelope catalog: the retained mutation owners, the mutation session, the snapshot and mutation decode authorities and the owned field catalog of the document envelope.
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#![allow(unused_imports)]
use semio_s_artifact_procedural_3d_mut_generations::change_generation_value::ChangeGenerationValue;
use semio_s_artifact_procedural_3d_mut_document::change_schema::ChangeSchema;
use semio_s_artifact_procedural_3d_mut_synapses::connect_synapse::ConnectSynapse;
use semio_s_artifact_procedural_3d_mut_generations::create_generation::CreateGeneration;
use semio_s_artifact_procedural_3d_mut_widgets::create_widget::CreateWidget;
use semio_s_artifact_procedural_3d_mut_generations::delete_generation::DeleteGeneration;
use semio_s_artifact_procedural_3d_mut_widgets::delete_widget::DeleteWidget;
use semio_s_artifact_procedural_3d_mut_widgets::delete_widget_position::DeleteWidgetPosition;
use semio_s_artifact_procedural_3d_mut_synapses::disconnect_synapse::DisconnectSynapse;
use semio_s_artifact_procedural_3d_mut_widgets::move_widget::MoveWidget;
use semio_s_artifact_procedural_3d_mut_generations::rename_generation::RenameGeneration;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_mut_document::update_camera::UpdateCamera;
use semio_s_artifact_procedural_3d_mut_synapses::update_synapse::UpdateSynapse;
use semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget;
use semio_s_artifact_procedural_3d_io_text::snapshot::camera_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::camera_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::form_generation_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::form_generation_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::layout_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::layout_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::synapse_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::synapse_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::widget_from_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::widget_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::CameraJsonDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::FormGenerationDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::SynapseSpecDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::WidgetDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::WidgetLayoutDsl;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_mut_widgets::change_slider_value::ChangeSliderValue;
use semio_s_artifact_procedural_3d_mut_transforms::drag_transforms::DragTransforms;
use semio_s_artifact_procedural_3d_mut_transforms::rotate_transforms::RotateTransforms;
use semio_s_artifact_procedural_3d_mut_transforms::scale_transforms::ScaleTransforms;
use semio_s_artifact_procedural_3d_mut_widgets::move_nodes::MoveNodes;
use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::ChangeWidgetInput;
use semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputValue;
use semio_s_artifact_procedural_3d_mut_generations::change_generation_preview::ChangeGenerationPreview;
use semio_s_artifact_procedural_3d_mut_generations::select_generation::SelectGeneration;
use protocol::OpBinary;
use store::ErasedSnapshotRetirement;
use semio_s_artifact_procedural_3d_host::GENERATION3D_MAXIMUM_DOMAIN_BYTES;
use semio_s_artifact_procedural_3d_host::GENERATION3D_MAXIMUM_DOMAIN_ITEMS;
use semio_s_artifact_procedural_3d_host::GENERATION3D_MUTATION_VARIANT_COUNT;
use semio_s_artifact_procedural_3d_host::GENERATION3D_OWNER_BYTES;
use semio_s_artifact_procedural_3d_host::GENERATION3D_RETAINED_STACK_CAPACITY;
use semio_s_artifact_procedural_3d_host::generation3d_close_flow_frontier;
use semio_s_artifact_procedural_3d_host::generation3d_park_flow_strings;
use semio_s_artifact_procedural_3d_host::generation3d_publication_item_credit;

//#region 🔖️TypedOwnedEnvelopeCatalog

#[derive(Default)]
struct Generation3dMutationWidgetOwner {
    keyword: String,
    strings: [String; 4],
    numbers: [f64; 4],
    boolean: bool,
    lists: [Vec<String>; 2],
    dictionaries: [semio_framework_artifact_flow_flow::neural::Dictionary; 2],
    dynamic: [Option<semio_framework_value::DslValue>; 2],
}

#[derive(Default)]
struct Generation3dMutationSynapseOwner {
    id: String,
    from: String,
    to: String,
    from_port: String,
    to_port: String,
}

#[derive(Default)]
struct Generation3dMutationDictionaryEntryOwner {
    key: String,
    value: Option<semio_framework_artifact_flow_flow::neural::Value>,
}

#[derive(Clone, Copy)]
enum Generation3dMutationDictionaryDestination {
    Widget { parent: usize, field: u16 },
    Value { parent: usize },
}

/// 🗂️ Where a decoded neural value belongs. A `Dictionary` reaches the wire either as a columnar
/// `Table` (many rows) or as a `List` of one-entry records, and the retained owner has to write the
/// value back into whichever of the two shapes it is standing in.
enum Generation3dMutationNeuralOwner {
    TableRow { table: usize, row: usize },
    EntryRow { entries: usize, row: usize },
}

enum Generation3dMutationFrame {
    Root { field: Option<u16> },
    Statements { keyword: Option<String> },
    Widget { field: Option<u16>, owner: Generation3dMutationWidgetOwner },
    Synapse { field: Option<u16>, owner: Generation3dMutationSynapseOwner },
    Layout { field: Option<u16>, value: semio_framework_artifact_flow_flow::WidgetLayout },
    Camera { field: Option<u16>, value: semio_framework_artifact_flow_flow::CameraJson },
    Generation { field: Option<u16>, id: String, name: String, values: Vec<(String, semio_framework_value::DslValue)> },
    Dictionary { destination: Generation3dMutationDictionaryDestination, rows: Vec<Generation3dMutationDictionaryEntryOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    DictionaryEntries { destination: Generation3dMutationDictionaryDestination, rows: Vec<Generation3dMutationDictionaryEntryOwner> },
    DictionaryEntry { entries: usize, row: usize, field: Option<u16> },
    NeuralValue { owner: Generation3dMutationNeuralOwner, field: Option<u16>, value: Option<semio_framework_artifact_flow_flow::neural::Value> },
    Strings { parent: usize, field: u16, values: Vec<String> },
    Wire { parent: usize, roles: [u8; 6], roles_len: usize, role: usize, nodes: usize },
    Structural(store::mounted_pack_rt::RetainedValueContainer),
}

#[derive(Clone, Copy)]
enum Generation3dMutationStringTarget {
    Root(u16),
    Widget(usize, u16),
    Generation(usize, u16),
    DictionaryKey(usize, usize),
    DictionaryEntryKey(usize),
    NeuralText(usize),
    Sequence(usize),
    Statement(usize),
    SynapseId(usize),
    Wire(usize, u8),
    JsonKey,
    JsonValue,
    DslKey,
    DslValue,
}

enum Generation3dMutationJsonFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

enum Generation3dMutationDslFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

#[derive(Clone, Copy)]
enum Generation3dMutationJsonDestination {
    ChangeValue,
    Generation(usize),
}

struct Generation3dMutationStringOwner {
    target: Generation3dMutationStringTarget,
    value: String,
    remaining: Option<u64>,
    symbol: Option<(u64, usize, usize)>,
}

/// 🧬️ Fixed-depth typed owner for the exact twenty-two Generation3d mutation records.
/// Dynamic JSON is admitted only at the ChangeGenerationValue value and the ChangeWidgetInput input leaves.
struct Generation3dRetainedMutationOwner {
    ordinal: u8,
    stack: Vec<Generation3dMutationFrame>,
    string: Option<Generation3dMutationStringOwner>,
    strings: [String; 3],
    targets: Vec<String>,
    closing_targets: semio_framework_artifact_flow_flow::retained::FlowRetirement,
    numbers: [f64; 4],
    index: usize,
    index_present: bool,
    widget: Option<semio_framework_artifact_flow_flow::Widget>,
    synapse: Option<semio_framework_artifact_flow_flow::SynapseSpec>,
    layout: Option<semio_framework_artifact_flow_flow::WidgetLayout>,
    camera: Option<semio_framework_artifact_flow_flow::CameraJson>,
    generation: Option<semio_framework_artifact_playbook_playbook::FormGeneration>,
    json: semio_framework_value::DslValue,
    json_stack: Vec<Generation3dMutationJsonFrame>,
    json_destination: Option<Generation3dMutationJsonDestination>,
    dsl_stack: Vec<Generation3dMutationDslFrame>,
    dsl_destination: Option<(usize, usize)>,
    pending_table_rows: Option<u64>,
    value: std::mem::ManuallyDrop<Option<Generation3dMutation>>,
    complete: bool,
    handed_back: bool,
}

impl Generation3dRetainedMutationOwner {
    fn new(ordinal: u8) -> Result<Self, &'static str> {
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

    fn push(&mut self, frame: Generation3dMutationFrame) -> Result<(), &'static str> {
        if self.stack.len() == self.stack.capacity() {
            return Err("generation3d-mutation.depth");
        }
        self.stack.push(frame);
        Ok(())
    }

    fn root_field(&self) -> Option<u16> {
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

    fn begin_string(&mut self) -> Result<(), &'static str> {
        if self.string.is_some() {
            return Err("generation3d-mutation.string-overlap");
        }
        self.string = Some(Generation3dMutationStringOwner { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    fn begin_symbol(&mut self, symbol: u64, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<(), &'static str> {
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

    fn grant_symbol(&mut self, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<bool, &'static str> {
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

    fn finish_string(&mut self) -> Result<(), &'static str> {
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

    fn assign_json(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
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

    fn begin_json(&mut self, destination: Generation3dMutationJsonDestination) -> Result<(), &'static str> {
        if self.json_destination.is_some() {
            return Err("generation3d-mutation.json-overlap");
        }
        self.json_destination = Some(destination);
        Ok(())
    }

    fn assign_dsl(&mut self, value: semio_framework_value::DslValue) -> Result<(), &'static str> {
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

    fn begin_dsl(&mut self) -> bool {
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

    fn end_dsl(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer) -> Result<bool, &'static str> {
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

    fn begin_dictionary(&mut self, destination: Generation3dMutationDictionaryDestination, count: u64) -> Result<(), &'static str> {
        let rows = usize::try_from(count).map_err(|_| "generation3d-mutation.dictionary-count")?;
        let mut values = Vec::new();
        values.try_reserve_exact(rows).map_err(|_| "generation3d-mutation.dictionary-preflight")?;
        values.resize_with(rows, Generation3dMutationDictionaryEntryOwner::default);
        let mut present = Vec::new();
        present.try_reserve_exact(rows).map_err(|_| "generation3d-mutation.dictionary-presence-preflight")?;
        present.resize(rows, false);
        self.push(Generation3dMutationFrame::Dictionary { destination, rows: values, field: None, present, next: 0 })
    }

    fn finish_dictionary(&mut self, destination: Generation3dMutationDictionaryDestination, rows: Vec<Generation3dMutationDictionaryEntryOwner>) -> Result<(), &'static str> {
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

    fn finish_widget(owner: Generation3dMutationWidgetOwner) -> Result<semio_framework_artifact_flow_flow::Widget, &'static str> {
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

    fn begin_record(&mut self) -> Result<(), &'static str> {
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

    fn accept(&mut self, token: store::mounted_pack_rt::RetainedValueToken, body: &store::mounted_pack_rt::RetainedRecordBodyCursor) -> Result<(), &'static str> {
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
            Token::Begin { kind: Container::Table, count } => {
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
            }
            Token::Begin { kind: Container::Record, .. } => self.begin_record()?,
            Token::Begin { kind: Container::Statements, .. } => self.push(Generation3dMutationFrame::Statements { keyword: None })?,
            Token::Begin { kind: Container::List | Container::Tuple, count } => {
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
            }
            Token::Begin { kind: Container::Wire, .. } => {
                let parent = self.stack.len().checked_sub(1).ok_or("generation3d-mutation.wire-parent")?;
                self.push(Generation3dMutationFrame::Wire { parent, roles: [0; 6], roles_len: 0, role: 0, nodes: 0 })?;
            }
            Token::Begin { kind, .. } => self.push(Generation3dMutationFrame::Structural(kind))?,
            Token::Unsigned { role: Role::FieldId, value } if value <= u16::MAX as u64 => match self.stack.last_mut() {
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
            },
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
            Token::F64(bits) => {
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
            }
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
            Token::Tag { value: 0x01 | 0x02, .. } => {
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
            }
            Token::Tag { value: 0x12, .. } if self.json_destination.is_some() => self.assign_json(semio_framework_value::DslValue::Null)?,
            Token::Tag { value: 0x12, .. } if self.dsl_destination.is_some() => self.assign_dsl(semio_framework_value::DslValue::Null)?,
            Token::WirePresence(_) => {}
            Token::WireNodePresence(presence) => match self.stack.last_mut() {
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
            },
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
            Token::End(kind) => {
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
            }
            Token::Complete { .. } => {
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
            }
            Token::Tag { .. } | Token::Unsigned { .. } | Token::Byte(_) | Token::WireLabelPresence(_) => {}
        }
        Ok(())
    }

    fn take(&mut self) -> Option<Generation3dMutation> {
        if !self.complete || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.value.take()
    }

    fn close_step(&mut self) -> bool {
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

    fn terminal_is_empty(&self) -> bool {
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

impl Drop for Generation3dRetainedMutationOwner {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "Generation3d retained mutation owner reached Drop before handoff or terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation3dMutationSessionPhase {
    Format,
    Ordinal,
    Body,
    Ready,
    Published,
    Closing,
    Closed,
}

struct Generation3dMutationSession {
    phase: Generation3dMutationSessionPhase,
    expected_bytes: usize,
    maximum_items: usize,
    admitted: usize,
    pending: Option<(u64, u8)>,
    ordinal: u64,
    ordinal_shift: u32,
    ordinal_bytes: u8,
    body_bytes: u64,
    sealed: bool,
    body: std::mem::ManuallyDrop<Option<store::mounted_pack_rt::RetainedRecordBodyCursor>>,
    owner: std::mem::ManuallyDrop<Option<Generation3dRetainedMutationOwner>>,
}

impl Generation3dMutationSession {
    fn new(expected_bytes: usize, maximum_items: usize) -> Result<Self, &'static str> {
        if !(3..=GENERATION3D_OWNER_BYTES).contains(&expected_bytes) || maximum_items == 0 {
            return Err("generation3d-mutation.exact-credits");
        }
        Ok(Self {
            phase: Generation3dMutationSessionPhase::Format,
            expected_bytes,
            maximum_items,
            admitted: 0,
            pending: None,
            ordinal: 0,
            ordinal_shift: 0,
            ordinal_bytes: 0,
            body_bytes: 0,
            sealed: false,
            body: std::mem::ManuallyDrop::new(None),
            owner: std::mem::ManuallyDrop::new(None),
        })
    }

    fn admit_byte(&mut self, value: u8) -> Result<(), u8> {
        if self.pending.is_some() || self.sealed || self.admitted == self.expected_bytes {
            return Err(value);
        }
        self.pending = Some((self.admitted as u64, value));
        self.admitted += 1;
        Ok(())
    }

    fn ingress_ready(&self) -> bool {
        self.pending.is_none()
            && (self.phase != Generation3dMutationSessionPhase::Body
                || self.body.as_ref().is_some_and(store::mounted_pack_rt::RetainedRecordBodyCursor::ingress_ready))
    }

    fn seal(&mut self) -> Result<(), &'static str> {
        if self.pending.is_some() || self.admitted != self.expected_bytes || self.phase != Generation3dMutationSessionPhase::Body {
            return Err("generation3d-mutation.exact-byte-seal");
        }
        self.body.as_mut().ok_or("generation3d-mutation.body-owner")?.seal(self.body_bytes).map_err(|_| "generation3d-mutation.body-seal")?;
        self.sealed = true;
        Ok(())
    }

    fn grant(&mut self) -> Result<bool, &'static str> {
        if matches!(self.phase, Generation3dMutationSessionPhase::Ready | Generation3dMutationSessionPhase::Published) {
            return Ok(true);
        }
        if let (Some(owner), Some(body)) = (self.owner.as_mut(), self.body.as_ref()) {
            if owner.grant_symbol(body)? {
                return Ok(false);
            }
        }
        match self.phase {
            Generation3dMutationSessionPhase::Format => {
                let (_, byte) = self.pending.take().ok_or("generation3d-mutation.format-input")?;
                if byte != dsl::variants_binary::OP_BINARY_FORMAT {
                    return Err("generation3d-mutation.format");
                }
                self.phase = Generation3dMutationSessionPhase::Ordinal;
            }
            Generation3dMutationSessionPhase::Ordinal => {
                let (_, byte) = self.pending.take().ok_or("generation3d-mutation.ordinal-input")?;
                if self.ordinal_bytes >= 10 || (self.ordinal_bytes == 9 && (byte & 0xfe) != 0) {
                    return Err("generation3d-mutation.ordinal-overflow");
                }
                self.ordinal |= u64::from(byte & 0x7f) << self.ordinal_shift;
                self.ordinal_shift += 7;
                self.ordinal_bytes += 1;
                if byte & 0x80 == 0 {
                    if self.ordinal_bytes > 1 && byte & 0x7f == 0 {
                        return Err("generation3d-mutation.ordinal-noncanonical");
                    }
                    let ordinal = u8::try_from(self.ordinal).map_err(|_| "generation3d-mutation.variant")?;
                    let limits = store::mounted_pack_rt::PackLimits {
                        max_file_len: self.expected_bytes as u64,
                        max_segment_len: self.expected_bytes as u64,
                        max_symbols: self.maximum_items.min(GENERATION3D_MAXIMUM_DOMAIN_ITEMS) as u32,
                        max_depth: GENERATION3D_RETAINED_STACK_CAPACITY as u16,
                        max_items: self.maximum_items.min(GENERATION3D_MAXIMUM_DOMAIN_ITEMS) as u64,
                        max_total_alloc: GENERATION3D_MAXIMUM_DOMAIN_BYTES as u64,
                    };
                    *self.body = Some(
                        store::mounted_pack_rt::RetainedRecordBodyCursor::try_new(
                            limits,
                            self.maximum_items.min(GENERATION3D_MAXIMUM_DOMAIN_ITEMS),
                            self.expected_bytes,
                            self.expected_bytes,
                            store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES,
                        )
                        .map_err(|_| "generation3d-mutation.body-preflight")?,
                    );
                    *self.owner = Some(Generation3dRetainedMutationOwner::new(ordinal)?);
                    self.phase = Generation3dMutationSessionPhase::Body;
                }
            }
            Generation3dMutationSessionPhase::Body => {
                if self.pending.is_some() && self.body.as_ref().ok_or("generation3d-mutation.body-owner")?.ingress_ready() {
                    let (_, byte) = self.pending.take().ok_or("generation3d-mutation.body-input")?;
                    self.body.as_mut().ok_or("generation3d-mutation.body-owner")?.admit_byte(self.body_bytes, byte).map_err(|(_, byte)| if byte == 0 { "generation3d-mutation.body-handback-zero" } else { "generation3d-mutation.body-handback" })?;
                    self.body_bytes += 1;
                }
                if let Some(store::mounted_pack_rt::RetainedRecordBodyToken::Value(token)) = self.body.as_mut().ok_or("generation3d-mutation.body-owner")?.grant().map_err(|_| "generation3d-mutation.body-malformed")? {
                    let complete = matches!(token, store::mounted_pack_rt::RetainedValueToken::Complete { .. });
                    let body = self.body.as_ref().expect("P3 retained mutation body");
                    self.owner.as_mut().expect("P3 retained mutation owner").accept(token, body)?;
                    if complete {
                        self.phase = Generation3dMutationSessionPhase::Ready;
                        return Ok(true);
                    }
                }
            }
            _ => return Err("generation3d-mutation.session-state"),
        }
        Ok(false)
    }

    fn take(&mut self) -> Option<Generation3dMutation> {
        if self.phase != Generation3dMutationSessionPhase::Ready {
            return None;
        }
        let value = self.owner.as_mut()?.take()?;
        self.phase = Generation3dMutationSessionPhase::Published;
        Some(value)
    }

    fn next_retained_allocation_bytes(&mut self) -> Result<Option<usize>, &'static str> {
        let Some(body) = self.body.as_mut() else { return Ok(None) };
        body.next_allocation_bytes().map_err(|_| "generation3d-mutation.body-allocation")
    }

    fn reserve_retained_allocation(&mut self, maximum_bytes: usize) -> Result<(bool, usize), &'static str> {
        let body = self.body.as_mut().ok_or("generation3d-mutation.body-owner")?;
        body.reserve_allocation(maximum_bytes)
            .map(|step| (step.progressed, step.allocated_bytes))
            .map_err(|_| "generation3d-mutation.body-allocation")
    }

    fn retained_allocated_bytes(&self) -> usize {
        self.body.as_ref().map_or(0, store::mounted_pack_rt::RetainedRecordBodyCursor::allocated_bytes)
    }

    fn next_retained_release_allocation_bytes(&self) -> Option<usize> {
        if self.pending.is_some() || self.owner.is_some() {
            return None;
        }
        self.body.as_ref()?.next_release_allocation_bytes().ok().flatten()
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::mounted_pack_rt::RetainedPackCloseStep, store::PackRefusal> {
        use store::mounted_pack_rt::RetainedPackCloseStep;
        if self.phase == Generation3dMutationSessionPhase::Closed {
            return Ok(RetainedPackCloseStep::Complete);
        }
        if maximum_items == 0 && (self.pending.is_some() || self.owner.is_some() || self.body.is_none()) {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.phase = Generation3dMutationSessionPhase::Closing;
        if maximum_items != 0 && self.pending.take().is_some() {
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(owner) = self.owner.as_mut() {
            if !owner.close_step() {
                return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            drop(self.owner.take());
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(body) = self.body.as_mut() {
            match body.close_step(maximum_items, maximum_bytes)? {
                RetainedPackCloseStep::Pending { released_items, released_bytes } => {
                    return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                }
                RetainedPackCloseStep::Complete => {
                    if maximum_items == 0 {
                        return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                }
            }
            drop(self.body.take());
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.phase = Generation3dMutationSessionPhase::Closed;
        Ok(RetainedPackCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == Generation3dMutationSessionPhase::Closed && self.owner.is_none() && self.body.is_none() && self.pending.is_none()
    }
}

impl Drop for Generation3dMutationSession {
    /// 🔒️ Fail-closed on a live drop, but never while the thread is already unwinding — a second
    /// panic in a destructor during cleanup aborts the process instead of reporting the first
    /// failure. Same guard `OrderedMap::drop` and the framework's retirement roots carry.
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Generation3d mutation session reached Drop before terminal-empty close");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation3dPackSnapshotState {
    AwaitToken,
    Ingest,
    Drive,
    CloseSession,
    Ready,
    Published,
    Closing,
    Complete,
}

struct Generation3dPackSnapshotAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    state: Generation3dPackSnapshotState,
    token: Option<store::os_store::owned_schema_decode::OwnedSchemaToken>,
    relative: usize,
    high: Option<u8>,
    session: std::mem::ManuallyDrop<Option<semio_s_artifact_procedural_3d_io_binary::snapshot::Generation3dMountedPackSession>>,
    value: std::mem::ManuallyDrop<Option<Generation3dSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl Generation3dPackSnapshotAuthority {
    fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let refusal = move |error: semio_framework_value::ValueError| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before("generation3d-envelope.snapshot-close-demand", path).with_native(error);
        if let Some(session) = self.session.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: session.next_retained_release_allocation_bytes().unwrap_or(0), depth: 1, ..Default::default() });
        }
        if let Some(owner) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(refusal);
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*self.value).map_err(refusal);
        }
        Ok(Default::default())
    }

    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: Generation3dPackSnapshotState::AwaitToken,
            token: None,
            relative: 1,
            high: None,
            session: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
        store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }

    fn owners_terminal_empty(&self) -> bool {
        matches!(self.state, Generation3dPackSnapshotState::Published | Generation3dPackSnapshotState::Complete) && self.session.is_none() && self.value.is_none() && self.retirement.is_none()
    }

    fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<Generation3dSnapshot> for Generation3dPackSnapshotAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("generation3d-envelope.snapshot-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("generation3d-envelope.snapshot-pack-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dPackSnapshotState::AwaitToken {
            if !terminal || token.kind != store::os_store::owned_schema_decode::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("generation3d-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("generation3d-envelope.snapshot-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-length", token.start))?;
            let maximum_items = generation3d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-item-authority", token.start))?;
            *self.session = Some(semio_s_artifact_procedural_3d_io_binary::snapshot::generation3d_mounted_pack_session(expected, maximum_items).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-preflight", token.start))?);
            self.token = Some(token);
            self.state = Generation3dPackSnapshotState::Ingest;
        }
        if self.state == Generation3dPackSnapshotState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("generation3d-envelope.snapshot-token-replayed", token.start));
            }
            if retained.start + self.relative as u64 + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("generation3d-envelope.snapshot-pack-odd-hex", retained.start + self.relative as u64));
                }
                self.session.as_mut().expect("P3 mounted pack session retained").seal().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-seal", retained.end))?;
                self.state = Generation3dPackSnapshotState::Drive;
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            if let Some(exact) = self.session.as_mut().expect("P3 mounted pack session retained").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-source-allocation", retained.start + self.relative as u64))? {
                let step = self.session.as_mut().expect("P3 mounted pack session retained").reserve_retained_allocation(exact).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-source-allocation", retained.start + self.relative as u64))?;
                if !step.progressed {
                    return Err(self.diagnostic("generation3d-envelope.snapshot-source-allocation-stalled", retained.start + self.relative as u64));
                }
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("generation3d-envelope.snapshot-pack-source", retained.start + self.relative as u64));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-pack-hex", retained.start + self.relative as u64))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session.as_mut().expect("P3 mounted pack session retained").admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-handback", retained.start + self.relative as u64))?;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dPackSnapshotState::Drive {
            cx.set_stage("generation3d-retained-canonical-pack");
            if let Some(exact) = self.session.as_mut().expect("P3 mounted pack session retained").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-retained-allocation", token.start))? {
                let step = self.session.as_mut().expect("P3 mounted pack session retained").reserve_retained_allocation(exact).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-retained-allocation", token.start))?;
                if !step.progressed {
                    return Err(self.diagnostic("generation3d-envelope.snapshot-retained-allocation-stalled", token.start));
                }
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P3 mounted pack session retained").grant().map_err(|_| self.diagnostic("generation3d-envelope.snapshot-pack-malformed", token.start))? {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            *self.value = Some(self.session.as_mut().expect("P3 mounted pack session retained").take().ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-pack-handoff", token.start))?);
            self.session.as_mut().expect("P3 mounted pack session retained").request_cancel();
            self.state = Generation3dPackSnapshotState::CloseSession;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dPackSnapshotState::CloseSession {
            cx.consume_fuel(1);
            let maximum_bytes = self.session.as_ref().expect("P3 mounted pack session retained").next_retained_release_allocation_bytes().unwrap_or(0);
            if matches!(
                self.session.as_mut().expect("P3 mounted pack session retained").close_step(1, maximum_bytes).map_err(|_| self.diagnostic("generation3d-envelope.snapshot-session-close", token.start))?,
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { .. }
            ) {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation3dPackSnapshotState::Ready;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("generation3d-envelope.snapshot-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldTarget<Generation3dSnapshot>,
        reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, Generation3dPackSnapshotState::Ready) {
            return Err(self.diagnostic("generation3d-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("generation3d-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = Generation3dPackSnapshotState::Published;
        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.depth)
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let path = self.path;
        let diagnostic = move |code: &'static str, error: semio_framework_value::ValueError| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before(code, path).with_native(error);
        let empty = RetainedCloneProgress::default();
        if self.session.is_none() && self.retirement.is_none() && self.value.is_none() {
            self.state = Generation3dPackSnapshotState::Complete;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let demand = self.close_demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(session) = self.session.as_mut() {
            session.request_cancel();
            self.state = Generation3dPackSnapshotState::Closing;
            return match session.close_step(1, grant.maximum_release_bytes).map_err(|_| diagnostic("generation3d-envelope.snapshot-session-close", semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted pack session refused its close")))? {
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..empty })),
                store::mounted_pack_rt::RetainedTypedPackCloseStep::Complete => {
                    drop(self.session.take());
                    self.token = None;
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
                }
            };
        }
        if self.retirement.is_none() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.snapshot-retirement-fault", error))?;
            self.state = Generation3dPackSnapshotState::Closing;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.snapshot-retirement-fault", error))?;
        if self.retirement.is_none() {
            self.state = Generation3dPackSnapshotState::Complete;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for Generation3dPackSnapshotAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "Generation3d pack snapshot authority reached Drop before publication or bounded retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Generation3dMutationDecodeState {
    AwaitToken,
    Ingest,
    Drive,
    CloseSession,
    Ready,
    Published,
    Closing,
    Complete,
}

struct Generation3dMutationDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    state: Generation3dMutationDecodeState,
    token: Option<store::os_store::owned_schema_decode::OwnedSchemaToken>,
    relative: usize,
    high: Option<u8>,
    drive_ingress: bool,
    session: std::mem::ManuallyDrop<Option<Generation3dMutationSession>>,
    value: std::mem::ManuallyDrop<Option<Generation3dMutation>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl Generation3dMutationDecodeAuthority {
    fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let refusal = move |error: semio_framework_value::ValueError| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before("generation3d-envelope.mutation-close-demand", path).with_native(error);
        if let Some(session) = self.session.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: session.next_retained_release_allocation_bytes().unwrap_or(0), depth: 1, ..Default::default() });
        }
        if let Some(owner) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(refusal);
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*self.value).map_err(refusal);
        }
        Ok(Default::default())
    }

    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: Generation3dMutationDecodeState::AwaitToken,
            token: None,
            relative: 1,
            high: None,
            drive_ingress: false,
            session: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
        store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }

    fn nibble(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }

    fn owners_terminal_empty(&self) -> bool {
        matches!(self.state, Generation3dMutationDecodeState::Published | Generation3dMutationDecodeState::Complete) && self.session.is_none() && self.value.is_none() && self.retirement.is_none()
    }
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<Generation3dMutation> for Generation3dMutationDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("generation3d-envelope.mutation-stale-authority", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("generation3d-envelope.mutation-cancelled", token.start));
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dMutationDecodeState::AwaitToken {
            if !terminal || token.kind != store::os_store::owned_schema_decode::OwnedSchemaTokenKind::String {
                return Err(self.diagnostic("generation3d-envelope.mutation-pack-must-be-scalar", token.start));
            }
            let span = token.end.checked_sub(token.start).and_then(|span| span.checked_sub(2)).ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-pack-length", token.start))?;
            if span == 0 || span & 1 != 0 {
                return Err(self.diagnostic("generation3d-envelope.mutation-pack-odd-hex", token.start));
            }
            let expected = usize::try_from(span / 2).map_err(|_| self.diagnostic("generation3d-envelope.mutation-pack-length", token.start))?;
            let maximum_items = generation3d_publication_item_credit(self.operation, self.generation).map_err(|_| self.diagnostic("generation3d-envelope.mutation-item-authority", token.start))?;
            *self.session = Some(Generation3dMutationSession::new(expected, maximum_items).map_err(|_| self.diagnostic("generation3d-envelope.mutation-preflight", token.start))?);
            self.token = Some(token);
            self.state = Generation3dMutationDecodeState::Ingest;
        }
        if self.state == Generation3dMutationDecodeState::Ingest {
            let retained = self.token.ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-token-owner", token.start))?;
            if retained != token {
                return Err(self.diagnostic("generation3d-envelope.mutation-token-replayed", token.start));
            }
            if self.drive_ingress {
                if let Some(exact) = self.session.as_mut().expect("P3 retained mutation session").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", retained.start + self.relative as u64))? {
                    let (progressed, _) = self
                        .session
                        .as_mut()
                        .expect("P3 retained mutation session")
                        .reserve_retained_allocation(exact)
                        .map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", retained.start + self.relative as u64))?;
                    if !progressed {
                        return Err(self.diagnostic("generation3d-envelope.mutation-retained-allocation-stalled", retained.start + self.relative as u64));
                    }
                    cx.consume_fuel(1);
                    return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
                }
                self.session.as_mut().expect("P3 retained mutation session").grant().map_err(|_| self.diagnostic("generation3d-envelope.mutation-ingress-malformed", retained.start + self.relative as u64))?;
                self.drive_ingress = !self.session.as_ref().expect("P3 retained mutation session").ingress_ready();
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            if retained.start + self.relative as u64 + 1 >= retained.end {
                if self.high.is_some() {
                    return Err(self.diagnostic("generation3d-envelope.mutation-pack-odd-hex", retained.start + self.relative as u64));
                }
                self.session.as_mut().expect("P3 retained mutation session").seal().map_err(|_| self.diagnostic("generation3d-envelope.mutation-seal", retained.end))?;
                self.state = Generation3dMutationDecodeState::Drive;
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(retained, self.relative, &mut byte) != 1 {
                return Err(self.diagnostic("generation3d-envelope.mutation-source", retained.start + self.relative as u64));
            }
            let nibble = Self::nibble(byte[0]).ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-pack-hex", retained.start + self.relative as u64))?;
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                self.session.as_mut().expect("P3 retained mutation session").admit_byte((high << 4) | nibble).map_err(|_| self.diagnostic("generation3d-envelope.mutation-handback", retained.start + self.relative as u64))?;
                self.drive_ingress = true;
            } else {
                self.high = Some(nibble);
            }
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dMutationDecodeState::Drive {
            cx.set_stage("generation3d-retained-mutation");
            if let Some(exact) = self.session.as_mut().expect("P3 retained mutation session").next_retained_allocation_bytes().map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", token.start))? {
                let (progressed, _) = self
                    .session
                    .as_mut()
                    .expect("P3 retained mutation session")
                    .reserve_retained_allocation(exact)
                    .map_err(|_| self.diagnostic("generation3d-envelope.mutation-retained-allocation", token.start))?;
                if !progressed {
                    return Err(self.diagnostic("generation3d-envelope.mutation-retained-allocation-stalled", token.start));
                }
                cx.consume_fuel(1);
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            cx.consume_fuel(1);
            if !self.session.as_mut().expect("P3 retained mutation session").grant().map_err(|_| self.diagnostic("generation3d-envelope.mutation-malformed", token.start))? {
                return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
            }
            *self.value = Some(self.session.as_mut().expect("P3 retained mutation session").take().ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-handoff", token.start))?);
            self.state = Generation3dMutationDecodeState::CloseSession;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        if self.state == Generation3dMutationDecodeState::CloseSession {
            cx.consume_fuel(1);
            let maximum_bytes = self.session.as_ref().expect("P3 retained mutation session").next_retained_release_allocation_bytes().unwrap_or(0);
            match self
                .session
                .as_mut()
                .expect("P3 retained mutation session")
                .close_step(1, maximum_bytes)
                .map_err(|_| self.diagnostic("generation3d-envelope.mutation-session-close", token.start))?
            {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { .. } => return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => {}
            }
            drop(self.session.take());
            self.token = None;
            self.state = Generation3dMutationDecodeState::Ready;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete);
        }
        Err(self.diagnostic("generation3d-envelope.mutation-token-replayed", token.start))
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldTarget<Generation3dMutation>,
        reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if self.state != Generation3dMutationDecodeState::Ready {
            return Err(self.diagnostic("generation3d-envelope.mutation-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("generation3d-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = Generation3dMutationDecodeState::Published;
        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands()?.depth)
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let path = self.path;
        let diagnostic = move |code: &'static str, error: semio_framework_value::ValueError| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic::before(code, path).with_native(error);
        let empty = RetainedCloneProgress::default();
        if self.session.is_none() && self.retirement.is_none() && self.value.is_none() {
            self.state = Generation3dMutationDecodeState::Complete;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        let demand = self.close_demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(session) = self.session.as_mut() {
            self.state = Generation3dMutationDecodeState::Closing;
            return match session.close_step(1, grant.maximum_release_bytes).map_err(|_| diagnostic("generation3d-envelope.mutation-session-close", semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mounted mutation session refused its close")))? {
                store::mounted_pack_rt::RetainedPackCloseStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..empty })),
                store::mounted_pack_rt::RetainedPackCloseStep::Complete => {
                    drop(self.session.take());
                    self.token = None;
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
                }
            };
        }
        if self.retirement.is_none() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.mutation-retirement-fault", error))?;
            self.state = Generation3dMutationDecodeState::Closing;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|error| diagnostic("generation3d-envelope.mutation-retirement-fault", error))?;
        if self.retirement.is_none() {
            self.state = Generation3dMutationDecodeState::Complete;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl Drop for Generation3dMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "Generation3d mutation authority reached Drop before publication or terminal-empty close");
    }
}

struct Generation3dRejectedConflictAuthority {
    terminal: bool,
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority for Generation3dRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Err(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code: "generation3d-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::os_store::owned_schema_decode::OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(usize::from(!self.terminal))
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        if self.terminal {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

/// 🎭️ Owner-local exact catalog for the Generation3d fresh-envelope decode cohort.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct Generation3dEnvelopeOwnedFieldCatalog;

/// 📦️ Installs Generation3d's exact field catalog and nested owner retirement factories as
/// one indivisible app decode authority.
pub fn generation3d_envelope_decode_owner_bundle() -> store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Generation3dSnapshot, Generation3dMutation> {
    store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(Generation3dEnvelopeOwnedFieldCatalog), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dSnapshot>::default()), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dMutation>::default()))
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeOwnedFieldCatalog<Generation3dSnapshot, Generation3dMutation> for Generation3dEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Result<Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<Generation3dSnapshot, Generation3dMutation>>, Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<Generation3dSnapshot>>> {
        store::os_store::owned_schema_decode::ArtifactEnvelopeFreshVcsAuthority::try_new(
            self.begin_snapshot(operation, generation, path),
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dSnapshot>::default()),
            std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dMutation>::default()),
            self.edit_history_decoder(),
        )
        .map(|authority| Box::new(authority) as Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<Generation3dSnapshot, Generation3dMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<Generation3dSnapshot>> {
        Box::new(Generation3dPackSnapshotAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<Generation3dMutation>> {
        Box::new(Generation3dMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(Generation3dRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::os_store::owned_schema_decode::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<Generation3dMutation>>> {
        store::os_store::owned_schema_decode::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Generation3dMutation>::default()))
    }
}
//#endregion 🔖️TypedOwnedEnvelopeCatalog
