//! Norm command — `apply-remedy`.

use crate::op::En1993Mutation;
use crate::En1993Snapshot;
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
pub fn handle(payload: &ApplyRemedy, doc: &ArtifactView<'_, En1993Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1993Mutation, NoConfigMutation>, Fault> {
    crate::app_surface::dispatch_apply_remedy::<crate::editor::en1993::En1993Family, _>(doc.snapshot, &payload.check_id, payload.remedy_index as usize, |base, target| En1993Mutation::from_snapshot(base, target))
}
//#endregion 🔖️Handler
