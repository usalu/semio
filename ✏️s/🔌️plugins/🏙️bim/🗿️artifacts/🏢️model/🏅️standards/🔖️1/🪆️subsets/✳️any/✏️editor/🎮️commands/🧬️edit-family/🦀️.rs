//! 🧬️ `editFamily`: one edit of a family from the family window (its name or category, a parameter, a solid or one formula of a solid). The edit is a pure function of the family (see the window's `edit` node)
//! and becomes the `set-family`, `set-family-parameter`, `remove-family-parameter`, `create-family-solid`, `set-family-solid` or `delete-family-solid` mutation that carries exactly the change; the mutation decides
//! whether the result stands (a formula that parses, names only existing parameters and closes no circle). The command never writes a value, a mesh or an outline.

use crate::editor::bim::entities::id_taken;
use crate::editor::bim::kit::{fault, IdMint};
use crate::editor::bim::modes::edit::windows::family::edit::{apply, Edit};
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "edit-family")]
pub struct EditFamily {
    pub id: String,
    pub part: String,
    pub op: String,
    pub key: String,
    pub value: String,
}

pub fn handle(payload: &EditFamily, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = doc.snapshot;
    let edit = Edit { part: payload.part.clone(), op: payload.op.clone(), key: payload.key.clone(), value: payload.value.clone() };
    let fresh = IdMint::new(doc.operation_optional()).mint("family-solid", |id| id_taken(snapshot, id));
    let mutations = apply(snapshot, &payload.id, &edit, &fresh).map_err(|code| fault(code, format!("the family '{}' cannot take the edit '{} {}'", payload.id, payload.part, payload.op)))?;
    Ok(Emit::mutations(mutations))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
