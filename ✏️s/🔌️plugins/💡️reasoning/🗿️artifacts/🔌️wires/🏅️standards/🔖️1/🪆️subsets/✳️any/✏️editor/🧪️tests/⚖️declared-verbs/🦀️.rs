//! ⚖️ Every verb the wires editor declares honours its declaration — the framework's declared-verb law
//! (`artifact_app_laws::assert_declared_verbs_honour_their_declarations`) over this surface, with the
//! example invocations of `🧫️fixtures/⚖️declared-verb-examples.json` on the metabolism boot example.

use super::*;
use semio_framework_plugin::artifact_app_laws::{assert_declared_verbs_honour_their_declarations, declared_verb_agent_divergences, declared_verb_probe, declared_verb_staged_wrote_document, declared_verb_wrote_document};

#[semio_framework_async_macros::async_test]
async fn every_declared_wires_verb_honours_its_declaration() {
    let examples = include_str!("../../🧫️fixtures/⚖️declared-verb-examples.json");
    let probes = assert_declared_verbs_honour_their_declarations::<EditorApp<ReasoningWiresPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(create_wires_app, Some(examples)).await;
    assert_eq!(probes.len(), 8, "the wires editor declares eight verbs");
    let relate = declared_verb_probe(&probes, "addRelationship");
    assert!(declared_verb_staged_wrote_document(relate), "the example relationship is written: {:?}", relate.windows);
    let canvas = &relate.windows[0];
    assert_eq!(canvas.arguments.iter().map(|argument| argument.argument.as_str()).collect::<Vec<_>>(), ["sourceId", "targetId", "kind"], "every declared argument is perturbed");
    assert!(declared_verb_wrote_document(&canvas.arguments[2].first) && declared_verb_wrote_document(&canvas.arguments[2].second), "both kinds write an edge");
    assert_eq!(declared_verb_agent_divergences(&probes), ["setActiveExample"], "the agent lane previews no operation for a LoadDocument example switch");
}
