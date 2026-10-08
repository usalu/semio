//! 🌐️ `worldPointerMove`: the ground point under the pointer in the 3D window while a drawing utility holds an engagement session; it moves the gesture's rubber band.

use crate::editor::bim::gestures::session::ToolEvent;
use crate::editor::bim::gestures::{run, Raw};
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(default)]
#[dsl(keyword = "world-pointer-move")]
pub struct WorldPointerMove {
    pub pane: String,
    pub position: Vec<f64>,
}

pub fn handle(payload: &WorldPointerMove, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let Some(ground) = crate::editor::bim::commands::world_pointer_down::ground_of(&payload.position) else { return Ok(Emit::default()) };
    run(ctx, doc, |pointer| ToolEvent::Move(*pointer), &Raw { ground: Some(ground), ..Raw::default() })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
