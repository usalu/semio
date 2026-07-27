//! ⚙️ Imperative core: path host and WASM session.

pub use imperative_engine::{compile_to_text, imperative_catalogue_json, imperative_module_registry, EffectLogEntry, Executor, Path, RunResult, Step};
pub use imperative_module_core::{catalogue_json, module_registry, register};
pub use neural_engine::{Dictionary, Registry};

use neural_engine::{Atom, Value};
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
pub struct ImperativeOperation {
    pub path_ref: PathRef,
    pub collection: vcs::CollectionOperation<String, Step, Dictionary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ImperativeDiff(pub Option<ImperativeOperation>);

impl vcs::OperationDiff<ImperativeDocument> for ImperativeDiff {
    fn apply(&self, projection: &ImperativeDocument) -> ImperativeDocument {
        let mut next = projection.clone();
        if let Some(operation) = &self.0 {
            if let Some(steps) = resolve_steps_mut(&mut next, &operation.path_ref) {
                vcs::apply_collection_operation(steps, &operation.collection);
            }
            prune_empty_slot(&mut next, &operation.path_ref);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.0.is_some() {
            self.0 = other.0;
        }
    }
}

impl vcs::Operation<ImperativeDocument> for ImperativeOperation {
    type Diff = ImperativeDiff;

    fn diff(&self, _projection: &ImperativeDocument) -> Self::Diff {
        ImperativeDiff(Some(self.clone()))
    }

