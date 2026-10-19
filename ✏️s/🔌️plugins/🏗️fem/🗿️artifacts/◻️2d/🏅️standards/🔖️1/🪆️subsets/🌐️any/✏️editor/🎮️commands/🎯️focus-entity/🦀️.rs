//! 🎯️ Fem2d play app command — `focus-entity`: frames the addressed window's camera on one entity.

use crate::editor::fem2d::commands::set_camera::self;

use crate::editor::fem2d::commands::set_camera::SetCamera;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_entity_model_point;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::screen_2d;
use crate::editor::fem2d::modes::edit::fem2d_addressed_camera;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

type Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️FocusEntity
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
