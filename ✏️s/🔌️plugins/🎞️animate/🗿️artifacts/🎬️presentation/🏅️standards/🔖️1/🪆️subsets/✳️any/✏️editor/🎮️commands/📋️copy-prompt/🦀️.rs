//! 🐚️ 🐚️ Animate presentation app commands command — `copy-prompt`.

#![allow(clippy::result_large_err)]

use crate::op::PresentationMutation;
use crate::PresentationSnapshot;
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::{tile_morph_prompt_effect, PresentationDispatchCtx};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CopyPrompt
//#endregion 🔖️CopyPrompt

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "copy-prompt")]
pub struct CopyPrompt {}

pub fn handle(_payload: &CopyPrompt, doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    Ok(Emit { effects: vec![tile_morph_prompt_effect(doc.snapshot)], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
