//! Norm command — `apply-remedy`.

use crate::standards::v1::subsets::any::schema::mutations::En1996Mutation;
use crate::En1996Snapshot;
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
pub fn handle(payload: &ApplyRemedy, doc: &ArtifactView<'_, En1996Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1996Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_apply_remedy::<crate::editor::en1996::En1996Family, _>(doc.snapshot, &payload.check_id, payload.remedy_index as usize, |document, edit| crate::mutations::EDIT_RULES.resolve(document, edit))
}
//#endregion 🔖️Handler
