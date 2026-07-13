//! 📏 Procedural 2d document model on `vcs`.

use flow_core::{CameraJson, FlowFixture};
use semio_framework_plugin::{apply_generation_op, invert_generation_op, GenerationOp, GenerationPlayState};
use serde::{Deserialize, Serialize};
use vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};

pub const PROCEDURAL_2D_SCHEMA: &str = "procedural.2d";

//#region 🔖Document
/// 🧾 Persistent procedural-2d document — the flow fixture plus the generation vocabulary state.
/// Ephemeral view state (selection, show mode, preview evaluations) lives in the plugin app struct.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDocument {
    pub fixture: FlowFixture,
    #[serde(default)]
    pub generation: GenerationPlayState,
}

impl Default for Procedural2dDocument {
    fn default() -> Self {
        Self { fixture: FlowFixture::default(), generation: GenerationPlayState::default() }
    }
}
//#endregion 🔖Document

//#region 🔖Ops
/// 🩹 Sparse procedural-2d diff — a whole-fixture replacement, a coalescible canvas camera, and an
/// ordered list of generation edits applied in sequence.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDiff {
    pub fixture: Option<FlowFixture>,
    pub camera: Option<CameraJson>,
    #[serde(default)]
    pub generation: Vec<GenerationOp>,
}

impl OperationDiff<Procedural2dDocument> for Procedural2dDiff {
    fn apply(&self, projection: &Procedural2dDocument) -> Procedural2dDocument {
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

/// 🧮 Procedural-2d operation: a whole-fixture replacement (structural graph edits), a coalesced
/// canvas camera move, or a single {@link GenerationOp} generation edit with its true inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum Procedural2dOp {
    SetFixture { fixture: FlowFixture },
    SetCamera { camera: CameraJson },
    Generation(GenerationOp),
}

impl Operation<Procedural2dDocument> for Procedural2dOp {
    type Diff = Procedural2dDiff;

    fn diff(&self, _projection: &Procedural2dDocument) -> Procedural2dDiff {
        match self {
            Procedural2dOp::SetFixture { fixture } => Procedural2dDiff { fixture: Some(fixture.clone()), ..Default::default() },
            Procedural2dOp::SetCamera { camera } => Procedural2dDiff { camera: Some(camera.clone()), ..Default::default() },
            Procedural2dOp::Generation(op) => Procedural2dDiff { generation: vec![op.clone()], ..Default::default() },
        }
    }

    fn backwards(&self, projection: &Procedural2dDocument) -> Vec<Self> {
        match self {
            Procedural2dOp::SetFixture { .. } => vec![Procedural2dOp::SetFixture { fixture: projection.fixture.clone() }],
            Procedural2dOp::SetCamera { .. } => vec![Procedural2dOp::SetCamera { camera: projection.fixture.camera.clone() }],
            Procedural2dOp::Generation(op) => invert_generation_op(&projection.generation, op)
                .into_iter()
                .map(Procedural2dOp::Generation)
                .collect(),
        }
    }
}

/// 🔀 Diffs two fixtures into the minimal op set: a whole-fixture replacement when structure
/// (widgets/synapses/layout/schema) changed, else a coalescible camera op when only the canvas
/// camera moved. Lets action handlers keep computing the target fixture via `FlowHost` while
/// emitting a granular, invertible op.
pub fn procedural2d_fixture_ops(before: &FlowFixture, after: &FlowFixture) -> Vec<Procedural2dOp> {
    let structure_changed = before.widgets != after.widgets
        || before.synapses != after.synapses
        || before.layout != after.layout
        || before.schema != after.schema;
    if structure_changed {
        vec![Procedural2dOp::SetFixture { fixture: after.clone() }]
    } else if before.camera != after.camera {
        vec![Procedural2dOp::SetCamera { camera: after.camera.clone() }]
    } else {
        Vec::new()
    }
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
                    let envelope: Procedural2dEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Procedural2dStore::new(envelope)
                }
                None => Procedural2dStore::new(create_document_vcs_envelope(
                    PROCEDURAL_2D_SCHEMA,
                    "procedural2d",
                    empty_procedural2d_projection(),
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

    fn sample_fixture() -> FlowFixture {
        FlowFixture::default()
    }

    #[test]
    fn procedural2d_document_vcs_replays_fixture_op() {
        let mut store = Procedural2dStore::new(create_document_vcs_envelope(
            PROCEDURAL_2D_SCHEMA,
            "procedural2d",
            empty_procedural2d_projection(),
            None,
        ));
        let mut fixture = sample_fixture();
        fixture.camera = CameraJson { x: 7.0, y: 8.0, zoom: 2.0 };
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Procedural2dOp::SetCamera { camera: fixture.camera.clone() }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").fixture.camera.zoom, 2.0);
    }

    #[test]
    fn set_fixture_op_round_trips() {
        let before = empty_procedural2d_projection();
        let mut fixture = sample_fixture();
        fixture.widgets.clear();
        let op = Procedural2dOp::SetFixture { fixture };
        let forward = apply_operation(&before, &op);
        assert!(forward.fixture.widgets.is_empty());
        let mut restored = forward.clone();
        for back in op.backwards(&before) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(restored, before);
    }

    #[test]
    fn generation_op_round_trips() {
        let before = empty_procedural2d_projection();
        let generation = semio_framework_plugin::FormGeneration {
            id: "generation-1".into(),
            name: "Generation 1".into(),
            values: serde_json::Map::new(),
        };
        let op = Procedural2dOp::Generation(GenerationOp::Add { generation });
        let forward = apply_operation(&before, &op);
        assert_eq!(forward.generation.generations.len(), 1);
        let mut restored = forward.clone();
        for back in op.backwards(&before) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(restored, before);
    }

    #[test]
    fn fixture_ops_prefers_camera_only_op() {
        let before = sample_fixture();
        let mut after = before.clone();
        after.camera = CameraJson { x: 1.0, y: 2.0, zoom: 3.0 };
        let ops = procedural2d_fixture_ops(&before, &after);
        assert!(matches!(ops.as_slice(), [Procedural2dOp::SetCamera { .. }]));
    }
}
