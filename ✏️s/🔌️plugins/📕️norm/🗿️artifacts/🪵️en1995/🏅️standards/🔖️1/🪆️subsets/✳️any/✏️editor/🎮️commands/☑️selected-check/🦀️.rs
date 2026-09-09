//! ☑️ EN 1995 play app command — point the inspection panel at a different computed check.
//!
//! 📌️ Config-only: it emits `config_mutations`, never document operations — the selected row is view
//! state, not compliance content. Declared as a `view_action`, so the registry's kind discipline
//! actively rejects it if it ever starts emitting document operations.

use crate::config::{NormConfig, NormConfigMutation};
use crate::op::En1995Mutation;
use crate::En1995Snapshot;
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
pub fn handle(payload: &SetSelectedCheckIndex, _doc: &ArtifactView<'_, En1995Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1995Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_selected_check_index::<En1995Mutation>(payload.index)
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
