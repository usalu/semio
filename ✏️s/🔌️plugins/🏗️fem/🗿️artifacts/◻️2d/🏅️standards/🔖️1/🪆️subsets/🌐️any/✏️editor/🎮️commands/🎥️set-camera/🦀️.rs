//! 🎥️ 🎥️ Fem2d play app commands command — `set-camera`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::fem2d::modes::edit::windows::{model, results};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::Viewport2d;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️SetCamera
//#endregion 🔖️SetCamera

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "camera")]
pub struct SetCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

pub fn handle(_payload: &SetCamera, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem2d.camera.window-context-required"))
}

pub fn handle_window(payload: &SetCamera, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let camera = Viewport2d { x: payload.x, y: payload.y, zoom: payload.zoom };
    camera.validate().map_err(|_| Fault::from("fem2d.camera.invalid"))?;
    let id = view.window_id.as_deref().ok_or_else(|| Fault::from("fem2d.camera.window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| Fault::from("fem2d.camera.window-stale"))?;
    let mutation = match kind {
        model::WINDOW_KIND_ID => {
            let mut next = model::config::current(cfg);
            next.camera = camera;
            model::config::addressed(view, next)?
        }
        results::WINDOW_KIND_ID => {
            let mut next = results::config::current(cfg);
            next.camera = camera;
            results::config::addressed(view, next)?
        }
        _ => return Err(Fault::from("fem2d.camera.window-kind")),
    };
    Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
