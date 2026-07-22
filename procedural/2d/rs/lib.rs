//! 📏 Procedural 2d document model on `vcs`.

use flow_core::{CameraJson, FlowFixture, SynapseSpec, Widget, WidgetLayout};
use protocol::{apply_generation_op, invert_generation_op, GenerationOp, GenerationPlayState};
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

//#region 🔖Ops
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
    pub generation: Vec<GenerationOp>,
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
        for op in &self.generation {
            apply_generation_op(&mut next.generation, op);
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
/// camera and fixture schema, and a single {@link GenerationOp} generation edit with its true inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum Procedural2dOp {
    SetWidget { index: usize, widget: Widget },
    RemoveWidget { id: String },
    SetSynapse { index: usize, synapse: SynapseSpec },
    RemoveSynapse { id: String },
    SetLayout { id: String, layout: WidgetLayout },
    RemoveLayout { id: String },
    SetCamera { camera: CameraJson },
    SetSchema { schema: String },
    Generation(GenerationOp),
}

fn widget_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.widgets.iter().position(|widget| widget_id(widget) == id)
}

fn synapse_index(fixture: &FlowFixture, id: &str) -> Option<usize> {
    fixture.synapses.iter().position(|synapse| synapse.id == id)
}

impl Operation<Procedural2dDocument> for Procedural2dOp {
    type Diff = Procedural2dDiff;

    fn diff(&self, _projection: &Procedural2dDocument) -> Procedural2dDiff {
        let mut diff = Procedural2dDiff::default();
        match self {
            Procedural2dOp::SetWidget { index, widget } => diff.widgets.set.push((*index, widget.clone())),
            Procedural2dOp::RemoveWidget { id } => diff.widgets.removed.push(id.clone()),
            Procedural2dOp::SetSynapse { index, synapse } => diff.synapses.set.push((*index, synapse.clone())),
            Procedural2dOp::RemoveSynapse { id } => diff.synapses.removed.push(id.clone()),
            Procedural2dOp::SetLayout { id, layout } => diff.layout.set.push((id.clone(), layout.clone())),
            Procedural2dOp::RemoveLayout { id } => diff.layout.removed.push(id.clone()),
            Procedural2dOp::SetCamera { camera } => diff.camera = Some(camera.clone()),
            Procedural2dOp::SetSchema { schema } => diff.schema = Some(schema.clone()),
            Procedural2dOp::Generation(op) => diff.generation.push(op.clone()),
        }
        diff
    }

    fn backwards(&self, projection: &Procedural2dDocument) -> Vec<Self> {
        let fixture = &projection.fixture;
        match self {
            Procedural2dOp::SetWidget { widget, .. } => match widget_index(fixture, widget_id(widget)) {
                Some(index) => vec![Procedural2dOp::SetWidget { index, widget: fixture.widgets[index].clone() }],
                None => vec![Procedural2dOp::RemoveWidget { id: widget_id(widget).to_string() }],
            },
            Procedural2dOp::RemoveWidget { id } => widget_index(fixture, id).map(|index| vec![Procedural2dOp::SetWidget { index, widget: fixture.widgets[index].clone() }]).unwrap_or_default(),
            Procedural2dOp::SetSynapse { synapse, .. } => match synapse_index(fixture, &synapse.id) {
                Some(index) => vec![Procedural2dOp::SetSynapse { index, synapse: fixture.synapses[index].clone() }],
                None => vec![Procedural2dOp::RemoveSynapse { id: synapse.id.clone() }],
            },
            Procedural2dOp::RemoveSynapse { id } => synapse_index(fixture, id).map(|index| vec![Procedural2dOp::SetSynapse { index, synapse: fixture.synapses[index].clone() }]).unwrap_or_default(),
            Procedural2dOp::SetLayout { id, .. } => match fixture.layout.get(id) {
                Some(layout) => vec![Procedural2dOp::SetLayout { id: id.clone(), layout: layout.clone() }],
                None => vec![Procedural2dOp::RemoveLayout { id: id.clone() }],
            },
            Procedural2dOp::RemoveLayout { id } => fixture.layout.get(id).map(|layout| vec![Procedural2dOp::SetLayout { id: id.clone(), layout: layout.clone() }]).unwrap_or_default(),
            Procedural2dOp::SetCamera { .. } => vec![Procedural2dOp::SetCamera { camera: fixture.camera.clone() }],
            Procedural2dOp::SetSchema { .. } => vec![Procedural2dOp::SetSchema { schema: fixture.schema.clone() }],
            Procedural2dOp::Generation(op) => invert_generation_op(&projection.generation, op).into_iter().map(Procedural2dOp::Generation).collect(),
        }
    }
}

