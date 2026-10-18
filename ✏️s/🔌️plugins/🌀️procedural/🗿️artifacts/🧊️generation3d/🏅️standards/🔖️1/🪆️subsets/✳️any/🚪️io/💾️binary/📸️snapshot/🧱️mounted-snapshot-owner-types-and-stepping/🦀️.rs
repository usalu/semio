//! 🧱️ Mounted snapshot owner types and stepping: the mounted constants, the widget, synapse, generation and dictionary owners, the typed snapshot owner and its string, symbol and finish stepping. See [`super`].

use super::*;

//#region 🔖️MountedCanonicalPackSession
pub(super) const GENERATION3D_MOUNTED_PREFIX: [u8; 4] = *b"P3D3";

/// 📐️ The structural nesting bound this canonical route admits, kept equal to the mutation route's
/// `GENERATION3D_RETAINED_STACK_CAPACITY`. A `Widget::Neuron`'s `params` is a neural `Dictionary`
/// whose entries are themselves `Value::Dictionary`, and each such level costs several pack frames,
/// so a bound in the low teens rejects documents the initializer copies happily
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) const GENERATION3D_MOUNTED_TYPED_DEPTH: usize = 64;

pub(super) const GENERATION3D_REQUIRED_SNAPSHOT_FIELDS: u16 = 0b1001_1111;

#[derive(Default)]
pub(super) struct Generation3dMountedWidgetOwner {
    pub(super) keyword: String,
    pub(super) strings: [String; 4],
    pub(super) numbers: [f64; 4],
    pub(super) boolean: bool,
    pub(super) lists: [Vec<String>; 2],
    pub(super) dictionaries: [semio_framework_artifact_flow_flow::neural::Dictionary; 2],
    pub(super) dynamic: [Option<semio_framework_value::DslValue>; 2],
}

#[derive(Default)]
pub(super) struct Generation3dMountedSynapseOwner {
    pub(super) id: String,
    pub(super) from: String,
    pub(super) to: String,
    pub(super) from_port: String,
    pub(super) to_port: String,
}

#[derive(Default)]
pub(super) struct Generation3dMountedGenerationOwner {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) values: Vec<(String, semio_framework_value::DslValue)>,
}

#[derive(Default)]
pub(super) struct Generation3dMountedDictionaryEntryOwner {
    pub(super) key: String,
    pub(super) value: Option<semio_framework_artifact_flow_flow::neural::Value>,
}

#[derive(Clone, Copy)]
pub(super) enum Generation3dMountedDictionaryDestination {
    Widget { parent: usize, field: u16 },
    Value { parent: usize },
}

/// 🗂️ Where a decoded neural value belongs. A `Dictionary` reaches the wire either as a columnar
/// `Table` (many rows) or as a `List` of one-entry records, and the retained owner has to write the
/// value back into whichever of the two shapes it is standing in.
pub(super) enum Generation3dMountedNeuralOwner {
    TableRow { table: usize, row: usize },
    EntryRow { entries: usize, row: usize },
}

pub(super) enum Generation3dMountedRecordOwner {
    Root,
    Camera(semio_framework_artifact_flow_flow::CameraJson),
    Layout { key: String, value: semio_framework_artifact_flow_flow::WidgetLayout },
    Widget(Generation3dMountedWidgetOwner),
    NeuralValue { owner: Generation3dMountedNeuralOwner, value: Option<semio_framework_artifact_flow_flow::neural::Value> },
    Structural,
}

