//! 📏 Procedural 2d document model on `vcs`.

use flow_core::neural::{Atom, Dictionary, Value as NeuralValue};
use flow_core::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use protocol::{apply_generation_operation, invert_generation_operation, FormGeneration, GenerationOperation, GenerationPlayState};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use vcs::{DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

pub const PROCEDURAL_2D_SCHEMA: &str = "procedural.2d";

//#region 🔖Document
/// 🧾 Persistent procedural-2d document — the flow fixture plus the generation vocabulary state.
/// Ephemeral view state (selection, show mode, preview evaluations) lives in the plugin app struct.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDocument {
    pub fixture: FlowFixture,
    #[serde(default)]
    pub generation: GenerationPlayState,
}

/// 🪪 A flow widget's stable id, across every widget variant (mirrors flow_core's private accessor).
fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}
//#endregion 🔖Document

//#region 🔖Collections
/// 🩹 Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id already
/// exists, else insert at the recorded index). Disjoint `set`s on different ids merge cleanly, which
/// is what lets two backbone peers converge on concurrent edits to different widgets/synapses.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Widget)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynapsesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, SynapseSpec)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutDiff {
    pub removed: Vec<String>,
    pub set: Vec<(String, WidgetLayout)>,
}

fn apply_widgets_diff(widgets: &mut Vec<Widget>, diff: &WidgetsDiff) {
    for id in &diff.removed {
        widgets.retain(|widget| widget_id(widget) != id);
    }
    for (index, widget) in &diff.set {
        if let Some(pos) = widgets.iter().position(|entry| widget_id(entry) == widget_id(widget)) {
            widgets[pos] = widget.clone();
        } else {
            widgets.insert((*index).min(widgets.len()), widget.clone());
        }
    }
}

fn apply_synapses_diff(synapses: &mut Vec<SynapseSpec>, diff: &SynapsesDiff) {
    for id in &diff.removed {
        synapses.retain(|synapse| synapse.id != *id);
    }
    for (index, synapse) in &diff.set {
        if let Some(pos) = synapses.iter().position(|entry| entry.id == synapse.id) {
            synapses[pos] = synapse.clone();
        } else {
            synapses.insert((*index).min(synapses.len()), synapse.clone());
        }
    }
}
//#endregion 🔖Collections

//#region 🔖Operations
/// 🩹 Sparse procedural-2d diff over the flow fixture's collections plus scalar canvas/schema fields
/// and an ordered list of generation edits applied in sequence.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDiff {
    pub widgets: WidgetsDiff,
    pub synapses: SynapsesDiff,
    pub layout: LayoutDiff,
    pub camera: Option<CameraJson>,
    pub schema: Option<String>,
    #[serde(default)]
    pub generation: Vec<GenerationOperation>,
}

impl OperationDiff<Procedural2dDocument> for Procedural2dDiff {
    fn apply(&self, projection: &Procedural2dDocument) -> Procedural2dDocument {
        let mut next = projection.clone();
        apply_widgets_diff(&mut next.fixture.widgets, &self.widgets);
        apply_synapses_diff(&mut next.fixture.synapses, &self.synapses);
        for id in &self.layout.removed {
            next.fixture.layout.remove(id);
        }
        for (id, layout) in &self.layout.set {
            next.fixture.layout.insert(id.clone(), layout.clone());
        }
        if let Some(camera) = &self.camera {
            next.fixture.camera = camera.clone();
        }
        if let Some(schema) = &self.schema {
            next.fixture.schema = schema.clone();
        }
        for operation in &self.generation {
            apply_generation_operation(&mut next.generation, operation);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.widgets.removed.extend(other.widgets.removed);
        self.widgets.set.extend(other.widgets.set);
        self.synapses.removed.extend(other.synapses.removed);
        self.synapses.set.extend(other.synapses.set);
        self.layout.removed.extend(other.layout.removed);
        self.layout.set.extend(other.layout.set);
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.generation.extend(other.generation);
    }
}

/// 🧮 Procedural-2d operation: id-keyed widget/synapse/layout collection edits, the scalar canvas
/// camera and fixture schema, and a single {@link GenerationOperation} generation edit with its true inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Procedural2dOperation {
    SetWidget { index: usize, widget: Widget },
    RemoveWidget { id: String },
    SetSynapse { index: usize, synapse: SynapseSpec },
    RemoveSynapse { id: String },
    SetLayout { id: String, layout: WidgetLayout },
    RemoveLayout { id: String },
    SetCamera { camera: CameraJson },
    SetSchema { schema: String },
    Generation(GenerationOperation),
}

fn widget_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.widgets.iter().position(|widget| widget_id(widget) == id)
}

fn synapse_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.synapses.iter().position(|synapse| synapse.id == id)
}

impl Operation<Procedural2dDocument> for Procedural2dOperation {
    type Diff = Procedural2dDiff;

