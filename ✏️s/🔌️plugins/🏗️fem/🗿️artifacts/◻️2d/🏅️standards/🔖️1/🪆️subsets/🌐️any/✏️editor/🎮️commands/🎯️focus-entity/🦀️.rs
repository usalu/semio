//! 🎯️ Fem2d play app command — `focus-entity`: frames the addressed window's camera on one entity.

use crate::editor::fem2d::commands::set_camera::{self, SetCamera};
use crate::editor::fem2d::interaction::{fem2d_addressed_camera, fem2d_entity_model_point};
use crate::editor::fem2d::modes::edit::windows::model::screen_2d;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

type Fem2dSnapshot = crate::Fem2dSnapshot;

//#region 🔖️FocusEntity
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "focus-entity")]
pub struct FocusEntity {
    pub id: String,
}

pub fn handle(_payload: &FocusEntity, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    Err(Fault::from("fem2d.focus-entity.window-context-required"))
}

/// 🎯️ Centres the addressed window's camera on the entity — its own point for a node, the midpoint for
/// a member, the outline centroid for a region, the carrying node for a support or a nodal load — at
/// the zoom the window already has, written through the one `setCamera` window-config path.
pub fn handle_window(payload: &FocusEntity, doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: &semio_framework_plugin::ViewModel) -> Result<Emit<Fem2dMutation, NoConfigMutation>, Fault> {
    let camera = fem2d_addressed_camera(cfg, view, "fem2d.focus-entity")?;
    let point = fem2d_entity_model_point(doc.snapshot, &payload.id).ok_or_else(|| Fault::from("fem2d.focus-entity.unknown-entity"))?;
    let (x, y) = screen_2d(point.0, point.1);
    set_camera::handle_window(&SetCamera { x, y, zoom: camera.zoom }, cfg, view)
}
//#endregion 🔖️FocusEntity

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
