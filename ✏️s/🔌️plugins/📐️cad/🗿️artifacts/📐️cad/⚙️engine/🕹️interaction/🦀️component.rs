//! 🎮️ CAD interaction statechart — a generic interpreter over `spatial.interaction` JSON assets
//! (`cad/asset/modelDefinition/*/interaction/*.json`, mirroring `cad/schema/json/🔣️inter🔣️action.json`),
//! plus a small commit-action runner mapping each spec's `commit.operation.action` onto real
//! `kernel_3d_brepkit` calls. Four "building.building.*" ids have no JSON asset (aec.building has
//! no interaction directory) and keep a bespoke hand-written statechart (`legacy_*` functions)
//! identical to the pre-engine behavior.

use crate::artifacts::cad::{evaluate_expr, CadObject, CadPaneId, CadPrimitiveSlot, DisplayItemSpec, Effect, ExprEnv, ExprPathRoot, ExprPathSegment, ExprPathTarget, InteractionSpec};

use semio_s_3d::brep::engine::BrepKernel;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::OnceLock;

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadEngagementSession {
    pub interaction_id: String,
    pub state: String,
    pub context: HashMap<String, Value>,
    pub pane: CadPaneId,
    #[serde(default)]
    pub last_response: Option<String>,
}

#[derive(Clone, Debug)]
pub struct KeyedTransition {
    pub key: String,
    pub label: String,
    pub event_kind: String,
}

#[derive(Clone, Debug)]
pub struct InteractionCatalogEntry {
    pub id: String,
    pub label: String,
    pub key: String,
    pub model_definition_id: String,
    pub produces_typology: String,
}
//#endregion 🔖️Types

//#region 🔖️Registry
/// `(modelDefinitionId, raw JSON)` for every `interaction/*.json` asset embedded at build time.
/// `aec.building` has no interaction assets of its own — see `LEGACY_BUILDING_INTERACTION_IDS`.
const RAW_INTERACTION_ASSETS: &[(&str, &str)] = &[
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️arc.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️area.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️booleanDifference.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️booleanIntersection.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️booleanUnion.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️box.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️chamfer.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️circle.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️constructCurve.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️constructSurface.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️controlPointCurve.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️copy.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️createAnchor.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️cylinder.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️explode.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️extrudeCrv.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️extrudeWire.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️fillet.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️interpolateCurve.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️join.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️length.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️line.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️loft.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️mirror.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️move.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️networkSrf.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️offsetSurface.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️plane.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️polyline.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️rotate.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️scale1d.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️scale3d.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️sphere.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️split.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️sweep1.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️sweep2.json")),
    ("spatial.shape", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📐️spatial.shape/🎬️interactions/🔣️trim.json")),
    ("aec.building.energy", include_str!("../../../../🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🎬️interactions/🔣️constructBasePlate.json")),
    ("aec.building.energy", include_str!("../../../../🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🎬️interactions/🔣️constructExternalWall.json")),
    ("aec.building.energy", include_str!("../../../../🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🎬️interactions/🔣️constructHull.json")),
    ("aec.building.energy", include_str!("../../../../🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🎬️interactions/🔣️constructRoof.json")),
    ("aec.building.energy", include_str!("../../../../🖼️assets/🏗️modelDefinitions/🔥️aec.building.energy/🎬️interactions/🔣️constructWindows.json")),
    ("aec.building.structure.classic", include_str!("../../../../🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure.classic/🎬️interactions/🔣️constructOneWayReinforcedConcreteSlab.json")),
    ("aec.building.structure.classic", include_str!("../../../../🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteColumn.json")),
    ("aec.building.structure.classic", include_str!("../../../../🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteExternalWall.json")),
    ("aec.building.structure.classic", include_str!("../../../../🖼️assets/🏗️modelDefinitions/🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteInternalWall.json")),
    ("aec.building.structure.fem.line", include_str!("../../../../🖼️assets/🏗️modelDefinitions/📏️aec.building.structure.fem.line/🎬️interactions/🔣️constructLineElement.json")),
    ("aec.building.structure.fem.solid", include_str!("../../../../🖼️assets/🏗️modelDefinitions/🧊️aec.building.structure.fem.solid/🎬️interactions/🔣️constructSolidElement.json")),
    ("aec.building.structure.fem.surface", include_str!("../../../../🖼️assets/🏗️modelDefinitions/🗺️aec.building.structure.fem.surface/🎬️interactions/🔣️constructSurfaceElement.json")),
];

const LEGACY_BUILDING_INTERACTION_IDS: &[&str] = &["building.building.constructWall", "building.building.constructBeam", "building.building.constructColumn", "building.building.constructSlab"];

fn is_legacy_building_id(id: &str) -> bool {
    LEGACY_BUILDING_INTERACTION_IDS.contains(&id)
}

static PARSED_SPECS: OnceLock<Vec<(&'static str, InteractionSpec)>> = OnceLock::new();

fn parsed_specs() -> &'static [(&'static str, InteractionSpec)] {
    PARSED_SPECS.get_or_init(|| RAW_INTERACTION_ASSETS.iter().filter_map(|(model_def, raw)| serde_json::from_str::<InteractionSpec>(raw).ok().map(|spec| (*model_def, spec))).collect())
}

