//! 🏗️ `createEntity`: brings one entity of a kind into the model. The entity table owns what a new entity of the kind looks like and which `create-*` mutation brings it; this
//! command resolves its container (the explicit parent, else the selection, else the first container), mints its id from the authoring seed, names it and selects it.

use crate::editor::bim::entities::{id_taken, kind_of, ordered_storeys, EntityKind, ENTITIES};
use crate::editor::bim::interaction::{BIM_ELEMENT_DOMAIN, BIM_LIBRARY_DOMAIN};
use crate::editor::bim::kit::{fault, select_effect, IdMint};
use crate::editor::bim::terminology::BimLabels;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "create-entity")]
pub struct CreateEntity {
    pub kind: String,
    pub parent: String,
    pub name: String,
}

/// 🌳️ The kind of container an entity of `kind` is created in.
fn container_kind(kind: &str) -> Option<&'static str> {
    match kind {
        "building" => Some("site"),
        "storey" | "grid" => Some("building"),
        "opening" => Some("wall"),
        "site" | "material" | "wall-type" | "slab-type" | "roof-type" | "column-type" | "beam-type" | "window-type" | "door-type" => None,
        _ => Some("storey"),
    }
}

/// 🌳️ The container the new entity goes into: the explicit parent, else a selected container of the right kind, else the first one.
fn resolve_parent(snapshot: &ModelSnapshot, kind: &str, parent: &str, selected: &[String]) -> String {
    let Some(container) = container_kind(kind).and_then(kind_of) else { return String::new() };
    let holds = |id: &str| (container.name)(snapshot, id).is_some();
    if holds(parent) {
        return parent.to_string();
    }
    let selection = selected.iter().find(|id| holds(id)).cloned();
    let first = || if container.kind == "storey" { snapshot.buildings.keys().flat_map(|building| ordered_storeys(snapshot, building)).next() } else { (container.ids)(snapshot).into_iter().next() };
    selection.or_else(first).unwrap_or_default()
}

fn default_name(row: &EntityKind, labels: Option<&'static BimLabels>, count: usize) -> String {
    let kind = labels.map_or_else(|| row.kind.to_string(), |labels| (row.label)(labels).as_str().to_string());
    format!("{kind} {}", count + 1)
}

pub fn handle(payload: &CreateEntity, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let row = kind_of(&payload.kind).ok_or_else(|| fault("bim.create.kind-unknown", format!("'{}' is not an entity kind of the model", payload.kind)))?;
    let create = row.create.ok_or_else(|| fault("bim.create.unsupported", format!("no create mutation exists for '{}' yet", payload.kind)))?;
    let parent = resolve_parent(snapshot, row.kind, &payload.parent, &ctx.selected);
    let id = IdMint::new(doc.operation_optional()).mint(row.kind, |id| id_taken(snapshot, id));
    let name = if payload.name.is_empty() { default_name(row, ctx.labels(), (row.ids)(snapshot).len()) } else { payload.name.clone() };
    let mutation = create(snapshot, &id, &parent, &name).map_err(|code| fault(code, format!("a {} needs its container first", row.kind)))?;
    let domain = if row.library { BIM_LIBRARY_DOMAIN } else { BIM_ELEMENT_DOMAIN };
    let mut emit = Emit::mutations(vec![mutation]);
    emit.effects.push(select_effect(domain, &[(row.kind.to_string(), id)], "replace"));
    Ok(emit)
}

/// 🧱️ Every kind a `createEntity` can currently bring, for the panels' add rows.
pub fn creatable() -> Vec<&'static EntityKind> {
    ENTITIES.iter().filter(|row| row.create.is_some()).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
