//! Norm command — `apply-remedy` (the shared remedy resolution coerces Exactly→bool when the target leaf is boolean).

use crate::editor::en1998::En1998Family;
use crate::standards::v1::subsets::any::schema::mutations::En1998Mutation;
use crate::En1998Snapshot;
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
pub fn handle(payload: &ApplyRemedy, doc: &ArtifactView<'_, En1998Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1998Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_apply_remedy::<En1998Family, _>(doc.snapshot, &payload.check_id, payload.remedy_index as usize, |document, edit| crate::mutations::EDIT_RULES.resolve(document, edit))
}
//#endregion 🔖️Handler
