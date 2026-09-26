//! ⚖️ Every verb the architect editor declares honours its declaration — the framework's declared-verb
//! law (`artifact_app_laws::assert_declared_verbs_honour_their_declarations`) over this surface, on the
//! sample program the editor boots with.

use super::*;
use semio_framework_plugin::artifact_app_laws::{assert_declared_verbs_honour_their_declarations, declared_verb_agent_divergences, declared_verb_ever_wrote_document, declared_verb_probe};

/// ⚖️ LAW: all 22 declared verbs honour their declarations. The agent lane runs the same retained job the shell runs
/// and refuses by name (`interactive-job.agent-lane-uncarried`) the four whose result an agent transaction cannot carry
/// yet: the example switch (a host `LoadDocument`), the two exports (a file download) and the import picker (a host file
/// request) — routed to the MCP gateway (ticket 26/09/23 `wp-p8.md` § routed). The pin turns red when a carrier lands.
#[semio_framework_async_macros::async_test]
async fn every_declared_architect_verb_honours_its_declaration() {
    let probes = assert_declared_verbs_honour_their_declarations::<semio_framework_plugin::EditorApp<ArchitectPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(create_architect_app, None).await;
    assert_eq!(probes.len(), 22, "the architect editor declares twenty-two verbs");
    for verb in ["runAnalysis", "runReport"] {
        assert!(declared_verb_ever_wrote_document(declared_verb_probe(&probes, verb)), "{verb} stores its record in the program");
    }
    assert_eq!(declared_verb_agent_divergences(&probes), ["setActiveExample", "exportProgram", "exportRegistersCsv", "importProgramRequest"], "agent-lane divergences");
}
