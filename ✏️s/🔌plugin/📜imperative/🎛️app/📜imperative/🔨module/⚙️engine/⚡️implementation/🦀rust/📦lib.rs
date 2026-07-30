//! ⚙️ Imperative app — headless compute (constitutional: engine).

use imperative::{Dictionary, ImperativeDocument, Path, PathRef, Registry, Step};
use imperative_engine::{compile_to_text, imperative_catalogue_json, imperative_module_registry, Executor, RunResult};

//#region ⚠️ Errors
/// 🚨 Imperative core's fallible operations.
#[derive(Debug, thiserror::Error)]
pub enum ImperativeCoreError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("unsupported schema: {0}")]
    UnsupportedSchema(String),
    #[error("missing owner")]
    MissingOwner,
    #[error("missing slot")]
    MissingSlot,
    #[error("unknown owner step: {0}")]
    UnknownOwnerStep(String),
    #[error("unknown step: {0}")]
    UnknownStep(String),
}
//#endregion ⚠️ Errors

/// 📄 The default `imperative` document, handcrafted in the `.imperative` DSL (see `imperative_dsl`)
/// instead of a hand-built Rust literal or a JSON fixture — {@link default_document} is the only way it
/// should be consumed.
pub fn default_document() -> ImperativeDocument {
    <ImperativeDocument as store::DocumentDsl>::parse_dsl(imperative_dsl::IMPERATIVE_EXAMPLE_TEXT)
        .expect("📜default.imperative is a static, hand-authored fixture that must always parse")
}

// #region 🔖Host
/// 🎛️ Native imperative path host.
pub struct ImperativeHost {
    pub document: ImperativeDocument,
    registry: Registry,
    next_serial: u64,
}

impl Default for ImperativeHost {
    fn default() -> Self {
        Self::from_document(default_document())
    }
}

impl ImperativeHost {
    pub fn from_document(document: ImperativeDocument) -> Self {
        Self { document, registry: imperative_module_registry(), next_serial: 100 }
    }

    pub fn load_json(json: &str) -> Result<Self, ImperativeCoreError> {
        let document: ImperativeDocument = serde_json::from_str(json)?;
        if document.schema != "imperative.document" {
            return Err(ImperativeCoreError::UnsupportedSchema(document.schema));
        }
        Ok(Self::from_document(document))
    }

    pub fn to_json(&self) -> Result<String, ImperativeCoreError> {
        Ok(serde_json::to_string(&self.document)?)
    }

