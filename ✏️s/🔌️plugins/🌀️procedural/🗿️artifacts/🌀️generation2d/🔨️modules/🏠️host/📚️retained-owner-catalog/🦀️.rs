//! 📚️ Retained owner catalog: the mutation session phase, the variant count, the frame, target and destination enums, the owner structs and the retained owner catalog. See [`super`].

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Generation2dMutationSessionPhase {
    Format,
    Ordinal,
    Body,
    Ready,
    Published,
    Closing,
    Closed,
}

pub(super) const GENERATION2D_MUTATION_VARIANT_COUNT: usize = 17;

#[derive(Default)]
pub(super) struct Generation2dMutationWidgetOwner {
    pub(super) keyword: String,
    pub(super) strings: [String; 4],
    pub(super) numbers: [f64; 4],
    pub(super) boolean: bool,
    pub(super) lists: [Vec<String>; 2],
    pub(super) dictionaries: [semio_framework_artifact_flow_flow::neural::Dictionary; 2],
    pub(super) dynamic: [Option<semio_framework_value::DslValue>; 2],
}

#[derive(Default)]
pub(super) struct Generation2dMutationDictionaryEntryOwner {
    pub(super) key: String,
    pub(super) value: Option<semio_framework_artifact_flow_flow::neural::Value>,
}

#[derive(Clone, Copy)]
pub(super) enum Generation2dMutationDictionaryDestination {
    Widget { parent: usize, field: u16 },
    Value { parent: usize },
}

/// 🗂️ Where a decoded `neural::Value` belongs. A `neural::Dictionary` reaches the wire in TWO exact
/// shapes and both are canonical: a field whose declared shape is `Shape::Table` (`Widget::Neuron`'s
/// `params`) is columnar `TAG_TABLE_SOA`, while the same dictionary nested inside
/// `neural::Value::Dictionary` carries no declared shape and falls back to `encode_seq` — a plain
/// `TAG_LIST` of two-field entry records (`🎒️pack/🌱️value/📝️record-encode/🦀️.rs`'s `FieldValue::List` arm). The value
/// owner therefore addresses either a table row or a list entry.
#[derive(Clone, Copy)]
pub(super) enum Generation2dMutationNeuralOwner {
    TableRow { table: usize, row: usize },
    EntryRow { entries: usize, row: usize },
}

pub(super) enum Generation2dMutationFrame {
    Root { field: Option<u16> },
    Statements { keyword: Option<String> },
    Widget { field: Option<u16>, owner: Generation2dMutationWidgetOwner },
    Synapse { field: Option<u16>, owner: Generation2dMutationSynapseOwner },
    Layout { field: Option<u16>, value: semio_framework_artifact_flow_flow::WidgetLayout },
    Camera { field: Option<u16>, value: semio_framework_artifact_flow_flow::CameraJson },
    Generation { field: Option<u16>, id: String, name: String, values: Vec<(String, semio_framework_value::DslValue)> },
    Dictionary { destination: Generation2dMutationDictionaryDestination, rows: Vec<Generation2dMutationDictionaryEntryOwner>, field: Option<u16>, present: Vec<bool>, next: usize },
    DictionaryEntries { destination: Generation2dMutationDictionaryDestination, rows: Vec<Generation2dMutationDictionaryEntryOwner> },
    DictionaryEntry { entries: usize, row: usize, field: Option<u16> },
    NeuralValue { owner: Generation2dMutationNeuralOwner, field: Option<u16>, value: Option<semio_framework_artifact_flow_flow::neural::Value> },
    Strings { parent: usize, field: u16, values: Vec<String> },
    Wire { parent: usize, roles: [u8; 6], roles_len: usize, role: usize, nodes: usize },
    Structural(store::mounted_pack_rt::RetainedValueContainer),
}

