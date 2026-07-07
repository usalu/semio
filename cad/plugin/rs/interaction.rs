//! 🎮 CAD interaction statechart — ports premigration `InteractionRuntime` for wgpu play engagement.

use cad_document::{CadObject, CadPaneId, CadPrimitiveSlot};
use kernel_3d_brepkit::BrepkitKernel;
use kernel_3d_engine::BrepKernel;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖Types
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
    pub id: &'static str,
    pub label: &'static str,
    pub key: &'static str,
    pub model_definition_id: &'static str,
    pub produces_typology: &'static str,
}

const INTERACTION_CATALOG: &[InteractionCatalogEntry] = &[
    InteractionCatalogEntry {
        id: "primitive.box",
        label: "Box",
        key: "b",
        model_definition_id: "spatial.shape",
        produces_typology: "spatial.shape.primitive.box",
    },
    InteractionCatalogEntry {
        id: "energy.energy.constructExternalWall",
        label: "External Wall",
        key: "e",
        model_definition_id: "aec.building.energy",
        produces_typology: "energy.energy.externalwall",
    },
    InteractionCatalogEntry {
        id: "structure.structure.constructOneWayReinforcedConcreteSlab",
        label: "Slab",
        key: "o",
        model_definition_id: "aec.building.structure.classic",
        produces_typology: "structure.structure.onewayreinforcedconcreteslab",
    },
    InteractionCatalogEntry {
        id: "structure.structure.constructReinforcedConcreteColumn",
        label: "Column",
        key: "r",
        model_definition_id: "aec.building.structure.classic",
        produces_typology: "structure.structure.reinforcedconcretecolumn",
    },
];
//#endregion 🔖Types

//#region 🔖Catalog
pub fn list_interactions_for_model_definition(model_definition_id: &str) -> Vec<&'static InteractionCatalogEntry> {
    INTERACTION_CATALOG
        .iter()
        .filter(|entry| entry.model_definition_id == model_definition_id)
        .collect()
}

pub fn resolve_interaction_key(input: &str, model_definition_id: &str) -> Option<&'static InteractionCatalogEntry> {
    let trimmed = input.trim().to_lowercase();
    INTERACTION_CATALOG.iter().find(|entry| {
        entry.model_definition_id == model_definition_id
            && (entry.key == trimmed || entry.id.eq_ignore_ascii_case(&trimmed) || entry.id.ends_with(&format!(".{trimmed}")))
    })
}

pub fn interaction_by_id(id: &str) -> Option<&'static InteractionCatalogEntry> {
    INTERACTION_CATALOG.iter().find(|entry| entry.id == id)
}
//#endregion 🔖Catalog

//#region 🔖Statechart
fn vec3_json(point: [f64; 3]) -> Value {
    json!([point[0], point[1], point[2]])
}

fn parse_vec3(value: &Value) -> Option<[f64; 3]> {
    let array = value.as_array()?;
    if array.len() < 3 {
        return None;
    }
    Some([
        array[0].as_f64()?,
        array[1].as_f64()?,
        array[2].as_f64()?,
    ])
}

fn context_point(session: &CadEngagementSession, field: &str) -> Option<[f64; 3]> {
    session.context.get(field).and_then(parse_vec3)
}

pub fn start_session(interaction_id: &str, pane: CadPaneId) -> Option<CadEngagementSession> {
    let entry = interaction_by_id(interaction_id)?;
    let initial = match entry.id {
        "primitive.box" => "idle",
        "energy.energy.constructExternalWall" => "choose_mode",
        _ => "idle",
    };
    Some(CadEngagementSession {
        interaction_id: entry.id.into(),
        state: initial.into(),
        context: HashMap::new(),
        pane,
        last_response: None,
    })
}

