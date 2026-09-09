//! 🧬️ 🧬️ Wires play app commands command — `set-active-example`.

use crate::empty_wires_snapshot;
use crate::op::WiresMutation;
use crate::schema::metabolism_wires_example_snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧬️ Manifest `.example` id for the metabolism fixture — shared by `SetActiveExample`'s payload check
/// and `crate::editor::wires::create_wires_app`'s `.example(...)` registration.
pub const WIRES_PLAY_EXAMPLE_METABOLISM_ID: &str = "metabolism";

//#region 🔖️SetActiveExample
//#endregion 🔖️SetActiveExample

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🧬️ Whole-document replace has no in-history mutation (a whole-snapshot variant is banned
/// outright — see `📓️taxonomy.md`'s forbidden vocabulary), so loading a named example builds
/// `editor::wires::reset_wires_document_effect` (a `Effect::LoadDocument`, outside undo history)
/// instead of an `artifact_mutations` entry.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, crate::WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    let next = if payload.example_id.as_str() == WIRES_PLAY_EXAMPLE_METABOLISM_ID {
        metabolism_wires_example_snapshot().map_err(|error| {
            let message = if error.target.is_empty() { error.message.clone() } else { format!("{} at {}", error.message, error.target.join(".")) };
            Fault::new(FaultOrigin::App, FaultCode::new(error.code), message)
        })?
    } else {
        empty_wires_snapshot()
    };
    Ok(Emit { effects: vec![crate::editor::wires::reset_wires_document_effect(&next)], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