    fn diff(&self, _projection: &Procedural2dDocument) -> Procedural2dDiff {
        let mut diff = Procedural2dDiff::default();
        match self {
            Procedural2dOperation::SetWidget { index, widget } => diff.widgets.set.push((*index, widget.clone())),
            Procedural2dOperation::RemoveWidget { id } => diff.widgets.removed.push(id.clone()),
            Procedural2dOperation::SetSynapse { index, synapse } => diff.synapses.set.push((*index, synapse.clone())),
            Procedural2dOperation::RemoveSynapse { id } => diff.synapses.removed.push(id.clone()),
            Procedural2dOperation::SetLayout { id, layout } => diff.layout.set.push((id.clone(), layout.clone())),
            Procedural2dOperation::RemoveLayout { id } => diff.layout.removed.push(id.clone()),
            Procedural2dOperation::SetCamera { camera } => diff.camera = Some(camera.clone()),
            Procedural2dOperation::SetSchema { schema } => diff.schema = Some(schema.clone()),
            Procedural2dOperation::Generation(operation) => diff.generation.push(operation.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Procedural2dDocument) -> Vec<Self> {
        let fixture = &projection.fixture;
        match self {
            Procedural2dOperation::SetWidget { widget, .. } => match widget_index(fixture, widget_id(widget)) {
                Some(index) => vec![Procedural2dOperation::SetWidget { index, widget: fixture.widgets[index].clone() }],
                None => vec![Procedural2dOperation::RemoveWidget { id: widget_id(widget).to_string() }],
            },
            Procedural2dOperation::RemoveWidget { id } => widget_index(fixture, id).map(|index| vec![Procedural2dOperation::SetWidget { index, widget: fixture.widgets[index].clone() }]).unwrap_or_default(),
            Procedural2dOperation::SetSynapse { synapse, .. } => match synapse_index(fixture, &synapse.id) {
                Some(index) => vec![Procedural2dOperation::SetSynapse { index, synapse: fixture.synapses[index].clone() }],
                None => vec![Procedural2dOperation::RemoveSynapse { id: synapse.id.clone() }],
            },
            Procedural2dOperation::RemoveSynapse { id } => synapse_index(fixture, id).map(|index| vec![Procedural2dOperation::SetSynapse { index, synapse: fixture.synapses[index].clone() }]).unwrap_or_default(),
            Procedural2dOperation::SetLayout { id, .. } => match fixture.layout.get(id) {
                Some(layout) => vec![Procedural2dOperation::SetLayout { id: id.clone(), layout: layout.clone() }],
                None => vec![Procedural2dOperation::RemoveLayout { id: id.clone() }],
            },
            Procedural2dOperation::RemoveLayout { id } => fixture.layout.get(id).map(|layout| vec![Procedural2dOperation::SetLayout { id: id.clone(), layout: layout.clone() }]).unwrap_or_default(),
            Procedural2dOperation::SetCamera { .. } => vec![Procedural2dOperation::SetCamera { camera: fixture.camera.clone() }],
            Procedural2dOperation::SetSchema { .. } => vec![Procedural2dOperation::SetSchema { schema: fixture.schema.clone() }],
            Procedural2dOperation::Generation(operation) => invert_generation_operation(&projection.generation, operation).into_iter().map(Procedural2dOperation::Generation).collect(),
        }
    }
}

/// 🔀 Diffs two fixtures into a minimal, invertible, mergeable operation set: removed/added/patched widgets
/// and synapses (keyed by id), layout entries, and the fixture schema. The canvas camera is ephemeral
/// view state (plugin runtime), never a document operation. Lets action handlers keep computing the target
/// fixture via `FlowHost` while emitting granular operations.
pub fn procedural2d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural2dOperation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(Procedural2dOperation::RemoveWidget { id: widget_id(widget).to_string() });
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let prior = before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget));
        if prior != Some(widget) {
            operations.push(Procedural2dOperation::SetWidget { index, widget: widget.clone() });
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(Procedural2dOperation::RemoveSynapse { id: synapse.id.clone() });
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        let prior = before.synapses.iter().find(|entry| entry.id == synapse.id);
        if prior != Some(synapse) {
            operations.push(Procedural2dOperation::SetSynapse { index, synapse: synapse.clone() });
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(Procedural2dOperation::RemoveLayout { id: id.clone() });
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(Procedural2dOperation::SetLayout { id: id.clone(), layout: layout.clone() });
        }
    }
    if before.schema != after.schema {
        operations.push(Procedural2dOperation::SetSchema { schema: after.schema.clone() });
    }
    operations
}
//#endregion 🔖Operations

//#region 🔖Dsl
//#region 🔖DslMirror
/// 🔒 `FlowFixture`/`Widget`/`SynapseSpec`/`WidgetLayout`/`CameraJson` (from `flow_core`) and
/// `GenerationPlayState`/`FormGeneration`/`GenerationOperation` (from `protocol`) are all foreign to
/// this crate, so none can carry a `#[derive(dsl::Dsl...)]` themselves — Rust's orphan rule requires
/// the impl target type to live in the crate that also owns the trait or the type, and neither is
/// true here. The `*Dsl` types below are LOCAL structural twins the real types convert to/from right
/// at the `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary (same pattern as `fem_2d`'s `FemDof`
/// and `imperative_core`'s `ValueDsl`/`StepNodeDsl`/`PathDsl`) — `Procedural2dDocument`/
/// `Procedural2dOperation` themselves keep their ORIGINAL foreign field types unchanged, so
/// `procedural-plugin` (which constructs/matches on them directly) keeps compiling unmodified.
///
/// `ValueDsl` mirrors `flow_core::neural::Value`/`Atom` field-for-field (duplicating
/// `imperative_core::ValueDsl`'s approach for the identical `neural_engine` types, since crates can't
/// share a private type across the orphan boundary either) rather than routing through the engine's
/// dynamic `Shape::Value`/`DslValue` escape hatch, which merges `Atom::Integer`/`Atom::Decimal` into
/// one `Number(f64)` case — a real, observable loss of fidelity `ValueDsl`'s own mutually-exclusive
/// `Option` fields avoid entirely.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct ValueDsl {
    /// 🕳️ Presence-only flag (the payload is never inspected) — `Atom::Null`'s tag.
    null: Option<bool>,
    #[dsl(key = "bool")]
    boolean: Option<bool>,
    #[dsl(key = "int")]
    integer: Option<i64>,
    decimal: Option<f64>,
    text: Option<String>,
    #[dsl(key = "dict")]
    dictionary: Option<Vec<DictEntryDsl>>,
}

/// 🗝️ One `Dictionary`/`Value::Dictionary` entry — a `Vec` of `(key, value)` records rather than a
/// bare `Shape::Map` (`{ key=value }`): a `Shape::Map` key is a bare identifier (the engine lexer's
/// `is_ident_start` only accepts alphabetic/`_`), but real `Dictionary` keys are arbitrary strings —
/// notably `neural_engine::SCHEMA_KEY` (`"$schema"`), which every schema-tagged value in this
/// codebase carries and which starts with `$`, a character no bare identifier can start with.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct DictEntryDsl {
    key: String,
    #[dsl(block)]
    value: ValueDsl,
}

fn value_to_value_dsl(value: &NeuralValue) -> ValueDsl {
    let mut dsl_value = ValueDsl { null: None, boolean: None, integer: None, decimal: None, text: None, dictionary: None };
    match value {
        NeuralValue::Atom(Atom::Null) => dsl_value.null = Some(true),
        NeuralValue::Atom(Atom::Boolean(b)) => dsl_value.boolean = Some(*b),
        NeuralValue::Atom(Atom::Integer(i)) => dsl_value.integer = Some(*i),
        NeuralValue::Atom(Atom::Decimal(d)) => dsl_value.decimal = Some(*d),
        NeuralValue::Atom(Atom::String(s)) => dsl_value.text = Some(s.clone()),
        NeuralValue::Dictionary(dict) => dsl_value.dictionary = Some(dictionary_to_value_dsl_entries(dict)),
    }
    dsl_value
}

