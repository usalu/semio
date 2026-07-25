//! 📏 Procedural 2d document model on `vcs`.

use flow_core::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use protocol::{apply_generation_operation, invert_generation_operation, GenerationOperation, GenerationPlayState};
use serde::{Deserialize, Serialize};
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

#[cfg(test)]
mod tests {
    use super::*;
    use vcs::apply_operation;

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
}
