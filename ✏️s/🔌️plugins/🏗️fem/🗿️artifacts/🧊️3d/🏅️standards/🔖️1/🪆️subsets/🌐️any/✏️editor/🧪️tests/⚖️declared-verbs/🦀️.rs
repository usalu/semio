//! ⚖️ Every verb the FEM 3D editor declares honours its declaration — the framework's declared-verb law (`artifact_app_laws::assert_declared_verbs_honour_their_declarations`) over this surface, booted on the demo example with the invocations of `🧫️fixtures/⚖️declared-verb-examples/🔣️.json`: no verb of the Model Actions pane is a silent no-op, and each acts or refuses by name.

use semio_plugin::artifact_app_laws::assert_declared_verbs_honour_their_declarations;
use semio_plugin::artifact_app_laws::DeclaredVerbOutcome;
use semio_s_artifact_fem_3d_editor::editor::fem3d::create_fem3d_app;
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dPlayApp;

/// ⚖️ LAW (F5): every declared verb either changes the document, publishes an effect, or is refused with a named fault; `addCombination` and `addSolid` act on their documented inputs.
#[semio_framework_async_macros::async_test]
async fn every_declared_fem3d_verb_acts_or_refuses_by_name() {
    semio_s_artifact_fem_3d_editor_session::editor::fem3d::session::initialize();
    let probes = assert_declared_verbs_honour_their_declarations::<semio_plugin::EditorApp<Fem3dPlayApp>, semio_os_kernel::NoMembers>(create_fem3d_app, Some(include_str!("../🧫️fixtures/⚖️declared-verb-examples/🔣️.json")), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
    assert!(probes.len() >= 30, "declared verbs: {}", probes.len());
    for verb in ["addCombination", "addSolid", "addBar", "addFrame", "addSupport", "addNodalLoad", "addMemberUdl", "addAreaLoad", "setSelfWeight"] {
        let probe = probes.iter().find(|probe| probe.verb == verb).unwrap_or_else(|| panic!("{verb} is declared"));
        assert!(probe.windows.iter().any(|window| matches!(&window.staged, DeclaredVerbOutcome::Settled(effect) if effect.touches_document())), "{verb} writes the document on its example: {:?}", probe.windows.iter().map(|window| &window.staged).collect::<Vec<_>>());
    }
}