fn spec_by_id(id: &str) -> Option<&'static InteractionSpec> {
    parsed_specs().iter().find(|(_, spec)| spec.id == id).map(|(_, spec)| spec)
}

static CATALOG: OnceLock<Vec<InteractionCatalogEntry>> = OnceLock::new();

fn catalog() -> &'static [InteractionCatalogEntry] {
    CATALOG.get_or_init(|| {
        let mut entries = vec![
            InteractionCatalogEntry { id: "building.building.constructWall".to_string(), label: "Wall".to_string(), key: "w".to_string(), model_definition_id: "aec.building".to_string(), produces_typology: "building.building.wall".to_string() },
            InteractionCatalogEntry { id: "building.building.constructBeam".to_string(), label: "Beam".to_string(), key: "m".to_string(), model_definition_id: "aec.building".to_string(), produces_typology: "building.building.beam".to_string() },
            InteractionCatalogEntry {
                id: "building.building.constructColumn".to_string(),
                label: "Column".to_string(),
                key: "c".to_string(),
                model_definition_id: "aec.building".to_string(),
                produces_typology: "building.building.column".to_string(),
            },
            InteractionCatalogEntry { id: "building.building.constructSlab".to_string(), label: "Slab".to_string(), key: "l".to_string(), model_definition_id: "aec.building".to_string(), produces_typology: "building.building.slab".to_string() },
        ];
        for (model_def, spec) in parsed_specs() {
            entries.push(InteractionCatalogEntry {
                id: spec.id.clone(),
                label: spec.label.clone().unwrap_or_else(|| spec.id.clone()),
                key: spec.key.clone().unwrap_or_default(),
                model_definition_id: (*model_def).to_string(),
                produces_typology: spec.produces.typology.clone().unwrap_or_default(),
            });
        }
        entries
    })
}
//#endregion 🔖️Registry

//#region 🔖️Catalog
pub fn list_interactions_for_model_definition(model_definition_id: &str) -> Vec<&'static InteractionCatalogEntry> {
    catalog().iter().filter(|entry| entry.model_definition_id == model_definition_id).collect()
}

pub fn resolve_interaction_key(input: &str, model_definition_id: &str) -> Option<&'static InteractionCatalogEntry> {
    let trimmed = input.trim().to_lowercase();
    catalog().iter().find(|entry| entry.model_definition_id == model_definition_id && (entry.key == trimmed || entry.id.eq_ignore_ascii_case(&trimmed) || entry.id.to_lowercase().ends_with(&format!(".{trimmed}"))))
}

pub fn interaction_by_id(id: &str) -> Option<&'static InteractionCatalogEntry> {
    catalog().iter().find(|entry| entry.id == id)
}
//#endregion 🔖️Catalog

//#region 🔖️Statechart
fn vec3_json(point: [f64; 3]) -> Value {
    json!([point[0], point[1], point[2]])
}

fn parse_vec3(value: &Value) -> Option<[f64; 3]> {
    let array = value.as_array()?;
    if array.len() < 3 {
        return None;
    }
    Some([array[0].as_f64()?, array[1].as_f64()?, array[2].as_f64()?])
}

fn context_point(session: &CadEngagementSession, field: &str) -> Option<[f64; 3]> {
    session.context.get(field).and_then(parse_vec3)
}

pub fn start_session(interaction_id: &str, pane: CadPaneId) -> Option<CadEngagementSession> {
    if is_legacy_building_id(interaction_id) {
        return Some(CadEngagementSession { interaction_id: interaction_id.to_string(), state: "idle".to_string(), context: HashMap::new(), pane, last_response: None });
    }
    let spec = spec_by_id(interaction_id)?;
    Some(CadEngagementSession { interaction_id: spec.id.clone(), state: spec.machine.initial.clone(), context: HashMap::new(), pane, last_response: None })
}

pub fn keyed_transitions(session: &CadEngagementSession) -> Vec<KeyedTransition> {
    if is_legacy_building_id(&session.interaction_id) {
        return legacy_keyed_transitions(session);
    }
    let Some(spec) = spec_by_id(&session.interaction_id) else {
        return Vec::new();
    };
    let Some(state) = spec.state(&session.state) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for handler in &state.on {
        for transition in &handler.transitions {
            if let Some(key) = &transition.key {
                out.push(KeyedTransition { key: key.clone(), label: transition.label.clone().unwrap_or_else(|| handler.event.clone()), event_kind: handler.event.clone() });
            }
        }
    }
    out
}

pub fn can_commit(session: &CadEngagementSession) -> bool {
    if is_legacy_building_id(&session.interaction_id) {
        return session.state == "ready";
    }
    let Some(spec) = spec_by_id(&session.interaction_id) else {
        return false;
    };
    if !spec.commit.from_states.iter().any(|state| state == &session.state) {
        return false;
    }
    match &spec.commit.when {
        None => true,
        Some(guard_name) => {
            let env = ExprEnv { context: &session.context, event: None };
            spec.guard(guard_name, &env)
        }
    }
}

fn context_target_field(target: &ExprPathTarget) -> Option<&str> {
    if target.root != ExprPathRoot::Context {
        return None;
    }
    match target.segments.as_slice() {
        [ExprPathSegment::Field { name }] => Some(name.as_str()),
        _ => None,
    }
}

