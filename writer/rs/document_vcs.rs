// #region 🔖DocumentVcs
use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use vcs::{
    create_document_vcs_envelope, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterProjection {
    pub schema: String,
    pub id: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum WriterOp {
    SetText { text: String },
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterDiff {
    pub text: Option<String>,
}

impl OperationDiff<WriterProjection> for WriterDiff {
    fn apply(&self, projection: &WriterProjection) -> WriterProjection {
        WriterProjection {
            text: self.text.clone().unwrap_or_else(|| projection.text.clone()),
            ..projection.clone()
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.text.is_some() {
            self.text = other.text;
        }
    }
}

impl Operation<WriterProjection> for WriterOp {
    type Diff = WriterDiff;

    fn diff(&self, _projection: &WriterProjection) -> WriterDiff {
        match self {
            WriterOp::SetText { text } => WriterDiff { text: Some(text.clone()) },
        }
    }

    fn backwards(&self, projection: &WriterProjection) -> Vec<Self> {
        vec![WriterOp::SetText {
            text: projection.text.clone(),
        }]
    }
}

pub type WriterEnvelope = DocumentVcsEnvelope<WriterProjection, WriterOp>;
pub type WriterStore = DocumentVcsStore<WriterProjection, WriterOp>;

pub fn empty_writer_projection() -> WriterProjection {
    WriterProjection {
        schema: "writer.document".into(),
        id: "writer".into(),
        text: String::new(),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WriterDocumentVcs {
    store: RefCell<WriterStore>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WriterDocumentVcs {
    #[wasm_bindgen(constructor)]
    pub fn new(envelope_json: &str) -> Result<WriterDocumentVcs, JsValue> {
        let envelope: WriterEnvelope =
            serde_json::from_str(envelope_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(Self {
            store: RefCell::new(WriterStore::new(envelope)),
        })
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

#[cfg(test)]
mod writer_vcs_tests {
    use super::*;
    use vcs::DocumentVcsCommand;

    #[test]
    fn writer_document_vcs_replays_text_ops() {
        let mut store = WriterStore::new(create_document_vcs_envelope(
            "writer.document",
            "writer",
            empty_writer_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![WriterOp::SetText { text: "hello".into() }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").text, "hello");
    }
}
// #endregion 🔖DocumentVcs
