//! 📜️ Norm command — `remove-item`.

use semio_s_artifact_norm_en1997_mutations::En1997Mutation;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "remove-item")]
pub struct RemoveItem {
    pub path: String,
    pub index: u32,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &RemoveItem, doc: &ArtifactView<'_, En1997Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1997Mutation, NoConfigMutation>, Fault> {
    semio_s_plugin_norm_app_surface::dispatch_remove_item(doc.snapshot, &payload.path, payload.index as usize, |document, edit| semio_s_artifact_norm_en1997_mutations::resolve_edit(document, edit))
}
//#endregion 🔖️Handler