/// Wraps a raw event payload into the shape the JSON specs' `event.*` path expressions expect:
/// `pointer.down`/`pointer.move` read `event.point`, `set.*` events read `event.value`. Callers
/// (both `lib.rs`'s command handlers and this module's own tests) pass raw values (a `[x,y,z]`
/// array, a bare number) for brevity — already-wrapped objects pass through unchanged.
fn normalize_event_payload(event_kind: &str, payload: Option<&Value>) -> Option<Value> {
    let payload = payload?;
    if payload.is_object() {
        return Some(payload.clone());
    }
    if event_kind == "pointer.down" || event_kind == "pointer.move" {
        return Some(json!({ "point": payload }));
    }
    if event_kind.starts_with("set.") {
        return Some(json!({ "value": payload }));
    }
    Some(payload.clone())
}

/// Executes an `action` effect by name.
///
/// `command.addPoint` (used by sphere/circle/etc.) records a named point into a
/// `context[field][key]` map. `box.aabbFromDiagonalCorners` (box's default diagonal-mode second
/// click) derives `context.origin`/`context.corner` — the axis-aligned min/max of `context.diagA`
/// and `event.point` — which `hasValidBox` and the commit params then read.
///
/// The remaining `box.*` rubber-band helpers and selection-driven actions (used only by box's
/// advanced cube/3-point/center sub-modes and by selection-based utilities) are a documented
/// follow-up; they no-operation here rather than error.
fn run_named_action_effect(context: &mut HashMap<String, Value>, payload: Option<&Value>, action: &str, params: &HashMap<String, Value>) {
    match action {
        "command.addPoint" => {
            let field = params.get("field").and_then(|value| value.as_str()).unwrap_or("points").to_string();
            let key = params.get("key").and_then(|value| value.as_str()).map(str::to_string);
            let point = params.get("point").cloned().unwrap_or(Value::Null);
            let entry = context.entry(field).or_insert_with(|| json!({}));
            if !entry.is_object() {
                *entry = json!({});
            }
            if let (Some(key), Some(object)) = (key, entry.as_object_mut()) {
                object.insert(key, point);
            }
        }
        "box.aabbFromDiagonalCorners" => {
            let diag_a = context.get("diagA").and_then(parse_vec3);
            let second = payload.and_then(|value| value.get("point")).and_then(parse_vec3);
            if let (Some(a), Some(b)) = (diag_a, second) {
                let origin = [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])];
                let corner = [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])];
                context.insert("origin".into(), vec3_json(origin));
                context.insert("corner".into(), vec3_json(corner));
            }
        }
        _ => {}
    }
}

fn apply_effect(session: &mut CadEngagementSession, payload: Option<&Value>, effect: &Effect, raised: &mut Vec<String>) {
    let empty_vars = HashMap::new();
    match effect {
        Effect::Assign { target, value } => {
            if let Some(field) = context_target_field(target) {
                let env = ExprEnv { context: &session.context, event: payload };
                let evaluated = evaluate_expr(value, &env, &empty_vars);
                session.context.insert(field.to_string(), evaluated);
            }
        }
        Effect::Clear { target } => {
            if let Some(field) = context_target_field(target) {
                session.context.remove(field);
            }
        }
        Effect::Append { target, value } => {
            if let Some(field) = context_target_field(target) {
                let env = ExprEnv { context: &session.context, event: payload };
                let evaluated = evaluate_expr(value, &env, &empty_vars);
                let entry = session.context.entry(field.to_string()).or_insert_with(|| json!([]));
                if let Some(array) = entry.as_array_mut() {
                    array.push(evaluated);
                } else {
                    *entry = json!([evaluated]);
                }
            }
        }
        Effect::Raise { event } => raised.push(event.clone()),
        Effect::Action { action, params, .. } => {
            let env = ExprEnv { context: &session.context, event: payload };
            let evaluated: HashMap<String, Value> = params.iter().map(|(key, value)| (key.clone(), evaluate_expr(value, &env, &empty_vars))).collect();
            run_named_action_effect(&mut session.context, payload, action, &evaluated);
        }
        // Emit/OpenTransaction/CommitTransaction/RollbackTransaction/RequestPreview/KernelQuery/
        // ResolveEditable/SetDiagnostic/ClearDiagnostic/InteractionCall are not yet interpreted —
        // InteractionCall (nested sub-interaction composition) is a documented follow-up used only
        // by the curve-drawing sub-flow (`mode.curve`); the primary `mode.2points` flow doesn't
        // depend on it. The others have no observable effect on committed geometry.
        _ => {}
    }
}

fn apply_event_generic(session: &mut CadEngagementSession, event_kind: &str, raw_payload: Option<&Value>, depth: u8) -> bool {
    if depth > 8 {
        return false;
    }
    let Some(spec) = spec_by_id(&session.interaction_id) else {
        return false;
    };
    let Some(state) = spec.state(&session.state) else {
        return false;
    };
    let Some(handler) = state.on.iter().find(|handler| handler.event == event_kind) else {
        return false;
    };
    let normalized = normalize_event_payload(event_kind, raw_payload);
    let payload = normalized.as_ref();
    let chosen = handler.transitions.iter().find(|transition| match &transition.guard {
        None => true,
        Some(name) => {
            let env = ExprEnv { context: &session.context, event: payload };
            spec.guard(name, &env)
        }
    });
    let Some(transition) = chosen else {
        return false;
    };
    let mut raised = Vec::new();
    for effect in &transition.effects {
        apply_effect(session, payload, effect, &mut raised);
    }
    if let Some(target) = &transition.target {
        session.state = target.clone();
    }
    session.last_response = Some("OK".into());
    for raised_event in raised {
        apply_event_generic(session, &raised_event, None, depth + 1);
    }
    true
}

