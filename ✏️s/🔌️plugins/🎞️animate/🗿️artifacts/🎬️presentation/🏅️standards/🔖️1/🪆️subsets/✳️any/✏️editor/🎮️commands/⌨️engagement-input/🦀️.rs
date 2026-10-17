//! ⌨️ ⌨️ Animate presentation app commands command — `engagement-input`.

#![allow(clippy::result_large_err)]

use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::support::PresentationDispatchCtx;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::PresentationSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "engagement-input")]
pub struct EngagementInput {
    pub value: String,
}

pub fn handle(payload: &EngagementInput, _doc: &ArtifactView<'_, PresentationSnapshot>, _cfg: &ConfigView<'_, PresentationConfig>, _ctx: &mut PresentationDispatchCtx) -> Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault> {
    Ok(Emit::config(vec![PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value: payload.value.clone() })]))
}

//#region 🧩️ArtifactItems9
pub mod artifact_items_9 {
pub use crate::{snapshot::schema::demo_snapshot, snapshot::schema::PresentationSnapshot};

/// 🎬️ The `demo` example document — see [`demo_snapshot`].
pub fn demo_presentation_snapshot() -> PresentationSnapshot {
    demo_snapshot()
}
}
//#endregion 🧩️ArtifactItems9
