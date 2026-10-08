//! 🗑️ `deleteSelection`: deletes the given entities, or the current selection of either domain when none are given. Each entity is removed by the `delete-*` mutation its table row
//! names, in one gesture and so one history row: an entity whose container is deleted too leaves with the container's cascade, elements go before the library entries they
//! use, and children go before their parents (reverse table order). Kinds without a delete mutation are skipped and reported only when nothing could be deleted.

use crate::editor::bim::entities::{kind_holding, ENTITIES};
use crate::editor::bim::interaction::{BIM_ELEMENT_DOMAIN, BIM_LIBRARY_DOMAIN};
use crate::editor::bim::kit::{fault, select_effect};
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {
    pub ids: Vec<String>,
}

/// 🪢️ Whether a container of `id` is itself among `ids`, so `id` leaves with that container's cascade.
fn covered(snapshot: &ModelSnapshot, id: &str, ids: &[String]) -> bool {
    let mut current = kind_holding(snapshot, id).and_then(|row| (row.parent)(snapshot, id));
    for _ in 0..ENTITIES.len() {
        let Some(parent) = current else { return false };
        if ids.contains(&parent) {
            return true;
        }
        current = kind_holding(snapshot, &parent).and_then(|row| (row.parent)(snapshot, &parent));
    }
    false
}

/// 🗑️ The delete mutations for `ids`; the ids whose kind has no delete mutation.
pub fn plan(snapshot: &ModelSnapshot, ids: &[String]) -> (Vec<ModelMutation>, Vec<String>) {
    let mut rows: Vec<((bool, std::cmp::Reverse<usize>), ModelMutation)> = Vec::new();
    let mut unsupported = Vec::new();
    for id in ids.iter().filter(|id| !covered(snapshot, id, ids)) {
        let Some(row) = kind_holding(snapshot, id) else { continue };
        match row.delete {
            Some(delete) => rows.push(((row.library, std::cmp::Reverse(ENTITIES.iter().position(|candidate| candidate.kind == row.kind).unwrap_or_default())), delete(id))),
            None => unsupported.push(id.clone()),
        }
    }
    rows.sort_by(|left, right| left.0.cmp(&right.0));
    (rows.into_iter().map(|(_, mutation)| mutation).collect(), unsupported)
}

pub fn handle(payload: &DeleteSelection, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let ids: Vec<String> = if payload.ids.is_empty() { ctx.selected.iter().chain(&ctx.library_selected).cloned().collect() } else { payload.ids.clone() };
    if ids.is_empty() {
        return Ok(Emit::default());
    }
    let (mutations, unsupported) = plan(doc.snapshot, &ids);
    if mutations.is_empty() {
        return Err(fault("bim.delete.unsupported", format!("no delete mutation exists yet for: {}", unsupported.join(", "))));
    }
    let mut emit = Emit::mutations(mutations);
    emit.effects.push(select_effect(BIM_ELEMENT_DOMAIN, &[], "replace"));
    emit.effects.push(select_effect(BIM_LIBRARY_DOMAIN, &[], "replace"));
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
