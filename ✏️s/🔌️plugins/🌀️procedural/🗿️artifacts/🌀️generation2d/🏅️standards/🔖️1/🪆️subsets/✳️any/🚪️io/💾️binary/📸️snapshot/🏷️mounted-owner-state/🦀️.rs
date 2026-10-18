//! 🏷️ Mounted owner state of the generation2d binary snapshot: the canonical pack prefix, depth and required field constants and the owner records and enums the typed snapshot owner is built from. See [`super`].

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use store::mounted_pack_rt as mounted;

//#region 🔖️MountedCanonicalPackSession
pub(super) const GENERATION2D_MOUNTED_PREFIX: [u8; 4] = *b"P2D2";

/// 📐️ The structural nesting bound this canonical route admits, kept equal to the mutation route's
/// `GENERATION2D_RETAINED_STACK_CAPACITY`. A `Widget::Neuron`'s `params` is a neural `Dictionary`
/// whose entries are themselves `Value::Dictionary`, and each such level costs several pack frames,
/// so a bound in the low teens rejects documents the initializer copies happily
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(super) const GENERATION2D_MOUNTED_TYPED_DEPTH: usize = 64;

pub(super) const GENERATION2D_REQUIRED_SNAPSHOT_FIELDS: u16 = 0b1001_1111;

#[derive(Default)]
pub(super) struct Generation2dMountedWidgetOwner {
    pub(super) keyword: String,
    pub(super) strings: [String; 4],
    pub(super) numbers: [f64; 4],
    pub(super) boolean: bool,
    pub(super) lists: [Vec<String>; 2],
    pub(super) dictionaries: [semio_framework_artifact_flow_flow::neural::Dictionary; 2],
    pub(super) dynamic: [Option<semio_framework_value::DslValue>; 2],
}

#[derive(Default)]
pub(super) struct Generation2dMountedSynapseOwner {
    pub(super) id: String,
    pub(super) from: String,
    pub(super) to: String,
    pub(super) from_port: String,
    pub(super) to_port: String,
}

#[derive(Default)]
pub(super) struct Generation2dMountedGenerationOwner {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) values: Vec<(String, semio_framework_value::DslValue)>,
}

#[derive(Default)]
pub(super) struct Generation2dMountedDictionaryEntryOwner {
    pub(super) key: String,
    pub(super) value: Option<semio_framework_artifact_flow_flow::neural::Value>,
}

#[derive(Clone, Copy)]
pub(super) enum Generation2dMountedDictionaryDestination {
    Widget { parent: usize, field: u16 },
    Value { parent: usize },
}

/// 🗂️ Where a decoded neural value belongs. A `Dictionary` reaches the wire either as a columnar
/// `Table` (many rows) or as a `List` of one-entry records, and the retained owner has to write the
/// value back into whichever of the two shapes it is standing in.
pub(super) enum Generation2dMountedNeuralOwner {
    TableRow { table: usize, row: usize },
    EntryRow { entries: usize, row: usize },
}

pub(super) enum Generation2dMountedRecordOwner {
    Root,
    Camera(semio_framework_artifact_flow_flow::CameraJson),
    Layout { key: String, value: semio_framework_artifact_flow_flow::WidgetLayout },
    Widget(Generation2dMountedWidgetOwner),
    NeuralValue { owner: Generation2dMountedNeuralOwner, value: Option<semio_framework_artifact_flow_flow::neural::Value> },
    Structural,
}

pub(super) enum Generation2dMountedContainerOwner {
    Record { root_field: Option<u16>, field: Option<u16>, seen: u16, owner: Generation2dMountedRecordOwner },
    Statements { root_field: u16, keyword: Option<String> },
    Strings { parent: usize, field: u16, values: Vec<String> },
    Synapses { rows: Vec<Generation2dMountedSynapseOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    LayoutMap { key: Option<String> },
    Generations { rows: Vec<Generation2dMountedGenerationOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    Dictionary { destination: Generation2dMountedDictionaryDestination, rows: Vec<Generation2dMountedDictionaryEntryOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    DictionaryEntries { destination: Generation2dMountedDictionaryDestination, rows: Vec<Generation2dMountedDictionaryEntryOwner> },
    DictionaryEntry { entries: usize, row: usize, field: Option<u16> },
    Wire { table: usize, row: usize, roles: [u8; 6], roles_len: usize, role: usize, nodes: usize },
    Structural { kind: mounted::RetainedValueContainer, root_field: Option<u16> },
}

#[derive(Clone, Copy)]
pub(super) enum Generation2dMountedStringTarget {
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

pub(super) struct Generation2dMountedStringOwner {
    pub(super) target: Generation2dMountedStringTarget,
    pub(super) value: String,
    pub(super) remaining: Option<u64>,
    pub(super) symbol: Option<(u64, usize, usize)>,
}

pub(super) enum Generation2dMountedJsonFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

pub(super) enum Generation2dMountedDslFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

/// 🧬️ Fixed-depth schema owner consuming catalog/value events directly into P2 domain
/// fields, with one scalar byte opportunity per retained grant. It has no generic record tree
/// and cannot invoke a batch pack decoder.
pub struct Generation2dMountedTypedSnapshotOwner {
    pub(super) candidate: std::mem::ManuallyDrop<Option<Generation2dSnapshot>>,
    pub(super) stack: Vec<Generation2dMountedContainerOwner>,
    pub(super) string: Option<Generation2dMountedStringOwner>,
    pub(super) pending_table_rows: Option<u64>,
    pub(super) json_stack: Vec<Generation2dMountedJsonFrame>,
    pub(super) json_destination: Option<(usize, usize)>,
    pub(super) dsl_stack: Vec<Generation2dMountedDslFrame>,
    pub(super) dsl_destination: Option<(usize, usize)>,
    pub(super) complete: bool,
    pub(super) handed_back: bool,
}
//#endregion 🔖️MountedCanonicalPackSession
