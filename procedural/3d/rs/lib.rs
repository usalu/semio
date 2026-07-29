//! 📐 Procedural 3d document model on `vcs`.

use flow_core::neural::{Atom, Dictionary, Value as NeuralValue};
use flow_core::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use playbook::{apply_generation_operation, invert_generation_operation, FormGeneration, GenerationOperation, GenerationPlayState};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use store::{DocumentEnvelope, DocumentStore};
use protocol::{Operation, OperationDiff};

pub const PROCEDURAL_3D_SCHEMA: &str = "procedural.3d";

//#region 🔖Document
/// 🧾 Persistent procedural-3d document — the flow fixture plus the generation vocabulary state.
/// Ephemeral view state (selection, sun, LOD, preview caches) lives in the plugin app struct.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dDocument {
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
/// 🩹 Sparse procedural-3d diff over the flow fixture's collections plus scalar canvas/schema fields
/// and an ordered list of generation edits applied in sequence.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dDiff {
    pub widgets: WidgetsDiff,
    pub synapses: SynapsesDiff,
    pub layout: LayoutDiff,
    pub camera: Option<CameraJson>,
    pub schema: Option<String>,
    #[serde(default)]
    pub generation: Vec<GenerationOperation>,
}

impl OperationDiff<Procedural3dDocument> for Procedural3dDiff {
    fn apply(&self, projection: &Procedural3dDocument) -> Procedural3dDocument {
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

/// 🧮 Procedural-3d operation: id-keyed widget/synapse/layout collection edits, the scalar canvas
/// camera and fixture schema, and a single {@link GenerationOperation} generation edit with its true inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Procedural3dOperation {
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

impl Operation<Procedural3dDocument> for Procedural3dOperation {
    type Diff = Procedural3dDiff;

    fn diff(&self, _projection: &Procedural3dDocument) -> Procedural3dDiff {
        let mut diff = Procedural3dDiff::default();
        match self {
            Procedural3dOperation::SetWidget { index, widget } => diff.widgets.set.push((*index, widget.clone())),
            Procedural3dOperation::RemoveWidget { id } => diff.widgets.removed.push(id.clone()),
            Procedural3dOperation::SetSynapse { index, synapse } => diff.synapses.set.push((*index, synapse.clone())),
            Procedural3dOperation::RemoveSynapse { id } => diff.synapses.removed.push(id.clone()),
            Procedural3dOperation::SetLayout { id, layout } => diff.layout.set.push((id.clone(), layout.clone())),
            Procedural3dOperation::RemoveLayout { id } => diff.layout.removed.push(id.clone()),
            Procedural3dOperation::SetCamera { camera } => diff.camera = Some(camera.clone()),
            Procedural3dOperation::SetSchema { schema } => diff.schema = Some(schema.clone()),
            Procedural3dOperation::Generation(operation) => diff.generation.push(operation.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Procedural3dDocument) -> Vec<Self> {
        let fixture = &projection.fixture;
        match self {
            Procedural3dOperation::SetWidget { widget, .. } => match widget_index(fixture, widget_id(widget)) {
                Some(index) => vec![Procedural3dOperation::SetWidget { index, widget: fixture.widgets[index].clone() }],
                None => vec![Procedural3dOperation::RemoveWidget { id: widget_id(widget).to_string() }],
            },
            Procedural3dOperation::RemoveWidget { id } => widget_index(fixture, id).map(|index| vec![Procedural3dOperation::SetWidget { index, widget: fixture.widgets[index].clone() }]).unwrap_or_default(),
            Procedural3dOperation::SetSynapse { synapse, .. } => match synapse_index(fixture, &synapse.id) {
                Some(index) => vec![Procedural3dOperation::SetSynapse { index, synapse: fixture.synapses[index].clone() }],
                None => vec![Procedural3dOperation::RemoveSynapse { id: synapse.id.clone() }],
            },
            Procedural3dOperation::RemoveSynapse { id } => synapse_index(fixture, id).map(|index| vec![Procedural3dOperation::SetSynapse { index, synapse: fixture.synapses[index].clone() }]).unwrap_or_default(),
            Procedural3dOperation::SetLayout { id, .. } => match fixture.layout.get(id) {
                Some(layout) => vec![Procedural3dOperation::SetLayout { id: id.clone(), layout: layout.clone() }],
                None => vec![Procedural3dOperation::RemoveLayout { id: id.clone() }],
            },
            Procedural3dOperation::RemoveLayout { id } => fixture.layout.get(id).map(|layout| vec![Procedural3dOperation::SetLayout { id: id.clone(), layout: layout.clone() }]).unwrap_or_default(),
            Procedural3dOperation::SetCamera { .. } => vec![Procedural3dOperation::SetCamera { camera: fixture.camera.clone() }],
            Procedural3dOperation::SetSchema { .. } => vec![Procedural3dOperation::SetSchema { schema: fixture.schema.clone() }],
            Procedural3dOperation::Generation(operation) => invert_generation_operation(&projection.generation, operation).into_iter().map(Procedural3dOperation::Generation).collect(),
        }
    }
}

/// 🔀 Diffs two fixtures into a minimal, invertible, mergeable operation set: removed/added/patched widgets
/// and synapses (keyed by id), layout entries, and the fixture schema. The canvas camera is ephemeral
/// view state (plugin runtime), never a document operation. Lets action handlers keep computing the target
/// fixture via `FlowHost` while emitting granular operations.
pub fn procedural3d_fixture_operations(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural3dOperation> {
    let mut operations = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            operations.push(Procedural3dOperation::RemoveWidget { id: widget_id(widget).to_string() });
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let prior = before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget));
        if prior != Some(widget) {
            operations.push(Procedural3dOperation::SetWidget { index, widget: widget.clone() });
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            operations.push(Procedural3dOperation::RemoveSynapse { id: synapse.id.clone() });
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        let prior = before.synapses.iter().find(|entry| entry.id == synapse.id);
        if prior != Some(synapse) {
            operations.push(Procedural3dOperation::SetSynapse { index, synapse: synapse.clone() });
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(Procedural3dOperation::RemoveLayout { id: id.clone() });
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(Procedural3dOperation::SetLayout { id: id.clone(), layout: layout.clone() });
        }
    }
    if before.schema != after.schema {
        operations.push(Procedural3dOperation::SetSchema { schema: after.schema.clone() });
    }
    operations
}
//#endregion 🔖Operations

//#region 🔖Dsl
//#region 🔖DslMirror
/// 🔒 `FlowFixture`/`Widget`/`SynapseSpec`/`WidgetLayout`/`CameraJson` (from `flow_core`) and
/// `GenerationPlayState`/`FormGeneration`/`GenerationOperation` (from `playbook`) are all foreign to
/// this crate, so none can carry a `#[derive(dsl::Dsl...)]` themselves — Rust's orphan rule requires
/// the impl target type to live in the crate that also owns the trait or the type, and neither is
/// true here. The `*Dsl` types below are LOCAL structural twins the real types convert to/from right
/// at the `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary (same pattern as `fem_2d`'s `FemDof`
/// and `imperative_core`'s `ValueDsl`/`StepNodeDsl`/`PathDsl`) — `Procedural3dDocument`/
/// `Procedural3dOperation` themselves keep their ORIGINAL foreign field types unchanged, so
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
/// `procedural3d_text::kv_json` gave them, just bound through the engine's own `Shape::Value` bridge
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

fn widget_from_dsl(widget: WidgetDsl) -> Result<Widget, store::TextError> {
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
            tree: serde_json::from_value(tree).map_err(|error| store::TextError::new(format!("invalid cluster tree JSON: {error}"), store::TextSpan::at(1, 1)))?,
            flow: serde_json::from_value(flow).map_err(|error| store::TextError::new(format!("invalid cluster flow JSON: {error}"), store::TextSpan::at(1, 1)))?,
        },
    })
}

