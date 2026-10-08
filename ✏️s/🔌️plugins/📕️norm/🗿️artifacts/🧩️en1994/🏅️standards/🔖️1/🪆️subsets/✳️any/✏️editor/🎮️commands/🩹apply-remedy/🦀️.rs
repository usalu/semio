//! Norm command — `apply-remedy`.

use crate::standards::v1::subsets::any::schema::mutations::En1994Mutation;
use crate::En1994Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "apply-remedy")]
pub struct ApplyRemedy {
    pub check_id: String,
    pub remedy_index: u32,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ApplyRemedy, doc: &ArtifactView<'_, En1994Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1994Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_apply_remedy::<crate::editor::en1994::En1994Family, _>(doc.snapshot, &payload.check_id, payload.remedy_index as usize, |document, edit| crate::mutations::resolve_edit(document, edit))
}
//#endregion 🔖️Handler
