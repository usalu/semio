//! Norm command — `apply-remedy`.

use crate::op::Din16798Mutation;
use crate::Din16798Snapshot;
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
pub fn handle(payload: &ApplyRemedy, doc: &ArtifactView<'_, Din16798Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Din16798Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_apply_remedy::<crate::editor::din16798::DinEn16798Family, _>(doc.snapshot, &payload.check_id, payload.remedy_index as usize, |base, target| Din16798Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