/// 🔀 Diffs two fixtures into a minimal, invertible, mergeable op set: removed/added/patched widgets
/// and synapses (keyed by id), layout entries, and the scalar canvas camera and schema. Lets action
/// handlers keep computing the target fixture via `FlowHost` while emitting granular ops.
pub fn procedural2d_fixture_ops(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural2dOp> {
    let mut ops = Vec::new();
    for widget in &before.widgets {
        if !after.widgets.iter().any(|entry| widget_id(entry) == widget_id(widget)) {
            ops.push(Procedural2dOp::RemoveWidget { id: widget_id(widget).to_string() });
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let prior = before.widgets.iter().find(|entry| widget_id(entry) == widget_id(widget));
        if prior != Some(widget) {
            ops.push(Procedural2dOp::SetWidget { index, widget: widget.clone() });
        }
    }
    for synapse in &before.synapses {
        if !after.synapses.iter().any(|entry| entry.id == synapse.id) {
            ops.push(Procedural2dOp::RemoveSynapse { id: synapse.id.clone() });
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        let prior = before.synapses.iter().find(|entry| entry.id == synapse.id);
        if prior != Some(synapse) {
            ops.push(Procedural2dOp::SetSynapse { index, synapse: synapse.clone() });
        }
    }
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            ops.push(Procedural2dOp::RemoveLayout { id: id.clone() });
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            ops.push(Procedural2dOp::SetLayout { id: id.clone(), layout: layout.clone() });
        }
    }
    if before.camera != after.camera {
        ops.push(Procedural2dOp::SetCamera { camera: after.camera.clone() });
    }
    if before.schema != after.schema {
        ops.push(Procedural2dOp::SetSchema { schema: after.schema.clone() });
    }
    ops
}
//#endregion 🔖Ops

pub type Procedural2dEnvelope = DocumentVcsEnvelope<Procedural2dDocument, Procedural2dOp>;
pub type Procedural2dStore = DocumentVcsStore<Procedural2dDocument, Procedural2dOp>;

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
    use vcs::{apply_operation, create_document_vcs_envelope, DocumentVcsCommand};

    fn round_trip(projection: &Procedural2dDocument, op: &Procedural2dOp) -> Procedural2dDocument {
        let forward = apply_operation(projection, op);
        let mut restored = forward.clone();
        for back in op.backwards(projection) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, projection, "backwards() must restore the pre-op document");
        forward
    }

    #[test]
    fn store_applies_camera_op() {
        let mut store = Procedural2dStore::new(create_document_vcs_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", empty_procedural2d_projection(), None));
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Procedural2dOp::SetCamera { camera: CameraJson { x: 7.0, y: 8.0, zoom: 2.0 } }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").fixture.camera.zoom, 2.0);
    }

    #[test]
    fn remove_and_readd_widget_round_trips() {
        let base = empty_procedural2d_projection();
        let removed_id = widget_id(&base.fixture.widgets[0]).to_string();
        let after = round_trip(&base, &Procedural2dOp::RemoveWidget { id: removed_id.clone() });
        assert!(!after.fixture.widgets.iter().any(|w| widget_id(w) == removed_id));
    }

    #[test]
    fn fixture_ops_capture_widget_add() {
        let before = FlowFixture::default();
        let mut after = before.clone();
        after.widgets.push(Widget::InputNote { id: "note-1".into(), text: String::new() });
        let ops = procedural2d_fixture_ops(&before, &after);
        assert!(ops.iter().any(|op| matches!(op, Procedural2dOp::SetWidget { widget, .. } if widget_id(widget) == "note-1")));
    }

    #[test]
    fn generation_op_round_trips() {
        let before = empty_procedural2d_projection();
        let generation = protocol::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        let after = round_trip(&before, &Procedural2dOp::Generation(GenerationOp::Add { generation }));
        assert_eq!(after.generation.generations.len(), 1);
    }
}
