//! 🪟️ `setView`: sets one named view parameter of the addressed window, persisted in that window's own config: the plan's storey and cut height, the world's projection, storey
//! isolation and visibility and section plane, the section's line and depth. It never touches the document. Choosing the working storey is also shared as presence.

use crate::editor::bim::kit::fault;
use crate::editor::bim::modes::edit::windows::{plan, section, world};
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "set-view")]
pub struct SetView {
    pub field: String,
    pub value: String,
    pub pressed: Option<bool>,
}

fn number(payload: &SetView) -> Result<f64, Fault> {
    payload.value.trim().parse::<f64>().ok().filter(|value| value.is_finite()).ok_or_else(|| fault("bim.view.value-invalid", format!("'{}' is not a number", payload.value)))
}

fn flag(payload: &SetView) -> Result<bool, Fault> {
    payload.pressed.map_or_else(|| payload.value.trim().parse::<bool>().map_err(|_| fault("bim.view.value-invalid", format!("'{}' is not a flag", payload.value))), Ok)
}

fn unknown(window: &str, field: &str) -> Fault {
    fault("bim.view.field-unknown", format!("the window '{window}' has no view parameter '{field}'"))
}

fn plan_view(payload: &SetView, snapshot: &ModelSnapshot, ctx: &mut BimDispatchCtx) -> Result<plan::config::BimPlanWindowConfig, Fault> {
    let mut config = ctx.plan.clone();
    match payload.field.as_str() {
        "storey" => {
            if !payload.value.is_empty() && !snapshot.storeys.contains_key(&payload.value) {
                return Err(fault("bim.view.storey-missing", format!("no storey '{}'", payload.value)));
            }
            config.storey = payload.value.clone();
            config.framed = false;
            ctx.presence_out.push(ctx.presence.on_storey(&payload.value));
        }
        "cut_height" => config.cut_height = number(payload)?.clamp(0.0, 10.0),
        other => return Err(unknown(plan::WINDOW_KIND_ID, other)),
    }
    Ok(config)
}

fn world_view(payload: &SetView, snapshot: &ModelSnapshot, ctx: &BimDispatchCtx) -> Result<world::config::BimWorldWindowConfig, Fault> {
    let mut config = ctx.world.clone();
    match payload.field.as_str() {
        "projection" => {
            config.projection.kind = payload.value.clone();
            config.framed = false;
        }
        "isolated_storey" => {
            if !payload.value.is_empty() && !snapshot.storeys.contains_key(&payload.value) {
                return Err(fault("bim.view.storey-missing", format!("no storey '{}'", payload.value)));
            }
            config.isolated_storey = payload.value.clone();
        }
        "hidden_storey" => {
            let hide = payload.pressed.unwrap_or_else(|| !config.hidden_storeys.contains(&payload.value));
            config.hidden_storeys.retain(|storey| storey != &payload.value);
            if hide {
                config.hidden_storeys.push(payload.value.clone());
            }
        }
        "section_enabled" => config.section_enabled = flag(payload)?,
        "section_axis" if matches!(payload.value.as_str(), "x" | "y" | "z") => config.section_axis = payload.value.clone(),
        "section_axis" => return Err(fault("bim.view.value-invalid", format!("'{}' is not an axis", payload.value))),
        "section_offset" => config.section_offset = number(payload)?,
        other => return Err(unknown(world::WINDOW_KIND_ID, other)),
    }
    Ok(config)
}

fn section_view(payload: &SetView, ctx: &BimDispatchCtx) -> Result<section::config::BimSectionWindowConfig, Fault> {
    let mut config = ctx.section.clone();
    match payload.field.as_str() {
        "line" => {
            let parts: Vec<f64> = payload.value.split(',').filter_map(|part| part.trim().parse::<f64>().ok()).collect();
            match parts.as_slice() {
                [start_x, start_y, end_x, end_y] => (config.start_x, config.start_y, config.end_x, config.end_y, config.framed) = (*start_x, *start_y, *end_x, *end_y, false),
                _ => return Err(fault("bim.view.value-invalid", "a section line is 'x1, y1, x2, y2'")),
            }
        }
        "depth" => config.depth = number(payload)?.max(0.0),
        other => return Err(unknown(section::WINDOW_KIND_ID, other)),
    }
    Ok(config)
}

pub fn handle(payload: &SetView, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let view = ctx.view.clone().ok_or_else(|| fault("bim.view.window-required", "a view parameter belongs to one open window"))?;
    let mut emit = Emit::default();
    let mutation = match ctx.window_kind.as_str() {
        plan::WINDOW_KIND_ID => plan::config::addressed(&view, plan_view(payload, doc.snapshot, ctx)?)?,
        world::WINDOW_KIND_ID => world::config::addressed(&view, world_view(payload, doc.snapshot, ctx)?)?,
        section::WINDOW_KIND_ID => section::config::addressed(&view, section_view(payload, ctx)?)?,
        other => return Err(fault("bim.view.window-unsupported", format!("the window '{other}' has no view parameters"))),
    };
    emit.window_config_mutations.push(mutation);
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