/// 🧬 Local twin of `playbook::FormGeneration` — `values` is already a `serde_json::Map`/`Value` pair
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

/// 🧾 Local twin of `Procedural3dDocument`, flattening `FlowFixture`/`GenerationPlayState`'s fields
/// into one top-level `#[derive(dsl::DslDocument)]` grammar.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "procedural3d", layout = "lines")]
struct Procedural3dDocumentDsl {
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

fn procedural3d_document_to_dsl(document: &Procedural3dDocument) -> Procedural3dDocumentDsl {
    let fixture = &document.fixture;
    let generation = &document.generation;
    Procedural3dDocumentDsl {
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

fn procedural3d_document_from_dsl(parsed: Procedural3dDocumentDsl) -> Result<Procedural3dDocument, store::TextError> {
    let widgets = parsed.widgets.into_iter().map(widget_from_dsl).collect::<Result<Vec<_>, _>>()?;
    let synapses = parsed.synapses.into_iter().map(synapse_from_dsl).collect();
    let layout = parsed.layout.into_iter().map(|(id, entry)| (id, layout_from_dsl(entry))).collect();
    Ok(Procedural3dDocument {
        fixture: FlowFixture { schema: parsed.schema, camera: camera_from_dsl(parsed.camera), widgets, synapses, layout },
        generation: GenerationPlayState { generations: parsed.generations.into_iter().map(form_generation_from_dsl).collect(), selected_generation_id: parsed.selected_generation_id, preview_text: parsed.preview_text },
    })
}
//#endregion 🔖DslMirror

/// 📜 `.procedural3d` textual document — derive-engine grammar via `Procedural3dDocumentDsl`
/// (see `🔖DslMirror`); `parse_dsl`/`print_dsl` convert at the boundary.
impl store::DocumentDsl for Procedural3dDocument {
    const EXTENSION: &'static str = "procedural3d";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural3dDocumentDsl as store::DocumentDsl>::parse_dsl(text)?;
        procedural3d_document_from_dsl(parsed)
    }

    fn print_dsl(&self) -> String {
        <Procedural3dDocumentDsl as store::DocumentDsl>::print_dsl(&procedural3d_document_to_dsl(self))
    }
}

/// 📦 `.procedural3d` binary pack — same `Procedural3dDocumentDsl` mirror as `DocumentDsl` above (see
/// `🔖DslMirror`); `dsl::DslDocument`'s derive already gives `Procedural3dDocumentDsl` its own
/// `DocumentPack` impl, so this just routes through the same to/from-dsl boundary functions.
impl store::DocumentPack for Procedural3dDocument {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Procedural3dDocumentDsl as store::DocumentPack>::encode_pack_with(&procedural3d_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Procedural3dDocumentDsl as store::DocumentPack>::decode_pack_with(bytes, options)?;
        procedural3d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
/// ⚡ Local twin of `Procedural3dOperation` — flattens the `Generation(GenerationOperation)` newtype
/// variant into its own four top-level keyword variants (mirroring the OLD hand-rolled op-line
/// keywords `generation-add`/`generation-remove`/`generation-rename`/`generation-update-values`)
/// since a `#[derive(dsl::DslOps)]` enum's variants are each their own tagged record, not a nested
/// enum-in-enum.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum Procedural3dOperationDsl {
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

fn procedural3d_operation_to_dsl(operation: &Procedural3dOperation) -> Procedural3dOperationDsl {
    match operation {
        Procedural3dOperation::SetWidget { index, widget } => Procedural3dOperationDsl::SetWidget { index: *index, widget: Box::new(widget_to_dsl(widget)) },
        Procedural3dOperation::RemoveWidget { id } => Procedural3dOperationDsl::RemoveWidget { id: id.clone() },
        Procedural3dOperation::SetSynapse { index, synapse } => Procedural3dOperationDsl::SetSynapse { index: *index, synapse: synapse_to_dsl(synapse) },
        Procedural3dOperation::RemoveSynapse { id } => Procedural3dOperationDsl::RemoveSynapse { id: id.clone() },
        Procedural3dOperation::SetLayout { id, layout } => Procedural3dOperationDsl::SetLayout { id: id.clone(), layout: layout_to_dsl(layout) },
        Procedural3dOperation::RemoveLayout { id } => Procedural3dOperationDsl::RemoveLayout { id: id.clone() },
        Procedural3dOperation::SetCamera { camera } => Procedural3dOperationDsl::SetCamera { camera: camera_to_dsl(camera) },
        Procedural3dOperation::SetSchema { schema } => Procedural3dOperationDsl::SetSchema { schema: schema.clone() },
        Procedural3dOperation::Generation(GenerationOperation::Add { generation }) => Procedural3dOperationDsl::GenerationAdd { generation: form_generation_to_dsl(generation) },
        Procedural3dOperation::Generation(GenerationOperation::Remove { id }) => Procedural3dOperationDsl::GenerationRemove { id: id.clone() },
        Procedural3dOperation::Generation(GenerationOperation::Rename { id, name }) => Procedural3dOperationDsl::GenerationRename { id: id.clone(), name: name.clone() },
        Procedural3dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value }) => {
            Procedural3dOperationDsl::GenerationUpdateValues { id: id.clone(), question_id: question_id.clone(), value: value.clone() }
        }
    }
}

fn procedural3d_operation_from_dsl(operation: Procedural3dOperationDsl) -> Result<Procedural3dOperation, store::TextError> {
    Ok(match operation {
        Procedural3dOperationDsl::SetWidget { index, widget } => Procedural3dOperation::SetWidget { index, widget: widget_from_dsl(*widget)? },
        Procedural3dOperationDsl::RemoveWidget { id } => Procedural3dOperation::RemoveWidget { id },
        Procedural3dOperationDsl::SetSynapse { index, synapse } => Procedural3dOperation::SetSynapse { index, synapse: synapse_from_dsl(synapse) },
        Procedural3dOperationDsl::RemoveSynapse { id } => Procedural3dOperation::RemoveSynapse { id },
        Procedural3dOperationDsl::SetLayout { id, layout } => Procedural3dOperation::SetLayout { id, layout: layout_from_dsl(layout) },
        Procedural3dOperationDsl::RemoveLayout { id } => Procedural3dOperation::RemoveLayout { id },
        Procedural3dOperationDsl::SetCamera { camera } => Procedural3dOperation::SetCamera { camera: camera_from_dsl(camera) },
        Procedural3dOperationDsl::SetSchema { schema } => Procedural3dOperation::SetSchema { schema },
        Procedural3dOperationDsl::GenerationAdd { generation } => Procedural3dOperation::Generation(GenerationOperation::Add { generation: form_generation_from_dsl(generation) }),
        Procedural3dOperationDsl::GenerationRemove { id } => Procedural3dOperation::Generation(GenerationOperation::Remove { id }),
        Procedural3dOperationDsl::GenerationRename { id, name } => Procedural3dOperation::Generation(GenerationOperation::Rename { id, name }),
        Procedural3dOperationDsl::GenerationUpdateValues { id, question_id, value } => Procedural3dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value }),
    })
}

