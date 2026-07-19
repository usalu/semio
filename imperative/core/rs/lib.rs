//! ⚙️ Imperative core: path host and WASM session.

pub use imperative_engine::{compile_to_text, imperative_catalogue_json, imperative_module_registry, EffectLogEntry, Executor, Path, RunResult, Step};
pub use imperative_module_core::{catalogue_json, module_registry, register};
pub use neural_engine::{Dictionary, Registry};

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// #region 🔖Document
/// 📍 Address of a nested step list inside a control step body.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
}

/// 📄 Imperative path document envelope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeDocument {
    pub schema: String,
    pub path: Path,
    #[serde(default)]
    pub seed: Dictionary,
}

impl Default for ImperativeDocument {
    fn default() -> Self {
        Self { schema: "imperative.document".into(), path: Path::new(), seed: Dictionary::new() }
    }
}

//#region 🔖Operation
/// @emoji ✂️ A step-collection edit at a `PathRef` — root path or a nested `control.*` step's slot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeOp {
    pub path_ref: PathRef,
    pub collection: vcs::CollectionOp<String, Step, Dictionary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ImperativeDiff(pub Option<ImperativeOp>);

impl vcs::OperationDiff<ImperativeDocument> for ImperativeDiff {
    fn apply(&self, projection: &ImperativeDocument) -> ImperativeDocument {
        let mut next = projection.clone();
        if let Some(op) = &self.0 {
            if let Some(steps) = resolve_steps_mut(&mut next, &op.path_ref) {
                vcs::apply_collection_op(steps, &op.collection);
            }
            prune_empty_slot(&mut next, &op.path_ref);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.0.is_some() {
            self.0 = other.0;
        }
    }
}

impl vcs::Operation<ImperativeDocument> for ImperativeOp {
    type Diff = ImperativeDiff;

    fn diff(&self, _projection: &ImperativeDocument) -> Self::Diff {
        ImperativeDiff(Some(self.clone()))
    }

    fn backwards(&self, projection: &ImperativeDocument) -> Vec<Self> {
        match resolve_steps(projection, &self.path_ref) {
            Some(steps) => vec![ImperativeOp { path_ref: self.path_ref.clone(), collection: vcs::invert_collection_op(steps, &self.collection) }],
            None => Vec::new(),
        }
    }
}

/// 🔎 Resolves the step list a `PathRef` addresses; a not-yet-materialized nested slot reads as empty.
fn resolve_steps<'a>(document: &'a ImperativeDocument, path_ref: &PathRef) -> Option<&'a [Step]> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&document.path.steps);
    }
    let owner = path_ref.owner.as_ref()?;
    let slot = path_ref.slot.as_ref()?;
    let owner_step = document.path.steps.iter().find(|step| &step.id == owner)?;
    Some(owner_step.bodies.get(slot).map(|path| path.steps.as_slice()).unwrap_or(&[]))
}

fn resolve_steps_mut<'a>(document: &'a mut ImperativeDocument, path_ref: &PathRef) -> Option<&'a mut Vec<Step>> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&mut document.path.steps);
    }
    let owner = path_ref.owner.clone()?;
    let slot = path_ref.slot.clone()?;
    let owner_step = document.path.steps.iter_mut().find(|step| step.id == owner)?;
    Some(&mut owner_step.bodies.entry(slot).or_insert_with(Path::new).steps)
}

/// 🧹 Drops a nested slot's `bodies` entry once it's empty, so an emptied slot is bit-identical to
/// a never-touched one — required for `Add` then `Remove` to be a true, exact inverse pair.
fn prune_empty_slot(document: &mut ImperativeDocument, path_ref: &PathRef) {
    let (Some(owner), Some(slot)) = (&path_ref.owner, &path_ref.slot) else {
        return;
    };
    if let Some(owner_step) = document.path.steps.iter_mut().find(|step| &step.id == owner) {
        if owner_step.bodies.get(slot).is_some_and(|path| path.steps.is_empty()) {
            owner_step.bodies.remove(slot);
        }
    }
}
//#endregion 🔖Operation

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

pub fn default_document() -> ImperativeDocument {
    ImperativeDocument {
        path: Path {
            steps: vec![
                Step {
                    id: "step-1".into(),
                    kind: "state.set".into(),
                    params: Dictionary::new().insert("key", neural_engine::Value::Atom(neural_engine::Atom::String("counter".into()))).insert("value", neural_engine::Value::Atom(neural_engine::Atom::Decimal(0.0))),
                    bodies: BTreeMap::new(),
                },
                Step { id: "step-2".into(), kind: "log.print".into(), params: Dictionary::new().insert("message", neural_engine::Value::Atom(neural_engine::Atom::String("hello imperative".into()))), bodies: BTreeMap::new() },
            ],
        },
        ..Default::default()
    }
}
// #endregion 🔖Document

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
        let step = Step { id: id.clone(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() };
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

    fn step(id: &str, kind: &str) -> Step {
        Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
    }

    #[test]
    fn add_step_op_round_trips() {
        let document = default_document();
        let op = ImperativeOp { path_ref: PathRef::default(), collection: vcs::CollectionOp::Add { index: 0, item: step("step-x", "log.print") } };
        vcs::test_support::assert_operation_round_trip(&document, op.clone());
        vcs::test_support::assert_store_roundtrip(document, op);
    }

    #[test]
    fn remove_step_op_round_trips() {
        let document = default_document();
        let op = ImperativeOp { path_ref: PathRef::default(), collection: vcs::CollectionOp::Remove { id: "step-1".into() } };
        vcs::test_support::assert_operation_round_trip(&document, op.clone());
        vcs::test_support::assert_store_roundtrip(document, op);
    }

    #[test]
    fn move_step_op_round_trips() {
        let document = default_document();
        let op = ImperativeOp { path_ref: PathRef::default(), collection: vcs::CollectionOp::Move { id: "step-1".into(), to_index: 1 } };
        vcs::test_support::assert_operation_round_trip(&document, op.clone());
        vcs::test_support::assert_store_roundtrip(document, op);
    }

    #[test]
    fn patch_step_params_op_round_trips() {
        let document = default_document();
        let op = ImperativeOp { path_ref: PathRef::default(), collection: vcs::CollectionOp::Patch { id: "step-1".into(), patch: Dictionary::new().insert("key", neural_engine::Value::Atom(neural_engine::Atom::String("renamed".into()))) } };
        vcs::test_support::assert_operation_round_trip(&document, op.clone());
        vcs::test_support::assert_store_roundtrip(document, op);
    }

    #[test]
    fn add_step_into_nested_control_body_round_trips() {
        let mut document = default_document();
        document.path.steps.push(step("step-if", "control.if"));
        let path_ref = PathRef { owner: Some("step-if".into()), slot: Some("then".into()) };
        let op = ImperativeOp { path_ref: path_ref.clone(), collection: vcs::CollectionOp::Add { index: 0, item: step("step-nested", "log.print") } };
        vcs::test_support::assert_operation_round_trip(&document, op.clone());
        let post = vcs::apply_operation(&document, &op);
        let owner_step = post.path.steps.iter().find(|entry| entry.id == "step-if").expect("owner step");
        assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
        vcs::test_support::assert_store_roundtrip(document, op);
    }
}
// [DEBUG] compile-speed verification touch 1784460360
