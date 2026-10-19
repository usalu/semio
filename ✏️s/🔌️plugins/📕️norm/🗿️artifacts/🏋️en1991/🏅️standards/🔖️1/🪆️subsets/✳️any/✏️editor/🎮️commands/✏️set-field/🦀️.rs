//! 📜️ Norm command — `set-field`.

use semio_s_artifact_norm_en1991_mutations::En1991Mutation;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "set-field")]
pub struct SetField {
    pub path: String,
    pub value_json: String,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &SetField, doc: &ArtifactView<'_, En1991Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1991Mutation, NoConfigMutation>, Fault> {
    let path = crate::field_meta::flatten_editor_path(&payload.path);
    semio_s_plugin_norm_app_surface::dispatch_set_field(doc.snapshot, &path, &payload.value_json, |document, edit| semio_s_artifact_norm_en1991_mutations::EDIT_RULES.resolve(document, edit))
}
//#endregion 🔖️Handler
