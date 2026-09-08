
use super::*;
use crate::schema::mutations::connect_widgets::ConnectWidgets;
use crate::schema::mutations::create_widget::CreateWidget;
use crate::schema::mutations::delete_widget::DeleteWidget;
use crate::schema::mutations::disconnect_widgets::DisconnectWidgets;
use crate::schema::mutations::move_widgets::MoveWidgets;
use crate::schema::mutations::reorder_synapses::ReorderSynapses;
use crate::schema::mutations::reorder_widgets::ReorderWidgets;
use crate::schema::mutations::replace_widget::ReplaceWidget;
use crate::schema::mutations::update_synapse_endpoints::UpdateSynapseEndpoints;
use protocol::os_spr::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};
use semio_framework_artifact_flow_flow::{FlowLayoutEntry, Widget, WidgetLayout};

fn widget_note(id: &str) -> Widget {
    Widget::InputNote { id: id.into(), text: String::new() }
}
fn widget_slider(id: &str) -> Widget {
    Widget::InputSlider { id: id.into(), label: id.into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 }
}

fn apply(base: &FlowSnapshot, mutation: &FlowMutation) -> FlowSnapshot {
    <FlowMutation as Mutation<FlowSnapshot>>::diff(mutation, base).diff().apply(base).expect("valid mutation diff")
}

fn base_with_two_widgets() -> FlowSnapshot {
    let base = apply(&FlowSnapshot::default(), &FlowMutation::CreateWidget(CreateWidget { index: 0, widget: widget_note("w1") }));
    apply(&base, &FlowMutation::CreateWidget(CreateWidget { index: 1, widget: widget_slider("w2") }))
}

fn base_with_synapse() -> FlowSnapshot {
    let base = base_with_two_widgets();
    apply(&base, &FlowMutation::ConnectWidgets(ConnectWidgets { index: 0, id: "s1".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }))
}

//#region 🔖️OutcomeLaws
/// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
/// one `assert_missing_target_is_error`/Fatal check per verb family this facet implements
/// (create/delete/connect/disconnect/move/replace/reorder/update).
#[semio_framework_async_macros::async_test]
async fn create_widget_duplicate_id_is_fatal() {
    let base = base_with_two_widgets();
    let outcome = FlowMutation::CreateWidget(CreateWidget { index: 0, widget: widget_note("w1") }).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn delete_widget_missing_target_is_error() {
    let base = FlowSnapshot::default();
    assert_missing_target_is_error(&base, &FlowMutation::DeleteWidget(DeleteWidget { id: "ghost".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_widget_cascades_severed_synapses() {
    let base = base_with_synapse();
    let outcome = FlowMutation::DeleteWidget(DeleteWidget { id: "w1".into() }).diff(&base);
    assert!(
        outcome.messages().iter().any(|message| message.level == protocol::Severity::Info && message.code.0 == "mutation.cascade"),
        "deleting a widget that severs a synapse must carry an Info mutation.cascade message, got {:?}",
        outcome.messages()
    );
    assert!(outcome.diff().apply(&base).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn connect_widgets_missing_endpoint_is_error() {
    let base = base_with_two_widgets();
    assert_missing_target_is_error(&base, &FlowMutation::ConnectWidgets(ConnectWidgets { index: 0, id: "edge-99".into(), from: "ghost".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn connect_widgets_duplicate_id_is_fatal() {
    let base = base_with_synapse();
    let outcome = FlowMutation::ConnectWidgets(ConnectWidgets { index: 0, id: "s1".into(), from: "w2".into(), from_port: "out".into(), to: "w1".into(), to_port: "in".into() }).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn connect_widgets_parallel_is_no_op() {
    let base = base_with_synapse();
    let outcome = FlowMutation::ConnectWidgets(ConnectWidgets { index: 1, id: "s2".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }).diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    assert_eq!(outcome.diff(), &FlowDiff::default());
}

#[semio_framework_async_macros::async_test]
async fn disconnect_widgets_missing_target_is_error() {
    let base = base_with_two_widgets();
    assert_missing_target_is_error(&base, &FlowMutation::DisconnectWidgets(DisconnectWidgets { id: "ghost".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn move_widgets_missing_target_is_error() {
    let base = base_with_two_widgets();
    assert_missing_target_is_error(&base, &FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "ghost".into(), layout: Some(WidgetLayout { x: 1.0, y: 1.0 }) }] })).await;
}

#[semio_framework_async_macros::async_test]
async fn move_widgets_non_finite_is_fatal() {
    let base = base_with_two_widgets();
    let outcome = FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: f64::NAN, y: 0.0 }) }] }).diff(&base);
    assert_fatal_never_applies(&outcome).await;
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
}

#[semio_framework_async_macros::async_test]
async fn move_widgets_unchanged_is_no_op() {
    let base = base_with_two_widgets();
    let moved = apply(&base, &FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: 5.0, y: 5.0 }) }] }));
    let outcome = FlowMutation::MoveWidgets(MoveWidgets { entries: vec![FlowLayoutEntry { id: "w1".into(), layout: Some(WidgetLayout { x: 5.0, y: 5.0 }) }] }).diff(&moved);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
}

