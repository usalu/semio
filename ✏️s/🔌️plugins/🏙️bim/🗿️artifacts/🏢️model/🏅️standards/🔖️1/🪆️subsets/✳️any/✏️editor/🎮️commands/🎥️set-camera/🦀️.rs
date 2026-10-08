//! 🎥️ `setCamera`: stores the navigation pose of the addressed window in that window's own config, so each window keeps its own pose and a reopened window finds it again.
//! The plan and section windows take a 2D pose, the world window an orbit pose; a plan camera is also shared as presence.

use crate::editor::bim::kit::fault;
use crate::editor::bim::modes::edit::windows::{plan, section, world};
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera2d: Option<store::Viewport2d>,
    #[dsl(block)]
    pub camera3d: Option<store::Viewport3dOrbit>,
}

pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let view = ctx.view.clone().ok_or_else(|| fault("bim.camera.window-required", "a camera belongs to one open window"))?;
    let invalid = |error: semio_framework_value::ValueError| fault("bim.camera.invalid", error.to_string());
    let mutation = match (ctx.window_kind.as_str(), payload.camera2d, payload.camera3d) {
        (plan::WINDOW_KIND_ID, Some(camera), _) => {
            camera.validate().map_err(invalid)?;
            ctx.presence_out.push(ctx.presence.looking_through(camera));
            plan::config::addressed(&view, plan::config::BimPlanWindowConfig { viewport: camera, framed: true, ..ctx.plan.clone() })?
        }
        (section::WINDOW_KIND_ID, Some(camera), _) => {
            camera.validate().map_err(invalid)?;
            section::config::addressed(&view, section::config::BimSectionWindowConfig { viewport: camera, framed: true, ..ctx.section.clone() })?
        }
        (world::WINDOW_KIND_ID, _, Some(camera)) => {
            camera.validate().map_err(invalid)?;
            world::config::addressed(&view, world::config::BimWorldWindowConfig { camera, framed: true, ..ctx.world.clone() })?
        }
        (window, _, _) => return Err(fault("bim.camera.pose-mismatch", format!("the window '{window}' takes no such camera pose"))),
    };
    let mut emit = Emit::default();
    emit.window_config_mutations.push(mutation);
    Ok(emit)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