#[derive(Clone, Copy)]
pub(super) enum Generation2dMutationStringTarget {
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

pub(super) enum Generation2dMutationJsonFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

pub(super) enum Generation2dMutationDslFrame {
    Array(Vec<semio_framework_value::DslValue>),
    Object { values: Vec<(String, semio_framework_value::DslValue)>, key: Option<String> },
}

#[derive(Clone, Copy)]
pub(super) enum Generation2dMutationJsonDestination {
    ChangeValue,
    Generation(usize),
}

pub(super) struct Generation2dMutationStringOwner {
    pub(super) target: Generation2dMutationStringTarget,
    pub(super) value: String,
    pub(super) remaining: Option<u64>,
    pub(super) symbol: Option<(u64, usize, usize)>,
}

pub const GENERATION2D_RETAINED_MUTATION_OWNERS: [&str; GENERATION2D_MUTATION_VARIANT_COUNT] = [
    "create-widget",
    "replace-widget",
    "delete-widget",
    "connect-synapse",
    "replace-synapse",
    "disconnect-synapse",
    "move-widget",
    "clear-widget-layout",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
    "change-slider-value",
    "move-nodes",
    "select-generation",
];

#[derive(Default)]
struct Generation2dMutationSynapseOwner {
    pub(super) id: String,
    pub(super) from: String,
    pub(super) to: String,
    pub(super) from_port: String,
    pub(super) to_port: String,
}

pub fn generation2d_retained_catalog_is_complete() -> bool {
    GENERATION2D_RETAINED_MUTATION_OWNERS == semio_s_artifact_procedural_2d_mutations::KINDS
        && GENERATION2D_RETAINED_OWNER_CATALOG.contains(&"mutations.clear-widget-layout.2d-only")
        && !GENERATION2D_RETAINED_OWNER_CATALOG.iter().any(|owner| owner.contains("process3d"))
}

/// 🧭️ Fixed ownership grammar for every Generation2d retained domain and lifecycle owner.
/// The 2d-only ClearWidgetLayout entry is deliberately explicit: a 3d mutation catalog cannot satisfy this table.
pub const GENERATION2D_RETAINED_OWNER_CATALOG: &[&str] = &[
    "snapshot.host_snapshot.schema",
    "snapshot.host_snapshot.camera.x",
    "snapshot.host_snapshot.camera.y",
    "snapshot.host_snapshot.camera.zoom",
    "snapshot.host_snapshot.widgets.length",
    "snapshot.host_snapshot.widgets.item.neuron",
    "snapshot.host_snapshot.widgets.item.input-slider",
    "snapshot.host_snapshot.widgets.item.input-note",
    "snapshot.host_snapshot.widgets.item.input-image",
    "snapshot.host_snapshot.widgets.item.variable",
    "snapshot.host_snapshot.widgets.item.output-preview",
    "snapshot.host_snapshot.widgets.item.output-action",
    "snapshot.host_snapshot.widgets.item.output-export",
    "snapshot.host_snapshot.widgets.item.cluster",
    "snapshot.host_snapshot.widgets.item.strings",
    "snapshot.host_snapshot.widgets.item.dictionary.entries",
    "snapshot.host_snapshot.widgets.item.tree",
    "snapshot.host_snapshot.widgets.item.flow",
    "snapshot.host_snapshot.synapses.length",
    "snapshot.host_snapshot.synapses.item.id",
    "snapshot.host_snapshot.synapses.item.from",
    "snapshot.host_snapshot.synapses.item.to",
    "snapshot.host_snapshot.synapses.item.from-port",
    "snapshot.host_snapshot.synapses.item.to-port",
    "snapshot.host_snapshot.layout.length",
    "snapshot.host_snapshot.layout.item.id",
    "snapshot.host_snapshot.layout.item.x",
    "snapshot.host_snapshot.layout.item.y",
    "snapshot.generation.generations.length",
    "snapshot.generation.generations.item.id",
    "snapshot.generation.generations.item.name",
    "snapshot.generation.generations.item.values.length",
    "snapshot.generation.generations.item.values.key",
    "snapshot.generation.generations.item.values.scalar",
    "snapshot.generation.selected-generation-id",
    "snapshot.generation.preview-text",
    "mutations.create-widget",
    "mutations.replace-widget",
    "mutations.delete-widget",
    "mutations.connect-synapse",
    "mutations.replace-synapse",
    "mutations.disconnect-synapse",
    "mutations.move-widget",
    "mutations.clear-widget-layout.2d-only",
    "mutations.update-camera",
    "mutations.change-schema",
    "mutations.create-generation",
    "mutations.delete-generation",
    "mutations.rename-generation",
    "mutations.change-generation-value",
    "mutations.change-slider-value",
    "mutations.move-nodes",
    "mutations.select-generation",
    "history.edit.id",
    "history.edit.actor",
    "history.edit.forward",
    "history.edit.inverse",
    "history.edit.mutation-meta",
    "history.cursor.applied",
    "history.cursor.redo",
    "history.cursor.checkpoint",
    "conflict.rejected-fresh",
    "child.widget.cluster.tree",
    "child.widget.cluster.flow",
    "control.cancel",
    "control.retry",
    "control.close",
    "output.progress",
    "output.checkpoint",
    "output.preview",
    "output.terminal",
];
