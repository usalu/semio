//! ⚖️ Every verb the flow editor declares honours its declaration — the framework's declared-verb law
//! (`artifact_app_laws::assert_declared_verbs_honour_their_declarations`) over this surface, booted on the demo
//! example with the invocations of `🧫️fixtures/⚖️declared-verb-examples.json`.

use super::*;
use semio_framework_plugin::artifact_app_laws::{assert_declared_verbs_honour_their_declarations, declared_verb_agent_divergences};

#[semio_framework_async_macros::async_test]
async fn every_declared_flow_verb_honours_its_declaration() {
    let probes = assert_declared_verbs_honour_their_declarations::<semio_framework_plugin::EditorApp<FlowPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(create_flow_app, Some(include_str!("../../🧫️fixtures/⚖️declared-verb-examples.json"))).await;
    assert_eq!(probes.len(), 0, "declared verbs");
    assert_eq!(declared_verb_agent_divergences(&probes), Vec::<String>::new(), "agent-lane divergences");
}
