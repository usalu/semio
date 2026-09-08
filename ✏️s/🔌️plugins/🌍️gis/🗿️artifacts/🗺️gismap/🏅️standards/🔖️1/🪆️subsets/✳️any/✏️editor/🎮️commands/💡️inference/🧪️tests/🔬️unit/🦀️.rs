
use super::*;
use crate::editor::gis2d::Gis2dCommand;
use crate::editor::gis2d::testkit::{app, dispatch};

/// 💡️ Exactly one host-owned intent leaves the app, and it names nothing but the proposal kind.
#[semio_framework_async_macros::async_test]
async fn propose_bounds_region_emits_one_intent_and_no_document_state() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis2dCommand::ProposeBoundsRegion(propose_bounds_region::ProposeBoundsRegion {})).await;
    assert!(result.mutations.is_empty(), "an inference intent never mutates the document");
    assert_eq!(result.requested_effects.len(), 1);
    assert_eq!(result.requested_effects[0], Effect::RequestInferenceProposal { kind: InferenceProposalKind::GisMapBoundsRegion });
}

/// 🌐️ A Shell action never emits document operations — the registry's kind-discipline guard
/// rejects one that does.
#[semio_framework_async_macros::async_test]
async fn propose_bounds_region_is_a_shell_action_that_emits_no_operations() {
    let definition = crate::editor::gis2d::create_gis2d_app();
    let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "proposeBoundsRegion").expect("proposeBoundsRegion declared");
    assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Shell));
    let mut app = crate::editor::gis2d::testkit::app_with_registry().await;
    assert!(dispatch(&mut app, Gis2dCommand::ProposeBoundsRegion(propose_bounds_region::ProposeBoundsRegion {})).await.mutations.is_empty());
}
