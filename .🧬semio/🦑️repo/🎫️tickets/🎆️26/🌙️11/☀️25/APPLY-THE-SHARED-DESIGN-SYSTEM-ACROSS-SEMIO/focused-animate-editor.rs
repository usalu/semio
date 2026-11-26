//! 🎞️ Exercise Animate's real mounted editor actions with only the focused native acceptance cases.
use semio_s_artifact_animate_presentation_animate::editor::animate::{
    AnimatePresentationPlayApp, PresentationCommand, create_animate_presentation_app,
    commands::{add_tile::AddTile, export_video_from_deck::ExportVideoFromDeck},
};
use semio_s_artifact_animate_presentation_animate::presentation_working_scene;
use semio_plugin::{App, EditorApp, Effect, InvocationResult, MountedOwnerPolicyV1, PluginApp, VcsArtifactApp};
use semio_plugin::artifact_app_laws::{
    OriginalFixtureScope, close_registered_fixture_app, dispatch_original_fixture_command,
    fixture_identity, meta, new_app_with_registry_and_members, settle_registered_typed_operation,
};
use semio_s_artifact_stdio_semio::SemioMembers;

type Mounted = VcsArtifactApp<EditorApp<AnimatePresentationPlayApp>, SemioMembers>;
const POLICY: MountedOwnerPolicyV1 = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;

fn manifest() -> App {
    App { definition: create_animate_presentation_app(), examples: Vec::new() }
}

async fn invoke(app: &mut Mounted, scope: &mut OriginalFixtureScope, command: PresentationCommand) -> InvocationResult {
    let view = meta("local");
    let mut identity = fixture_identity();
    let mut result = dispatch_original_fixture_command(app, &command, &view, POLICY, &mut identity, scope.cx()).await.expect("admitted action");
    let settled = settle_registered_typed_operation(app, view.instance_id, POLICY, &mut identity, scope.cx()).await.expect("settled action");
    result.requested_effects.extend(settled.effects);
    result
}

#[semio_framework_async_macros::async_test]
async fn mounted_editor_appends_distinct_tiles_and_exports_the_window_paint() {
    let mut scope = OriginalFixtureScope::admit();
    let actor = semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into());
    let mut app = new_app_with_registry_and_members::<EditorApp<AnimatePresentationPlayApp>, SemioMembers>(
        manifest, actor, POLICY, &mut fixture_identity()).await;
    app.bind_instance_id(meta("local").instance_id).await;
    for _ in 0..2 {
        invoke(&mut app, &mut scope, PresentationCommand::AddTile(AddTile { crop: None })).await;
    }
    let (_, tiles) = presentation_working_scene(&app.snapshot().expect("projection"));
    let exported = invoke(&mut app, &mut scope, PresentationCommand::ExportVideoFromDeck(ExportVideoFromDeck { scene_json: String::new() })).await;
    close_registered_fixture_app(&mut app, POLICY, scope.cx()).expect("paid close");
    assert_eq!(tiles.len(), 2);
    assert_ne!(tiles[0].id, tiles[1].id);
    assert!(exported.mutations.is_empty());
    let [Effect::VideoRenderExport { program, .. }] = exported.requested_effects.as_slice() else {
        panic!("expected one video export");
    };
    assert_eq!(program.validate(), Ok(()));
    assert_eq!((program.width, program.height, program.fps, program.frame_count()), (1280, 720, 15, 105));
    assert_eq!(program.background.map(|channel| (channel * 255.0).round() as u8), [6, 26, 31, 255]);
}