fn legacy_keyed_transitions(session: &CadEngagementSession) -> Vec<KeyedTransition> {
    if session.state == "idle" {
        return vec![KeyedTransition { key: "s".into(), label: "Start".into(), event_kind: "start".into() }];
    }
    Vec::new()
}

fn legacy_apply_event(session: &mut CadEngagementSession, event_kind: &str, payload: Option<&Value>) -> bool {
    let is_column = session.interaction_id == "building.building.constructColumn";
    let changed = match (session.state.as_str(), event_kind) {
        ("idle", "start") => {
            session.state = if is_column { "column_base" } else { "footprint_first" }.into();
            true
        }
        ("footprint_first", "pointer.down") => {
            if let Some(point) = payload.and_then(parse_vec3) {
                session.context.insert("cornerA".into(), vec3_json(point));
                session.state = "footprint_second".into();
                true
            } else {
                false
            }
        }
        ("footprint_second", "pointer.down") => {
            if let Some(point) = payload.and_then(parse_vec3) {
                session.context.insert("cornerB".into(), vec3_json(point));
                session.state = "slab_height".into();
                true
            } else {
                false
            }
        }
        ("slab_height", "set.height") => {
            if let Some(height) = payload.and_then(|value| value.as_f64()) {
                session.context.insert("height".into(), json!(height));
                session.state = "ready".into();
                true
            } else {
                false
            }
        }
        ("column_base", "pointer.down") => {
            if let Some(point) = payload.and_then(parse_vec3) {
                session.context.insert("base".into(), vec3_json(point));
                session.state = "column_height".into();
                true
            } else {
                false
            }
        }
        ("column_height", "set.height") => {
            if let Some(height) = payload.and_then(|value| value.as_f64()) {
                session.context.insert("height".into(), json!(height));
                session.state = "ready".into();
                true
            } else {
                false
            }
        }
        _ => false,
    };
    if changed {
        session.last_response = Some("OK".into());
    }
    changed
}

pub fn apply_event(session: &mut CadEngagementSession, event_kind: &str, payload: Option<&Value>) -> bool {
    if is_legacy_building_id(&session.interaction_id) {
        return legacy_apply_event(session, event_kind, payload);
    }
    apply_event_generic(session, event_kind, payload, 0)
}

/// States where a numeric-only line commits the pending height (premigration `tryCommitNumericEntry`).
const NUMERIC_ENTRY_STATES: &[&str] = &["first_corner_height", "two_points_height", "slab_height", "column_height", "radius", "curve_height"];

fn strip_prefix_ignore_case<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    if text.len() < prefix.len() {
        return None;
    }
    let (head, tail) = text.split_at(prefix.len());
    head.eq_ignore_ascii_case(prefix).then_some(tail)
}

/// Parses a REPL command line into an `(event_kind, payload)` pair.
///
/// `current_state` is the active engagement session's state (if any) — required to disambiguate a
/// bare numeric line (e.g. `"3.5"`) as a height commit only while a numeric-entry state is active,
/// mirroring premigration's `trySubmitLine` numeric-entry step.
pub fn parse_repl_line(line: &str, current_state: Option<&str>) -> Option<(String, Option<Value>)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Legacy raw forms (still used by the wgpu renderer's REPL, which does not PascalCase drafts).
    if let Some(rest) = trimmed.strip_prefix("set.height ") {
        return rest.trim().parse::<f64>().ok().map(|height| ("set.height".into(), Some(json!(height))));
    }
    if let Some(rest) = trimmed.strip_prefix("dist ") {
        return rest.trim().parse::<f64>().ok().map(|distance| ("set.distance".into(), Some(json!(distance))));
    }
    // Normalized forms: the React shell's engagement input PascalCases every draft (no separators),
    // so `set.height 3.5` arrives as `SetHeight3.5` (framework/renderer/react `Engagement.applyDraft`
    // via `normalizeEngagementCommandText`).
    if let Some(rest) = strip_prefix_ignore_case(trimmed, "SetHeight") {
        if let Ok(height) = rest.parse::<f64>() {
            return Some(("set.height".into(), Some(json!(height))));
        }
    }
    if let Some(rest) = strip_prefix_ignore_case(trimmed, "Dist") {
        if let Ok(distance) = rest.parse::<f64>() {
            return Some(("set.distance".into(), Some(json!(distance))));
        }
    }
    // Bare numeric entry commits height while a numeric-entry state is active.
    if current_state.is_some_and(|state| NUMERIC_ENTRY_STATES.contains(&state)) {
        if let Ok(height) = trimmed.parse::<f64>() {
            return Some(("set.height".into(), Some(json!(height))));
        }
    }
    Some((trimmed.into(), None))
}
//#endregion 🔖️Statechart

