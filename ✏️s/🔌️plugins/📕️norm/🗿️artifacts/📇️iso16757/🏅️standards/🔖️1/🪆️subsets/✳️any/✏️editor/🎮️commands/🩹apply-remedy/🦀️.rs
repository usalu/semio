//! Norm command — `apply-remedy`.

use crate::op::Iso16757Mutation;
use crate::Iso16757Snapshot;
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
pub fn handle(payload: &ApplyRemedy, doc: &ArtifactView<'_, Iso16757Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Iso16757Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_apply_remedy::<crate::editor::iso16757::Iso16757Family, _>(doc.snapshot, &payload.check_id, payload.remedy_index as usize, |base, target| Iso16757Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
