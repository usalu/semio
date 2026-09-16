//! 🎯️ Fem3d play app command — `focus-entity`: aims the addressed window's orbit at one entity.

use crate::editor::fem3d::commands::set_camera::{self, SetCamera};
use crate::editor::fem3d::interaction::fem3d_entity_point;
use crate::editor::fem3d::modes::edit::windows::{model, results};
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::Viewport3dOrbit;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem3dSnapshot = crate::Fem3dSnapshot;

//#region 🔖️FocusEntity
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "focus-entity")]
pub struct FocusEntity {
    pub id: String,
}

pub fn handle(_payload: &FocusEntity, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem3d.focus-entity.window-context-required"))
}

/// 🎥️ The orbit the addressed World3d window is currently drawn with — both window kinds keep it
/// in their own persisted window config.
pub fn addressed_camera(cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Viewport3dOrbit, Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| Fault::from("fem3d.focus-entity.window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| Fault::from("fem3d.focus-entity.window-stale"))?;
    match kind {
        model::FEM3D_WINDOW_MODEL => Ok(model::config::current(cfg).camera),
        results::FEM3D_WINDOW_RESULTS => Ok(results::config::current(cfg).camera),
        _ => Err(Fault::from("fem3d.focus-entity.window-kind")),
    }
}

/// 🎯️ Re-aims the orbit at the entity — its own point for a node, the midpoint for a member, the
/// centroid for a solid, the carrying node for a support or a nodal load — carrying the current
/// eye offset along, so the view slides over rather than jumps, written through the one `setCamera`
/// window-config path.
pub fn handle_window(payload: &FocusEntity, doc: &ArtifactView<'_, Fem3dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let camera = addressed_camera(cfg, view)?;
    let target = fem3d_entity_point(doc.snapshot, &payload.id).ok_or_else(|| Fault::from("fem3d.focus-entity.unknown-entity"))?;
    set_camera::handle_window(&SetCamera { camera: focused_orbit(&camera, target) }, cfg, view)
}

/// 🎯️ The orbit `camera` becomes when it is re-aimed at `target`: the eye keeps its offset from the
/// old target, zoom and up are untouched.
pub fn focused_orbit(camera: &Viewport3dOrbit, target: [f64; 3]) -> Viewport3dOrbit {
    let offset = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
    Viewport3dOrbit { position: [target[0] + offset[0], target[1] + offset[1], target[2] + offset[2]], target, zoom: camera.zoom, up: camera.up }
}
//#endregion 🔖️FocusEntity

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