//#region 🔖️CommitRunner
fn commit_primitive_box(kernel: &mut dyn BrepKernel, params: &HashMap<String, Value>, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    let corner_a = params.get("cornerA").and_then(parse_vec3)?;
    let corner_b = params.get("cornerB").and_then(parse_vec3)?;
    let height = params.get("height").and_then(|value| value.as_f64()).unwrap_or(1.0);
    let width = (corner_b[0] - corner_a[0]).abs().max(0.05);
    let depth = (corner_b[1] - corner_a[1]).abs().max(0.05);
    let solid = semio_s_3d::brep::engine::block_on(kernel.box_prim(width, depth, height.max(0.05))).ok()?;
    Some(CadObject {
        id: next_id("object"),
        label: format!("Box {}", label_count + 1),
        typology: "spatial.shape.primitive.box".into(),
        visible: true,
        locked: false,
        origin: corner_a,
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        mesh_url: None,
        extent: Some([width, depth, height.max(0.05)]),
        solid_handle: Some(solid.0.clone()),
        primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
    })
}

/// Generic commit for the "2 points + height" family shared by every `aec.building.energy`,
/// `aec.building.structure.classic`, and `aec.building.structure.fem.*` construction interaction
/// (`commit.operation.action` ending in `From2PointsAndHeight`/`FromSurface`) — differentiated only
/// by the `typology` commit param.
fn commit_from_2_points_and_height(kernel: &mut dyn BrepKernel, params: &HashMap<String, Value>, label: &str, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    let typology = params.get("typology").and_then(|value| value.as_str()).unwrap_or("").to_string();
    let lower = typology.to_lowercase();
    let point_a = params.get("pointA").and_then(parse_vec3)?;
    let height = params.get("height").and_then(|value| value.as_f64()).unwrap_or(3.0);

    if lower.contains("column") {
        let radius = 0.25;
        let solid = semio_s_3d::brep::engine::block_on(kernel.cylinder_prim(radius, height.max(0.05))).ok()?;
        return Some(CadObject {
            id: next_id("object"),
            label: format!("{label} {}", label_count + 1),
            typology,
            visible: true,
            locked: false,
            origin: point_a,
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: Some([radius * 2.0, radius * 2.0, height.max(0.05)]),
            solid_handle: Some(solid.0.clone()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
        });
    }

    let point_b = params.get("pointB").and_then(parse_vec3)?;
    let span = ((point_b[0] - point_a[0]).powi(2) + (point_b[1] - point_a[1]).powi(2)).sqrt().max(0.5);
    let (width, depth, solid_height) = if lower.contains("wall") {
        (span, 0.2, height.max(0.05))
    } else if lower.contains("windows") {
        (span, 0.05, height.max(0.05))
    } else {
        // slab / baseplate / roof / hull / fem elements: flat footprint extruded by `height`.
        let w = (point_b[0] - point_a[0]).abs().max(0.5);
        let d = (point_b[1] - point_a[1]).abs().max(0.5);
        (w, d, height.max(0.05))
    };
    let solid = semio_s_3d::brep::engine::block_on(kernel.box_prim(width, depth, solid_height)).ok()?;
    Some(CadObject {
        id: next_id("object"),
        label: format!("{label} {}", label_count + 1),
        typology,
        visible: true,
        locked: false,
        origin: point_a,
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        mesh_url: None,
        extent: Some([width, depth, solid_height]),
        solid_handle: Some(solid.0.clone()),
        primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
    })
}

/// `command.finish` dispatches by the commit's `resultKind` param, reading whatever context fields
/// that interaction's machine populated (`points.<key>`, `radius`, ...). Only `sphere` is
/// implemented so far; other result kinds (cylinder/circle/plane/curve/boolean/...) are a
/// documented follow-up — this returns `None` for them, matching the pre-engine fallback behavior
/// for any not-yet-implemented interaction.
fn commit_command_finish(kernel: &mut dyn BrepKernel, params: &HashMap<String, Value>, context: &HashMap<String, Value>, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    let result_kind = params.get("resultKind").and_then(|value| value.as_str())?;
    match result_kind {
        "sphere" => {
            let points = context.get("points")?.as_object()?;
            let center = points.get("center").and_then(parse_vec3)?;
            let radius = if let Some(radius) = context.get("radius").and_then(|value| value.as_f64()) {
                radius
            } else {
                let radius_point = points.get("radiusPoint").and_then(parse_vec3)?;
                ((radius_point[0] - center[0]).powi(2) + (radius_point[1] - center[1]).powi(2) + (radius_point[2] - center[2]).powi(2)).sqrt()
            }
            .max(0.05);
            let solid = semio_s_3d::brep::engine::block_on(kernel.sphere_prim(radius)).ok()?;
            Some(CadObject {
                id: next_id("object"),
                label: format!("Sphere {}", label_count + 1),
                typology: "spatial.shape.solid.sphere".into(),
                visible: true,
                locked: false,
                origin: center,
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: Some([radius * 2.0, radius * 2.0, radius * 2.0]),
                solid_handle: Some(solid.0.clone()),
                primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
            })
        }
        _ => None,
    }
}

fn legacy_commit_object(kernel: &mut dyn BrepKernel, session: &CadEngagementSession, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    let entry = interaction_by_id(&session.interaction_id)?;
    if session.interaction_id == "building.building.constructColumn" {
        let base = context_point(session, "base")?;
        let height = session.context.get("height").and_then(|value| value.as_f64()).unwrap_or(3.0);
        let radius = 0.25;
        let solid = semio_s_3d::brep::engine::block_on(kernel.cylinder_prim(radius, height.max(0.05))).ok()?;
        return Some(CadObject {
            id: next_id("object"),
            label: format!("{} {}", entry.label, label_count + 1),
            typology: entry.produces_typology.clone(),
            visible: true,
            locked: false,
            origin: base,
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: Some([radius * 2.0, radius * 2.0, height.max(0.05)]),
            solid_handle: Some(solid.0.clone()),
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
        });
    }
    let corner_a = context_point(session, "cornerA")?;
    let corner_b = context_point(session, "cornerB")?;
    let id = session.interaction_id.as_str();
    let default_height = if id.contains("Slab") {
        0.25
    } else if id.contains("Beam") {
        0.4
    } else {
        3.0
    };
    let height = session.context.get("height").and_then(|value| value.as_f64()).unwrap_or(default_height);
    let span = ((corner_b[0] - corner_a[0]).powi(2) + (corner_b[1] - corner_a[1]).powi(2)).sqrt().max(0.5);
    let width = (corner_b[0] - corner_a[0]).abs().max(0.5);
    let depth = (corner_b[1] - corner_a[1]).abs().max(0.5);
    let (solid_width, solid_depth, solid_height) = if id.contains("Beam") {
        (span, 0.3, 0.3)
    } else if id.contains("Wall") {
        (span, 0.2, height.max(0.05))
    } else {
        (width, depth, height.max(0.05))
    };
    let solid = semio_s_3d::brep::engine::block_on(kernel.box_prim(solid_width, solid_depth, solid_height)).ok()?;
    Some(CadObject {
        id: next_id("object"),
        label: format!("{} {}", entry.label, label_count + 1),
        typology: entry.produces_typology.clone(),
        visible: true,
        locked: false,
        origin: corner_a,
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        mesh_url: None,
        extent: Some([solid_width, solid_depth, solid_height]),
        solid_handle: Some(solid.0.clone()),
        primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: solid.0, kind: "solid".into() }],
    })
}

