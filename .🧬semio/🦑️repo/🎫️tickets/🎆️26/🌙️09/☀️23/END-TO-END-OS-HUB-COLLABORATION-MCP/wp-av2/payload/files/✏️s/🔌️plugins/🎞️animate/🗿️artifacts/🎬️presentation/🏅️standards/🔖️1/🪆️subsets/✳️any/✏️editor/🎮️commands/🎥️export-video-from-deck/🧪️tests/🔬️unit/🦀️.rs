use super::*;
use crate::editor::animate::commands::seed_grid::SeedGrid;
use crate::editor::animate::unit_tests::context::{dispatch, dispatch_refusal, presentation_app_with_registry};
use crate::editor::animate::PresentationCommand;

/// ⚖️ LAW: the UI route runs the export (it is `Migrated`, never refused as `interactive-job.not-ui-safe`) and answers
/// exactly one `VideoRenderExport` whose program the host admits: one slide per tile, each tile's demo scene captured
/// for its 0.2 s at the Medium preset (4 frames at 15 fps), one still scene for the whole timeline, well inside the
/// single-effect byte ceiling — and no document mutation.
#[semio_framework_async_macros::async_test]
async fn export_video_from_deck_emits_the_decks_video_program() {
    let mut app = presentation_app_with_registry().await;
    dispatch(&mut app, PresentationCommand::SeedGrid(SeedGrid { rows: 2, columns: 2 })).await;
    let result = dispatch(&mut app, PresentationCommand::ExportVideoFromDeck(ExportVideoFromDeck { scene_json: String::new() })).await;
    assert!(result.mutations.is_empty(), "a video export is a host effect, not a document operation");
    match result.requested_effects.as_slice() {
        [Effect::VideoRenderExport { filename, program }] => {
            assert!(filename.ends_with(".mp4"), "{filename}");
            assert_eq!(program.validate(), Ok(()));
            assert_eq!((program.width, program.height, program.fps), (1280, 720, 15));
            assert_eq!(program.frame_count(), 16, "4 tiles x 4 captured frames");
            assert_eq!(program.duration_milliseconds(), 1067);
            assert_eq!(program.scenes.len(), 1, "every demo frame is the same still picture");
            assert!(video_program_packed_bytes(program) <= ANIMATE_VIDEO_PROGRAM_MAXIMUM_BYTES);
        }
        other => panic!("expected exactly one video render export, got {other:?}"),
    }
}

/// ⚖️ LAW: a stated scene that references no scene is refused with a named fault — never an empty video, never a
/// silent default.
#[semio_framework_async_macros::async_test]
async fn a_stated_scene_without_scene_hashes_is_refused_by_name() {
    let mut app = presentation_app_with_registry().await;
    let refused = dispatch_refusal(&mut app, PresentationCommand::ExportVideoFromDeck(ExportVideoFromDeck { scene_json: r#"{"schema":"animate.presentation.scene","title":"Empty","sections":[]}"#.into() })).await;
    assert!(format!("{refused:?}").contains("animate.video.export.no-scenes"), "{refused:?}");
    let malformed = dispatch_refusal(&mut app, PresentationCommand::ExportVideoFromDeck(ExportVideoFromDeck { scene_json: "{".into() })).await;
    assert!(format!("{malformed:?}").contains("animate.video.export.scene-json"), "{malformed:?}");
}

/// ⚖️ LAW: the file is named after the scene title.
#[test]
fn the_video_file_is_named_after_the_scene() {
    let mut scene = PresentationScene::empty("Hero Shot #2");
    assert_eq!(video_filename_for_scene(&scene), "hero-shot-2.mp4");
    scene.title = "  ".into();
    assert_eq!(video_filename_for_scene(&scene), "animate-presentation.mp4");
}
