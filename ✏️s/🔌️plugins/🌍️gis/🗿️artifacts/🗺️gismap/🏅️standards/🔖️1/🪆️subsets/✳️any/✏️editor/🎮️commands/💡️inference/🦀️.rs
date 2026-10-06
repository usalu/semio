//! 💡️ GIS 2D play app command — the Shell-kind effect that asks the host to open its own ephemeral
//! inference port and offer one reviewable bounds-region proposal.
//!
//! This is deliberately NOT a document command: it writes no `GisMapMutation`, holds no job state,
//! names no model/provider/transport, and carries no document, space, request or credential
//! identity. Everything the lifecycle needs — the scope, the idempotency key, the execution-target
//! lease precondition, the progress cursor, the proposal hash, and the Cancel/Approve controls —
//! is host-owned. The proposal itself only ever reaches this artifact through the hub's
//! server-stamped approval command, never through this effect.

use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use crate::GisMapSnapshot;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 💡️ProposeBoundsRegion
pub mod propose_bounds_region {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
    #[dsl(keyword = "propose-bounds-region")]
    pub struct ProposeBoundsRegion {}

    pub fn handle(_payload: &ProposeBoundsRegion, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<GisMapMutation, NoConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestServiceOperation { owner: "gis".into(), service_id: "s.gis.gismap.inference".into(), action: "propose".into(), payload: semio_framework::DslValue::Object(Vec::new()) }))
    }
}
//#endregion 💡️ProposeBoundsRegion

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