    pub fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    fn resolve_path_mut<'a>(&'a mut self, path_ref: &PathRef) -> Result<&'a mut Path, ImperativeCoreError> {
        if path_ref.owner.is_none() && path_ref.slot.is_none() {
            return Ok(&mut self.document.path);
        }
        let owner = path_ref.owner.as_ref().ok_or(ImperativeCoreError::MissingOwner)?;
        let slot = path_ref.slot.as_ref().ok_or(ImperativeCoreError::MissingSlot)?;
        let owner_step = self.document.path.steps.iter_mut().find(|step| step.id == *owner).ok_or_else(|| ImperativeCoreError::UnknownOwnerStep(owner.clone()))?;
        Ok(owner_step.bodies.entry(slot.clone()).or_insert_with(Path::new))
    }

    pub fn add_step(&mut self, kind: &str, index: Option<usize>) -> String {
        self.add_step_at(&PathRef::default(), kind, index).expect("root PathRef always resolves — resolve_path_mut only fails for a non-default owner/slot")
    }

    pub fn add_step_at(&mut self, path_ref: &PathRef, kind: &str, index: Option<usize>) -> Result<String, ImperativeCoreError> {
        self.next_serial += 1;
        let id = format!("step-{}", self.next_serial);
        let step = Step { id: id.clone(), kind: kind.into(), params: Dictionary::new(), bodies: std::collections::BTreeMap::new() };
        let path = self.resolve_path_mut(path_ref)?;
        let insert_at = index.unwrap_or(path.steps.len()).min(path.steps.len());
        path.steps.insert(insert_at, step);
        Ok(id)
    }

    pub fn remove_step(&mut self, id: &str) -> bool {
        self.remove_step_at(&PathRef::default(), id)
    }

    pub fn remove_step_at(&mut self, path_ref: &PathRef, id: &str) -> bool {
        let path = match self.resolve_path_mut(path_ref) {
            Ok(path) => path,
            Err(_) => return false,
        };
        let before = path.steps.len();
        path.steps.retain(|step| step.id != id);
        path.steps.len() != before
    }

    pub fn move_step(&mut self, id: &str, new_index: usize) -> bool {
        self.move_step_at(&PathRef::default(), id, new_index)
    }

    pub fn move_step_at(&mut self, path_ref: &PathRef, id: &str, new_index: usize) -> bool {
        let path = match self.resolve_path_mut(path_ref) {
            Ok(path) => path,
            Err(_) => return false,
        };
        let Some(current) = path.steps.iter().position(|step| step.id == id) else {
            return false;
        };
        let step = path.steps.remove(current);
        let insert_at = new_index.min(path.steps.len());
        path.steps.insert(insert_at, step);
        true
    }

    pub fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        self.set_step_params_at(&PathRef::default(), id, json)
    }

    pub fn set_step_params_at(&mut self, path_ref: &PathRef, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        let params: Dictionary = serde_json::from_str(json)?;
        let path = self.resolve_path_mut(path_ref)?;
        let Some(step) = path.steps.iter_mut().find(|step| step.id == id) else {
            return Err(ImperativeCoreError::UnknownStep(id.into()));
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
            Self { state: Rc::new(RefCell::new(ImperativeSessionInner { host: ImperativeHost::default() })) }
        }

        #[wasm_bindgen(js_name = loadPathJson)]
        pub fn load_path_json(&self, json: &str) -> Result<(), JsValue> {
            let host = ImperativeHost::load_json(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = pathJson)]
        pub fn path_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.to_json().map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = catalogueJson)]
        pub fn catalogue_json(&self) -> String {
            self.state.borrow().host.catalogue_json()
        }

        #[wasm_bindgen(js_name = addStep)]
        pub fn add_step(&self, kind: &str, index: Option<usize>) -> String {
            self.state.borrow_mut().host.add_step(kind, index)
        }

        #[wasm_bindgen(js_name = addStepAt)]
        pub fn add_step_at(&self, path_ref_json: &str, kind: &str, index: Option<usize>) -> Result<String, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.add_step_at(&path_ref, kind, index).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = removeStep)]
        pub fn remove_step(&self, id: &str) -> bool {
            self.state.borrow_mut().host.remove_step(id)
        }

        #[wasm_bindgen(js_name = removeStepAt)]
        pub fn remove_step_at(&self, path_ref_json: &str, id: &str) -> Result<bool, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            Ok(self.state.borrow_mut().host.remove_step_at(&path_ref, id))
        }

        #[wasm_bindgen(js_name = moveStep)]
        pub fn move_step(&self, id: &str, new_index: usize) -> bool {
            self.state.borrow_mut().host.move_step(id, new_index)
        }

        #[wasm_bindgen(js_name = moveStepAt)]
        pub fn move_step_at(&self, path_ref_json: &str, id: &str, new_index: usize) -> Result<bool, JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            Ok(self.state.borrow_mut().host.move_step_at(&path_ref, id, new_index))
        }

        #[wasm_bindgen(js_name = setStepParamsJson)]
        pub fn set_step_params_json(&self, id: &str, json: &str) -> Result<(), JsValue> {
            self.state.borrow_mut().host.set_step_params_json(id, json).map_err(|err| JsValue::from_str(&err.to_string()))
        }

        #[wasm_bindgen(js_name = setStepParamsAt)]
        pub fn set_step_params_at(&self, path_ref_json: &str, id: &str, json: &str) -> Result<(), JsValue> {
            let path_ref: PathRef = serde_json::from_str(path_ref_json).map_err(|err| JsValue::from_str(&err.to_string()))?;
            self.state.borrow_mut().host.set_step_params_at(&path_ref, id, json).map_err(|err| JsValue::from_str(&err.to_string()))
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

//#region 🧪Tests
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

    #[test]
    fn host_adds_nested_step_in_control_body() {
        let mut host = ImperativeHost::default();
        let owner = host.add_step("control.if", None);
        let path_ref = PathRef { owner: Some(owner.clone()), slot: Some("then".into()) };
        let nested = host.add_step_at(&path_ref, "log.print", None).expect("add nested");
        assert_eq!(nested, "step-102");
        let owner_step = host.document.path.steps.iter().find(|step| step.id == owner).expect("owner");
        assert_eq!(owner_step.bodies.get("then").map(|path| path.steps.len()), Some(1));
    }

    //#region ImperativeCoreError
    #[test]
    fn imperative_core_error_messages() {
        assert_eq!(ImperativeCoreError::MissingOwner.to_string(), "missing owner");
        assert_eq!(ImperativeCoreError::MissingSlot.to_string(), "missing slot");
        assert_eq!(ImperativeCoreError::UnsupportedSchema("bad.schema".into()).to_string(), "unsupported schema: bad.schema");
        assert_eq!(ImperativeCoreError::UnknownOwnerStep("step-9".into()).to_string(), "unknown owner step: step-9");
        assert_eq!(ImperativeCoreError::UnknownStep("step-9".into()).to_string(), "unknown step: step-9");
    }
    //#endregion ImperativeCoreError

    //#region ImperativeHost
    #[test]
    fn host_load_json_rejects_unsupported_schema() {
        let json = r#"{"schema":"not.imperative","path":{"steps":[]},"seed":{}}"#;
        assert!(matches!(ImperativeHost::load_json(json), Err(ImperativeCoreError::UnsupportedSchema(schema)) if schema == "not.imperative"));
    }

    #[test]
    fn host_load_json_rejects_invalid_json() {
        assert!(matches!(ImperativeHost::load_json("not json"), Err(ImperativeCoreError::Json(_))));
    }

    #[test]
    fn host_load_json_and_to_json_round_trip() {
        let json = ImperativeHost::default().to_json().expect("serializes");
        let host = ImperativeHost::load_json(&json).expect("parses back");
        assert_eq!(host.to_json().expect("serializes again"), json);
    }

    #[test]
    fn host_catalogue_json_is_nonempty() {
        assert!(!ImperativeHost::default().catalogue_json().is_empty());
    }

    #[test]
    fn host_add_step_at_reports_missing_owner_and_slot() {
        let mut host = ImperativeHost::default();
        let missing_owner = PathRef { owner: None, slot: Some("then".into()) };
        assert!(matches!(host.add_step_at(&missing_owner, "log.print", None), Err(ImperativeCoreError::MissingOwner)));
        let missing_slot = PathRef { owner: Some("step-1".into()), slot: None };
        assert!(matches!(host.add_step_at(&missing_slot, "log.print", None), Err(ImperativeCoreError::MissingSlot)));
    }

    #[test]
    fn host_add_step_at_reports_unknown_owner_step() {
        let mut host = ImperativeHost::default();
        let path_ref = PathRef { owner: Some("does-not-exist".into()), slot: Some("then".into()) };
        assert!(matches!(host.add_step_at(&path_ref, "log.print", None), Err(ImperativeCoreError::UnknownOwnerStep(owner)) if owner == "does-not-exist"));
    }

    #[test]
    fn host_add_step_clamps_out_of_range_index() {
        let mut host = ImperativeHost::default();
        let before = host.document.path.steps.len();
        let id = host.add_step("log.print", Some(9999));
        assert_eq!(host.document.path.steps.last().map(|step| &step.id), Some(&id));
        assert_eq!(host.document.path.steps.len(), before + 1);
    }

    #[test]
    fn host_remove_step_false_for_unresolvable_path_ref_and_unknown_id() {
        let mut host = ImperativeHost::default();
        let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(!host.remove_step_at(&bad_path_ref, "step-1"));
        assert!(!host.remove_step("does-not-exist"));
    }

    #[test]
    fn host_remove_step_true_when_removed() {
        let mut host = ImperativeHost::default();
        assert!(host.remove_step("step-1"));
        assert!(host.document.path.steps.iter().all(|step| step.id != "step-1"));
    }

    #[test]
    fn host_move_step_false_for_unresolvable_path_ref_and_unknown_id() {
        let mut host = ImperativeHost::default();
        let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(!host.move_step_at(&bad_path_ref, "step-1", 0));
        assert!(!host.move_step("does-not-exist", 0));
    }

    #[test]
    fn host_move_step_true_and_reorders() {
        let mut host = ImperativeHost::default();
        assert!(host.move_step("step-2", 0));
        assert_eq!(host.document.path.steps[0].id, "step-2");
    }

    #[test]
    fn host_set_step_params_at_rejects_invalid_json_and_unknown_step() {
        let mut host = ImperativeHost::default();
        assert!(matches!(host.set_step_params_json("step-1", "not json"), Err(ImperativeCoreError::Json(_))));
        assert!(matches!(host.set_step_params_json("does-not-exist", "{}"), Err(ImperativeCoreError::UnknownStep(id)) if id == "does-not-exist"));
        let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(matches!(host.set_step_params_at(&bad_path_ref, "step-1", "{}"), Err(ImperativeCoreError::UnknownOwnerStep(_))));
    }

    #[test]
    fn host_set_step_params_updates_existing_step() {
        use neural_engine::{Atom, Value};
        let mut host = ImperativeHost::default();
        host.set_step_params_json("step-2", r#"{"message":"updated"}"#).expect("sets params");
        let step = host.document.path.steps.iter().find(|step| step.id == "step-2").expect("step-2 exists");
        assert_eq!(step.params.get("message"), Some(&Value::Atom(Atom::String("updated".into()))));
    }

    #[test]
    fn host_compile_text_contains_step_kinds() {
        let host = ImperativeHost::default();
        let compiled = host.compile_text();
        assert!(compiled.contains("state.set"));
        assert!(compiled.contains("log.print"));
    }
    //#endregion ImperativeHost
}
//#endregion 🧪Tests
