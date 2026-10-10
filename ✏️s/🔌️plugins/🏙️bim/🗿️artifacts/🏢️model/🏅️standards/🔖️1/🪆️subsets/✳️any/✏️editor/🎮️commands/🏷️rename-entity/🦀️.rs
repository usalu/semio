//! 🏷️ `renameEntity`: gives one entity a new name through the `rename-*` mutation its table row names.

use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::kit::fault;
use crate::editor::bim::BimDispatchCtx;
use crate::{ModelMutation, ModelSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use value_derive::{FromValue, ToValue};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "rename-entity")]
pub struct RenameEntity {
    pub id: String,
    pub name: String,
}

pub fn handle(payload: &RenameEntity, doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _ctx: &mut BimDispatchCtx) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let row = kind_holding(doc.snapshot, &payload.id).ok_or_else(|| fault("bim.rename.target-missing", format!("no entity '{}' to rename", payload.id)))?;
    let rename = row.rename.ok_or_else(|| fault("bim.rename.unsupported", format!("no rename mutation exists for '{}' yet", row.kind)))?;
    let mutation = rename(doc.snapshot, &payload.id, &payload.name).ok_or_else(|| fault("bim.rename.rejected", "the name is not valid"))?;
    Ok(Emit::mutations(vec![mutation]))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
