//! 🌍️ `worldPointerDown`: a press in the 3D window while a drawing utility holds an engagement session: the host ray-casts the ground and sends the point. It is the click of the
//! click-click tools (walls, columns, beams, slabs, railings, stairs, spaces, grid lines, openings and the transforms); the pointer position is a plan point in metres.

use crate::editor::bim::gestures::session::{Modifiers, ToolEvent};
use crate::editor::bim::gestures::{run, Raw};
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(default)]
#[dsl(keyword = "world-pointer-down")]
pub struct WorldPointerDown {
    pub pane: String,
    pub position: Vec<f64>,
    pub shift_key: bool,
    pub ctrl_key: bool,
    pub meta_key: bool,
}

impl WorldPointerDown {
    pub fn raw(&self) -> Raw {
        Raw { modifiers: Modifiers { shift: self.shift_key, ctrl: self.ctrl_key, meta: self.meta_key }, ground: ground_of(&self.position), ..Raw::default() }
    }
}

/// 🌍️ The plan point of a ray-cast ground position `[x, y, z]`; none when the host sent no usable point.
pub fn ground_of(position: &[f64]) -> Option<[f64; 2]> {
    match position {
        [x, y, ..] if x.is_finite() && y.is_finite() => Some([*x, *y]),
        _ => None,
    }
}

pub fn handle(payload: &WorldPointerDown, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let raw = payload.raw();
    if raw.ground.is_none() {
        return Ok(Emit::default());
    }
    run(ctx, doc, |pointer| ToolEvent::Down(*pointer), &raw)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