pub fn commit_object(kernel: &mut dyn BrepKernel, session: &CadEngagementSession, label_count: usize, next_id: impl Fn(&str) -> String) -> Option<CadObject> {
    if is_legacy_building_id(&session.interaction_id) {
        return legacy_commit_object(kernel, session, label_count, next_id);
    }
    let spec = spec_by_id(&session.interaction_id)?;
    let env = ExprEnv { context: &session.context, event: None };
    let empty_vars = HashMap::new();
    let params: HashMap<String, Value> = spec.commit.operation.params.iter().map(|(key, value)| (key.clone(), evaluate_expr(value, &env, &empty_vars))).collect();
    let action = spec.commit.operation.action.as_str();
    let label = spec.label.clone().unwrap_or_else(|| spec.id.clone());
    if action == "primitive.createBoxFromCorners" {
        return commit_primitive_box(kernel, &params, label_count, next_id);
    }
    if action.ends_with("From2PointsAndHeight") || action.ends_with("FromSurface") {
        return commit_from_2_points_and_height(kernel, &params, &label, label_count, next_id);
    }
    if action == "command.finish" {
        return commit_command_finish(kernel, &params, &session.context, label_count, next_id);
    }
    None
}
//#endregion 🔖️CommitRunner

//#region 🔖️Preview
fn preview_two_point_footprint(session: &CadEngagementSession, include_segment: bool) -> Vec<Value> {
    let mut items = Vec::new();
    if let Some(corner_a) = context_point(session, "cornerA") {
        items.push(json!({ "kind": "point", "role": "cornerA", "position": corner_a }));
    }
    if include_segment {
        if let (Some(corner_a), Some(corner_b)) = (context_point(session, "cornerA"), context_point(session, "cornerB")) {
            items.push(json!({ "kind": "segment", "role": "footprint", "from": corner_a, "to": corner_b }));
        }
    }
    items
}

fn legacy_preview_display_items(session: &CadEngagementSession) -> Vec<Value> {
    if session.interaction_id == "building.building.constructColumn" {
        return match session.state.as_str() {
            "column_height" | "ready" => {
                let mut items = Vec::new();
                if let Some(base) = context_point(session, "base") {
                    items.push(json!({ "kind": "point", "role": "base", "position": base }));
                }
                items
            }
            _ => Vec::new(),
        };
    }
    match session.state.as_str() {
        "footprint_first" => preview_two_point_footprint(session, false),
        "footprint_second" | "slab_height" | "ready" => preview_two_point_footprint(session, true),
        _ => Vec::new(),
    }
}

