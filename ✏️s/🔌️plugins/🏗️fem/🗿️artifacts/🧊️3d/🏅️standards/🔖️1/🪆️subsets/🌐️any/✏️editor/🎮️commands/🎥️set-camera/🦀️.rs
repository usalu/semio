//! 🎥️ 🎥️ FEM 3D app commands command — `set-camera`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::fem3d::modes::edit::windows::{model, results};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::{Fem3dSnapshot, Viewport3dOrbit};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    #[dsl(block)]
    pub camera: Viewport3dOrbit,
}

pub fn handle(_payload: &SetCamera, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem3d.camera.window-context-required"))
}

pub fn handle_window(payload: &SetCamera, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    payload.camera.validate().map_err(|error| Fault::from(error.to_string()))?;
    let camera = payload.camera;
    let id = view.window_id.as_deref().ok_or_else(|| Fault::from("fem3d.camera.window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| Fault::from("fem3d.camera.window-stale"))?;
    let mutation = match kind {
        model::FEM3D_WINDOW_MODEL => {
            let mut next = model::config::current(cfg);
            next.camera = camera;
            model::config::addressed(view, next)?
        }
        results::FEM3D_WINDOW_RESULTS => {
            let mut next = results::config::current(cfg);
            next.camera = camera;
            results::config::addressed(view, next)?
        }
        _ => return Err(Fault::from("fem3d.camera.window-kind")),
    };
    Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
