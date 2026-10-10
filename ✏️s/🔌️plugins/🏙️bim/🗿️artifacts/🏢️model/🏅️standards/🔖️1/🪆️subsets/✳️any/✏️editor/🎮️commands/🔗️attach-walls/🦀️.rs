//! 🔗️ `attachWalls`: attaches the top of the given walls, or of the selected walls, to the underside of a roof, slab or ceiling through one `set-wall-top` mutation each, in one gesture and so one history row. The surface is the
//! one named by `target`, else the first roof, slab or ceiling of the selection; the offset stays what the wall already has over a surface (zero for a free top). Anything that is not a wall is skipped; with no wall among the targets, or
//! no surface to attach to, the command is refused instead of doing nothing silently.

use crate::editor::bim::entities::wall_sweeps::write_top_attach;
use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "attach-walls")]
pub struct AttachWalls {
    pub ids: Vec<String>,
    pub target: String,
}

/// 🔝️ Whether `id` is a roof, a slab or a ceiling: a surface the top of a wall can follow.
pub fn is_surface(snapshot: &ModelSnapshot, id: &str) -> bool {
    snapshot.roofs.contains_key(id) || snapshot.slabs.contains_key(id) || snapshot.ceilings.contains_key(id)
}

/// 🔝️ The surface to attach to: the explicit target, else the first roof, slab or ceiling among the ids, else among the selection.
pub fn target_of<'a>(snapshot: &ModelSnapshot, target: &'a str, ids: &'a [String], selected: &'a [String]) -> Option<&'a str> {
    Some(target).filter(|target| !target.is_empty()).or_else(|| ids.iter().chain(selected).map(String::as_str).find(|id| is_surface(snapshot, id))).filter(|target| is_surface(snapshot, target))
}

pub fn handle(payload: &AttachWalls, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let wanted = if payload.ids.is_empty() { &ctx.selected } else { &payload.ids };
    let walls: Vec<&String> = wanted.iter().filter(|id| snapshot.walls.contains_key(id.as_str())).collect();
    if walls.is_empty() {
        return Err(fault("bim.attach.wall-missing", "no wall to attach among the targets"));
    }
    let target = target_of(snapshot, &payload.target, wanted, &ctx.selected).ok_or_else(|| fault("bim.attach.target-missing", "no roof, slab or ceiling to attach the walls to"))?;
    Ok(Emit::mutations(walls.into_iter().filter_map(|id| write_top_attach(snapshot, id, target)).collect()))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