fn display_item_to_json(item: &DisplayItemSpec, env: &ExprEnv<'_>, vars: &HashMap<String, Value>) -> Option<Value> {
    match item {
        DisplayItemSpec::Point { role, position, .. } => {
            let position = evaluate_expr(position, env, vars);
            if position.is_null() {
                return None;
            }
            Some(json!({ "kind": "point", "role": role, "position": position }))
        }
        DisplayItemSpec::Label { role, text, position, .. } => {
            let position = evaluate_expr(position, env, vars);
            Some(json!({ "kind": "label", "role": role, "text": text, "position": position }))
        }
        DisplayItemSpec::Segment { role, from, to, .. } => {
            let from = evaluate_expr(from, env, vars);
            let to = evaluate_expr(to, env, vars);
            if from.is_null() || to.is_null() {
                return None;
            }
            Some(json!({ "kind": "segment", "role": role, "from": from, "to": to }))
        }
        DisplayItemSpec::LinearHandle { role, axis, origin, .. } => {
            let origin = evaluate_expr(origin, env, vars);
            if origin.is_null() {
                return None;
            }
            Some(json!({ "kind": "linear-handle", "role": role, "axis": axis, "origin": origin }))
        }
        DisplayItemSpec::BoxPreview { role, corner_a, corner_b, height, .. } => {
            let corner_a = evaluate_expr(corner_a, env, vars);
            let corner_b = evaluate_expr(corner_b, env, vars);
            if corner_a.is_null() || corner_b.is_null() {
                return None;
            }
            let height = evaluate_expr(height, env, vars);
            Some(json!({ "kind": "box-preview", "role": role, "cornerA": corner_a, "cornerB": corner_b, "height": height }))
        }
        DisplayItemSpec::EntityHighlight { role, geometry_entity_kind, entity_id, .. } => {
            let entity_id = evaluate_expr(entity_id, env, vars);
            if entity_id.is_null() {
                return None;
            }
            Some(json!({ "kind": "entity-highlight", "role": role, "geometryEntityKind": geometry_entity_kind, "entityId": entity_id }))
        }
        DisplayItemSpec::Curve { role, .. } => Some(json!({ "kind": "curve", "role": role })),
        DisplayItemSpec::Mesh { role, .. } => Some(json!({ "kind": "mesh", "role": role })),
        DisplayItemSpec::Preview { role, preview_kind, params, .. } => {
            let evaluated_params: serde_json::Map<String, Value> = params.iter().map(|(key, value)| (key.clone(), evaluate_expr(value, env, vars))).collect();
            Some(json!({ "kind": "preview", "role": role, "previewKind": preview_kind, "params": evaluated_params }))
        }
    }
}

pub fn preview_display_items(session: &CadEngagementSession) -> Vec<Value> {
    if is_legacy_building_id(&session.interaction_id) {
        return legacy_preview_display_items(session);
    }
    let Some(spec) = spec_by_id(&session.interaction_id) else {
        return Vec::new();
    };
    let Some(display_state) = spec.display.states.iter().find(|state| state.state == session.state) else {
        return Vec::new();
    };
    let env = ExprEnv { context: &session.context, event: None };
    let empty_vars = HashMap::new();
    display_state.items.iter().filter_map(|item| display_item_to_json(item, &env, &empty_vars)).collect()
}
//#endregion 🔖️Preview