fn value_dsl_to_value(dsl_value: &ValueDsl) -> NeuralValue {
    if dsl_value.null.is_some() {
        return NeuralValue::Atom(Atom::Null);
    }
    if let Some(b) = dsl_value.boolean {
        return NeuralValue::Atom(Atom::Boolean(b));
    }
    if let Some(i) = dsl_value.integer {
        return NeuralValue::Atom(Atom::Integer(i));
    }
    if let Some(d) = dsl_value.decimal {
        return NeuralValue::Atom(Atom::Decimal(d));
    }
    if let Some(s) = &dsl_value.text {
        return NeuralValue::Atom(Atom::String(s.clone()));
    }
    match &dsl_value.dictionary {
        Some(entries) => NeuralValue::Dictionary(value_dsl_entries_to_dictionary(entries)),
        None => NeuralValue::Atom(Atom::Null),
    }
}

fn dictionary_to_value_dsl_entries(dict: &Dictionary) -> Vec<DictEntryDsl> {
    dict.keys().map(|key| DictEntryDsl { key: key.clone(), value: value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")) }).collect()
}

fn value_dsl_entries_to_dictionary(entries: &[DictEntryDsl]) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, entry| dict.insert(entry.key.clone(), value_dsl_to_value(&entry.value)))
}

/// 🎥 Local twin of `flow_core::CameraJson`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct CameraJsonDsl {
    x: f64,
    y: f64,
    zoom: f64,
}

fn camera_to_dsl(camera: &CameraJson) -> CameraJsonDsl {
    CameraJsonDsl { x: camera.x, y: camera.y, zoom: camera.zoom }
}

fn camera_from_dsl(camera: CameraJsonDsl) -> CameraJson {
    CameraJson { x: camera.x, y: camera.y, zoom: camera.zoom }
}

/// 📍 Local twin of `flow_core::WidgetLayout`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct WidgetLayoutDsl {
    x: f64,
    y: f64,
}

fn layout_to_dsl(layout: &WidgetLayout) -> WidgetLayoutDsl {
    WidgetLayoutDsl { x: layout.x, y: layout.y }
}

fn layout_from_dsl(layout: WidgetLayoutDsl) -> WidgetLayout {
    WidgetLayout { x: layout.x, y: layout.y }
}

/// 🔗 Local twin of `flow_core::SynapseSpec` — a graph edge (`from@fromPort->to@toPort`) via the
/// engine's unified `dsl::Wire` shape; an empty `from_port`/`to_port` (the "no explicit port" sentinel
/// the real `SynapseSpec` uses) round-trips through an absent `WireNode::port`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct SynapseSpecDsl {
    id: String,
    wire: dsl::Wire,
}

fn synapse_to_dsl(synapse: &SynapseSpec) -> SynapseSpecDsl {
    SynapseSpecDsl {
        id: synapse.id.clone(),
        wire: dsl::Wire(dsl::WireValue {
            from: dsl::WireNode { id: synapse.from.clone(), kind: None, port: (!synapse.from_port.is_empty()).then(|| synapse.from_port.clone()) },
            edge: Some((true, dsl::WireNode { id: synapse.to.clone(), kind: None, port: (!synapse.to_port.is_empty()).then(|| synapse.to_port.clone()) })),
            properties: dsl::DslValue::Object(Vec::new()),
        }),
    }
}

fn synapse_from_dsl(synapse: SynapseSpecDsl) -> SynapseSpec {
    let wire = synapse.wire.0;
    let to = wire.edge.map(|(_, to)| to).unwrap_or_default();
    SynapseSpec { id: synapse.id, from: wire.from.id, to: to.id, from_port: wire.from.port.unwrap_or_default(), to_port: to.port.unwrap_or_default() }
}

/// 🎛️ Local twin of `flow_core::Widget` — `Neuron`/`OutputPreview`'s `Dictionary` fields route
/// through `ValueDsl`; `Cluster`'s `tree`/`flow` (a `neural_engine::Tree`/`flow_core::FlowGui` pair,
/// each themselves nested foreign aggregates several layers deep) are carried as an opaque
/// `serde_json::Value` blob — exactly the same "already-JSON-shaped" treatment the OLD hand-rolled
/// `procedural2d_text::kv_json` gave them, just bound through the engine's own `Shape::Value` bridge
/// instead of a hand-rolled quoted-string re-parse.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum WidgetDsl {
    Neuron {
        id: String,
        neuron_kind: String,
        preview: bool,
        input_ports: Vec<String>,
        output_ports: Vec<String>,
        #[dsl(table)]
        params: Vec<DictEntryDsl>,
    },
    InputSlider { id: String, value: f64, min: f64, max: f64, step: f64 },
    InputNote { id: String, text: String },
    InputImage { id: String, src: String },
    Variable { id: String, name: String, schema: String },
    OutputPreview { id: String, #[dsl(table)] preview: Vec<DictEntryDsl>, expanded: Vec<String> },
    OutputAction { id: String, action: String },
    OutputExport { id: String, format: String },
    Cluster { id: String, name: String, tree: serde_json::Value, flow: serde_json::Value },
}

fn widget_to_dsl(widget: &Widget) -> WidgetDsl {
    match widget {
        Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => {
            WidgetDsl::Neuron { id: id.clone(), neuron_kind: neuron_kind.clone(), preview: *preview, input_ports: input_ports.clone(), output_ports: output_ports.clone(), params: dictionary_to_value_dsl_entries(params) }
        }
        Widget::InputSlider { id, value, min, max, step } => WidgetDsl::InputSlider { id: id.clone(), value: *value, min: *min, max: *max, step: *step },
        Widget::InputNote { id, text } => WidgetDsl::InputNote { id: id.clone(), text: text.clone() },
        Widget::InputImage { id, src } => WidgetDsl::InputImage { id: id.clone(), src: src.clone() },
        Widget::Variable { id, name, schema } => WidgetDsl::Variable { id: id.clone(), name: name.clone(), schema: schema.clone() },
        Widget::OutputPreview { id, preview, expanded } => WidgetDsl::OutputPreview { id: id.clone(), preview: dictionary_to_value_dsl_entries(preview), expanded: expanded.iter().cloned().collect() },
        Widget::OutputAction { id, action } => WidgetDsl::OutputAction { id: id.clone(), action: action.clone() },
        Widget::OutputExport { id, format } => WidgetDsl::OutputExport { id: id.clone(), format: format.clone() },
        Widget::Cluster { id, name, tree, flow } => {
            WidgetDsl::Cluster { id: id.clone(), name: name.clone(), tree: serde_json::to_value(tree).unwrap_or(serde_json::Value::Null), flow: serde_json::to_value(flow).unwrap_or(serde_json::Value::Null) }
        }
    }
}