#[semio_framework_async_macros::async_test]
async fn replace_widget_missing_target_is_error() {
    let base = base_with_two_widgets();
    assert_missing_target_is_error(&base, &FlowMutation::ReplaceWidget(ReplaceWidget { id: "ghost".into(), widget: widget_note("ghost") })).await;
}

#[semio_framework_async_macros::async_test]
async fn replace_widget_unchanged_is_no_op() {
    let base = base_with_two_widgets();
    let outcome = FlowMutation::ReplaceWidget(ReplaceWidget { id: "w1".into(), widget: widget_note("w1") }).diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
}

#[semio_framework_async_macros::async_test]
async fn reorder_widgets_missing_target_is_error() {
    let base = base_with_two_widgets();
    assert_missing_target_is_error(&base, &FlowMutation::ReorderWidgets(ReorderWidgets { id: "ghost".into(), to_index: 0 })).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_widgets_already_current_is_no_op() {
    let base = base_with_two_widgets();
    let outcome = FlowMutation::ReorderWidgets(ReorderWidgets { id: "w1".into(), to_index: 0 }).diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
}

#[semio_framework_async_macros::async_test]
async fn reorder_synapses_missing_target_is_error() {
    let base = base_with_synapse();
    assert_missing_target_is_error(&base, &FlowMutation::ReorderSynapses(ReorderSynapses { id: "ghost".into(), to_index: 0 })).await;
}

#[semio_framework_async_macros::async_test]
async fn reorder_synapses_already_current_is_no_op() {
    let base = base_with_synapse();
    let outcome = FlowMutation::ReorderSynapses(ReorderSynapses { id: "s1".into(), to_index: 0 }).diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
}

#[semio_framework_async_macros::async_test]
async fn update_synapse_endpoints_missing_target_is_error() {
    let base = base_with_synapse();
    assert_missing_target_is_error(&base, &FlowMutation::UpdateSynapseEndpoints(UpdateSynapseEndpoints { id: "ghost".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn update_synapse_endpoints_missing_endpoint_is_error() {
    let base = base_with_synapse();
    assert_missing_target_is_error(&base, &FlowMutation::UpdateSynapseEndpoints(UpdateSynapseEndpoints { id: "s1".into(), from: "ghost".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() })).await;
}

#[semio_framework_async_macros::async_test]
async fn update_synapse_endpoints_unchanged_is_no_op() {
    let base = base_with_synapse();
    let outcome = FlowMutation::UpdateSynapseEndpoints(UpdateSynapseEndpoints { id: "s1".into(), from: "w1".into(), from_port: "out".into(), to: "w2".into(), to_port: "in".into() }).diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
}
//#endregion 🔖️OutcomeLaws

//#region 🔖️KindsCatalog
/// 🏷️ [`KINDS`] is the bridge between this enum and the language-neutral test platform, which
/// never parses Rust. This proves it names every variant, in declaration order, with the same
/// kebab spelling `#[derive(protocol::Mutations)]` derives — and that this subset's own committed
/// catalog declares exactly the same set, so the completeness gate cannot be measuring a
/// vocabulary that has drifted away from the code.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let declared: Vec<&str> = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
    assert_eq!(KINDS, declared.as_slice(), "KINDS must name every FlowMutation variant, in declaration order, spelled as its own MutationKind::SEMANTICS.kind");
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in this subset's committed oracle manifest catalog flow-1-any");
    }
}
//#endregion 🔖️KindsCatalog
