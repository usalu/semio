//! 💡️ GIS 2D play app command — the Shell-kind effect that asks the host to open its own ephemeral
//! inference port and offer one reviewable bounds-region proposal.
//!
//! This is deliberately NOT a document command: it writes no `GisMapMutation`, holds no job state,
//! names no model/provider/transport, and carries no document, space, request or credential
//! identity. Everything the lifecycle needs — the scope, the idempotency key, the execution-target
//! lease precondition, the progress cursor, the proposal hash, and the Cancel/Approve controls —
//! is host-owned. The proposal itself only ever reaches this artifact through the hub's
//! server-stamped approval command, never through this effect.

use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use crate::editor::gis2d::config::{Gis2dConfig, Gis2dConfigMutation};
use semio_framework_plugin::kernel::{Effect, InferenceProposalKind};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 💡️ProposeBoundsRegion
pub mod propose_bounds_region {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "propose-bounds-region")]
    pub struct ProposeBoundsRegion {}

    pub fn handle(_payload: &ProposeBoundsRegion, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestInferenceProposal { kind: InferenceProposalKind::GisMapBoundsRegion }))
    }
}
//#endregion 💡️ProposeBoundsRegion

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::testkit::{app, dispatch};
    use crate::editor::gis2d::Gis2dCommand;

    /// 💡️ Exactly one host-owned intent leaves the app, and it names nothing but the proposal kind.
    #[semio_framework_async_macros::async_test]
    async fn propose_bounds_region_emits_one_intent_and_no_document_state() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::ProposeBoundsRegion(propose_bounds_region::ProposeBoundsRegion {}));
        assert!(result.mutations.is_empty(), "an inference intent never mutates the document");
        assert_eq!(result.requested_effects.len(), 1);
        assert_eq!(result.requested_effects[0], Effect::RequestInferenceProposal { kind: InferenceProposalKind::GisMapBoundsRegion });
    }

    /// 🌐️ A Shell action never emits document operations — the registry's kind-discipline guard
    /// rejects one that does.
    #[semio_framework_async_macros::async_test]
    async fn propose_bounds_region_is_a_shell_action_that_emits_no_operations() {
        let definition = crate::editor::gis2d::create_gis2d_app().definition;
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "proposeBoundsRegion").expect("proposeBoundsRegion declared");
        assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Shell));
        let mut app = crate::editor::gis2d::testkit::app_with_registry();
        assert!(dispatch(&mut app, Gis2dCommand::ProposeBoundsRegion(propose_bounds_region::ProposeBoundsRegion {})).mutations.is_empty());
    }
}
//#endregion 🧪️Tests