pub fn keyed_transitions(session: &CadEngagementSession) -> Vec<KeyedTransition> {
    match (session.interaction_id.as_str(), session.state.as_str()) {
        ("primitive.box", "idle") => vec![KeyedTransition {
            key: "s".into(),
            label: "Start".into(),
            event_kind: "start".into(),
        }],
        ("energy.energy.constructExternalWall", "choose_mode") => vec![KeyedTransition {
            key: "1".into(),
            label: "2 points + height".into(),
            event_kind: "mode.2points".into(),
        }],
        ("structure.structure.constructOneWayReinforcedConcreteSlab", "idle") => vec![KeyedTransition {
            key: "s".into(),
            label: "Start".into(),
            event_kind: "start".into(),
        }],
        ("structure.structure.constructReinforcedConcreteColumn", "idle") => vec![KeyedTransition {
            key: "s".into(),
            label: "Start".into(),
            event_kind: "start".into(),
        }],
        _ => Vec::new(),
    }
}

pub fn can_commit(session: &CadEngagementSession) -> bool {
    match session.interaction_id.as_str() {
        "primitive.box" => session.state == "ready",
        "energy.energy.constructExternalWall" => session.state == "ready",
        "structure.structure.constructOneWayReinforcedConcreteSlab" => session.state == "ready",
        "structure.structure.constructReinforcedConcreteColumn" => session.state == "ready",
        _ => false,
    }
}