#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_3d::brep::kernel::BrepkitKernel;

    #[test]
    fn catalog_includes_json_driven_and_legacy_building_entries() {
        assert!(interaction_by_id("primitive.box").is_some());
        assert!(interaction_by_id("solid.sphere").is_some());
        assert!(interaction_by_id("energy.energy.constructExternalWall").is_some());
        assert!(interaction_by_id("structure.structure.constructReinforcedConcreteColumn").is_some());
        assert!(interaction_by_id("building.building.constructWall").is_some());
        assert_eq!(list_interactions_for_model_definition("spatial.shape").len(), 37);
    }

    #[test]
    fn box_interaction_commits_after_height() {
        let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
        assert!(apply_event(&mut session, "start", None));
        assert!(apply_event(&mut session, "mode.diagonal", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([0.0, 0.0, 0.0]))));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([2.0, 3.0, 0.0]))));
        assert!(apply_event(&mut session, "set.height", Some(&json!(2.5))));
        assert!(apply_event(&mut session, "confirm", None));
        assert!(can_commit(&session));
        let mut kernel = BrepkitKernel::new();
        let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
        assert!(object.is_some());
        assert_eq!(object.unwrap().typology, "spatial.shape.primitive.box");
    }

    #[test]
    fn box_interaction_default_mode_is_point_and_requires_length_prompt() {
        // 🔣️box.json's default `boxMode` (set by the `start` transition) is "point", not "diagonal" —
        // a plain pointer.down after start does NOT reach diagonal_rubber.
        let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
        assert!(apply_event(&mut session, "start", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([0.0, 0.0, 0.0]))));
        assert_eq!(session.state, "first_corner_other_or_length");
    }

    #[test]
    fn sphere_interaction_commits_via_command_finish() {
        let mut session = start_session("solid.sphere", CadPaneId::Shape).expect("session");
        assert!(apply_event(&mut session, "start", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [0.0, 0.0, 0.0] }))));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [2.0, 0.0, 0.0] }))));
        assert!(can_commit(&session));
        let mut kernel = BrepkitKernel::new();
        let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
        let object = object.expect("sphere commits");
        assert_eq!(object.typology, "spatial.shape.solid.sphere");
        assert_eq!(object.origin, [0.0, 0.0, 0.0]);
        assert_eq!(object.extent, Some([4.0, 4.0, 4.0]));
    }

    #[test]
    fn external_wall_interaction_commits_via_generic_from_2_points_and_height() {
        let mut session = start_session("energy.energy.constructExternalWall", CadPaneId::Energy).expect("session");
        assert!(apply_event(&mut session, "mode.2points", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [0.0, 0.0, 0.0] }))));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [4.0, 0.0, 0.0] }))));
        assert!(apply_event(&mut session, "set.height", Some(&json!({ "value": 3.0 }))));
        assert!(can_commit(&session));
        let mut kernel = BrepkitKernel::new();
        let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
        let object = object.expect("wall commits");
        assert_eq!(object.typology, "energy.energy.externalwall");
    }

    #[test]
    fn reinforced_concrete_column_interaction_commits_as_cylinder() {
        let mut session = start_session("structure.structure.constructReinforcedConcreteColumn", CadPaneId::StructureClassic).expect("session");
        assert!(apply_event(&mut session, "mode.2points", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [1.0, 1.0, 0.0] }))));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [1.5, 1.0, 0.0] }))));
        assert!(apply_event(&mut session, "set.height", Some(&json!({ "value": 3.0 }))));
        let mut kernel = BrepkitKernel::new();
        let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
        let object = object.expect("column commits");
        assert_eq!(object.typology, "structure.structure.reinforcedconcretecolumn");
        assert_eq!(object.origin, [1.0, 1.0, 0.0]);
    }

    #[test]
    fn slab_interaction_commits() {
        let mut session = start_session("structure.structure.constructOneWayReinforcedConcreteSlab", CadPaneId::StructureClassic).expect("session");
        assert!(apply_event(&mut session, "mode.2points", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [0.0, 0.0, 0.0] }))));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [4.0, 5.0, 0.0] }))));
        assert!(apply_event(&mut session, "set.height", Some(&json!({ "value": 0.3 }))));
        let mut kernel = BrepkitKernel::new();
        let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
        assert!(object.is_some());
    }

    #[test]
    fn slab_preview_shows_footprint_point() {
        let mut session = start_session("structure.structure.constructOneWayReinforcedConcreteSlab", CadPaneId::StructureClassic).expect("session");
        assert!(apply_event(&mut session, "mode.2points", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!({ "point": [0.0, 0.0, 0.0] }))));
        let items = preview_display_items(&session);
        assert!(items.iter().any(|item| item.get("kind").and_then(|value| value.as_str()) == Some("point")));
    }

    #[test]
    fn legacy_column_preview_shows_base_point() {
        let mut session = start_session("building.building.constructColumn", CadPaneId::Building).expect("session");
        assert!(apply_event(&mut session, "start", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([1.0, 2.0, 0.0]))));
        let items = preview_display_items(&session);
        assert!(items.iter().any(|item| item.get("kind").and_then(|value| value.as_str()) == Some("point")));
    }

    #[test]
    fn legacy_wall_interaction_still_commits() {
        let mut session = start_session("building.building.constructWall", CadPaneId::Building).expect("session");
        assert!(apply_event(&mut session, "start", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([0.0, 0.0, 0.0]))));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([4.0, 0.0, 0.0]))));
        assert!(apply_event(&mut session, "set.height", Some(&json!(3.0))));
        assert!(can_commit(&session));
        let mut kernel = BrepkitKernel::new();
        let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
        assert!(object.is_some());
        assert_eq!(object.unwrap().typology, "building.building.wall");
    }

    #[test]
    fn parse_repl_line_accepts_legacy_raw_forms() {
        assert_eq!(parse_repl_line("set.height 2.5", None), Some(("set.height".into(), Some(json!(2.5)))));
        assert_eq!(parse_repl_line("dist 12", None), Some(("set.distance".into(), Some(json!(12.0)))));
    }

    #[test]
    fn parse_repl_line_accepts_shell_normalized_forms() {
        // The React shell PascalCases every draft (framework/renderer/react `normalizeEngagementCommandText`),
        // so `set.height 3.5` arrives as `SetHeight3.5` with no separators.
        assert_eq!(parse_repl_line("SetHeight3.5", None), Some(("set.height".into(), Some(json!(3.5)))));
        assert_eq!(parse_repl_line("setheight0.25", None), Some(("set.height".into(), Some(json!(0.25)))));
        assert_eq!(parse_repl_line("Dist12.75", None), Some(("set.distance".into(), Some(json!(12.75)))));
    }

    #[test]
    fn parse_repl_line_commits_bare_number_only_in_numeric_entry_state() {
        // Bare numeric entry (premigration `tryCommitNumericEntry`) only applies while a
        // numeric-entry state (e.g. box's first_corner_height) is active.
        assert_eq!(parse_repl_line("3.5", Some("first_corner_height")), Some(("set.height".into(), Some(json!(3.5)))));
        assert_eq!(parse_repl_line("2", Some("column_height")), Some(("set.height".into(), Some(json!(2.0)))));
        // Outside a numeric-entry state, a bare number is treated as an (unresolvable) interaction key.
        assert_eq!(parse_repl_line("3.5", None), Some(("3.5".into(), None)));
        assert_eq!(parse_repl_line("3.5", Some("idle")), Some(("3.5".into(), None)));
    }

    #[test]
    fn box_interaction_commits_via_shell_normalized_repl_line() {
        let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
        assert!(apply_event(&mut session, "start", None));
        assert!(apply_event(&mut session, "mode.diagonal", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([0.0, 0.0, 0.0]))));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([2.0, 3.0, 0.0]))));
        let (event_kind, payload) = parse_repl_line("SetHeight2.5", Some(&session.state)).expect("parsed line");
        assert!(apply_event(&mut session, &event_kind, payload.as_ref()));
        assert!(apply_event(&mut session, "confirm", None));
        assert!(can_commit(&session));
    }
}
