//! Norm command — `apply-remedy`.

use crate::op::Din4108Mutation;
use crate::Din4108Snapshot;
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
pub fn handle(payload: &ApplyRemedy, doc: &ArtifactView<'_, Din4108Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Din4108Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_apply_remedy::<crate::editor::din4108::Din4108Family, _>(doc.snapshot, &payload.check_id, payload.remedy_index as usize, |base, target| Din4108Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