pub(super) enum Generation3dMountedContainerOwner {
    Record { root_field: Option<u16>, field: Option<u16>, seen: u16, owner: Generation3dMountedRecordOwner },
    Statements { root_field: u16, keyword: Option<String> },
    Strings { parent: usize, field: u16, values: Vec<String> },
    Synapses { rows: Vec<Generation3dMountedSynapseOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    LayoutMap { key: Option<String> },
    Generations { rows: Vec<Generation3dMountedGenerationOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    Dictionary { destination: Generation3dMountedDictionaryDestination, rows: Vec<Generation3dMountedDictionaryEntryOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    DictionaryEntries { destination: Generation3dMountedDictionaryDestination, rows: Vec<Generation3dMountedDictionaryEntryOwner> },
    DictionaryEntry { entries: usize, row: usize, field: Option<u16> },
    Wire { table: usize, row: usize, roles: [u8; 6], roles_len: usize, role: usize, nodes: usize },
    Structural { kind: mounted::RetainedValueContainer, root_field: Option<u16> },
}

#[derive(Clone, Copy)]
enum Generation3dMountedStringTarget {
    Root(u16),
    Record(usize, u16),
    StatementKeyword(usize),
    Sequence(usize),
    Synapse(usize, usize, u8),
    LayoutKey(usize),
    Generation(usize, usize, u8),
    DictionaryKey(usize, usize),
    DictionaryEntryKey(usize),
    NeuralText(usize),
    JsonKey,
    JsonValue,
    DslKey,
    DslValue,
    Wire(usize, u8),
}

struct Generation3dMountedStringOwner {
    pub(super) target: Generation3dMountedStringTarget,
    pub(super) value: String,
    pub(super) remaining: Option<u64>,
    pub(super) symbol: Option<(u64, usize, usize)>,
}

pub(super) enum Generation3dMountedJsonFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

pub(super) enum Generation3dMountedDslFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

/// 🧬️ Fixed-depth schema owner consuming catalog/value events directly into P3 domain
/// fields, with one scalar byte opportunity per retained grant. It has no generic record tree
/// and cannot invoke a batch pack decoder.
pub struct Generation3dMountedTypedSnapshotOwner {
    pub(super) candidate: std::mem::ManuallyDrop<Option<Generation3dSnapshot>>,
    pub(super) stack: Vec<Generation3dMountedContainerOwner>,
    pub(super) string: Option<Generation3dMountedStringOwner>,
    pub(super) pending_table_rows: Option<u64>,
    pub(super) json_stack: Vec<Generation3dMountedJsonFrame>,
    pub(super) json_destination: Option<(usize, usize)>,
    pub(super) dsl_stack: Vec<Generation3dMountedDslFrame>,
    pub(super) dsl_destination: Option<(usize, usize)>,
    pub(super) complete: bool,
    pub(super) handed_back: bool,
}

impl Generation3dMountedTypedSnapshotOwner {
    pub(super) fn new() -> Result<Self, &'static str> {
        let mut stack = Vec::new();
        stack.try_reserve_exact(GENERATION3D_MOUNTED_TYPED_DEPTH).map_err(|_| "generation3d-mounted.typed-stack-preflight")?;
        let mut json_stack = Vec::new();
        json_stack.try_reserve_exact(GENERATION3D_MOUNTED_TYPED_DEPTH).map_err(|_| "generation3d-mounted.json-stack-preflight")?;
        let mut dsl_stack = Vec::new();
        dsl_stack.try_reserve_exact(GENERATION3D_MOUNTED_TYPED_DEPTH).map_err(|_| "generation3d-mounted.dsl-stack-preflight")?;
        let candidate = Generation3dSnapshot {
            host_snapshot: semio_framework_artifact_flow_flow::FlowHostSnapshot {
                schema: String::new(),
                camera: semio_framework_artifact_flow_flow::CameraJson::default(),
                widgets: Vec::new(),
                synapses: Vec::new(),
                layout: semio_framework_artifact_flow_flow::OrderedMap::new(),
            },
            generation: semio_framework_artifact_playbook_playbook::GenerationPlayState::default().into(),
        };
        Ok(Self {
            candidate: std::mem::ManuallyDrop::new(Some(candidate)),
            stack,
            string: None,
            pending_table_rows: None,
            json_stack,
            json_destination: None,
            dsl_stack,
            dsl_destination: None,
            complete: false,
            handed_back: false,
        })
    }

    pub(super) fn push(&mut self, owner: Generation3dMountedContainerOwner) -> Result<(), &'static str> {
        if self.stack.len() == self.stack.capacity() {
            return Err("generation3d-mounted.typed-depth");
        }
        self.stack.push(owner);
        Ok(())
    }

    pub(super) fn current_root_field(&self) -> Option<u16> {
        self.stack.iter().rev().find_map(|owner| match owner {
            Generation3dMountedContainerOwner::Record { root_field, field, .. } => root_field.or(*field),
            Generation3dMountedContainerOwner::Statements { root_field, .. } => Some(*root_field),
            Generation3dMountedContainerOwner::Strings { parent, .. } => self.stack.get(*parent).and_then(|owner| match owner {
                Generation3dMountedContainerOwner::Record { root_field, .. } => *root_field,
                _ => None,
            }),
            Generation3dMountedContainerOwner::Synapses { .. } | Generation3dMountedContainerOwner::Wire { .. } => Some(3),
            Generation3dMountedContainerOwner::LayoutMap { .. } => Some(4),
            Generation3dMountedContainerOwner::Generations { .. } => Some(7),
            Generation3dMountedContainerOwner::Dictionary { .. } | Generation3dMountedContainerOwner::DictionaryEntries { .. } | Generation3dMountedContainerOwner::DictionaryEntry { .. } => Some(2),
            Generation3dMountedContainerOwner::Structural { root_field, .. } => *root_field,
        })
    }

    fn string_target(&mut self) -> Result<Generation3dMountedStringTarget, &'static str> {
        let index = self.stack.len().checked_sub(1).ok_or("generation3d-mounted.string-without-owner")?;
        if self.json_destination.is_some() {
            return Ok(match self.json_stack.last() {
                Some(Generation3dMountedJsonFrame::Object { key: None, .. }) => Generation3dMountedStringTarget::JsonKey,
                _ => Generation3dMountedStringTarget::JsonValue,
            });
        }
        if self.dsl_destination.is_some() {
            return Ok(match self.dsl_stack.last() {
                Some(Generation3dMountedDslFrame::Object { key: None, .. }) => Generation3dMountedStringTarget::DslKey,
                _ => Generation3dMountedStringTarget::DslValue,
            });
        }
        match &mut self.stack[index] {
            Generation3dMountedContainerOwner::Record { root_field: None, field: Some(field), .. } => Ok(Generation3dMountedStringTarget::Root(*field)),
            Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::NeuralValue { .. }, field: Some(4), .. } => Ok(Generation3dMountedStringTarget::NeuralText(index)),
            Generation3dMountedContainerOwner::DictionaryEntry { field: Some(0), .. } => Ok(Generation3dMountedStringTarget::DictionaryEntryKey(index)),
            Generation3dMountedContainerOwner::Record { field: Some(field), .. } => Ok(Generation3dMountedStringTarget::Record(index, *field)),
            Generation3dMountedContainerOwner::Statements { keyword: None, .. } => Ok(Generation3dMountedStringTarget::StatementKeyword(index)),
            Generation3dMountedContainerOwner::Strings { .. } => Ok(Generation3dMountedStringTarget::Sequence(index)),
            Generation3dMountedContainerOwner::Synapses { field: Some(field), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation3d-mounted.synapse-row")?;
                *next = row + 1;
                Ok(Generation3dMountedStringTarget::Synapse(index, row, *field as u8))
            }
            Generation3dMountedContainerOwner::LayoutMap { key: None } => Ok(Generation3dMountedStringTarget::LayoutKey(index)),
            Generation3dMountedContainerOwner::Generations { field: Some(field), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation3d-mounted.generation-row")?;
                *next = row + 1;
                Ok(Generation3dMountedStringTarget::Generation(index, row, *field as u8))
            }
            Generation3dMountedContainerOwner::Dictionary { field: Some(0), present, next, .. } => {
                let row = (*next..present.len()).find(|row| present[*row]).ok_or("generation3d-mounted.dictionary-key-row")?;
                *next = row + 1;
                Ok(Generation3dMountedStringTarget::DictionaryKey(index, row))
            }
            Generation3dMountedContainerOwner::Wire { roles, roles_len, role, .. } if *role < *roles_len => {
                let target = roles[*role];
                *role += 1;
                Ok(Generation3dMountedStringTarget::Wire(index, target))
            }
            _ => Err("generation3d-mounted.string-owner-role"),
        }
    }

    pub(super) fn begin_string(&mut self) -> Result<(), &'static str> {
        if self.string.is_some() {
            return Err("generation3d-mounted.string-overlap");
        }
        self.string = Some(Generation3dMountedStringOwner { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    pub(super) fn begin_symbol(&mut self, symbol: u64, catalog: &mounted::RetainedPackCatalogCursor) -> Result<(), &'static str> {
        if self.string.is_none() {
            self.begin_string()?;
        }
        let chars = catalog.symbol_chars(symbol).map_err(|_| "generation3d-mounted.symref")?;
        let owner = self.string.as_mut().expect("P3 mounted string retained");
        owner.value.try_reserve_exact(chars).map_err(|_| "generation3d-mounted.symbol-preflight")?;
        owner.symbol = Some((symbol, 0, chars));
        if chars == 0 {
            self.finish_string()?;
        }
        Ok(())
    }


    pub(super) fn finish_string(&mut self) -> Result<(), &'static str> {
        let owner = self.string.take().ok_or("generation3d-mounted.string-handoff")?;
        match owner.target {
            Generation3dMountedStringTarget::Root(0) => {
                self.candidate.as_mut().ok_or("generation3d-mounted.snapshot-owner")?.host_snapshot.schema = owner.value;
                if let Some(Generation3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMountedStringTarget::Root(5) => {
                self.candidate.as_mut().ok_or("generation3d-mounted.snapshot-owner")?.generation.cold_builder_mut()?.selected_generation_id = Some(owner.value);
                if let Some(Generation3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMountedStringTarget::Root(6) => {
                self.candidate.as_mut().ok_or("generation3d-mounted.snapshot-owner")?.generation.cold_builder_mut()?.preview_text = Some(owner.value);
                if let Some(Generation3dMountedContainerOwner::Record { field, .. }) = self.stack.last_mut() {
                    *field = None;
                }
            }
            Generation3dMountedStringTarget::Record(index, field) => match self.stack.get_mut(index) {
                Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::Widget(widget), field: active, .. }) => {
                    *widget.strings.get_mut(field as usize).ok_or("generation3d-mounted.widget-string-field")? = owner.value;
                    *active = None;
                }
                _ => return Err("generation3d-mounted.record-string-owner"),
            },
            Generation3dMountedStringTarget::StatementKeyword(index) => match self.stack.get_mut(index) {
                Some(Generation3dMountedContainerOwner::Statements { keyword, .. }) => *keyword = Some(owner.value),
                _ => return Err("generation3d-mounted.statement-keyword-owner"),
            },
            Generation3dMountedStringTarget::Sequence(index) => match self.stack.get_mut(index) {
                Some(Generation3dMountedContainerOwner::Strings { values, .. }) => values.push(owner.value),
                _ => return Err("generation3d-mounted.sequence-string-owner"),
            },
            Generation3dMountedStringTarget::Synapse(index, row, 0) => match self.stack.get_mut(index) {
                Some(Generation3dMountedContainerOwner::Synapses { rows, .. }) => rows.get_mut(row).ok_or("generation3d-mounted.synapse-row")?.id = owner.value,
                _ => return Err("generation3d-mounted.synapse-string-owner"),
            },
            Generation3dMountedStringTarget::LayoutKey(index) => match self.stack.get_mut(index) {
                Some(Generation3dMountedContainerOwner::LayoutMap { key }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation3d-mounted.layout-key-owner"),
            },
            Generation3dMountedStringTarget::Generation(index, row, 0) => match self.stack.get_mut(index) {
                Some(Generation3dMountedContainerOwner::Generations { rows, .. }) => rows.get_mut(row).ok_or("generation3d-mounted.generation-row")?.id = owner.value,
                _ => return Err("generation3d-mounted.generation-string-owner"),
            },
            Generation3dMountedStringTarget::Generation(index, row, 1) => match self.stack.get_mut(index) {
                Some(Generation3dMountedContainerOwner::Generations { rows, .. }) => rows.get_mut(row).ok_or("generation3d-mounted.generation-row")?.name = owner.value,
                _ => return Err("generation3d-mounted.generation-string-owner"),
            },
            Generation3dMountedStringTarget::DictionaryKey(index, row) => match self.stack.get_mut(index) {
                Some(Generation3dMountedContainerOwner::Dictionary { rows, .. }) => rows.get_mut(row).ok_or("generation3d-mounted.dictionary-key-row")?.key = owner.value,
                _ => return Err("generation3d-mounted.dictionary-key-owner"),
            },
            Generation3dMountedStringTarget::DictionaryEntryKey(index) => {
                let (entries, row) = match self.stack.get_mut(index) {
                    Some(Generation3dMountedContainerOwner::DictionaryEntry { entries, row, field }) => {
                        *field = None;
                        (*entries, *row)
                    }
                    _ => return Err("generation3d-mounted.dictionary-entry-owner"),
                };
                match self.stack.get_mut(entries) {
                    Some(Generation3dMountedContainerOwner::DictionaryEntries { rows, .. }) => rows.get_mut(row).ok_or("generation3d-mounted.dictionary-entry-row")?.key = owner.value,
                    _ => return Err("generation3d-mounted.dictionary-entries-owner"),
                }
            }
            Generation3dMountedStringTarget::NeuralText(index) => match self.stack.get_mut(index) {
                Some(Generation3dMountedContainerOwner::Record { owner: Generation3dMountedRecordOwner::NeuralValue { value, .. }, field, .. }) if *field == Some(4) && value.is_none() => {
                    *value = Some(semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String(owner.value)));
                    *field = None;
                }
                _ => return Err("generation3d-mounted.neural-text-owner"),
            },
            Generation3dMountedStringTarget::JsonKey => match self.json_stack.last_mut() {
                Some(Generation3dMountedJsonFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation3d-mounted.json-key-owner"),
            },
            Generation3dMountedStringTarget::JsonValue => self.assign_json(semio_framework_value::DslValue::String(owner.value))?,
            Generation3dMountedStringTarget::DslKey => match self.dsl_stack.last_mut() {
                Some(Generation3dMountedDslFrame::Object { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err("generation3d-mounted.dsl-key-owner"),
            },
            Generation3dMountedStringTarget::DslValue => self.assign_dsl(semio_framework_value::DslValue::String(owner.value))?,
            Generation3dMountedStringTarget::Wire(index, role) => {
                let (table, row) = match self.stack.get(index) {
                    Some(Generation3dMountedContainerOwner::Wire { table, row, .. }) => (*table, *row),
                    _ => return Err("generation3d-mounted.wire-owner"),
                };
                let synapse = match self.stack.get_mut(table) {
                    Some(Generation3dMountedContainerOwner::Synapses { rows, .. }) => rows.get_mut(row).ok_or("generation3d-mounted.wire-row")?,
                    _ => return Err("generation3d-mounted.wire-table"),
                };
                match role {
                    0 => synapse.from = owner.value,
                    2 => synapse.from_port = owner.value,
                    3 => synapse.to = owner.value,
                    5 => synapse.to_port = owner.value,
                    _ => drop(owner.value),
                }
            }
            _ => return Err("generation3d-mounted.string-field"),
        }
        Ok(())
    }

}
//#endregion 🔖️MountedCanonicalPackSession