    fn backwards(&self, projection: &ImperativeDocument) -> Vec<Self> {
        match resolve_steps(projection, &self.path_ref) {
            Some(steps) => vec![ImperativeOperation { path_ref: self.path_ref.clone(), collection: vcs::invert_collection_operation(steps, &self.collection) }],
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

//#region 🔖Dsl
/// 🌱 `Value`/`Atom`/`Dictionary`/`Step`/`Path` are all defined in `neural_engine`/`imperative_engine`
/// (foreign crates out of scope for this conversion), so none of them can carry a
/// `#[derive(dsl::Dsl...)]` themselves — Rust's orphan rule requires the impl target type to live in
/// the crate that also owns the trait or the type, and neither is true for a foreign type receiving a
/// foreign derive. `ValueDsl`/`StepNodeDsl`/`PathDsl` below are local structural twins that the real
/// types convert to/from right at the `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary.
///
/// `ValueDsl` deliberately does NOT route through `dsl_schema`'s built-in `Shape::Value`/`DslValue`
/// dynamic-literal primitive: `DslValue::Number(f64)` merges `Atom::Integer`/`Atom::Decimal` into one
/// case (the OLD hand-rolled printer distinguished them by always giving a `Decimal` a trailing `.`),
/// which is a real, observable loss of fidelity — an existing test parses a bare `1` and expects
/// `Atom::Integer(1)` back. So `ValueDsl` is its own typed record instead: exactly one of its
/// mutually-exclusive `Option` fields is ever `Some`, each keyed so the ACTUAL Rust variant (not a
/// text heuristic) decides which one, which is exactly as precise as the old hand-rolled `Atom` match.
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
    dictionary: Option<BTreeMap<String, ValueDsl>>,
}

fn value_to_value_dsl(value: &Value) -> ValueDsl {
    let mut dsl_value = ValueDsl { null: None, boolean: None, integer: None, decimal: None, text: None, dictionary: None };
    match value {
        Value::Atom(Atom::Null) => dsl_value.null = Some(true),
        Value::Atom(Atom::Boolean(b)) => dsl_value.boolean = Some(*b),
        Value::Atom(Atom::Integer(i)) => dsl_value.integer = Some(*i),
        Value::Atom(Atom::Decimal(d)) => dsl_value.decimal = Some(*d),
        Value::Atom(Atom::String(s)) => dsl_value.text = Some(s.clone()),
        Value::Dictionary(dict) => dsl_value.dictionary = Some(dictionary_to_value_dsl_map(dict)),
    }
    dsl_value
}

fn value_dsl_to_value(dsl_value: &ValueDsl) -> Value {
    if dsl_value.null.is_some() {
        return Value::Atom(Atom::Null);
    }
    if let Some(b) = dsl_value.boolean {
        return Value::Atom(Atom::Boolean(b));
    }
    if let Some(i) = dsl_value.integer {
        return Value::Atom(Atom::Integer(i));
    }
    if let Some(d) = dsl_value.decimal {
        return Value::Atom(Atom::Decimal(d));
    }
    if let Some(s) = &dsl_value.text {
        return Value::Atom(Atom::String(s.clone()));
    }
    match &dsl_value.dictionary {
        Some(entries) => Value::Dictionary(value_dsl_map_to_dictionary(entries)),
        None => Value::Atom(Atom::Null),
    }
}

fn dictionary_to_value_dsl_map(dict: &Dictionary) -> BTreeMap<String, ValueDsl> {
    dict.keys().map(|key| (key.clone(), value_to_value_dsl(dict.get(key).expect("key came from dict.keys()")))).collect()
}

fn value_dsl_map_to_dictionary(entries: &BTreeMap<String, ValueDsl>) -> Dictionary {
    entries.iter().fold(Dictionary::new(), |dict, (key, value)| dict.insert(key.clone(), value_dsl_to_value(value)))
}

/// 📦 `None` when `dict` is empty, mirroring the old printer's "omit an empty dictionary section".
fn dictionary_to_option_dsl_map(dict: &Dictionary) -> Option<BTreeMap<String, ValueDsl>> {
    (!dict.is_empty()).then(|| dictionary_to_value_dsl_map(dict))
}

fn option_dsl_map_to_dictionary(entries: Option<BTreeMap<String, ValueDsl>>) -> Dictionary {
    entries.map(|entries| value_dsl_map_to_dictionary(&entries)).unwrap_or_default()
}

/// 👣 `Step`'s recursive `bodies: BTreeMap<String, Path>` mirrors through `Path`'s own single field —
/// `StepNodeDsl` is a one-variant `DslEnum` (not a plain `DslRecord`) purely so the mutual recursion
/// with `PathDsl` goes through `dsl::DslVariants`'s LAZY `fn() -> RecordSpec` pointers: building
/// `PathDsl`'s `RecordSpec` needs `StepNodeDsl`'s variant table, and vice versa, and only the lazy
/// pointer indirection keeps that finite instead of recursing forever just to construct the schema.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum StepNodeDsl {
    Step {
        #[dsl(positional)]
        id: String,
        kind: String,
        params: Option<BTreeMap<String, ValueDsl>>,
        bodies: BTreeMap<String, PathDsl>,
    },
}

#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct PathDsl {
    #[dsl(statements, block)]
    steps: Vec<StepNodeDsl>,
}

fn step_to_step_node_dsl(step: &Step) -> StepNodeDsl {
    StepNodeDsl::Step {
        id: step.id.clone(),
        kind: step.kind.clone(),
        params: dictionary_to_option_dsl_map(&step.params),
        bodies: step.bodies.iter().map(|(slot, path)| (slot.clone(), path_to_path_dsl(path))).collect(),
    }
}

fn step_node_dsl_to_step(node: StepNodeDsl) -> Step {
    let StepNodeDsl::Step { id, kind, params, bodies } = node;
    Step { id, kind, params: option_dsl_map_to_dictionary(params), bodies: bodies.into_iter().map(|(slot, path)| (slot, path_dsl_to_path(path))).collect() }
}

fn path_to_path_dsl(path: &Path) -> PathDsl {
    PathDsl { steps: path.steps.iter().map(step_to_step_node_dsl).collect() }
}

fn path_dsl_to_path(path_dsl: PathDsl) -> Path {
    Path { steps: path_dsl.steps.into_iter().map(step_node_dsl_to_step).collect() }
}

/// 📄 Local mirror of `ImperativeDocument` — see the region's opening doc comment for why `path:
/// Path`/`seed: Dictionary` can't stay as-is under a direct `#[derive(dsl::DslDocument)]`.
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "imperative")]
#[dsl(layout = "lines")]
struct ImperativeDocumentDsl {
    schema: String,
    seed: Option<BTreeMap<String, ValueDsl>>,
    #[dsl(statements, block)]
    steps: Vec<StepNodeDsl>,
}

impl vcs::DocumentDsl for ImperativeDocument {
    const EXTENSION: &'static str = "imperative";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        let parsed = <ImperativeDocumentDsl as vcs::DocumentDsl>::parse_dsl(text)?;
        Ok(ImperativeDocument {
            schema: parsed.schema,
            path: Path { steps: parsed.steps.into_iter().map(step_node_dsl_to_step).collect() },
            seed: option_dsl_map_to_dictionary(parsed.seed),
        })
    }

    fn print_dsl(&self) -> String {
        let mirror = ImperativeDocumentDsl {
            schema: self.schema.clone(),
            seed: dictionary_to_option_dsl_map(&self.seed),
            steps: self.path.steps.iter().map(step_to_step_node_dsl).collect(),
        };
        <ImperativeDocumentDsl as vcs::DocumentDsl>::print_dsl(&mirror)
    }
}

