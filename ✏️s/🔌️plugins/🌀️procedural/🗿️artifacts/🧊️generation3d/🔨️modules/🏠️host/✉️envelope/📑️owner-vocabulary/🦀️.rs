//! 📑️ Vocabulary of the retained mutation owner: the widget, synapse and dictionary owners, the frame and destination enums and the fixed-depth owner state with its drop guard. See [`super`].

use super::*;

//#region 🔖️TypedOwnedEnvelopeCatalog
#[derive(Default)]
pub(super) struct Generation3dMutationWidgetOwner {
    pub(super) keyword: String,
    pub(super) strings: [String; 4],
    pub(super) numbers: [f64; 4],
    pub(super) boolean: bool,
    pub(super) lists: [Vec<String>; 2],
    pub(super) dictionaries: [semio_framework_artifact_flow_flow::neural::Dictionary; 2],
    pub(super) dynamic: [Option<semio_framework_value::DslValue>; 2],
}

#[derive(Default)]
struct Generation3dMutationSynapseOwner {
    pub(super) id: String,
    pub(super) from: String,
    pub(super) to: String,
    pub(super) from_port: String,
    pub(super) to_port: String,
}

#[derive(Default)]
pub(super) struct Generation3dMutationDictionaryEntryOwner {
    pub(super) key: String,
    pub(super) value: Option<semio_framework_artifact_flow_flow::neural::Value>,
}

#[derive(Clone, Copy)]
pub(super) enum Generation3dMutationDictionaryDestination {
    Widget { parent: usize, field: u16 },
    Value { parent: usize },
}

/// 🗂️ Where a decoded neural value belongs. A `Dictionary` reaches the wire either as a columnar
/// `Table` (many rows) or as a `List` of one-entry records, and the retained owner has to write the
/// value back into whichever of the two shapes it is standing in.
pub(super) enum Generation3dMutationNeuralOwner {
    TableRow { table: usize, row: usize },
    EntryRow { entries: usize, row: usize },
}

pub(super) enum Generation3dMutationFrame {
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
pub(super) enum Generation3dMutationStringTarget {
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

pub(super) enum Generation3dMutationJsonFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

pub(super) enum Generation3dMutationDslFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

#[derive(Clone, Copy)]
pub(super) enum Generation3dMutationJsonDestination {
    ChangeValue,
    Generation(usize),
}

pub(super) struct Generation3dMutationStringOwner {
    pub(super) target: Generation3dMutationStringTarget,
    pub(super) value: String,
    pub(super) remaining: Option<u64>,
    pub(super) symbol: Option<(u64, usize, usize)>,
}

/// 🧬️ Fixed-depth typed owner for the exact twenty-two Generation3d mutation records.
/// Dynamic JSON is admitted only at the ChangeGenerationValue value and the ChangeWidgetInput input leaves.
pub(super) struct Generation3dRetainedMutationOwner {
    pub(super) ordinal: u8,
    pub(super) stack: Vec<Generation3dMutationFrame>,
    pub(super) string: Option<Generation3dMutationStringOwner>,
    pub(super) strings: [String; 3],
    pub(super) targets: Vec<String>,
    pub(super) closing_targets: semio_framework_artifact_flow_flow::retained::FlowRetirement,
    pub(super) numbers: [f64; 4],
    pub(super) index: usize,
    pub(super) index_present: bool,
    pub(super) widget: Option<semio_framework_artifact_flow_flow::Widget>,
    pub(super) synapse: Option<semio_framework_artifact_flow_flow::SynapseSpec>,
    pub(super) layout: Option<semio_framework_artifact_flow_flow::WidgetLayout>,
    pub(super) camera: Option<semio_framework_artifact_flow_flow::CameraJson>,
    pub(super) generation: Option<semio_framework_artifact_playbook_playbook::FormGeneration>,
    pub(super) json: semio_framework_value::DslValue,
    pub(super) json_stack: Vec<Generation3dMutationJsonFrame>,
    pub(super) json_destination: Option<Generation3dMutationJsonDestination>,
    pub(super) dsl_stack: Vec<Generation3dMutationDslFrame>,
    pub(super) dsl_destination: Option<(usize, usize)>,
    pub(super) pending_table_rows: Option<u64>,
    pub(super) value: std::mem::ManuallyDrop<Option<Generation3dMutation>>,
    pub(super) complete: bool,
    pub(super) handed_back: bool,
}

impl Drop for Generation3dRetainedMutationOwner {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "Generation3d retained mutation owner reached Drop before handoff or terminal-empty close");
    }
}
//#endregion 🔖️TypedOwnedEnvelopeCatalog
