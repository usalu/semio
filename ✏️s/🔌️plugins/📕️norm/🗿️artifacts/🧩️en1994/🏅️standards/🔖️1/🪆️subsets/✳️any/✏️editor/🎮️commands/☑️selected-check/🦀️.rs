//! ☑️ EN 1994 play app command — point the inspection panel at a different computed check.
//!
//! 📌️ The editor router emits one exact Results `window_config_mutation`; this payload handler keeps the application lanes empty — the selected row is view
//! state, not compliance content. Declared as a `view_action`, so the registry's kind discipline
//! actively rejects it if it ever starts emitting document operations.

use crate::results_window_config::{ChangeSelectedCheckIndex, NormResultsWindowConfigMutation};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::En1994Mutation;
use crate::En1994Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "selected-check")]
pub struct SetSelectedCheckIndex {
    /// 👁️ `None` means "the first check" — the same fallback `crate::app_surface::render_inspection` applies.
    pub index: Option<u32>,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn window_mutation(payload: &SetSelectedCheckIndex) -> NormResultsWindowConfigMutation {
    ChangeSelectedCheckIndex { index: payload.index }.into()
}

pub fn handle(_payload: &SetSelectedCheckIndex, _doc: &ArtifactView<'_, En1994Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1994Mutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
