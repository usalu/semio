//! 📋️ `editSchedule`: one edit of a schedule definition from the schedule window (name, category, a column, sort key, filter, grouping level, itemization, a storey or a phase of the scope). The edit is a pure
//! function of the definition (see the window's `edit` node) and becomes one `set-schedule` mutation, which decides whether the result stands; the command never writes a row, a total or a cell.

use crate::editor::bim::kit::fault;
use crate::editor::bim::modes::edit::windows::schedule::edit::{apply, Edit};
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "edit-schedule")]
pub struct EditSchedule {
    pub id: String,
    pub part: String,
    pub op: String,
    pub key: String,
    pub value: String,
}

pub fn handle(payload: &EditSchedule, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let edit = Edit { part: payload.part.clone(), op: payload.op.clone(), key: payload.key.clone(), value: payload.value.clone() };
    let mutation = apply(doc.snapshot, &payload.id, &edit).map_err(|code| fault(code, format!("the schedule '{}' cannot take the edit '{} {}'", payload.id, payload.part, payload.op)))?;
    Ok(Emit::mutations(vec![ModelMutation::SetSchedule(mutation)]))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