fn widget_from_dsl(widget: WidgetDsl) -> Result<Widget, vcs::TextError> {
    Ok(match widget {
        WidgetDsl::Neuron { id, neuron_kind, preview, input_ports, output_ports, params } => Widget::Neuron { id, neuron_kind, params: value_dsl_entries_to_dictionary(&params), input_ports, output_ports, preview },
        WidgetDsl::InputSlider { id, value, min, max, step } => Widget::InputSlider { id, value, min, max, step },
        WidgetDsl::InputNote { id, text } => Widget::InputNote { id, text },
        WidgetDsl::InputImage { id, src } => Widget::InputImage { id, src },
        WidgetDsl::Variable { id, name, schema } => Widget::Variable { id, name, schema },
        WidgetDsl::OutputPreview { id, preview, expanded } => Widget::OutputPreview { id, preview: value_dsl_entries_to_dictionary(&preview), expanded: expanded.into_iter().collect() },
        WidgetDsl::OutputAction { id, action } => Widget::OutputAction { id, action },
        WidgetDsl::OutputExport { id, format } => Widget::OutputExport { id, format },
        WidgetDsl::Cluster { id, name, tree, flow } => Widget::Cluster {
            id,
            name,
            tree: serde_json::from_value(tree).map_err(|error| vcs::TextError::new(format!("invalid cluster tree JSON: {error}"), vcs::TextSpan::at(1, 1)))?,
            flow: serde_json::from_value(flow).map_err(|error| vcs::TextError::new(format!("invalid cluster flow JSON: {error}"), vcs::TextSpan::at(1, 1)))?,
        },
    })
}

/// 🧬 Local twin of `protocol::FormGeneration` — `values` is already a `serde_json::Map`/`Value` pair
/// in the real type, so it binds directly through the engine's `Shape::Value` bridge with no
/// intermediate conversion.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct FormGenerationDsl {
    id: String,
    name: String,
    values: serde_json::Map<String, serde_json::Value>,
}

fn form_generation_to_dsl(generation: &FormGeneration) -> FormGenerationDsl {
    FormGenerationDsl { id: generation.id.clone(), name: generation.name.clone(), values: generation.values.clone() }
}

fn form_generation_from_dsl(generation: FormGenerationDsl) -> FormGeneration {
    FormGeneration { id: generation.id, name: generation.name, values: generation.values }
}

/// 🧾 Local twin of `Procedural2dDocument`, flattening `FlowFixture`/`GenerationPlayState`'s fields
/// into one top-level `#[derive(dsl::DslDocument)]` grammar.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "procedural2d", layout = "lines")]
struct Procedural2dDocumentDsl {
    schema: String,
    #[dsl(block)]
    camera: CameraJsonDsl,
    #[dsl(statements, block)]
    widgets: Vec<WidgetDsl>,
    #[dsl(table)]
    synapses: Vec<SynapseSpecDsl>,
    layout: BTreeMap<String, WidgetLayoutDsl>,
    #[dsl(key = "selected-generation")]
    selected_generation_id: Option<String>,
    preview_text: Option<String>,
    #[dsl(table)]
    generations: Vec<FormGenerationDsl>,
}

fn procedural2d_document_to_dsl(document: &Procedural2dDocument) -> Procedural2dDocumentDsl {
    let fixture = &document.fixture;
    let generation = &document.generation;
    Procedural2dDocumentDsl {
        schema: fixture.schema.clone(),
        camera: camera_to_dsl(&fixture.camera),
        widgets: fixture.widgets.iter().map(widget_to_dsl).collect(),
        synapses: fixture.synapses.iter().map(synapse_to_dsl).collect(),
        layout: fixture.layout.iter().map(|(id, entry)| (id.clone(), layout_to_dsl(entry))).collect(),
        selected_generation_id: generation.selected_generation_id.clone(),
        preview_text: generation.preview_text.clone(),
        generations: generation.generations.iter().map(form_generation_to_dsl).collect(),
    }
}

fn procedural2d_document_from_dsl(parsed: Procedural2dDocumentDsl) -> Result<Procedural2dDocument, vcs::TextError> {
    let widgets = parsed.widgets.into_iter().map(widget_from_dsl).collect::<Result<Vec<_>, _>>()?;
    let synapses = parsed.synapses.into_iter().map(synapse_from_dsl).collect();
    let layout = parsed.layout.into_iter().map(|(id, entry)| (id, layout_from_dsl(entry))).collect();
    Ok(Procedural2dDocument {
        fixture: FlowFixture { schema: parsed.schema, camera: camera_from_dsl(parsed.camera), widgets, synapses, layout },
        generation: GenerationPlayState { generations: parsed.generations.into_iter().map(form_generation_from_dsl).collect(), selected_generation_id: parsed.selected_generation_id, preview_text: parsed.preview_text },
    })
}
//#endregion 🔖DslMirror

/// 📜 `.procedural2d` textual document — derive-engine grammar via `Procedural2dDocumentDsl`
/// (see `🔖DslMirror`); `parse_dsl`/`print_dsl` convert at the boundary.
impl vcs::DocumentDsl for Procedural2dDocument {
    const EXTENSION: &'static str = "procedural2d";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        let parsed = <Procedural2dDocumentDsl as vcs::DocumentDsl>::parse_dsl(text)?;
        procedural2d_document_from_dsl(parsed)
    }

    fn print_dsl(&self) -> String {
        <Procedural2dDocumentDsl as vcs::DocumentDsl>::print_dsl(&procedural2d_document_to_dsl(self))
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
/// ⚡ Local twin of `Procedural2dOperation` — flattens the `Generation(GenerationOperation)` newtype
/// variant into its own four top-level keyword variants (mirroring the OLD hand-rolled op-line
/// keywords `generation-add`/`generation-remove`/`generation-rename`/`generation-update-values`)
/// since a `#[derive(dsl::DslOps)]` enum's variants are each their own tagged record, not a nested
/// enum-in-enum.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum Procedural2dOperationDsl {
    SetWidget {
        index: usize,
        #[dsl(statements)]
        widget: Box<WidgetDsl>,
    },
    RemoveWidget { id: String },
    SetSynapse {
        index: usize,
        #[dsl(block)]
        synapse: SynapseSpecDsl,
    },
    RemoveSynapse { id: String },
    SetLayout {
        id: String,
        #[dsl(block)]
        layout: WidgetLayoutDsl,
    },
    RemoveLayout { id: String },
    SetCamera {
        #[dsl(block)]
        camera: CameraJsonDsl,
    },
    SetSchema { schema: String },
    GenerationAdd {
        #[dsl(block)]
        generation: FormGenerationDsl,
    },
    GenerationRemove { id: String },
    GenerationRename { id: String, name: String },
    GenerationUpdateValues {
        id: String,
        question_id: String,
        value: serde_json::Value,
    },
}