pub fn apply_event(session: &mut CadEngagementSession, event_kind: &str, payload: Option<&Value>) -> bool {
    let changed = match (session.interaction_id.as_str(), session.state.as_str(), event_kind) {
        ("primitive.box", "idle", "start") => {
            session.state = "first_corner".into();
            true
        }
        ("primitive.box", "first_corner", "pointer.down") => {
            if let Some(point) = payload.and_then(parse_vec3) {
                session.context.insert("origin".into(), vec3_json(point));
                session.state = "diagonal_rubber".into();
                true
            } else {
                false
            }
        }
        ("primitive.box", "diagonal_rubber", "pointer.down") => {
            if let Some(point) = payload.and_then(parse_vec3) {
                session.context.insert("cornerB".into(), vec3_json(point));
                session.state = "first_corner_height".into();
                true
            } else {
                false
            }
        }
        ("primitive.box", "first_corner_height", "set.height") => {
            if let Some(height) = payload.and_then(|value| value.as_f64()) {
                session.context.insert("height".into(), json!(height));
                session.state = "ready".into();
                true
            } else {
                false
            }
        }
        ("primitive.box", "ready", "confirm") => true,
        ("energy.energy.constructExternalWall", "choose_mode", "mode.2points") => {
            session.state = "two_points_first".into();
            true
        }
        ("energy.energy.constructExternalWall", "two_points_first", "pointer.down") => {
            if let Some(point) = payload.and_then(parse_vec3) {
                session.context.insert("cornerA".into(), vec3_json(point));
                session.state = "two_points_second".into();
                true
            } else {
                false
            }
        }
        ("energy.energy.constructExternalWall", "two_points_second", "pointer.down") => {
            if let Some(point) = payload.and_then(parse_vec3) {
                session.context.insert("cornerB".into(), vec3_json(point));
                session.state = "two_points_height".into();
                true
            } else {
                false
            }
        }
        ("energy.energy.constructExternalWall", "two_points_height", "set.height") => {
            if let Some(height) = payload.and_then(|value| value.as_f64()) {
                session.context.insert("height".into(), json!(height));
                session.state = "ready".into();
                true
            } else {
                false
            }
        }
        ("structure.structure.constructOneWayReinforcedConcreteSlab", "idle", "start") => {
            session.state = "footprint_first".into();
            true
        }
        ("structure.structure.constructOneWayReinforcedConcreteSlab", "footprint_first", "pointer.down") => {
            if let Some(point) = payload.and_then(parse_vec3) {
                session.context.insert("cornerA".into(), vec3_json(point));
                session.state = "footprint_second".into();
                true
            } else {
                false
            }
        }
        ("structure.structure.constructOneWayReinforcedConcreteSlab", "footprint_second", "pointer.down") => {
            if let Some(point) = payload.and_then(parse_vec3) {
                session.context.insert("cornerB".into(), vec3_json(point));
                session.state = "slab_height".into();
                true
            } else {
                false
            }
        }
        ("structure.structure.constructOneWayReinforcedConcreteSlab", "slab_height", "set.height") => {
            if let Some(height) = payload.and_then(|value| value.as_f64()) {
                session.context.insert("height".into(), json!(height));
                session.state = "ready".into();
                true
            } else {
                false
            }
        }
        ("structure.structure.constructReinforcedConcreteColumn", "idle", "start") => {
            session.state = "column_base".into();
            true
        }
        ("structure.structure.constructReinforcedConcreteColumn", "column_base", "pointer.down") => {
            if let Some(point) = payload.and_then(parse_vec3) {
                session.context.insert("base".into(), vec3_json(point));
                session.state = "column_height".into();
                true
            } else {
                false
            }
        }
        ("structure.structure.constructReinforcedConcreteColumn", "column_height", "set.height") => {
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

pub fn parse_repl_line(line: &str) -> Option<(String, Option<Value>)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(rest) = trimmed.strip_prefix("set.height ") {
        return rest
            .trim()
            .parse::<f64>()
            .ok()
            .map(|height| ("set.height".into(), Some(json!(height))));
    }
    if let Some(rest) = trimmed.strip_prefix("dist ") {
        return rest
            .trim()
            .parse::<f64>()
            .ok()
            .map(|distance| ("set.distance".into(), Some(json!(distance))));
    }
    Some((trimmed.into(), None))
}

pub fn commit_object(
    kernel: &mut BrepkitKernel,
    session: &CadEngagementSession,
    label_count: usize,
    next_id: impl Fn(&str) -> String,
) -> Option<CadObject> {
    let entry = interaction_by_id(&session.interaction_id)?;
    match session.interaction_id.as_str() {
        "primitive.box" => {
            let origin = context_point(session, "origin")?;
            let corner_b = context_point(session, "cornerB")?;
            let height = session.context.get("height").and_then(|value| value.as_f64()).unwrap_or(1.0);
            let width = (corner_b[0] - origin[0]).abs().max(0.05);
            let depth = (corner_b[1] - origin[1]).abs().max(0.05);
            let solid = kernel.box_prim_sync(width, depth, height.max(0.05)).ok()?;
            Some(CadObject {
                id: next_id("object"),
                label: format!("Box {}", label_count + 1),
                typology: entry.produces_typology.into(),
                visible: true,
                locked: false,
                origin,
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: Some([width, depth, height.max(0.05)]),
                solid_handle: Some(solid.0.clone()),
                primitives: vec![CadPrimitiveSlot {
                    slot: "solid".into(),
                    primitive_id: solid.0,
                    kind: "solid".into(),
                }],
            })
        }
        "energy.energy.constructExternalWall" => {
            let corner_a = context_point(session, "cornerA")?;
            let corner_b = context_point(session, "cornerB")?;
            let height = session.context.get("height").and_then(|value| value.as_f64()).unwrap_or(3.0);
            let width = ((corner_b[0] - corner_a[0]).powi(2) + (corner_b[1] - corner_a[1]).powi(2)).sqrt().max(0.5);
            let solid = kernel.box_prim_sync(width, 0.2, height.max(0.05)).ok()?;
            Some(CadObject {
                id: next_id("object"),
                label: format!("External Wall {}", label_count + 1),
                typology: entry.produces_typology.into(),
                visible: true,
                locked: false,
                origin: corner_a,
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: Some([width, 0.2, height.max(0.05)]),
                solid_handle: Some(solid.0.clone()),
                primitives: vec![CadPrimitiveSlot {
                    slot: "solid".into(),
                    primitive_id: solid.0,
                    kind: "solid".into(),
                }],
            })
        }
        "structure.structure.constructOneWayReinforcedConcreteSlab" => {
            let corner_a = context_point(session, "cornerA")?;
            let corner_b = context_point(session, "cornerB")?;
            let height = session.context.get("height").and_then(|value| value.as_f64()).unwrap_or(0.25);
            let width = (corner_b[0] - corner_a[0]).abs().max(0.5);
            let depth = (corner_b[1] - corner_a[1]).abs().max(0.5);
            let solid = kernel.box_prim_sync(width, depth, height.max(0.05)).ok()?;
            Some(CadObject {
                id: next_id("object"),
                label: format!("Slab {}", label_count + 1),
                typology: entry.produces_typology.into(),
                visible: true,
                locked: false,
                origin: corner_a,
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: Some([width, depth, height.max(0.05)]),
                solid_handle: Some(solid.0.clone()),
                primitives: vec![CadPrimitiveSlot {
                    slot: "solid".into(),
                    primitive_id: solid.0,
                    kind: "solid".into(),
                }],
            })
        }
        "structure.structure.constructReinforcedConcreteColumn" => {
            let base = context_point(session, "base")?;
            let height = session.context.get("height").and_then(|value| value.as_f64()).unwrap_or(3.0);
            let radius = 0.25;
            let solid = kernel.cylinder_prim_sync(radius, height.max(0.05)).ok()?;
            Some(CadObject {
                id: next_id("object"),
                label: format!("Column {}", label_count + 1),
                typology: entry.produces_typology.into(),
                visible: true,
                locked: false,
                origin: base,
                orientation: Some([0.0, 0.0, 0.0, 1.0]),
                scale: None,
                mesh_url: None,
                extent: Some([radius * 2.0, radius * 2.0, height.max(0.05)]),
                solid_handle: Some(solid.0.clone()),
                primitives: vec![CadPrimitiveSlot {
                    slot: "solid".into(),
                    primitive_id: solid.0,
                    kind: "solid".into(),
                }],
            })
        }
        _ => None,
    }
}

pub fn preview_display_items(session: &CadEngagementSession) -> Vec<Value> {
    match (session.interaction_id.as_str(), session.state.as_str()) {
        ("primitive.box", "diagonal_rubber" | "first_corner_height" | "ready") => {
            let mut items = Vec::new();
            if let Some(origin) = context_point(session, "origin") {
                items.push(json!({ "kind": "point", "role": "origin", "position": origin }));
            }
            if let Some(corner_b) = context_point(session, "cornerB") {
                items.push(json!({ "kind": "box-preview", "role": "preview", "cornerA": context_point(session, "origin"), "cornerB": corner_b }));
            }
            items
        }
        ("energy.energy.constructExternalWall", "two_points_second" | "two_points_height" | "ready") => {
            let mut items = Vec::new();
            if let Some(corner_a) = context_point(session, "cornerA") {
                items.push(json!({ "kind": "point", "role": "cornerA", "position": corner_a }));
            }
            if let (Some(corner_a), Some(corner_b)) = (context_point(session, "cornerA"), context_point(session, "cornerB")) {
                items.push(json!({ "kind": "segment", "role": "footprint", "from": corner_a, "to": corner_b }));
            }
            items
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖Statechart

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_interaction_commits_after_height() {
        let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
        assert!(apply_event(&mut session, "start", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([0.0, 0.0, 0.0]))));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([2.0, 3.0, 0.0]))));
        assert!(apply_event(&mut session, "set.height", Some(&json!(2.5))));
        assert!(can_commit(&session));
        let mut kernel = BrepkitKernel::new();
        let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
        assert!(object.is_some());
        assert_eq!(object.unwrap().typology, "spatial.shape.primitive.box");
    }

    #[test]
    fn slab_interaction_commits() {
        let mut session =
            start_session("structure.structure.constructOneWayReinforcedConcreteSlab", CadPaneId::StructureClassic)
                .expect("session");
        assert!(apply_event(&mut session, "start", None));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([0.0, 0.0, 0.0]))));
        assert!(apply_event(&mut session, "pointer.down", Some(&json!([4.0, 5.0, 0.0]))));
        assert!(apply_event(&mut session, "set.height", Some(&json!(0.3))));
        let mut kernel = BrepkitKernel::new();
        let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
        assert!(object.is_some());
    }
}
