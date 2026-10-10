//! 🔃️ `flipWalls`: reverses the direction of the given walls, or of the selected walls, through one `flip-wall` mutation each, in one gesture and so one history row. Anything that
//! is not a wall is skipped; with no wall among the targets the command is refused instead of doing nothing silently.

use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "flip-walls")]
pub struct FlipWalls {
    pub ids: Vec<String>,
}

/// 🧱️ The walls among the explicit ids, else among the selection.
pub fn walls_of<'a>(snapshot: &ModelSnapshot, ids: &'a [String], selected: &'a [String]) -> Vec<&'a String> {
    let wanted = if ids.is_empty() { selected } else { ids };
    wanted.iter().filter(|id| snapshot.walls.contains_key(id.as_str())).collect()
}

pub fn handle(payload: &FlipWalls, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let walls = walls_of(doc.snapshot, &payload.ids, &ctx.selected);
    if walls.is_empty() {
        return Err(fault("bim.flip.wall-missing", "no wall to flip among the targets"));
    }
    Ok(Emit::mutations(walls.into_iter().map(|id| ModelMutation::FlipWall(crate::mutations::flip_wall::FlipWall { id: id.clone() })).collect()))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
