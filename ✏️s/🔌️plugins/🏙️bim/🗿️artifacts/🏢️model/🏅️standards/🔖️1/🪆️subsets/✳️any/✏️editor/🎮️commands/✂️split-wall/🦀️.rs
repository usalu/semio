//! ✂️ `splitWall`: splits the given walls, or the selected walls, at one fraction of their axis through one `split-wall` mutation each, in one gesture and so one history row. The
//! fraction is typed as `0.25`, `0,25` or `25%` and is empty for the middle; the new halves get ids minted from the authoring seed and the result selects both parts.

use crate::editor::bim::entities::id_taken;
use crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN;
use crate::editor::bim::kit::{fault, select_effect, IdMint};
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// 🔢️ The fraction a wall splits at when none is typed.
pub const MIDDLE: f64 = 0.5;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "split-wall-at")]
pub struct SplitWallAt {
    pub ids: Vec<String>,
    pub at: String,
}

/// 🔢️ The fraction a typed text means: empty is the middle, `25%` is a quarter, a decimal comma is accepted; only a value strictly between 0 and 1 counts.
pub fn fraction(text: &str) -> Option<f64> {
    let text = text.trim();
    if text.is_empty() {
        return Some(MIDDLE);
    }
    let (number, scale) = text.strip_suffix('%').map_or((text, 1.0), |rest| (rest, 0.01));
    let value = number.trim().replace(',', ".").parse::<f64>().ok()? * scale;
    (value.is_finite() && value > 0.0 && value < 1.0).then_some(value)
}

pub fn handle(payload: &SplitWallAt, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let t = fraction(&payload.at).ok_or_else(|| fault("bim.split.fraction-invalid", format!("'{}' is not a fraction strictly between 0 and 1", payload.at)))?;
    let walls = crate::editor::bim::commands::flip_walls::walls_of(snapshot, &payload.ids, &ctx.selected);
    if walls.is_empty() {
        return Err(fault("bim.split.target-missing", "no wall to split among the targets"));
    }
    let mut mint = IdMint::new(doc.operation_optional());
    let mut mutations = Vec::new();
    let mut parts = Vec::new();
    for wall in walls {
        let new_id = mint.mint("wall", |id| id_taken(snapshot, id));
        parts.push(("wall".to_string(), wall.clone()));
        parts.push(("wall".to_string(), new_id.clone()));
        mutations.push(ModelMutation::SplitWall(crate::mutations::split_wall::SplitWall { id: wall.clone(), t, new_id }));
    }
    let mut emit = Emit::mutations(mutations);
    emit.effects.push(select_effect(BIM_ELEMENT_DOMAIN, &parts, "replace"));
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