/// ⚡ `Procedural3dOperation`'s compact single-line op encoding — derive-engine grammar via
/// `Procedural3dOperationDsl` (see above); `parse_op`/`print_op` convert at the boundary.
impl protocol::OpText for Procedural3dOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural3dOperationDsl as protocol::OpText>::parse_op(line)?;
        procedural3d_operation_from_dsl(parsed)
    }

    fn print_op(&self) -> String {
        <Procedural3dOperationDsl as protocol::OpText>::print_op(&procedural3d_operation_to_dsl(self))
    }
}

/// ⚡ Binary mirror of the `OpText` bridge above — `Procedural3dOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for Procedural3dOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        procedural3d_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = Procedural3dOperationDsl::decode_op(bytes)?;
        procedural3d_operation_from_dsl(parsed).map_err(|error| protocol::ProtocolError::Malformed { what: "procedural3d operation", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖OpText

pub type Procedural3dEnvelope = DocumentEnvelope<Procedural3dDocument, Procedural3dOperation>;
pub type Procedural3dStore = DocumentStore<Procedural3dDocument, Procedural3dOperation>;

pub fn empty_procedural3d_projection() -> Procedural3dDocument {
    Procedural3dDocument::default()
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Procedural3dDocumentVcs {
        store: RefCell<Procedural3dStore>,
    }

    #[wasm_bindgen]
    impl Procedural3dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Procedural3dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Procedural3dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Procedural3dStore::new(envelope)
                }
                None => Procedural3dStore::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
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
    use vcs::apply_operation;
use store::{create_document_envelope, test_support, DocumentDsl, DocumentCommand};
    use protocol::OpText;

    fn round_trip(projection: &Procedural3dDocument, operation: &Procedural3dOperation) -> Procedural3dDocument {
        let forward = apply_operation(projection, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(projection) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn store_applies_widget_add() {
        let mut store = Procedural3dStore::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Procedural3dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
        assert!(store.projection().expect("projection").fixture.widgets.iter().any(|w| widget_id(w) == "note-9"));
    }

    #[test]
    fn set_widget_round_trips() {
        let before = empty_procedural3d_projection();
        let after = round_trip(&before, &Procedural3dOperation::SetWidget { index: 9, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } });
        assert!(after.fixture.widgets.iter().any(|w| widget_id(w) == "note-9"));
    }

    #[test]
    fn generation_op_round_trips() {
        let before = empty_procedural3d_projection();
        let generation = playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        let after = round_trip(&before, &Procedural3dOperation::Generation(GenerationOperation::Add { generation }));
        assert_eq!(after.generation.generations.len(), 1);
    }

    #[test]
    fn fixture_ops_ignore_camera() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.camera = CameraJson { x: 7.0, y: 8.0, zoom: 2.0 };
        let operations = procedural3d_fixture_operations(&before, &after);
        assert!(operations.iter().all(|operation| !matches!(operation, Procedural3dOperation::SetCamera { .. })));
    }

    #[test]
    fn procedural3d_fixture_operations_detects_widget_synapse_layout_schema_changes() {
        let mut before = FlowFixture::default();
        before.schema = "old-schema".into();
        before.widgets = vec![
            Widget::InputNote { id: "w-gone".into(), text: String::new() },
            Widget::InputNote { id: "w-keep".into(), text: "old".into() },
        ];
        before.synapses = vec![
            SynapseSpec { id: "s-gone".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() },
            SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "old".into() },
        ];
        before.layout.insert("l-gone".into(), WidgetLayout { x: 0.0, y: 0.0 });
        before.layout.insert("l-keep".into(), WidgetLayout { x: 1.0, y: 1.0 });

        let mut after = FlowFixture::default();
        after.schema = "new-schema".into();
        after.widgets = vec![
            Widget::InputNote { id: "w-keep".into(), text: "new".into() },
            Widget::InputNote { id: "w-new".into(), text: String::new() },
        ];
        after.synapses = vec![
            SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "new".into() },
            SynapseSpec { id: "s-new".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() },
        ];
        after.layout.insert("l-keep".into(), WidgetLayout { x: 2.0, y: 2.0 });
        after.layout.insert("l-new".into(), WidgetLayout { x: 3.0, y: 3.0 });

        let operations = procedural3d_fixture_operations(&before, &after);
        assert!(operations.contains(&Procedural3dOperation::RemoveWidget { id: "w-gone".into() }));
        assert!(operations.contains(&Procedural3dOperation::SetWidget { index: 0, widget: Widget::InputNote { id: "w-keep".into(), text: "new".into() } }));
        assert!(operations.contains(&Procedural3dOperation::SetWidget { index: 1, widget: Widget::InputNote { id: "w-new".into(), text: String::new() } }));
        assert!(operations.contains(&Procedural3dOperation::RemoveSynapse { id: "s-gone".into() }));
        assert!(operations
            .contains(&Procedural3dOperation::SetSynapse { index: 0, synapse: SynapseSpec { id: "s-keep".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "new".into() } }));
        assert!(operations
            .contains(&Procedural3dOperation::SetSynapse { index: 1, synapse: SynapseSpec { id: "s-new".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() } }));
        assert!(operations.contains(&Procedural3dOperation::RemoveLayout { id: "l-gone".into() }));
        assert!(operations.contains(&Procedural3dOperation::SetLayout { id: "l-keep".into(), layout: WidgetLayout { x: 2.0, y: 2.0 } }));
        assert!(operations.contains(&Procedural3dOperation::SetLayout { id: "l-new".into(), layout: WidgetLayout { x: 3.0, y: 3.0 } }));
        assert!(operations.contains(&Procedural3dOperation::SetSchema { schema: "new-schema".into() }));
    }

    //#region 🔖WidgetIdTests
    #[test]
    fn widget_id_covers_all_widget_kinds() {
        let widgets: Vec<Widget> = vec![
            Widget::Neuron { id: "neuron-1".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: true },
            Widget::InputSlider { id: "slider-1".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 },
            Widget::InputNote { id: "note-1".into(), text: String::new() },
            Widget::InputImage { id: "image-1".into(), src: String::new() },
            Widget::Variable { id: "variable-1".into(), name: "x".into(), schema: "number".into() },
            Widget::OutputPreview { id: "preview-1".into(), preview: Default::default(), expanded: Default::default() },
            Widget::OutputAction { id: "action-1".into(), action: "run".into() },
            Widget::OutputExport { id: "export-1".into(), format: "gltf".into() },
            Widget::Cluster { id: "cluster-1".into(), name: "c".into(), tree: Default::default(), flow: Default::default() },
        ];
        for widget in &widgets {
            assert_eq!(widget_id(widget), &widget_id(widget).to_string());
        }
        let ids: Vec<&str> = widgets.iter().map(widget_id).collect();
        assert_eq!(
            ids,
            vec!["neuron-1", "slider-1", "note-1", "image-1", "variable-1", "preview-1", "action-1", "export-1", "cluster-1"]
        );
    }
    //#endregion 🔖WidgetIdTests

    //#region 🔖CollectionDiffTests
    #[test]
    fn set_widget_round_trip_replaces_existing_widget_by_id() {
        let mut before = empty_procedural3d_projection();
        // 🩹 Pre-existing bug fix (unrelated to the dsl:: engine conversion): `empty_procedural3d_projection`
        // returns `FlowFixture::default()`'s own demo widgets/synapses, not an empty fixture — this test
        // needs a clean slate to assert an exact post-replace length, matching the `.clear()` pattern
        // `fixture_ops_widget_id_matches_every_widget_kind` already uses for the same reason.
        before.fixture.widgets.clear();
        before.fixture.widgets.push(Widget::InputNote { id: "note-9".into(), text: "old".into() });
        let after = round_trip(&before, &Procedural3dOperation::SetWidget { index: 0, widget: Widget::InputNote { id: "note-9".into(), text: "new".into() } });
        assert_eq!(after.fixture.widgets.len(), 1);
        assert_eq!(after.fixture.widgets[0], Widget::InputNote { id: "note-9".into(), text: "new".into() });
    }

    #[test]
    fn backwards_remove_widget_when_missing_returns_empty() {
        let projection = empty_procedural3d_projection();
        assert!(Procedural3dOperation::RemoveWidget { id: "ghost".into() }.backwards(&projection).is_empty());
    }

    #[test]
    fn set_synapse_round_trip_replaces_existing_synapse_by_id() {
        let mut before = empty_procedural3d_projection();
        // 🩹 Pre-existing bug fix (unrelated to the dsl:: engine conversion): see the sibling widget
        // test above for why a clean slate is needed before asserting an exact post-replace length.
        before.fixture.synapses.clear();
        before.fixture.synapses.push(SynapseSpec { id: "e1".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() });
        let after = round_trip(
            &before,
            &Procedural3dOperation::SetSynapse { index: 0, synapse: SynapseSpec { id: "e1".into(), from: "a".into(), to: "c".into(), from_port: "out".into(), to_port: "in".into() } },
        );
        assert_eq!(after.fixture.synapses.len(), 1);
        assert_eq!(after.fixture.synapses[0].to, "c");
    }

    #[test]
    fn backwards_remove_synapse_when_missing_returns_empty() {
        let projection = empty_procedural3d_projection();
        assert!(Procedural3dOperation::RemoveSynapse { id: "ghost".into() }.backwards(&projection).is_empty());
    }

    #[test]
    fn set_layout_round_trip_inserts_when_absent() {
        let before = empty_procedural3d_projection();
        let after = round_trip(&before, &Procedural3dOperation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } });
        assert_eq!(after.fixture.layout.get("extrude"), Some(&WidgetLayout { x: 1.0, y: 2.0 }));
    }

    #[test]
    fn set_layout_round_trip_replaces_when_present() {
        let mut before = empty_procedural3d_projection();
        before.fixture.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
        let after = round_trip(&before, &Procedural3dOperation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 5.0, y: 6.0 } });
        assert_eq!(after.fixture.layout.get("extrude"), Some(&WidgetLayout { x: 5.0, y: 6.0 }));
    }

    #[test]
    fn remove_layout_backwards_present_restores_set_layout_missing_returns_empty() {
        let mut projection = empty_procedural3d_projection();
        projection.fixture.layout.insert("extrude".into(), WidgetLayout { x: 1.0, y: 2.0 });
        assert_eq!(
            Procedural3dOperation::RemoveLayout { id: "extrude".into() }.backwards(&projection),
            vec![Procedural3dOperation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 1.0, y: 2.0 } }]
        );
        assert!(Procedural3dOperation::RemoveLayout { id: "ghost".into() }.backwards(&projection).is_empty());
    }

    #[test]
    fn set_camera_round_trip_updates_camera() {
        let before = empty_procedural3d_projection();
        let after = round_trip(&before, &Procedural3dOperation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        assert_eq!(after.fixture.camera, CameraJson { x: 1.0, y: 2.0, zoom: 3.0 });
    }

    #[test]
    fn set_schema_round_trip_updates_schema() {
        let before = empty_procedural3d_projection();
        let after = round_trip(&before, &Procedural3dOperation::SetSchema { schema: "flow.fixture.v2".into() });
        assert_eq!(after.fixture.schema, "flow.fixture.v2");
    }

    #[test]
    fn diff_absorb_merges_collections_and_prefers_incoming_scalars() {
        let mut first = Procedural3dDiff::default();
        first.widgets.removed.push("w-a".into());
        first.widgets.set.push((0, Widget::InputNote { id: "w-b".into(), text: String::new() }));
        first.synapses.removed.push("s-a".into());
        first.layout.removed.push("l-a".into());
        first.camera = Some(CameraJson { x: 1.0, y: 1.0, zoom: 1.0 });
        first.schema = Some("schema-1".into());
        first.generation.push(GenerationOperation::Rename { id: "generation-1".into(), name: "First".into() });

        let mut second = Procedural3dDiff::default();
        second.widgets.removed.push("w-c".into());
        second.synapses.set.push((0, SynapseSpec { id: "s-b".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() }));
        second.layout.set.push(("l-b".into(), WidgetLayout { x: 2.0, y: 2.0 }));
        second.camera = Some(CameraJson { x: 9.0, y: 9.0, zoom: 9.0 });
        second.schema = None;
        second.generation.push(GenerationOperation::Rename { id: "generation-1".into(), name: "Second".into() });

        first.absorb(second);

        assert_eq!(first.widgets.removed, vec!["w-a".to_string(), "w-c".to_string()]);
        assert_eq!(first.widgets.set.len(), 1);
        assert_eq!(first.synapses.removed, vec!["s-a".to_string()]);
        assert_eq!(first.synapses.set.len(), 1);
        assert_eq!(first.layout.removed, vec!["l-a".to_string()]);
        assert_eq!(first.layout.set.len(), 1);
        assert_eq!(first.camera, Some(CameraJson { x: 9.0, y: 9.0, zoom: 9.0 }));
        assert_eq!(first.schema, Some("schema-1".to_string()));
        assert_eq!(first.generation.len(), 2);
    }
    //#endregion 🔖CollectionDiffTests

    //#region 🔖DslTests
    #[test]
    fn dsl_round_trip_empty_projection() {
        test_support::assert_dsl_round_trip(&empty_procedural3d_projection());
        test_support::assert_dsl_pack_equivalence(&empty_procedural3d_projection());
    }

    #[test]
    fn dsl_round_trip_hexagonal_mushroom_column_fixture() {
        let text = include_str!("../example/hexagonal-mushroom-column.procedural3d");
        let projection = Procedural3dDocument::parse_dsl(text).expect("parse hexagonal-mushroom-column.procedural3d fixture");
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_round_trip_rectangle_extrude_volume_fixture() {
        let text = include_str!("../example/rectangle-extrude-volume.procedural3d");
        let projection = Procedural3dDocument::parse_dsl(text).expect("parse rectangle-extrude-volume.procedural3d fixture");
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_round_trip_sphere_cut_with_torus_fixture() {
        let text = include_str!("../example/sphere-cut-with-torus.procedural3d");
        let projection = Procedural3dDocument::parse_dsl(text).expect("parse sphere-cut-with-torus.procedural3d fixture");
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_round_trip_with_generation_state() {
        let mut projection = empty_procedural3d_projection();
        let mut values = serde_json::Map::new();
        // 🌱 A float literal, not `json!(3)` (an integer-backed `serde_json::Number`): the DSL
        // engine's `Shape::Value`/`DslValue::Number` is a single `f64` variant (see `dsl/rs/lib.rs`'s
        // own documented int-vs-float caveat), so a value round tripping through generation `values`
        // always comes back float-backed — this is the known, accepted engine limitation, not a bug
        // in this crate's mirror/conversion code.
        values.insert("count".into(), serde_json::json!(3.0));
        projection.generation.generations.push(playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values });
        projection.generation.selected_generation_id = Some("generation-1".into());
        projection.generation.preview_text = Some("42".into());
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_round_trip_covers_remaining_widget_kinds() {
        let mut projection = empty_procedural3d_projection();
        projection.fixture.widgets = vec![
            Widget::InputNote { id: "note-1".into(), text: "hello \"world\"".into() },
            Widget::InputImage { id: "image-1".into(), src: "https://example.test/a.png".into() },
            Widget::Variable { id: "variable-1".into(), name: "height".into(), schema: "number".into() },
            Widget::OutputAction { id: "action-1".into(), action: "export".into() },
            Widget::OutputExport { id: "export-1".into(), format: "gltf".into() },
            Widget::Cluster { id: "cluster-1".into(), name: "Cluster".into(), tree: Default::default(), flow: Default::default() },
        ];
        test_support::assert_dsl_round_trip(&projection);
        test_support::assert_dsl_pack_equivalence(&projection);
    }
    //#endregion 🔖DslTests

    //#region 🔖ParseErrorTests
    /// 🏷️ An unrecognized widget kind keyword is simply left unconsumed by `Shape::Statements`
    /// (the engine breaks its variant-matching loop rather than erroring — see `dsl_schema::parse`,
    /// out of this crate's ownership scope), so parsing ultimately fails at the enclosing `widgets
    /// { }` block's closing brace instead of with a dedicated "unknown widget kind" message.
    #[test]
    fn parse_dsl_rejects_unknown_widget_kind() {
        let text = "schema=\"flow.fixture\"\ncamera { x=0 y=0 zoom=1 }\nwidgets { bogus id=\"w-1\" }\nsynapses= [ ]\nlayout= { }\ngenerations= [ ]\n";
        let error = Procedural3dDocument::parse_dsl(text).expect_err("unknown widget kind must fail to parse");
        assert!(error.to_string().contains("expected RBrace"), "unexpected error: {error}");
    }

    #[test]
    fn parse_op_rejects_unknown_operation() {
        let error = Procedural3dOperation::parse_op("bogus-op id=\"w-1\"").expect_err("unknown operation must fail to parse");
        assert!(error.to_string().contains("unknown operation"), "unexpected error: {error}");
    }
    //#endregion 🔖ParseErrorTests

    //#region 🔖OpTextTests
    #[test]
    fn op_text_round_trip_set_widget() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetWidget { index: 2, widget: Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() } });
    }

    #[test]
    fn op_text_round_trip_remove_widget() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::RemoveWidget { id: "note-9".into() });
    }

    #[test]
    fn op_text_round_trip_set_synapse() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetSynapse {
            index: 1,
            synapse: SynapseSpec { id: "e1".into(), from: "height".into(), to: "extrude".into(), from_port: "number".into(), to_port: String::new() },
        });
    }

    #[test]
    fn op_text_round_trip_remove_synapse() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::RemoveSynapse { id: "e1".into() });
    }

    #[test]
    fn op_text_round_trip_set_layout() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 12.5, y: -8.25 } });
    }

    #[test]
    fn op_text_round_trip_remove_layout() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::RemoveLayout { id: "extrude".into() });
    }

    #[test]
    fn op_text_round_trip_set_camera() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetCamera { camera: CameraJson { x: 1.5, y: -2.5, zoom: 1.2 } });
    }

    #[test]
    fn op_text_round_trip_set_schema() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetSchema { schema: "flow.fixture".into() });
    }

    #[test]
    fn op_text_round_trip_generation() {
        let generation = playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        test_support::assert_op_line_round_trip(&Procedural3dOperation::Generation(GenerationOperation::Add { generation }));
    }
    //#endregion 🔖OpTextTests

    //#region 🔖DocumentTextTests
    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = Procedural3dStore::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Procedural3dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }],
                description: None,
            })
            .expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖DocumentTextTests
}
//#endregion 🧪Tests