fn procedural2d_operation_to_dsl(operation: &Procedural2dOperation) -> Procedural2dOperationDsl {
    match operation {
        Procedural2dOperation::SetWidget { index, widget } => Procedural2dOperationDsl::SetWidget { index: *index, widget: Box::new(widget_to_dsl(widget)) },
        Procedural2dOperation::RemoveWidget { id } => Procedural2dOperationDsl::RemoveWidget { id: id.clone() },
        Procedural2dOperation::SetSynapse { index, synapse } => Procedural2dOperationDsl::SetSynapse { index: *index, synapse: synapse_to_dsl(synapse) },
        Procedural2dOperation::RemoveSynapse { id } => Procedural2dOperationDsl::RemoveSynapse { id: id.clone() },
        Procedural2dOperation::SetLayout { id, layout } => Procedural2dOperationDsl::SetLayout { id: id.clone(), layout: layout_to_dsl(layout) },
        Procedural2dOperation::RemoveLayout { id } => Procedural2dOperationDsl::RemoveLayout { id: id.clone() },
        Procedural2dOperation::SetCamera { camera } => Procedural2dOperationDsl::SetCamera { camera: camera_to_dsl(camera) },
        Procedural2dOperation::SetSchema { schema } => Procedural2dOperationDsl::SetSchema { schema: schema.clone() },
        Procedural2dOperation::Generation(GenerationOperation::Add { generation }) => Procedural2dOperationDsl::GenerationAdd { generation: form_generation_to_dsl(generation) },
        Procedural2dOperation::Generation(GenerationOperation::Remove { id }) => Procedural2dOperationDsl::GenerationRemove { id: id.clone() },
        Procedural2dOperation::Generation(GenerationOperation::Rename { id, name }) => Procedural2dOperationDsl::GenerationRename { id: id.clone(), name: name.clone() },
        Procedural2dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value }) => {
            Procedural2dOperationDsl::GenerationUpdateValues { id: id.clone(), question_id: question_id.clone(), value: value.clone() }
        }
    }
}

fn procedural2d_operation_from_dsl(operation: Procedural2dOperationDsl) -> Result<Procedural2dOperation, vcs::TextError> {
    Ok(match operation {
        Procedural2dOperationDsl::SetWidget { index, widget } => Procedural2dOperation::SetWidget { index, widget: widget_from_dsl(*widget)? },
        Procedural2dOperationDsl::RemoveWidget { id } => Procedural2dOperation::RemoveWidget { id },
        Procedural2dOperationDsl::SetSynapse { index, synapse } => Procedural2dOperation::SetSynapse { index, synapse: synapse_from_dsl(synapse) },
        Procedural2dOperationDsl::RemoveSynapse { id } => Procedural2dOperation::RemoveSynapse { id },
        Procedural2dOperationDsl::SetLayout { id, layout } => Procedural2dOperation::SetLayout { id, layout: layout_from_dsl(layout) },
        Procedural2dOperationDsl::RemoveLayout { id } => Procedural2dOperation::RemoveLayout { id },
        Procedural2dOperationDsl::SetCamera { camera } => Procedural2dOperation::SetCamera { camera: camera_from_dsl(camera) },
        Procedural2dOperationDsl::SetSchema { schema } => Procedural2dOperation::SetSchema { schema },
        Procedural2dOperationDsl::GenerationAdd { generation } => Procedural2dOperation::Generation(GenerationOperation::Add { generation: form_generation_from_dsl(generation) }),
        Procedural2dOperationDsl::GenerationRemove { id } => Procedural2dOperation::Generation(GenerationOperation::Remove { id }),
        Procedural2dOperationDsl::GenerationRename { id, name } => Procedural2dOperation::Generation(GenerationOperation::Rename { id, name }),
        Procedural2dOperationDsl::GenerationUpdateValues { id, question_id, value } => Procedural2dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value }),
    })
}

/// ⚡ `Procedural2dOperation`'s compact single-line op encoding — derive-engine grammar via
/// `Procedural2dOperationDsl` (see above); `parse_op`/`print_op` convert at the boundary.
impl vcs::OpText for Procedural2dOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        let parsed = <Procedural2dOperationDsl as vcs::OpText>::parse_op(line)?;
        procedural2d_operation_from_dsl(parsed)
    }

    fn print_op(&self) -> String {
        <Procedural2dOperationDsl as vcs::OpText>::print_op(&procedural2d_operation_to_dsl(self))
    }
}
//#endregion 🔖OpText

pub type Procedural2dEnvelope = DocumentVcsEnvelope<Procedural2dDocument, Procedural2dOperation>;
pub type Procedural2dStore = DocumentVcsStore<Procedural2dDocument, Procedural2dOperation>;

