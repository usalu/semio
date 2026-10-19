//! 🎥️ `setCamera`: stores the navigation pose of the addressed window in that window's own config, so each window keeps its own pose and a reopened window finds it again.
//! The plan and section windows take a 2D pose, the world window an orbit pose; a plan camera is also shared as presence.

use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::{plan, section, world};
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
        (semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID, Some(camera), _) => {
            camera.validate().map_err(invalid)?;
            ctx.presence_out.push(ctx.presence.looking_through(camera));
            semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::config::addressed(&view, semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::config::BimPlanWindowConfig { viewport: camera, framed: true, ..ctx.plan.clone() })?
        }
        (semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::WINDOW_KIND_ID, Some(camera), _) => {
            camera.validate().map_err(invalid)?;
            semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::config::addressed(&view, semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::config::BimSectionWindowConfig { viewport: camera, framed: true, ..ctx.section.clone() })?
        }
        (semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID, _, Some(camera)) => {
            camera.validate().map_err(invalid)?;
            semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::config::addressed(&view, semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::config::BimWorldWindowConfig { camera, framed: true, ..ctx.world.clone() })?
        }
        (window, _, _) => return Err(fault("bim.camera.pose-mismatch", format!("the window '{window}' takes no such camera pose"))),
    };
    let mut emit = Emit::default();
    emit.window_config_mutations.push(mutation);
    Ok(emit)
}


