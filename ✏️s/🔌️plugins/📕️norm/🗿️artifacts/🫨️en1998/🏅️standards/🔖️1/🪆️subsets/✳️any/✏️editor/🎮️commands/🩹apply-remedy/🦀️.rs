//! 📜️ Norm command — `apply-remedy` (the shared remedy resolution coerces Exactly→bool when the target leaf is boolean).

use crate::En1998Family;
use semio_s_artifact_norm_en1998_mutations::En1998Mutation;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "apply-remedy")]
pub struct ApplyRemedy {
    pub check_id: String,
    pub remedy_index: u32,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ApplyRemedy, doc: &ArtifactView<'_, En1998Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1998Mutation, NoConfigMutation>, Fault> {
    semio_s_plugin_norm_app_surface::dispatch_apply_remedy::<En1998Family, _>(doc.snapshot, &payload.check_id, payload.remedy_index as usize, |document, edit| semio_s_artifact_norm_en1998_mutations::EDIT_RULES.resolve(document, edit))
}
//#endregion 🔖️Handler