pub fn empty_procedural2d_projection() -> Procedural2dDocument {
    Procedural2dDocument::default()
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use vcs::create_document_vcs_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Procedural2dDocumentVcs {
        store: RefCell<Procedural2dStore>,
    }

    #[wasm_bindgen]
    impl Procedural2dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Procedural2dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Procedural2dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Procedural2dStore::new(envelope)
                }
                None => Procedural2dStore::new(create_document_vcs_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", empty_procedural2d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use vcs::{apply_operation, create_document_vcs_envelope, test_support, DocumentDsl, DocumentVcsCommand, OpText};

    fn round_trip(projection: &Procedural2dDocument, operation: &Procedural2dOperation) -> Procedural2dDocument {
        let forward = apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn fixture_ops_ignore_camera() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.camera = CameraJson { x: 7.0, y: 8.0, zoom: 2.0 };
        let operations = procedural2d_fixture_operations(&before, &after);
        assert!(operations.iter().all(|operation| !matches!(operation, Procedural2dOperation::SetCamera { .. })));
    }

    #[test]
    fn remove_and_readd_widget_round_trips() {
        let base = empty_procedural2d_projection();
        let removed_id = widget_id(&base.fixture.widgets[0]).to_string();
        let after = round_trip(&base, &Procedural2dOperation::RemoveWidget { id: removed_id.clone() });
        assert!(!after.fixture.widgets.iter().any(|w| widget_id(w) == removed_id));
    }

    #[test]
    fn fixture_ops_capture_widget_add() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.widgets.push(Widget::InputNote { id: "note-1".into(), text: String::new() });
        let operations = procedural2d_fixture_operations(&before, &after);
        assert!(operations.iter().any(|operation| matches!(operation, Procedural2dOperation::SetWidget { widget, .. } if widget_id(widget) == "note-1")));
    }

    #[test]
    fn generation_op_round_trips() {
        let before = empty_procedural2d_projection();
        let generation = protocol::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        let after = round_trip(&before, &Procedural2dOperation::Generation(GenerationOperation::Add { generation }));
        assert_eq!(after.generation.generations.len(), 1);
    }

    //#region 🔖DslTests
    #[test]
    fn dsl_round_trip_empty_projection() {
        test_support::assert_dsl_round_trip(&empty_procedural2d_projection());
    }

    #[test]
    fn dsl_round_trip_example_fixture() {
        let text = include_str!("../example/default.procedural2d");
        let projection = Procedural2dDocument::parse_dsl(text).expect("parse default.procedural2d fixture");
        test_support::assert_dsl_round_trip(&projection);
    }

    #[test]
    fn dsl_round_trip_with_generation_state() {
        let mut projection = empty_procedural2d_projection();
        let mut values = serde_json::Map::new();
        // 🌱 A float literal, not `json!(3)` (an integer-backed `serde_json::Number`): the DSL
        // engine's `Shape::Value`/`DslValue::Number` is a single `f64` variant (see `dsl/rs/lib.rs`'s
        // own documented int-vs-float caveat), so a value round tripping through generation `values`
        // always comes back float-backed — this is the known, accepted engine limitation, not a bug
        // in this crate's mirror/conversion code.
        values.insert("count".into(), serde_json::json!(3.0));
        projection.generation.generations.push(protocol::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values });
        projection.generation.selected_generation_id = Some("generation-1".into());
        projection.generation.preview_text = Some("42".into());
        test_support::assert_dsl_round_trip(&projection);
    }

    #[test]
    fn dsl_round_trip_covers_every_widget_kind() {
        let mut projection = empty_procedural2d_projection();
        projection.fixture.widgets = vec![
            Widget::InputSlider { id: "slider".into(), value: 2.0, min: 0.0, max: 10.0, step: 0.5 },
            Widget::InputImage { id: "image".into(), src: "data:image/png;base64,abc".into() },
            Widget::Variable { id: "variable".into(), name: "value".into(), schema: "dictionary".into() },
            Widget::OutputAction { id: "action".into(), action: "export".into() },
            Widget::OutputExport { id: "export".into(), format: "svg".into() },
            Widget::Cluster { id: "cluster".into(), name: "Group".into(), tree: Default::default(), flow: Default::default() },
        ];
        projection.fixture.synapses = vec![];
        test_support::assert_dsl_round_trip(&projection);
    }
    //#endregion 🔖DslTests

    //#region 🔖OpTextTests
    #[test]
    fn op_text_round_trip_set_widget() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetWidget { index: 2, widget: Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() } });
    }

    #[test]
    fn op_text_round_trip_remove_widget() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveWidget { id: "note-9".into() });
    }

    #[test]
    fn op_text_round_trip_set_synapse() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetSynapse {
            index: 1,
            synapse: SynapseSpec { id: "s1".into(), from: "rect".into(), to: "fill".into(), from_port: "draw.drawing".into(), to_port: String::new() },
        });
    }

    #[test]
    fn op_text_round_trip_remove_synapse() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveSynapse { id: "s1".into() });
    }

    #[test]
    fn op_text_round_trip_set_layout() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetLayout { id: "rect".into(), layout: WidgetLayout { x: 12.5, y: -8.25 } });
    }

    #[test]
    fn op_text_round_trip_remove_layout() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveLayout { id: "rect".into() });
    }

    #[test]
    fn op_text_round_trip_set_camera() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetCamera { camera: CameraJson { x: 1.5, y: -2.5, zoom: 1.2 } });
    }

    #[test]
    fn op_text_round_trip_set_schema() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetSchema { schema: "flow.fixture".into() });
    }

    #[test]
    fn op_text_round_trip_generation() {
        let generation = protocol::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        test_support::assert_op_line_round_trip(&Procedural2dOperation::Generation(GenerationOperation::Add { generation }));
    }
    //#endregion 🔖OpTextTests

    //#region 🔖DocumentTextTests
    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = Procedural2dStore::new(create_document_vcs_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", empty_procedural2d_projection(), None));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Procedural2dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }],
                description: None,
            })
            .expect("apply");
        test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DocumentTextTests

    //#region 🔖DiffTests
    #[test]
    fn diff_absorb_merges_vecs_and_updates_scalars_when_present() {
        let mut diff = Procedural2dDiff { camera: Some(CameraJson { x: 1.0, y: 1.0, zoom: 1.0 }), ..Default::default() };
        diff.widgets.removed.push("w1".into());

        diff.absorb(Procedural2dDiff {
            widgets: WidgetsDiff { removed: vec!["w2".into()], set: vec![(0, Widget::InputNote { id: "note".into(), text: String::new() })] },
            synapses: SynapsesDiff { removed: vec!["s1".into()], set: vec![] },
            layout: LayoutDiff { removed: vec![], set: vec![("l1".into(), WidgetLayout { x: 3.0, y: 4.0 })] },
            camera: Some(CameraJson { x: 9.0, y: 9.0, zoom: 2.0 }),
            schema: Some("flow.fixture".into()),
            generation: vec![GenerationOperation::Remove { id: "g1".into() }],
        });

        assert_eq!(diff.widgets.removed, vec!["w1".to_string(), "w2".to_string()]);
        assert_eq!(diff.widgets.set.len(), 1);
        assert_eq!(diff.synapses.removed, vec!["s1".to_string()]);
        assert_eq!(diff.layout.set.len(), 1);
        assert_eq!(diff.camera, Some(CameraJson { x: 9.0, y: 9.0, zoom: 2.0 }));
        assert_eq!(diff.schema, Some("flow.fixture".to_string()));
        assert_eq!(diff.generation.len(), 1);
    }

    #[test]
    fn diff_absorb_keeps_scalar_when_incoming_is_none() {
        let mut diff = Procedural2dDiff { camera: Some(CameraJson { x: 1.0, y: 2.0, zoom: 1.0 }), schema: Some("flow.fixture".into()), ..Default::default() };
        diff.absorb(Procedural2dDiff::default());
        assert_eq!(diff.camera, Some(CameraJson { x: 1.0, y: 2.0, zoom: 1.0 }));
        assert_eq!(diff.schema, Some("flow.fixture".to_string()));
    }

    #[test]
    fn diff_apply_inserts_new_widget_and_replaces_existing_by_id() {
        let projection = empty_procedural2d_projection();
        let existing_id = widget_id(&projection.fixture.widgets[1]).to_string();
        let diff = Procedural2dDiff {
            widgets: WidgetsDiff {
                removed: vec![],
                set: vec![(0, Widget::InputNote { id: existing_id.clone(), text: "replaced".into() }), (999, Widget::InputNote { id: "brand-new".into(), text: "new".into() })],
            },
            ..Default::default()
        };
        let next = diff.apply(&projection);
        assert_eq!(next.fixture.widgets.len(), projection.fixture.widgets.len() + 1);
        let replaced = next.fixture.widgets.iter().find(|w| widget_id(w) == existing_id.as_str()).expect("replaced widget present");
        assert_eq!(replaced, &Widget::InputNote { id: existing_id, text: "replaced".into() });
        assert_eq!(widget_id(next.fixture.widgets.last().expect("inserted widget")), "brand-new");
    }

    #[test]
    fn diff_apply_updates_camera_and_schema_only_when_present() {
        let projection = empty_procedural2d_projection();
        let untouched = Procedural2dDiff::default().apply(&projection);
        assert_eq!(untouched.fixture.camera, projection.fixture.camera);
        assert_eq!(untouched.fixture.schema, projection.fixture.schema);

        let changed = Procedural2dDiff { camera: Some(CameraJson { x: 5.0, y: 6.0, zoom: 3.0 }), schema: Some("other.schema".into()), ..Default::default() }.apply(&projection);
        assert_eq!(changed.fixture.camera, CameraJson { x: 5.0, y: 6.0, zoom: 3.0 });
        assert_eq!(changed.fixture.schema, "other.schema");
    }
    //#endregion 🔖DiffTests

    //#region 🔖OperationBackwardsTests
    #[test]
    fn set_widget_backwards_restores_replaced_widget() {
        let base = empty_procedural2d_projection();
        let id = widget_id(&base.fixture.widgets[1]).to_string();
        round_trip(&base, &Procedural2dOperation::SetWidget { index: 1, widget: Widget::InputNote { id, text: "replaced".into() } });
    }

    #[test]
    fn set_widget_backwards_removes_newly_inserted_widget() {
        let base = empty_procedural2d_projection();
        let after = round_trip(&base, &Procedural2dOperation::SetWidget { index: 0, widget: Widget::InputNote { id: "brand-new".into(), text: String::new() } });
        assert!(after.fixture.widgets.iter().any(|w| widget_id(w) == "brand-new"));
    }

    #[test]
    fn remove_widget_on_unknown_id_is_a_noop_with_no_backwards_ops() {
        let base = empty_procedural2d_projection();
        let op = Procedural2dOperation::RemoveWidget { id: "does-not-exist".into() };
        assert!(op.backwards(&base).is_empty());
        let after = round_trip(&base, &op);
        assert_eq!(after, base);
    }

    #[test]
    fn set_synapse_backwards_restores_replaced_synapse() {
        let base = empty_procedural2d_projection();
        let id = base.fixture.synapses[0].id.clone();
        round_trip(&base, &Procedural2dOperation::SetSynapse { index: 0, synapse: SynapseSpec { id, from: "add".into(), to: "preview".into(), from_port: "sum".into(), to_port: "changed".into() } });
    }

    #[test]
    fn set_synapse_backwards_removes_newly_inserted_synapse() {
        let base = empty_procedural2d_projection();
        let synapse = SynapseSpec { id: "brand-new-synapse".into(), from: "slider".into(), to: "add".into(), from_port: "number".into(), to_port: "b".into() };
        let after = round_trip(&base, &Procedural2dOperation::SetSynapse { index: 0, synapse });
        assert!(after.fixture.synapses.iter().any(|s| s.id == "brand-new-synapse"));
    }

    #[test]
    fn remove_synapse_backwards_restores_removed_synapse() {
        let base = empty_procedural2d_projection();
        let id = base.fixture.synapses[0].id.clone();
        round_trip(&base, &Procedural2dOperation::RemoveSynapse { id });
    }

    #[test]
    fn remove_synapse_on_unknown_id_is_a_noop_with_no_backwards_ops() {
        let base = empty_procedural2d_projection();
        let op = Procedural2dOperation::RemoveSynapse { id: "missing".into() };
        assert!(op.backwards(&base).is_empty());
    }

    #[test]
    fn set_layout_backwards_restores_prior_layout_entry() {
        let mut base = empty_procedural2d_projection();
        base.fixture.layout.insert("slider".into(), WidgetLayout { x: 1.0, y: 1.0 });
        round_trip(&base, &Procedural2dOperation::SetLayout { id: "slider".into(), layout: WidgetLayout { x: 9.0, y: 9.0 } });
    }

    #[test]
    fn set_layout_backwards_removes_newly_created_layout_entry() {
        let base = empty_procedural2d_projection();
        assert!(base.fixture.layout.is_empty());
        let after = round_trip(&base, &Procedural2dOperation::SetLayout { id: "slider".into(), layout: WidgetLayout { x: 2.0, y: 2.0 } });
        assert!(after.fixture.layout.contains_key("slider"));
    }

    #[test]
    fn remove_layout_backwards_restores_removed_layout_entry() {
        let mut base = empty_procedural2d_projection();
        base.fixture.layout.insert("slider".into(), WidgetLayout { x: 4.0, y: 5.0 });
        round_trip(&base, &Procedural2dOperation::RemoveLayout { id: "slider".into() });
    }

    #[test]
    fn remove_layout_on_unknown_id_is_a_noop_with_no_backwards_ops() {
        let base = empty_procedural2d_projection();
        let op = Procedural2dOperation::RemoveLayout { id: "missing".into() };
        assert!(op.backwards(&base).is_empty());
    }

    #[test]
    fn set_camera_backwards_restores_prior_camera() {
        let base = empty_procedural2d_projection();
        round_trip(&base, &Procedural2dOperation::SetCamera { camera: CameraJson { x: 42.0, y: -3.0, zoom: 5.0 } });
    }

    #[test]
    fn set_schema_backwards_restores_prior_schema() {
        let base = empty_procedural2d_projection();
        round_trip(&base, &Procedural2dOperation::SetSchema { schema: "changed.schema".into() });
    }
    //#endregion 🔖OperationBackwardsTests

    //#region 🔖FixtureOpsTests
    #[test]
    fn fixture_ops_widget_id_matches_every_widget_kind() {
        let widgets = vec![
            Widget::Neuron { id: "w-neuron".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: true },
            Widget::InputSlider { id: "w-slider".into(), value: 1.0, min: 0.0, max: 2.0, step: 0.5 },
            Widget::InputNote { id: "w-note".into(), text: String::new() },
            Widget::InputImage { id: "w-image".into(), src: String::new() },
            Widget::Variable { id: "w-variable".into(), name: "value".into(), schema: "dictionary".into() },
            Widget::OutputPreview { id: "w-preview".into(), preview: Default::default(), expanded: Default::default() },
            Widget::OutputAction { id: "w-action".into(), action: String::new() },
            Widget::OutputExport { id: "w-export".into(), format: "svg".into() },
            Widget::Cluster { id: "w-cluster".into(), name: String::new(), tree: Default::default(), flow: Default::default() },
        ];
        let mut before = FlowFixture::default();
        before.widgets.clear();
        let mut after = before.clone();
        after.widgets = widgets.clone();
        let operations = procedural2d_fixture_operations(&before, &after);
        for widget in &widgets {
            let id = widget_id(widget);
            assert!(operations.iter().any(|op| matches!(op, Procedural2dOperation::SetWidget { widget, .. } if widget_id(widget) == id)));
        }
    }
    //#endregion 🔖FixtureOpsTests

    //#region 🔖DslErrorTests
    /// 📜 The derive-engine grammar (see `🔖DslMirror`) has no leading `document`/`widget`/`synapse`
    /// keyword and no document-level "trailing content is rejected" check (a `RecordSpec`'s own
    /// `parse` simply stops once every field is read — see `dsl_schema::parse`, out of this crate's
    /// ownership scope) — so error assertions below target the engine's OWN real error text for the
    /// equivalent malformed-input shape, not the OLD hand-rolled grammar's wording.
    #[test]
    fn dsl_parse_rejects_malformed_text() {
        let error = Procedural2dDocument::parse_dsl("schema=\"flow.fixture").unwrap_err();
        assert!(error.message.contains("found Error"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_missing_required_field() {
        let text = "camera { x=0 y=0 zoom=1 }\nwidgets { }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("found Absent"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_missing_camera_block() {
        let error = Procedural2dDocument::parse_dsl("schema=\"flow.fixture\"\n").unwrap_err();
        assert!(error.message.contains("expected Record, found Absent"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_unquoted_value_for_string_field() {
        let text = "schema=flow.fixture\ncamera { x=0 y=0 zoom=1 }\nwidgets { }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected Text"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_quoted_value_for_ident_field() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { neuron id=\"n\" neuronKind=\"math.add\" preview=true inputPorts= [ ] outputPorts= [ ] params= [ ] }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected Ident"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_non_numeric_value_for_number_field() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { inputSlider id=\"s\" value=abc min=0 max=1 step=1 }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected a float"), "unexpected error: {}", error.message);
    }

    #[test]
    fn dsl_parse_rejects_invalid_bool_value() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { neuron id=\"n\" neuronKind=math.add preview=maybe inputPorts= [ ] outputPorts= [ ] params= [ ] }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected 'true' or 'false'"), "unexpected error: {}", error.message);
    }

    /// 🧬 `Widget::Cluster`'s `tree`/`flow` fields are the only remaining genuinely free-form value
    /// literal (bound via the engine's `Shape::Value`, see `🔖DslMirror`) — `params`/`preview` moved
    /// to typed `DictEntryDsl` records, so a malformed *value literal* (not JSON text) is now only
    /// reachable through `tree`/`flow`.
    #[test]
    fn dsl_parse_rejects_malformed_value_literal() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { cluster id=\"n\" name=\"n\" tree=bogusvalue flow= [ ] }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected a value literal"), "unexpected error: {}", error.message);
    }

    /// 🏷️ An unrecognized widget kind keyword is simply left unconsumed by `Shape::Statements`
    /// (the engine breaks its variant-matching loop rather than erroring — see `dsl_schema::parse`,
    /// out of this crate's ownership scope), so parsing ultimately fails at the enclosing `widgets
    /// { }` block's closing brace instead of with a dedicated "unknown widget kind" message.
    #[test]
    fn dsl_parse_rejects_unknown_widget_kind() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { bogus id=\"n\" }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural2dDocument::parse_dsl(text).unwrap_err();
        assert!(error.message.contains("expected RBrace"), "unexpected error: {}", error.message);
    }
    //#endregion 🔖DslErrorTests

    //#region 🔖OpTextErrorTests
    #[test]
    fn op_text_parse_rejects_unknown_operation() {
        let error = Procedural2dOperation::parse_op("bogus-op id=\"x\"").unwrap_err();
        assert!(error.message.contains("unknown operation"), "unexpected error: {}", error.message);
    }

    #[test]
    fn op_text_parse_rejects_non_integer_index() {
        let error = Procedural2dOperation::parse_op("set-widget index=abc note text=\"\" id=\"x\"").unwrap_err();
        assert!(error.message.contains("expected Int"), "unexpected error: {}", error.message);
    }
    //#endregion 🔖OpTextErrorTests
}
//#endregion 🧪Tests
