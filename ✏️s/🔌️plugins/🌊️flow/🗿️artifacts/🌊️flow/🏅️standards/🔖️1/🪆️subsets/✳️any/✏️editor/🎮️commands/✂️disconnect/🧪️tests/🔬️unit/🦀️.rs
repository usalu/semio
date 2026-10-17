#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use crate::editor::flow::edit_rules::ContentEdit;
#[allow(unused_imports)]
use crate::editor::flow::host::{flow_composed_content, flow_content_leaves_emit};
#[allow(unused_imports)]
use crate::{FlowMutation, FlowSnapshot};
#[allow(unused_imports)]
use flow::FlowEvalSession;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::flow::commands::disconnect::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app, settle_refusal};
use crate::editor::flow::FlowCommand;

#[semio_framework_async_macros::async_test]
async fn disconnecting_an_unknown_synapse_is_refused_by_name() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::Disconnect(Disconnect { synapse_id: "nope".into() })).await;
    assert!(result.mutations.is_empty());
    let refusal = settle_refusal(&mut app).await;
    assert_eq!(refusal.code.0, "mutation.target-missing", "{refusal:?}");
    assert!(refusal.message.contains("\"nope\""), "{refusal:?}");
}
