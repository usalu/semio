//! 🖱️ 🖱️ Layout play app commands command — `canvas-pointer-up`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-pointer-up")]
pub struct CanvasPointerUp {}

pub fn handle(_payload: &CanvasPointerUp, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
