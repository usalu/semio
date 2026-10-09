//! 🗑️ `deleteSelection`: deletes the given entities, or the current selection of either domain when none are given, in one gesture and so one history row. Placed and structural elements
//! leave through `delete-elements` (one mutation per part of at most [`INVERSE_ROWS`] restored rows, so every part has a bounded inverse and any number of parts fits the store's
//! staged-row ceiling of a gesture); library entries and kinds outside the cascade leave through the `delete-*` mutation their table row names, elements before the library entries
//! they use. An entity whose container is deleted too leaves with the container's cascade. A selection whose parts exceed one gesture is streamed: the parts that fit are deleted
//! and the rest stays selected, so the next press continues. Kinds without a delete mutation are skipped and reported only when nothing could be deleted.

use crate::editor::bim::entities::{kind_holding, ENTITIES};
use crate::editor::bim::interaction::{BIM_ELEMENT_DOMAIN, BIM_LIBRARY_DOMAIN};
use crate::editor::bim::kit::{fault, select_effect};
use crate::editor::bim::BimDispatchCtx;
use crate::mutations::cascade::{closure, INVERSE_ROWS};
use crate::mutations::delete_elements::DeleteElements;
use crate::{ModelMutation, ModelSnapshot};
use protocol::OutcomeCode;
use std::collections::HashSet;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {
    pub ids: Vec<String>,
}

/// 🗑️ What one press deletes: the mutations of the gesture, the ids that stay selected for the next press and the ids whose kind has no delete mutation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Plan {
    pub mutations: Vec<ModelMutation>,
    pub remaining: Vec<String>,
    pub unsupported: Vec<String>,
}

/// 🪢️ Whether a container of `id` is itself among `ids`, so `id` leaves with that container's cascade.
fn covered(snapshot: &ModelSnapshot, id: &str, ids: &HashSet<&str>) -> bool {
    let mut current = kind_holding(snapshot, id).and_then(|row| (row.parent)(snapshot, id));
    for _ in 0..ENTITIES.len() {
        let Some(parent) = current else { return false };
        if ids.contains(parent.as_str()) {
            return true;
        }
        current = kind_holding(snapshot, &parent).and_then(|row| (row.parent)(snapshot, &parent));
    }
    false
}

/// 🌊️ Whether the cascade knows `id`: its record lives in a collection `delete-elements` removes.
fn cascades(snapshot: &ModelSnapshot, id: &str) -> bool {
    !matches!(closure(snapshot, std::slice::from_ref(id)), Err(refusal) if refusal.code == OutcomeCode::TargetMissing)
}

/// 🔢️ How many rows restoring `ids` takes; a refused removal counts one row, the mutation reports its own refusal.
fn rows(snapshot: &ModelSnapshot, ids: &[String]) -> usize {
    closure(snapshot, ids).map_or(1, |removal| removal.rows(snapshot))
}

/// 🧩️ `ids` cut into parts whose removal restores at most [`INVERSE_ROWS`] rows each (halving a part until it fits). An id whose own removal is larger is replaced by its contents (everything
/// the cascade takes except the id itself and the openings that leave with their hosts) and returned in `oversize`, to stay selected until its contents are gone.
fn parts(snapshot: &ModelSnapshot, ids: &[String]) -> (Vec<Vec<String>>, Vec<String>) {
    let (mut parts, mut oversize) = (Vec::new(), Vec::new());
    let mut queue: Vec<Vec<String>> = vec![ids.to_vec()];
    while let Some(part) = queue.pop() {
        if part.is_empty() {
            continue;
        }
        if rows(snapshot, &part) <= INVERSE_ROWS {
            parts.push(part);
        } else if part.len() > 1 {
            let (first, last) = part.split_at(part.len() / 2);
            queue.push(last.to_vec());
            queue.push(first.to_vec());
        } else {
            let id = &part[0];
            let contents: Vec<String> = closure(snapshot, &part).map(|removal| removal.ids().into_iter().filter(|content| content != id && !snapshot.openings.contains_key(content)).collect()).unwrap_or_default();
            if contents.is_empty() {
                parts.push(part);
            } else {
                oversize.push(id.clone());
                queue.push(contents);
            }
        }
    }
    (parts, oversize)
}

/// 🗑️ The press for `ids`: the parts and library deletes that fit one gesture, in the order elements, kinds outside the cascade, library entries.
pub fn plan(snapshot: &ModelSnapshot, ids: &[String]) -> Plan {
    let mut plan = Plan::default();
    let (mut bulk, mut single): (Vec<String>, Vec<((bool, std::cmp::Reverse<usize>), ModelMutation, String)>) = (Vec::new(), Vec::new());
    let listed: HashSet<&str> = ids.iter().map(String::as_str).collect();
    for id in ids.iter().filter(|id| !covered(snapshot, id, &listed)) {
        let Some(row) = kind_holding(snapshot, id) else { continue };
        match row.delete {
            Some(_) if !row.library && cascades(snapshot, id) => bulk.push(id.clone()),
            Some(delete) => single.push(((row.library, std::cmp::Reverse(ENTITIES.iter().position(|candidate| candidate.kind == row.kind).unwrap_or_default())), delete(id), id.clone())),
            None => plan.unsupported.push(id.clone()),
        }
    }
    single.sort_by(|left, right| left.0.cmp(&right.0));
    let fits = |batch: &[ModelMutation]| store::ArtifactStoreOneItemFootprint::for_gesture::<ModelSnapshot, ModelMutation>(batch).is_ok();
    let (chunks, oversize) = if bulk.is_empty() { (Vec::new(), Vec::new()) } else { parts(snapshot, &bulk) };
    plan.remaining.extend(oversize);
    for chunk in chunks {
        plan.mutations.push(ModelMutation::DeleteElements(DeleteElements { ids: chunk.clone() }));
        if !fits(&plan.mutations) {
            plan.mutations.pop();
            plan.remaining.extend(chunk);
        }
    }
    for (_, mutation, id) in single {
        plan.mutations.push(mutation);
        if !fits(&plan.mutations) {
            plan.mutations.pop();
            plan.remaining.push(id);
        }
    }
    plan
}

pub fn handle(payload: &DeleteSelection, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let ids: Vec<String> = if payload.ids.is_empty() { ctx.selected.iter().chain(&ctx.library_selected).cloned().collect() } else { payload.ids.clone() };
    if ids.is_empty() {
        return Ok(Emit::default());
    }
    let plan = plan(doc.snapshot, &ids);
    if plan.mutations.is_empty() {
        return Err(fault("bim.delete.unsupported", format!("no delete mutation exists yet for: {}", plan.unsupported.join(", "))));
    }
    let held = |library: bool| -> Vec<(String, String)> { plan.remaining.iter().filter_map(|id| kind_holding(doc.snapshot, id).filter(|row| row.library == library).map(|row| (row.kind.to_string(), id.clone()))).collect() };
    let mut emit = Emit::mutations(plan.mutations.clone());
    emit.effects.push(select_effect(BIM_ELEMENT_DOMAIN, &held(false), "replace"));
    emit.effects.push(select_effect(BIM_LIBRARY_DOMAIN, &held(true), "replace"));
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