//#region 🔖Pack
/// 📦 Binary counterpart of the `vcs::DocumentDsl` impl above. `ImperativeDocument` carries a manual
/// `DocumentDsl` impl (not `#[derive(dsl::DslDocument)]` itself — see the region's opening doc comment)
/// so it did NOT automatically gain `vcs::DocumentPack` from `dsl_derive`'s expansion in wave 1. This
/// mirrors the derive-emitted shape (wave1-report.txt §2) exactly, substituting `ImperativeDocumentDsl`'s
/// `__dsl_spec`/`__dsl_to_record`/`__dsl_from_record` trio for `Self`'s (unavailable here) and routing
/// the same mirror-struct conversion `parse_dsl`/`print_dsl` already use.
impl vcs::DocumentPack for ImperativeDocument {
    fn encode_pack_with(&self, options: &vcs::PackEncodeOptions) -> Result<Vec<u8>, vcs::PackError> {
        let mirror = ImperativeDocumentDsl {
            schema: self.schema.clone(),
            seed: dictionary_to_option_dsl_map(&self.seed),
            steps: self.path.steps.iter().map(step_to_step_node_dsl).collect(),
        };
        vcs::pack_rt::encode_document(&ImperativeDocumentDsl::__dsl_spec(), &mirror.__dsl_to_record(), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &vcs::PackDecodeOptions) -> Result<Self, vcs::PackError> {
        let (record, _report) = vcs::pack_rt::decode_document(bytes, &ImperativeDocumentDsl::__dsl_spec(), options)?;
        let parsed = ImperativeDocumentDsl::__dsl_from_record(&record).map_err(vcs::text_error_to_pack_error)?;
        Ok(ImperativeDocument {
            schema: parsed.schema,
            path: Path { steps: parsed.steps.into_iter().map(step_node_dsl_to_step).collect() },
            seed: option_dsl_map_to_dictionary(parsed.seed),
        })
    }
}
//#endregion 🔖Pack
//#endregion 🔖Dsl

//#region 🔖OpText
/// ✂️ Local mirror of `ImperativeOperation` — flattens `PathRef` into bare `owner`/`slot`
/// `Option<String>` fields (printed bare when the value lexes as a bare ident, per the engine's
/// default `Shape::Text` behavior — no per-field opt-in needed) since a `vcs::Operation` grammar is
/// a genuinely tagged enum (`#[derive(dsl::DslOps)]` requires an enum), not the single generic-struct
/// shape `ImperativeOperation`/`vcs::CollectionOperation` use at the Rust level.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum ImperativeOperationDsl {
    Add {
        owner: Option<String>,
        slot: Option<String>,
        index: usize,
        #[dsl(statements)]
        item: Box<StepNodeDsl>,
    },
    Remove { owner: Option<String>, slot: Option<String>, id: String },
    Move {
        owner: Option<String>,
        slot: Option<String>,
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    Patch { owner: Option<String>, slot: Option<String>, id: String, patch: BTreeMap<String, ValueDsl> },
}

fn imperative_operation_to_dsl(operation: &ImperativeOperation) -> ImperativeOperationDsl {
    let owner = operation.path_ref.owner.clone();
    let slot = operation.path_ref.slot.clone();
    match &operation.collection {
        vcs::CollectionOperation::Add { index, item } => ImperativeOperationDsl::Add { owner, slot, index: *index, item: Box::new(step_to_step_node_dsl(item)) },
        vcs::CollectionOperation::Remove { id } => ImperativeOperationDsl::Remove { owner, slot, id: id.clone() },
        vcs::CollectionOperation::Move { id, to_index } => ImperativeOperationDsl::Move { owner, slot, id: id.clone(), to_index: *to_index },
        vcs::CollectionOperation::Patch { id, patch } => ImperativeOperationDsl::Patch { owner, slot, id: id.clone(), patch: dictionary_to_value_dsl_map(patch) },
    }
}

fn imperative_operation_from_dsl(dsl_op: ImperativeOperationDsl) -> ImperativeOperation {
    match dsl_op {
        ImperativeOperationDsl::Add { owner, slot, index, item } => {
            ImperativeOperation { path_ref: PathRef { owner, slot }, collection: vcs::CollectionOperation::Add { index, item: step_node_dsl_to_step(*item) } }
        }
        ImperativeOperationDsl::Remove { owner, slot, id } => ImperativeOperation { path_ref: PathRef { owner, slot }, collection: vcs::CollectionOperation::Remove { id } },
        ImperativeOperationDsl::Move { owner, slot, id, to_index } => {
            ImperativeOperation { path_ref: PathRef { owner, slot }, collection: vcs::CollectionOperation::Move { id, to_index } }
        }
        ImperativeOperationDsl::Patch { owner, slot, id, patch } => {
            ImperativeOperation { path_ref: PathRef { owner, slot }, collection: vcs::CollectionOperation::Patch { id, patch: value_dsl_map_to_dictionary(&patch) } }
        }
    }
}

impl vcs::OpText for ImperativeOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        Ok(imperative_operation_from_dsl(<ImperativeOperationDsl as vcs::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <ImperativeOperationDsl as vcs::OpText>::print_op(&imperative_operation_to_dsl(self))
    }
}
//#endregion 🔖OpText


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

/// 📄 The default `imperative` document, handcrafted in the `.imperative` DSL (see `🔖Dsl`) instead of
/// a hand-built Rust literal or a JSON fixture — {@link default_document} is the only way it should be
/// consumed.
const DEFAULT_IMPERATIVE_DOCUMENT_TEXT: &str = include_str!("../../example/default.imperative");

pub fn default_document() -> ImperativeDocument {
    <ImperativeDocument as vcs::DocumentDsl>::parse_dsl(DEFAULT_IMPERATIVE_DOCUMENT_TEXT)
        .expect("default.imperative is a static, hand-authored fixture that must always parse")
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

    fn step(id: &str, kind: &str) -> Step {
        Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
    }

    #[test]
    fn add_step_op_round_trips() {
        let document = default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Add { index: 0, item: step("step-x", "log.print") } };
        vcs::test_support::assert_operation_round_trip(&document, operation.clone());
        vcs::test_support::assert_op_line_round_trip(&operation);
        vcs::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn remove_step_op_round_trips() {
        let document = default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Remove { id: "step-1".into() } };
        vcs::test_support::assert_operation_round_trip(&document, operation.clone());
        vcs::test_support::assert_op_line_round_trip(&operation);
        vcs::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn move_step_op_round_trips() {
        let document = default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Move { id: "step-1".into(), to_index: 1 } };
        vcs::test_support::assert_operation_round_trip(&document, operation.clone());
        vcs::test_support::assert_op_line_round_trip(&operation);
        vcs::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn patch_step_params_op_round_trips() {
        let document = default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Patch { id: "step-1".into(), patch: Dictionary::new().insert("key", neural_engine::Value::Atom(neural_engine::Atom::String("renamed".into()))) } };
        vcs::test_support::assert_operation_round_trip(&document, operation.clone());
        vcs::test_support::assert_op_line_round_trip(&operation);
        vcs::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn add_step_into_nested_control_body_round_trips() {
        let mut document = default_document();
        document.path.steps.push(step("step-if", "control.if"));
        let path_ref = PathRef { owner: Some("step-if".into()), slot: Some("then".into()) };
        let operation = ImperativeOperation { path_ref: path_ref.clone(), collection: vcs::CollectionOperation::Add { index: 0, item: step("step-nested", "log.print") } };
        vcs::test_support::assert_operation_round_trip(&document, operation.clone());
        vcs::test_support::assert_op_line_round_trip(&operation);
        let post = vcs::apply_operation(&document, &operation);
        let owner_step = post.path.steps.iter().find(|entry| entry.id == "step-if").expect("owner step");
        assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
        vcs::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn default_document_dsl_round_trips() {
        vcs::test_support::assert_dsl_round_trip(&default_document());
        vcs::test_support::assert_dsl_pack_equivalence(&default_document());
    }

    #[test]
    fn document_text_round_trip_with_applied_operation() {
        let document = default_document();
        let envelope = vcs::create_document_vcs_envelope::<ImperativeDocument, ImperativeOperation>("imperative.document/v1", "test", document, None);
        let mut store = vcs::DocumentVcsStore::new(envelope);
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Add { index: 0, item: step("step-x", "log.print") } };
        store
            .dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![operation], description: None })
            .expect("apply");
        vcs::test_support::assert_document_text_round_trip(&store);
        vcs::test_support::assert_document_pack_round_trip(&store);
    }

    //#region resolve_steps / resolve_steps_mut / prune_empty_slot
    #[test]
    fn resolve_steps_root_returns_document_steps() {
        let document = default_document();
        let steps = resolve_steps(&document, &PathRef::default()).expect("root always resolves");
        assert_eq!(steps.len(), document.path.steps.len());
    }

    #[test]
    fn resolve_steps_unknown_owner_is_none() {
        let document = default_document();
        let path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(resolve_steps(&document, &path_ref).is_none());
    }

    #[test]
    fn resolve_steps_missing_owner_or_slot_is_none() {
        let document = default_document();
        assert!(resolve_steps(&document, &PathRef { owner: Some("step-1".into()), slot: None }).is_none());
        assert!(resolve_steps(&document, &PathRef { owner: None, slot: Some("then".into()) }).is_none());
    }

    #[test]
    fn resolve_steps_unmaterialized_slot_reads_empty() {
        let mut document = default_document();
        document.path.steps.push(step("step-if", "control.if"));
        let path_ref = PathRef { owner: Some("step-if".into()), slot: Some("then".into()) };
        assert_eq!(resolve_steps(&document, &path_ref), Some(&[][..]));
    }

    #[test]
    fn resolve_steps_mut_unknown_owner_is_none() {
        let mut document = default_document();
        let path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(resolve_steps_mut(&mut document, &path_ref).is_none());
    }

    #[test]
    fn prune_empty_slot_removes_emptied_bodies_entry() {
        let mut document = default_document();
        document.path.steps.push(step("step-if", "control.if"));
        let path_ref = PathRef { owner: Some("step-if".into()), slot: Some("then".into()) };
        resolve_steps_mut(&mut document, &path_ref).expect("materializes slot").push(step("step-nested", "log.print"));
        let owner_step = document.path.steps.iter().find(|s| s.id == "step-if").expect("owner");
        assert!(owner_step.bodies.contains_key("then"));
        resolve_steps_mut(&mut document, &path_ref).expect("slot exists").clear();
        prune_empty_slot(&mut document, &path_ref);
        let owner_step = document.path.steps.iter().find(|s| s.id == "step-if").expect("owner");
        assert!(!owner_step.bodies.contains_key("then"));
    }

    #[test]
    fn prune_empty_slot_noop_without_owner_or_slot() {
        let mut document = default_document();
        prune_empty_slot(&mut document, &PathRef::default());
    }

    #[test]
    fn operation_backwards_on_unresolvable_path_ref_is_empty() {
        let document = default_document();
        let operation = ImperativeOperation {
            path_ref: PathRef { owner: Some("missing".into()), slot: Some("then".into()) },
            collection: vcs::CollectionOperation::Remove { id: "step-x".into() },
        };
        assert!(vcs::Operation::backwards(&operation, &document).is_empty());
    }

    #[test]
    fn imperative_diff_absorb_keeps_latest_some_and_ignores_none() {
        use vcs::OperationDiff;
        let first = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Remove { id: "step-1".into() } };
        let second = ImperativeOperation { path_ref: PathRef::default(), collection: vcs::CollectionOperation::Remove { id: "step-2".into() } };
        let mut diff = ImperativeDiff(Some(first));
        diff.absorb(ImperativeDiff(None));
        assert!(matches!(&diff.0, Some(op) if matches!(&op.collection, vcs::CollectionOperation::Remove { id } if id == "step-1")));
        diff.absorb(ImperativeDiff(Some(second)));
        assert!(matches!(&diff.0, Some(op) if matches!(&op.collection, vcs::CollectionOperation::Remove { id } if id == "step-2")));
    }
    //#endregion resolve_steps / resolve_steps_mut / prune_empty_slot

    //#region DSL text round trips and error paths
    // These two used to hand-author literal OLD-grammar text (`body then { ... }`-style nested
    // blocks) and parse it directly — the new grammar's shape is different enough (BTreeMap-keyed
    // `bodies`/`params`, `ValueDsl`'s own keyed atom representation) that a hand-typed literal is
    // error-prone to get exactly right. Building the document via Rust struct literals and checking
    // it survives a DSL round trip verifies the same intent (seed dictionaries, nested control
    // bodies, every atom variant all parse/print correctly) without depending on hand-typed syntax.
    #[test]
    fn dsl_parses_seed_and_nested_control_bodies() {
        let mut document = default_document();
        document.seed = Dictionary::new().insert("counter", Value::Atom(Atom::Integer(1))).insert("label", Value::Atom(Atom::String("x".into())));
        let inner = step("step-inner", "log.print");
        let mut owner = step("step-if", "control.if");
        owner.bodies.insert("then".to_string(), Path { steps: vec![inner] });
        document.path.steps = vec![owner];

        assert_eq!(document.seed.get("counter"), Some(&Value::Atom(Atom::Integer(1))));
        let owner = &document.path.steps[0];
        assert_eq!(owner.bodies.get("then").map(|body| body.steps.len()), Some(1));
        vcs::test_support::assert_dsl_round_trip(&document);
        vcs::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[test]
    fn dsl_parses_dictionary_and_atom_variants() {
        let mut document = default_document();
        document.seed = Dictionary::new()
            .insert("a", Value::Atom(Atom::Null))
            .insert("b", Value::Atom(Atom::Boolean(true)))
            .insert("c", Value::Atom(Atom::Boolean(false)))
            .insert("d", Value::Atom(Atom::Decimal(1.5)))
            .insert("e", Value::Atom(Atom::Decimal(f64::NEG_INFINITY)))
            .insert("f", Value::Dictionary(Dictionary::new()));

        assert_eq!(document.seed.get("a"), Some(&Value::Atom(Atom::Null)));
        assert_eq!(document.seed.get("b"), Some(&Value::Atom(Atom::Boolean(true))));
        assert_eq!(document.seed.get("c"), Some(&Value::Atom(Atom::Boolean(false))));
        assert_eq!(document.seed.get("d"), Some(&Value::Atom(Atom::Decimal(1.5))));
        assert_eq!(document.seed.get("e"), Some(&Value::Atom(Atom::Decimal(f64::NEG_INFINITY))));
        assert_eq!(document.seed.get("f"), Some(&Value::Dictionary(Dictionary::new())));
        vcs::test_support::assert_dsl_round_trip(&document);
        vcs::test_support::assert_dsl_pack_equivalence(&document);
    }

    #[test]
    fn dsl_rejects_unterminated_string() {
        let text = r#"imperative schema="unterminated"#;
        assert!(<ImperativeDocument as vcs::DocumentDsl>::parse_dsl(text).is_err());
    }

    #[test]
    fn dsl_rejects_wrong_leading_keyword() {
        let text = r#"notimperative schema="x""#;
        assert!(<ImperativeDocument as vcs::DocumentDsl>::parse_dsl(text).is_err());
    }

    #[test]
    fn dsl_rejects_invalid_number_literal() {
        let text = r#"imperative schema="imperative.document" seed={ n=1.2.3 }"#;
        assert!(<ImperativeDocument as vcs::DocumentDsl>::parse_dsl(text).is_err());
    }

    #[test]
    fn op_text_rejects_unknown_operation_keyword() {
        let line = r#"frobnicate owner=- slot=- id="step-1""#;
        assert!(<ImperativeOperation as vcs::OpText>::parse_op(line).is_err());
    }

    #[test]
    fn op_text_round_trips_add_with_owner_and_slot() {
        let operation = ImperativeOperation {
            path_ref: PathRef { owner: Some("step-if".into()), slot: Some("then".into()) },
            collection: vcs::CollectionOperation::Add { index: 0, item: step("step-nested", "log.print") },
        };
        let printed = <ImperativeOperation as vcs::OpText>::print_op(&operation);
        assert!(printed.contains("owner=step-if"), "printed: {printed}");
        assert!(printed.contains("slot=then"), "printed: {printed}");
        let parsed = <ImperativeOperation as vcs::OpText>::parse_op(&printed).expect("round trips");
        assert_eq!(parsed, operation);
    }
    //#endregion DSL text round trips and error paths

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
        let mut host = ImperativeHost::default();
        host.set_step_params_json("step-2", r#"{"message":"updated"}"#).expect("sets params");
        let step = host.document.path.steps.iter().find(|step| step.id == "step-2").expect("step-2 exists");
        assert_eq!(step.params.get("message"), Some(&neural_engine::Value::Atom(Atom::String("updated".into()))));
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
