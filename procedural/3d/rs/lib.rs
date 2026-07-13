//! 📐 Procedural 3d document model on `vcs`.

use flow_core::{CameraJson, FlowFixture};
use semio_framework_plugin::{apply_generation_op, invert_generation_op, GenerationOp, GenerationPlayState};
use serde::{Deserialize, Serialize};
use vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};

pub const PROCEDURAL_3D_SCHEMA: &str = "procedural.3d";

//#region 🔖Document
/// 🧾 Persistent procedural-3d document — the flow fixture plus the generation vocabulary state.
/// Ephemeral view state (selection, sun, LOD, preview caches) lives in the plugin app struct.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dDocument {
    pub fixture: FlowFixture,
    #[serde(default)]
    pub generation: GenerationPlayState,
}

impl Default for Procedural3dDocument {
    fn default() -> Self {
        Self { fixture: FlowFixture::default(), generation: GenerationPlayState::default() }
    }
}
//#endregion 🔖Document

//#region 🔖Ops
/// 🩹 Sparse procedural-3d diff — a whole-fixture replacement, a coalescible canvas camera, and an
/// ordered list of generation edits applied in sequence.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dDiff {
    pub fixture: Option<FlowFixture>,
    pub camera: Option<CameraJson>,
    #[serde(default)]
    pub generation: Vec<GenerationOp>,
}

impl OperationDiff<Procedural3dDocument> for Procedural3dDiff {
    fn apply(&self, projection: &Procedural3dDocument) -> Procedural3dDocument {
        let mut next = projection.clone();
        if let Some(fixture) = &self.fixture {
            next.fixture = fixture.clone();
        }
        if let Some(camera) = &self.camera {
            next.fixture.camera = camera.clone();
        }
        for op in &self.generation {
            apply_generation_op(&mut next.generation, op);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.fixture.is_some() {
            self.fixture = other.fixture;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        self.generation.extend(other.generation);
    }
}

/// 🧮 Procedural-3d operation: a whole-fixture replacement (structural graph edits), a coalesced
/// canvas camera move, or a single {@link GenerationOp} generation edit with its true inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum Procedural3dOp {
    SetFixture { fixture: FlowFixture },
    SetCamera { camera: CameraJson },
    Generation(GenerationOp),
}

impl Operation<Procedural3dDocument> for Procedural3dOp {
    type Diff = Procedural3dDiff;

    fn diff(&self, _projection: &Procedural3dDocument) -> Procedural3dDiff {
        match self {
            Procedural3dOp::SetFixture { fixture } => Procedural3dDiff { fixture: Some(fixture.clone()), ..Default::default() },
            Procedural3dOp::SetCamera { camera } => Procedural3dDiff { camera: Some(camera.clone()), ..Default::default() },
            Procedural3dOp::Generation(op) => Procedural3dDiff { generation: vec![op.clone()], ..Default::default() },
        }
    }

    fn backwards(&self, projection: &Procedural3dDocument) -> Vec<Self> {
        match self {
            Procedural3dOp::SetFixture { .. } => vec![Procedural3dOp::SetFixture { fixture: projection.fixture.clone() }],
            Procedural3dOp::SetCamera { .. } => vec![Procedural3dOp::SetCamera { camera: projection.fixture.camera.clone() }],
            Procedural3dOp::Generation(op) => invert_generation_op(&projection.generation, op)
                .into_iter()
                .map(Procedural3dOp::Generation)
                .collect(),
        }
    }
}

/// 🔀 Diffs two fixtures into the minimal op set: a whole-fixture replacement when structure
/// (widgets/synapses/layout/schema) changed, else a coalescible camera op when only the canvas
/// camera moved. Lets action handlers keep computing the target fixture via `FlowHost` while
/// emitting a granular, invertible op.
pub fn procedural3d_fixture_ops(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural3dOp> {
    let structure_changed = before.widgets != after.widgets
        || before.synapses != after.synapses
        || before.layout != after.layout
        || before.schema != after.schema;
    if structure_changed {
        vec![Procedural3dOp::SetFixture { fixture: after.clone() }]
    } else if before.camera != after.camera {
        vec![Procedural3dOp::SetCamera { camera: after.camera.clone() }]
    } else {
        Vec::new()
    }
}
//#endregion 🔖Ops

pub type Procedural3dEnvelope = DocumentVcsEnvelope<Procedural3dDocument, Procedural3dOp>;
pub type Procedural3dStore = DocumentVcsStore<Procedural3dDocument, Procedural3dOp>;

pub fn empty_procedural3d_projection() -> Procedural3dDocument {
    Procedural3dDocument::default()
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
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
                    let envelope: Procedural3dEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Procedural3dStore::new(envelope)
                }
                None => Procedural3dStore::new(create_document_vcs_envelope(
                    PROCEDURAL_3D_SCHEMA,
                    "procedural3d",
                    empty_procedural3d_projection(),
                    None,
                )),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_json(command_json)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .projection_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .envelope_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
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

    #[test]
    fn procedural3d_document_vcs_replays_fixture_op() {
        let mut store = Procedural3dStore::new(create_document_vcs_envelope(
            PROCEDURAL_3D_SCHEMA,
            "procedural3d",
            empty_procedural3d_projection(),
            None,
        ));
        let mut fixture = FlowFixture::default();
        fixture.widgets.clear();
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Procedural3dOp::SetFixture { fixture }],
                description: None,
            })
            .expect("apply");
        assert!(store.projection().expect("projection").fixture.widgets.is_empty());
    }

    #[test]
    fn set_fixture_op_round_trips() {
        let before = empty_procedural3d_projection();
        let mut fixture = FlowFixture::default();
        fixture.camera = CameraJson { x: 3.0, y: 4.0, zoom: 5.0 };
        let op = Procedural3dOp::SetFixture { fixture };
        let forward = apply_operation(&before, &op);
        assert_eq!(forward.fixture.camera.zoom, 5.0);
        let mut restored = forward.clone();
        for back in op.backwards(&before) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(restored, before);
    }

    #[test]
    fn generation_op_round_trips() {
        let before = empty_procedural3d_projection();
        let generation = semio_framework_plugin::FormGeneration {
            id: "generation-1".into(),
            name: "Generation 1".into(),
            values: serde_json::Map::new(),
        };
        let op = Procedural3dOp::Generation(GenerationOp::Add { generation });
        let forward = apply_operation(&before, &op);
        assert_eq!(forward.generation.generations.len(), 1);
        let mut restored = forward.clone();
        for back in op.backwards(&before) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(restored, before);
    }
}
