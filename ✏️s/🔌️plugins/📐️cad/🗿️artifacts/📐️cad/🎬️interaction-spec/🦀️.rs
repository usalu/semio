//! 🎬️ `spatial://schema/json/interaction` — the declarative construction-interaction spec types and
//! their little expression interpreter, as authored in `🖼️assets/🏗️modelDefinitions/*/🕹️interactions/*.json`.
//! Sibling topic file of the cad artifact's `🦀️.rs`; the statechart that RUNS these specs
//! lives in the artifact engine (`⚙️engine/🕹️interaction/🦀️.rs`).

use protocol::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️InteractionSpec
/// A path root within an expression/effect target — `context` (session context), `event` (the
/// event payload being handled), or `params` (an enclosing action's parameters; unused by the
/// interaction machine interpreter itself, only by `spatial.action` step specs).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub enum ExprPathRoot {
    Context,
    Event,
    Params,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum ExprPathSegment {
    Field { name: String },
    Index { index: usize },
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ExprPathTarget {
    pub root: ExprPathRoot,
    #[value(default)]
    pub segments: Vec<ExprPathSegment>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ExprBinding {
    pub name: String,
    pub value: Box<Expr>,
}

/// `spatial://schema/json/expression` — a small declarative expression AST. Only the kinds
/// actually used by the interaction machine specs' guards/effects/display are interpreted here
/// (`kernel.call`/`distance`/`fold` appear only in `spatial.action` step specs, which are not
/// executed generically — see the commit-action runner in `cad/plugin/rs/interaction.rs`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum Expr {
    Path {
        root: ExprPathRoot,
        #[value(default)]
        segments: Vec<ExprPathSegment>,
    },
    Const {
        value: DslValue,
    },
    Var {
        name: String,
    },
    Let {
        bindings: Vec<ExprBinding>,
        #[value(rename = "in")]
        body: Box<Expr>,
    },
    Exists {
        target: ExprPathTarget,
    },
    NotEmpty {
        target: ExprPathTarget,
    },
    All {
        args: Vec<Expr>,
    },
    Any {
        args: Vec<Expr>,
    },
    Not {
        arg: Box<Expr>,
    },
    Abs {
        arg: Box<Expr>,
    },
    Distance {
        a: Box<Expr>,
        b: Box<Expr>,
    },
    #[value(rename = "kernel.call")]
    KernelCall {
        function: String,
        #[value(default)]
        args: std::collections::HashMap<String, Expr>,
    },
    Binop {
        operation: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Fold {
        operation: String,
        args: Vec<Expr>,
    },
}

/// Evaluation environment for {@link Expr}: `context` is the engagement session's persistent
/// state, `event` is the payload of the event currently being handled (if any).
pub struct ExprEnv<'a> {
    pub context: &'a std::collections::HashMap<String, DslValue>,
    pub event: Option<&'a DslValue>,
}

fn expr_path_get(root_value: Option<&DslValue>, segments: &[ExprPathSegment]) -> Option<DslValue> {
    let mut current = root_value?.clone();
    for segment in segments {
        current = match segment {
            ExprPathSegment::Field { name } => current.get(name)?.clone(),
            ExprPathSegment::Index { index } => current.as_array()?.get(*index)?.clone(),
        };
    }
    Some(current)
}

fn expr_value_truthy(value: &DslValue) -> bool {
    match value {
        DslValue::Null => false,
        DslValue::Bool(b) => *b,
        DslValue::Number(_) => value.as_f64().is_some_and(|v| v != 0.0),
        DslValue::String(s) => !s.is_empty(),
        DslValue::Array(a) => !a.is_empty(),
        DslValue::Object(o) => !o.is_empty(),
    }
}

fn expr_value_not_empty(value: Option<&DslValue>) -> bool {
    match value {
        None => false,
        Some(DslValue::Null) => false,
        Some(DslValue::Array(a)) => !a.is_empty(),
        Some(DslValue::Object(o)) => !o.is_empty(),
        Some(DslValue::String(s)) => !s.is_empty(),
        Some(_) => true,
    }
}

fn expr_as_f64(value: &DslValue) -> f64 {
    value.as_f64().unwrap_or(0.0)
}

/// Evaluates an {@link Expr} against `env` and an outer `let`-binding scope (`vars`).
pub fn evaluate_expr(expr: &Expr, env: &ExprEnv<'_>, vars: &std::collections::HashMap<String, DslValue>) -> DslValue {
    match expr {
        Expr::Path { root, segments } => {
            let root_value = match root {
                ExprPathRoot::Context => Some(DslValue::object(env.context.iter().map(|(k, v)| (k.clone(), v.clone())))),
                ExprPathRoot::Event => env.event.cloned(),
                ExprPathRoot::Params => None,
            };
            expr_path_get(root_value.as_ref(), segments).unwrap_or(DslValue::Null)
        }
        Expr::Const { value } => value.clone(),
        Expr::Var { name } => vars.get(name).cloned().unwrap_or(DslValue::Null),
        Expr::Let { bindings, body } => {
            let mut scope = vars.clone();
            for binding in bindings {
                let value = evaluate_expr(&binding.value, env, &scope);
                scope.insert(binding.name.clone(), value);
            }
            evaluate_expr(body, env, &scope)
        }
        Expr::Exists { target } => {
            let root_value = match target.root {
                ExprPathRoot::Context => Some(DslValue::object(env.context.iter().map(|(k, v)| (k.clone(), v.clone())))),
                ExprPathRoot::Event => env.event.cloned(),
                ExprPathRoot::Params => None,
            };
            DslValue::Bool(expr_path_get(root_value.as_ref(), &target.segments).is_some())
        }
        Expr::NotEmpty { target } => {
            let root_value = match target.root {
                ExprPathRoot::Context => Some(DslValue::object(env.context.iter().map(|(k, v)| (k.clone(), v.clone())))),
                ExprPathRoot::Event => env.event.cloned(),
                ExprPathRoot::Params => None,
            };
            DslValue::Bool(expr_value_not_empty(expr_path_get(root_value.as_ref(), &target.segments).as_ref()))
        }
        Expr::All { args } => DslValue::Bool(args.iter().all(|arg| expr_value_truthy(&evaluate_expr(arg, env, vars)))),
        Expr::Any { args } => DslValue::Bool(args.iter().any(|arg| expr_value_truthy(&evaluate_expr(arg, env, vars)))),
        Expr::Not { arg } => DslValue::Bool(!expr_value_truthy(&evaluate_expr(arg, env, vars))),
        Expr::Abs { arg } => DslValue::float(expr_as_f64(&evaluate_expr(arg, env, vars)).abs()),
        Expr::Distance { a, b } => {
            let av = evaluate_expr(a, env, vars);
            let bv = evaluate_expr(b, env, vars);
            let da: Option<[f64; 3]> = <[f64; 3] as protocol::FromValue>::from_value(av).ok();
            let db: Option<[f64; 3]> = <[f64; 3] as protocol::FromValue>::from_value(bv).ok();
            match (da, db) {
                (Some(a), Some(b)) => DslValue::float(((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()),
                _ => DslValue::Null,
            }
        }
        // `kernel.call` expressions are only used inside `spatial.action` step specs (not executed
        // generically by this interpreter); evaluating one directly yields null.
        Expr::KernelCall { .. } => DslValue::Null,
        Expr::Binop { operation, left, right } => {
            let lv = evaluate_expr(left, env, vars);
            let rv = evaluate_expr(right, env, vars);
            match operation.as_str() {
                "==" => DslValue::Bool(lv == rv),
                "!=" => DslValue::Bool(lv != rv),
                ">" => DslValue::Bool(expr_as_f64(&lv) > expr_as_f64(&rv)),
                "<" => DslValue::Bool(expr_as_f64(&lv) < expr_as_f64(&rv)),
                ">=" => DslValue::Bool(expr_as_f64(&lv) >= expr_as_f64(&rv)),
                "<=" => DslValue::Bool(expr_as_f64(&lv) <= expr_as_f64(&rv)),
                "+" => DslValue::float(expr_as_f64(&lv) + expr_as_f64(&rv)),
                "-" => DslValue::float(expr_as_f64(&lv) - expr_as_f64(&rv)),
                "*" => DslValue::float(expr_as_f64(&lv) * expr_as_f64(&rv)),
                "/" => DslValue::float(expr_as_f64(&lv) / expr_as_f64(&rv)),
                _ => DslValue::Null,
            }
        }
        Expr::Fold { operation, args } => {
            let values: Vec<f64> = args.iter().map(|arg| expr_as_f64(&evaluate_expr(arg, env, vars))).collect();
            match operation.as_str() {
                "min" => DslValue::float(values.into_iter().fold(f64::INFINITY, f64::min)),
                "max" => DslValue::float(values.into_iter().fold(f64::NEG_INFINITY, f64::max)),
                _ => DslValue::Null,
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum Effect {
    Assign {
        target: ExprPathTarget,
        value: Expr,
    },
    Clear {
        target: ExprPathTarget,
    },
    Append {
        target: ExprPathTarget,
        value: Expr,
    },
    Emit {
        event: DslValue,
    },
    Raise {
        event: String,
    },
    OpenTransaction,
    CommitTransaction,
    RollbackTransaction,
    RequestPreview,
    #[value(rename = "kernel.query")]
    KernelQuery {
        #[value(default)]
        query: Option<String>,
        #[value(default, rename = "assignTo")]
        assign_to: Option<ExprPathTarget>,
    },
    ResolveEditable,
    SetDiagnostic {
        severity: String,
        code: String,
        message: String,
    },
    ClearDiagnostic {
        code: String,
    },
    Action {
        action: String,
        #[value(default)]
        params: std::collections::HashMap<String, Expr>,
        #[value(default, rename = "assignTo")]
        assign_to: Option<ExprPathTarget>,
    },
    /// Asset-only extension (not in the formal schema): delegates to a nested sub-interaction
    /// (`interaction`), then maps each of its `outputs[].value` expressions (evaluated against
    /// the sub-interaction's context) onto `outputs[].target` in the parent context. Used only by
    /// the curve-drawing sub-flow (`mode.curve` in the wall/slab/column specs) — not yet
    /// interpreted (sub-interaction composition is a follow-up; the primary `mode.2points` flow
    /// does not depend on it).
    #[value(rename = "interaction.call")]
    InteractionCall {
        interaction: String,
        #[value(default)]
        outputs: Vec<InteractionCallOutput>,
    },
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct InteractionCallOutput {
    pub target: ExprPathTarget,
    pub value: Expr,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TransitionSpec {
    #[value(default)]
    pub target: Option<String>,
    #[value(default)]
    pub guard: Option<String>,
    #[value(default)]
    pub transient: bool,
    #[value(default)]
    pub key: Option<String>,
    #[value(default)]
    pub label: Option<String>,
    #[value(default)]
    pub effects: Vec<Effect>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct EventHandlerSpec {
    pub event: String,
    #[value(default)]
    pub transitions: Vec<TransitionSpec>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SelectionSpec {
    #[value(default)]
    pub accept: Vec<String>,
    #[value(default)]
    pub multiple: bool,
    #[value(default)]
    pub prompt: Option<String>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct StateDefSpec {
    pub name: String,
    #[value(default)]
    pub r#final: bool,
    #[value(default)]
    pub selection: Option<SelectionSpec>,
    #[value(default)]
    pub on: Vec<EventHandlerSpec>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct MachineSpec {
    pub initial: String,
    pub states: Vec<StateDefSpec>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct GuardSpec {
    pub name: String,
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct LengthEntrySpec {
    pub state: String,
    pub anchor: String,
    pub field: String,
    #[value(default)]
    pub commit: Option<String>,
    #[value(default)]
    pub control: Option<String>,
    #[value(default)]
    pub min: Option<f64>,
    #[value(default)]
    pub max: Option<f64>,
    #[value(default)]
    pub step: Option<f64>,
    #[value(default)]
    pub unit: Option<String>,
    #[value(default)]
    pub default: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct ScalarEntrySpec {
    pub state: String,
    pub event: String,
    pub field: String,
    #[value(default)]
    pub commit: Option<String>,
    #[value(default)]
    pub axis_anchor: Option<String>,
    #[value(default)]
    pub axis_floor: Option<String>,
    #[value(default)]
    pub axis: Option<[f64; 3]>,
    #[value(default)]
    pub control: Option<String>,
    #[value(default)]
    pub min: Option<f64>,
    #[value(default)]
    pub max: Option<f64>,
    #[value(default)]
    pub step: Option<f64>,
    #[value(default)]
    pub unit: Option<String>,
    #[value(default)]
    pub default: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SpatialInteractionConfig {
    #[value(default)]
    pub spatial_ground_pick: bool,
    #[value(default)]
    pub pick_disabled_states: Vec<String>,
    #[value(default)]
    pub ground_pointer_move_states: Vec<String>,
    #[value(default)]
    pub height_drag_states: Vec<String>,
    #[value(default)]
    pub vertical_rod_states: Vec<String>,
    #[value(default)]
    pub height_confirm_state: Option<String>,
    #[value(default)]
    pub length_entry: Vec<LengthEntrySpec>,
    #[value(default)]
    pub scalar_entry: Vec<ScalarEntrySpec>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum DisplayItemSpec {
    Point {
        id: String,
        #[value(default)]
        role: Option<String>,
        position: Expr,
    },
    Label {
        id: String,
        #[value(default)]
        role: Option<String>,
        text: String,
        position: Expr,
    },
    Segment {
        id: String,
        #[value(default)]
        role: Option<String>,
        from: Expr,
        to: Expr,
    },
    #[value(rename = "linear-handle")]
    LinearHandle {
        id: String,
        #[value(default)]
        role: Option<String>,
        axis: [f64; 3],
        origin: Expr,
    },
    #[value(rename = "box-preview")]
    BoxPreview {
        id: String,
        #[value(default)]
        role: Option<String>,
        #[value(rename = "cornerA")]
        corner_a: Expr,
        #[value(rename = "cornerB")]
        corner_b: Expr,
        height: Expr,
    },
    #[value(rename = "entity-highlight")]
    EntityHighlight {
        id: String,
        #[value(default)]
        role: Option<String>,
        #[value(rename = "geometryEntityKind")]
        geometry_entity_kind: String,
        #[value(rename = "entityId")]
        entity_id: Expr,
    },
    Curve {
        id: String,
        #[value(default)]
        role: Option<String>,
    },
    Mesh {
        id: String,
        #[value(default)]
        role: Option<String>,
    },
    /// Asset-only extension kind (`"preview"`) not in the formal schema: a generic wireframe
    /// preview keyed by `previewKind`, evaluated params passed through verbatim to the renderer.
    Preview {
        id: String,
        #[value(default)]
        role: Option<String>,
        #[value(default, rename = "previewKind")]
        preview_kind: Option<String>,
        #[value(default)]
        params: std::collections::HashMap<String, Expr>,
    },
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DisplayStateSpec {
    pub state: String,
    pub items: Vec<DisplayItemSpec>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct DisplaySpec {
    #[value(default)]
    pub states: Vec<DisplayStateSpec>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CommitSpec {
    #[value(default)]
    pub when: Option<String>,
    #[value(default)]
    pub from_states: Vec<String>,
    pub operation: CommitOperationSpec,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CommitOperationSpec {
    pub action: String,
    #[value(default)]
    pub params: std::collections::HashMap<String, Expr>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct InteractionProducesSpec {
    #[value(default)]
    pub typology: Option<String>,
}

/// `spatial://schema/json/interaction` — the full declarative construction-interaction spec, as
/// authored in `cad/asset/modelDefinition/*/interaction/*.json`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct InteractionSpec {
    pub id: String,
    pub version: String,
    #[value(default)]
    pub label: Option<String>,
    #[value(default)]
    pub key: Option<String>,
    #[value(default)]
    pub produces: InteractionProducesSpec,
    #[value(default)]
    pub guards: Vec<GuardSpec>,
    pub machine: MachineSpec,
    #[value(default)]
    pub display: DisplaySpec,
    #[value(default)]
    pub interaction: SpatialInteractionConfig,
    pub commit: CommitSpec,
}

impl InteractionSpec {
    pub fn state<'a>(&'a self, name: &str) -> Option<&'a StateDefSpec> {
        self.machine.states.iter().find(|state| state.name == name)
    }

    pub fn guard(&self, name: &str, env: &ExprEnv<'_>) -> bool {
        self.guards.iter().find(|guard| guard.name == name).is_some_and(|guard| expr_value_truthy(&evaluate_expr(&guard.expr, env, &std::collections::HashMap::new())))
    }
}
//#endregion 🔖️InteractionSpec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
