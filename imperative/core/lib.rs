//! ⚙️ Imperative core: path host and WASM session.

pub use imperative_engine::{compile_to_text, EffectLogEntry, Executor, imperative_catalogue_json, imperative_module_registry, Path, RunResult, Step};
pub use imperative_module_core::{catalogue_json, module_registry, register};
pub use neural_engine::{Dictionary, Registry};

use serde::{Deserialize, Serialize};

// #region 🔖Document
/// 📄 Imperative path document envelope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeDocumentV1 {
    pub schema: String,
    pub path: Path,
    #[serde(default)]
    pub seed: Dictionary,
}

impl Default for ImperativeDocumentV1 {
    fn default() -> Self {
        Self {
            schema: "imperative.document/v1".into(),
            path: Path::new(),
            seed: Dictionary::new(),
        }
    }
}

pub fn default_document() -> ImperativeDocumentV1 {
    ImperativeDocumentV1 {
        path: Path {
            steps: vec![
                Step {
                    id: "step-1".into(),
                    kind: "state.set".into(),
                    params: Dictionary::new()
                        .insert("key", neural_engine::Value::Atom(neural_engine::Atom::String("counter".into())))
                        .insert("value", neural_engine::Value::Atom(neural_engine::Atom::Decimal(0.0))),
                },
                Step {
                    id: "step-2".into(),
                    kind: "log.print".into(),
                    params: Dictionary::new().insert(
                        "message",
                        neural_engine::Value::Atom(neural_engine::Atom::String("hello imperative".into())),
                    ),
                },
            ],
        },
        ..Default::default()
    }
}
// #endregion 🔖Document

// #region 🔖Host
/// 🎛️ Native imperative path host.
pub struct ImperativeHost {
    pub document: ImperativeDocumentV1,
    registry: Registry,
    next_serial: u64,
}

impl Default for ImperativeHost {
    fn default() -> Self {
        Self::from_document(default_document())
    }
}

impl ImperativeHost {
    pub fn from_document(document: ImperativeDocumentV1) -> Self {
        Self {
            document,
            registry: imperative_module_registry(),
            next_serial: 100,
        }
    }

    pub fn load_json(json: &str) -> Result<Self, String> {
        let document: ImperativeDocumentV1 = serde_json::from_str(json).map_err(|err| err.to_string())?;
        if document.schema != "imperative.document/v1" {
            return Err(format!("unsupported schema: {}", document.schema));
        }
        Ok(Self::from_document(document))
    }

    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.document).map_err(|err| err.to_string())
    }

    pub fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    pub fn add_step(&mut self, kind: &str, index: Option<usize>) -> String {
        self.next_serial += 1;
        let id = format!("step-{}", self.next_serial);
        let step = Step {
            id: id.clone(),
            kind: kind.into(),
            params: Dictionary::new(),
        };
        let insert_at = index.unwrap_or(self.document.path.steps.len());
        let insert_at = insert_at.min(self.document.path.steps.len());
        self.document.path.steps.insert(insert_at, step);
        id
    }

    pub fn remove_step(&mut self, id: &str) -> bool {
        let before = self.document.path.steps.len();
        self.document.path.steps.retain(|step| step.id != id);
        self.document.path.steps.len() != before
    }

    pub fn move_step(&mut self, id: &str, new_index: usize) -> bool {
        let Some(current) = self.document.path.steps.iter().position(|step| step.id == id) else {
            return false;
        };
        let mut step = self.document.path.steps.remove(current);
        let insert_at = new_index.min(self.document.path.steps.len());
        self.document.path.steps.insert(insert_at, step);
        true
    }

    pub fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), String> {
        let params: Dictionary = serde_json::from_str(json).map_err(|err| err.to_string())?;
        let Some(step) = self.document.path.steps.iter_mut().find(|step| step.id == id) else {
            return Err(format!("unknown step: {id}"));
        };
        step.params = params;
        Ok(())
    }

    pub fn run(&self) -> RunResult {
        Executor::new(&self.registry).run(&self.document.path, &self.document.seed)
    }

    pub fn compile_text(&self) -> String {
        compile_to_text(&self.document.path)
    }
}
// #endregion 🔖Host

// #region 🔖WasmSession
#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;

    struct ImperativeSessionInner {
        host: ImperativeHost,
    }

    #[wasm_bindgen]
    pub struct ImperativeSession {
        state: Rc<RefCell<ImperativeSessionInner>>,
    }

    #[wasm_bindgen]
    impl ImperativeSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self {
                state: Rc::new(RefCell::new(ImperativeSessionInner {
                    host: ImperativeHost::default(),
                })),
            }
        }

        #[wasm_bindgen(js_name = loadPathJson)]
        pub fn load_path_json(&self, json: &str) -> Result<(), JsValue> {
            let host = ImperativeHost::load_json(json).map_err(|err| JsValue::from_str(&err))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = pathJson)]
        pub fn path_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.to_json().map_err(|err| JsValue::from_str(&err))
        }

        #[wasm_bindgen(js_name = catalogueJson)]
        pub fn catalogue_json(&self) -> String {
            self.state.borrow().host.catalogue_json()
        }

        #[wasm_bindgen(js_name = addStep)]
        pub fn add_step(&self, kind: &str, index: Option<usize>) -> String {
            self.state.borrow_mut().host.add_step(kind, index)
        }

        #[wasm_bindgen(js_name = removeStep)]
        pub fn remove_step(&self, id: &str) -> bool {
            self.state.borrow_mut().host.remove_step(id)
        }

        #[wasm_bindgen(js_name = moveStep)]
        pub fn move_step(&self, id: &str, new_index: usize) -> bool {
            self.state.borrow_mut().host.move_step(id, new_index)
        }

        #[wasm_bindgen(js_name = setStepParamsJson)]
        pub fn set_step_params_json(&self, id: &str, json: &str) -> Result<(), JsValue> {
            self.state
                .borrow_mut()
                .host
                .set_step_params_json(id, json)
                .map_err(|err| JsValue::from_str(&err))
        }

        #[wasm_bindgen]
        pub fn run(&self) -> Result<String, JsValue> {
            let result = self.state.borrow().host.run();
            serde_json::to_string(&result).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = compileText)]
        pub fn compile_text(&self) -> String {
            self.state.borrow().host.compile_text()
        }
    }
}
// #endregion 🔖WasmSession

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_runs_default_document() {
        let host = ImperativeHost::default();
        let result = host.run();
        assert_eq!(result.effects.len(), 2);
        assert!(result.effects.iter().all(|entry| entry.error.is_none()));
    }
}
