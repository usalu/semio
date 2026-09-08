//! ☑️ VDI 3805 play app command — point the inspection panel at a different computed check.
//!
//! 📌️ Config-only: it emits `config_mutations`, never document operations — the selected row is view
//! state, not compliance content. Declared as a `view_action`, so the registry's kind discipline
//! actively rejects it if it ever starts emitting document operations.

use crate::op::Vdi3805Mutation;
use crate::Vdi3805Snapshot;
use crate::config::{NormConfig, NormConfigMutation};
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
pub fn handle(payload: &SetSelectedCheckIndex, _doc: &ArtifactView<'_, Vdi3805Snapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Vdi3805Mutation, NormConfigMutation>, Fault> {
    crate::app_surface::commit_selected_check_index::<Vdi3805Mutation>(payload.index)
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
