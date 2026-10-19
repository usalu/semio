//! 🏷️ `renameEntity`: gives one entity a new name through the `rename-*` mutation its table row names.

use semio_s_artifact_bim_model_entities::editor::bim::entities::kind_holding;
use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
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


