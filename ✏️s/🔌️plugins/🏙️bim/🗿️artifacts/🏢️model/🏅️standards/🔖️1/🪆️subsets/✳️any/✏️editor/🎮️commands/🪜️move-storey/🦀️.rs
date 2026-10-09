//! 🪜️ `storeyUp` and `storeyDown`: stand the given elements, or the selected ones, on the storey above or below their own in the same building, one `set-element-storey` each, in one gesture and so one
//! history row. They are the keyboard and menu counterpart of dropping an outliner row on a storey. Anything that stands on no storey (an opening follows its host) is skipped; with no movable target or no
//! storey beyond one of them the command is refused whole instead of moving part of the targets.

use crate::editor::bim::entities::{fields_of, kind_holding, ordered_storeys};
use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::mutations::set_element_storey::SetElementStorey;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

/// 🪜️ A command that moves its targets by a number of storeys: positive is up, negative down.
pub trait Shifts {
    const STEPS: i32;
    fn ids(&self) -> &[String];
}

macro_rules! shifts {
    ($($payload:ident => $keyword:literal, $steps:literal;)+) => {
        $(
            #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
            #[dsl(keyword = $keyword)]
            pub struct $payload {
                pub ids: Vec<String>,
            }

            impl Shifts for $payload {
                const STEPS: i32 = $steps;
                fn ids(&self) -> &[String] {
                    &self.ids
                }
            }
        )+
    };
}

shifts! {
    StoreyUp => "storey-up", 1;
    StoreyDown => "storey-down", -1;
}

/// 🪜️ The storey an element stands on, for the kinds whose table row has a writable `storey` field.
pub fn storey_of(snapshot: &ModelSnapshot, id: &str) -> Option<String> {
    let field = fields_of(kind_holding(snapshot, id)?).find(|field| field.key == "storey" && field.write.is_some())?;
    (field.read)(snapshot, id)
}

/// 🪜️ The storey `steps` levels above (positive) or below (negative) `storey`, in the level order of its building.
pub fn neighbour(snapshot: &ModelSnapshot, storey: &str, steps: i32) -> Option<String> {
    let building = &snapshot.storeys.get(storey)?.building;
    let order = ordered_storeys(snapshot, building);
    let at = order.iter().position(|candidate| candidate == storey)?;
    usize::try_from(i64::try_from(at).ok()? + i64::from(steps)).ok().and_then(|to| order.get(to)).cloned()
}

fn shift(snapshot: &ModelSnapshot, ids: &[String], selected: &[String], steps: i32) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let wanted = if ids.is_empty() { selected } else { ids };
    let movable: Vec<(&String, String)> = wanted.iter().filter_map(|id| storey_of(snapshot, id).map(|storey| (id, storey))).collect();
    if movable.is_empty() {
        return Err(fault("bim.storey.target-missing", "no element that stands on a storey among the targets"));
    }
    let mut moves = Vec::with_capacity(movable.len());
    for (id, storey) in movable {
        let to = neighbour(snapshot, &storey, steps).ok_or_else(|| fault("bim.storey.no-neighbour", format!("'{id}' stands on '{storey}' and there is no storey beyond it")))?;
        moves.push(ModelMutation::SetElementStorey(SetElementStorey { id: id.clone(), storey: to }));
    }
    Ok(Emit::mutations(moves))
}

pub fn handle<P: Shifts>(payload: &P, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    shift(doc.snapshot, payload.ids(), &ctx.selected, P::STEPS)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
