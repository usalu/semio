//! Norm command — `apply-remedy`.

use crate::op::En1994Mutation;
use crate::En1994Snapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "apply-remedy")]
pub struct ApplyRemedy {
    pub check_id: String,
    pub remedy_index: u32,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
pub fn handle(payload: &ApplyRemedy, doc: &ArtifactView<'_, En1994Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1994Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_apply_remedy::<crate::editor::en1994::En1994Family, _>(doc.snapshot, &payload.check_id, payload.remedy_index as usize, |base, target| En1994Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
